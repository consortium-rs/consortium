#[test]
fn tee_param_derive_trybuild() {
    let t = trybuild::TestCases::new();
    t.pass("tests/ui/tee_param/pass/*.rs");
    t.compile_fail("tests/ui/tee_param/fail/*.rs");
}
