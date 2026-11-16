//! Qualification-only code-path omissions. Absent from every normal binary.
pub fn on(name: &str) -> bool {
    std::env::var("SB1_TAS_T2_PLANT").as_deref() == Ok(name)
}
