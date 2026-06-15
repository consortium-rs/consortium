#[test]
fn tee_command_trybuild() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/tee_command/fail/*.rs");
}
