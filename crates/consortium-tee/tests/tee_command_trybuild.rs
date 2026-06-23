// The `#[tee_command]` compile-fail cases exercise the macro's generated code, which is
// split into a TA-side path (`feature = "ta"`) and a CA-side path (`feature = "ca"`). The
// TA path is host-agnostic: it only references `::optee_utee`, which the UI fixtures mock
// locally. The CA path pulls in the Linux-only `optee-teec`/`TeeResult` surface and must
// never be exercised by host trybuild. We therefore gate this test on `ta` so it runs in
// exactly the configuration where the expected `.stderr` snapshots are reproducible on any
// platform. Without a feature, the macro emits neither path and the fixtures would compile
// cleanly, defeating the compile-fail assertions.
#[cfg(feature = "ta")]
#[test]
fn tee_command_trybuild() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/tee_command/fail/*.rs");
}
