#[cfg(feature = "rkyv")]
#[test]
fn trusted_archive_trybuild() {
    let t = trybuild::TestCases::new();
    t.pass("tests/ui/trusted_archive/pass/*.rs");
    t.compile_fail("tests/ui/trusted_archive/fail/*.rs");
}
