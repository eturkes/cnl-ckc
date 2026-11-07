//! Test-only code-path plants; one selected case owns its thread-local switch.
use std::cell::RefCell;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Kind {
    HubTarget,
    Rectangle,
    Unary,
    Sign,
    Owner,
    Notification,
    MemoAlias,
    HashCollision,
    K1Support,
}
#[derive(Default)]
struct State {
    kind: Option<Kind>,
    consumed: bool,
    hits: usize,
}
thread_local! { static ACTIVE: RefCell<State> = const { RefCell::new(State { kind: None, consumed: false, hits: 0 }) }; }

pub(crate) struct Guard;
impl Guard {
    pub fn set(kind: Option<Kind>) -> Self {
        ACTIVE.with_borrow_mut(|s| {
            *s = State {
                kind,
                ..State::default()
            }
        });
        Self
    }
}
impl Drop for Guard {
    fn drop(&mut self) {
        ACTIVE.with_borrow_mut(|s| *s = State::default());
    }
}
pub(crate) fn enabled(kind: Kind) -> bool {
    ACTIVE.with_borrow(|s| s.kind == Some(kind))
}
pub(crate) fn take(kind: Kind) -> bool {
    ACTIVE.with_borrow_mut(|s| {
        if s.kind == Some(kind) && !s.consumed {
            s.consumed = true;
            s.hits += 1;
            true
        } else {
            false
        }
    })
}
pub(crate) fn again(kind: Kind) {
    ACTIVE.with_borrow_mut(|s| {
        if s.kind == Some(kind) {
            s.consumed = false;
        }
    });
}
pub(crate) fn hits() -> usize {
    ACTIVE.with_borrow(|s| s.hits)
}
