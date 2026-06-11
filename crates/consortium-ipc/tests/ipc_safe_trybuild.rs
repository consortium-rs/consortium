#[test]
fn ipc_safe_derive_trybuild() {
    let t = trybuild::TestCases::new();
    t.pass("tests/ui/ipc_safe/pass/*.rs");
    t.compile_fail("tests/ui/ipc_safe/fail/*.rs");
}
