use super::common::*;
use std::sync::atomic::{AtomicUsize, Ordering};

// Worker count: `CKC_JOBS` (a positive integer), else the host's available parallelism.
pub(super) fn jobs() -> usize {
    std::env::var("CKC_JOBS")
        .ok()
        .and_then(|v| v.parse().ok())
        .filter(|&n: &usize| n > 0)
        .unwrap_or_else(|| std::thread::available_parallelism().map_or(1, |n| n.get()))
}

// `f` over `items` on `jobs()` workers: the results in item order, or the failure of
// the first failing item in item order — the one a sequential loop reports. Items
// are taken in order, and none past a failure already seen, so every item before
// the first failure has run.
pub(super) fn map<T: Sync, R: Send>(
    items: &[T],
    f: impl Fn(&T) -> Result<R> + Sync,
) -> Result<Vec<R>> {
    let next = AtomicUsize::new(0);
    let failed = AtomicUsize::new(usize::MAX);
    let work = || {
        let mut done = Vec::new();
        loop {
            let i = next.fetch_add(1, Ordering::SeqCst);
            if i >= items.len() || i > failed.load(Ordering::SeqCst) {
                return done;
            }
            let result = f(&items[i]);
            if result.is_err() {
                failed.fetch_min(i, Ordering::SeqCst);
            }
            done.push((i, result));
        }
    };
    let mut slots: Vec<Option<Result<R>>> = items.iter().map(|_| None).collect();
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..jobs().min(items.len()))
            .map(|_| scope.spawn(work))
            .collect();
        for worker in workers {
            let done = worker
                .join()
                .unwrap_or_else(|panic| std::panic::resume_unwind(panic));
            for (i, result) in done {
                slots[i] = Some(result);
            }
        }
    });
    let mut out = Vec::with_capacity(items.len());
    for slot in slots {
        match slot {
            Some(Ok(r)) => out.push(r),
            Some(Err(e)) => return Err(e),
            None => {
                return Err(fail(
                    "parallel",
                    "an item before the first failure did not run",
                ));
            }
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Every failing item and every item after it may have run; the reported failure
    // is the first in item order whatever the completion order.
    #[test]
    fn first_failure_in_item_order() {
        let items: Vec<u64> = (0..64).collect();
        let ok = map(&items, |&i| Ok(i * 2)).unwrap();
        assert_eq!(ok, items.iter().map(|i| i * 2).collect::<Vec<_>>());
        let late = |&i: &u64| {
            // Later failures finish first: item 40 sleeps longest.
            std::thread::sleep(std::time::Duration::from_millis(if i == 40 {
                50
            } else {
                1
            }));
            if i == 40 || i == 50 || i == 63 {
                Err(violation("probe", format!("item {i}")))
            } else {
                Ok(i)
            }
        };
        let err = map(&items, late).unwrap_err();
        assert_eq!(err.out, b"ckc: probe: item 40\n");
        let empty: Vec<u64> = Vec::new();
        assert!(map(&empty, |&i| Ok(i)).unwrap().is_empty());
    }
}
