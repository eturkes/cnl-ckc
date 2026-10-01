#[test]
fn k3_naf_counterexample_rejected() {
    assert!(ckc_kernel::contract::k3_naf_counterexample());
}
