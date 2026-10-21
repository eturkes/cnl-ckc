use vstd::prelude::*;

verus! {
    fn width() {
        let w: usize = 65536;
        let _ = format!("{:1$}", 5u8, w);
    }

    fn precision() {
        let p: usize = 65536;
        let _ = format!("{:.*}", p, 5u8);
    }

    fn format_args_width() {
        let w: usize = 65536;
        let _ = format_args!("{:1$}", 5u8, w);
    }
}

fn main() {}
