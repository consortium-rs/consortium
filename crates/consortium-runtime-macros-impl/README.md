# consortium-runtime-macros-impl

Host-testable implementation of the Consortium entry macros. `entry.rs` validates the
Embassy-shaped arity and classifies the output as `()` or `Result<(), E>`. It emits an
inner async function preserving the user's parameter pattern, return type, and body,
then passes the awaited value to the selected runtime's `MainOutcome` trait.

This crate intentionally contains no generated-resource initialization policy and no
failure-handler name-shadowing mechanism. Initialization is user-called, and entry
errors are reported uniformly by the runtime crate.
