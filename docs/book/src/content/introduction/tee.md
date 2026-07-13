# Trusted Execution Environment (TEE)

A Trusted Execution Environment (TEE) is an isolated execution domain for security-sensitive code and data. On ARM platforms, a TEE is commonly built with TrustZone, which separates the system into the normal world and the secure world.

The normal world usually runs Linux and ordinary applications. The secure world runs a trusted OS and trusted applications.

## Traditional Path: OP-TEE

A common TrustZone-based TEE stack is OP-TEE.

```text
Normal world application
        |
        |  TEE Client API / libteec
        v
Linux TEE / OP-TEE driver
        |
        |  SMC or FF-A
        v
OP-TEE OS
        |
        v
Trusted application
```

In this model, developers typically implement two components: a normal-world client and a secure-world trusted application. The client opens a session, invokes command IDs, and passes parameters. The trusted application validates the request, executes the sensitive operation, and returns a result.

Trusted applications are commonly written in C using the OP-TEE TA development model. The main command dispatch point is usually `TA_InvokeCommandEntryPoint`.

## Apache Teaclave TrustZone SDK

Apache Teaclave TrustZone SDK is a Rust-oriented development SDK for writing trusted applications on OP-TEE-compatible TrustZone systems.

It does not replace OP-TEE as the TEE stack. Instead, it provides a different trusted-application development model, allowing secure-world logic to be written in Rust while still relying on the underlying TrustZone and OP-TEE execution path.

## A Refined Developer Experience

After investigating how TEE calls work, we can separate a TEE invocation into the following steps:

1. CA prepares data and invokes TA through `TA_InvokeCommandEntryPoint`;
2. TA processes the request and returns the expected result or error information;
3. CA receives the result and proceeds.

Essentially, this resembles a generic function call with additional TEE-related boilerplate. Therefore, in Consortium, we introduce a better TEE developer experience through proc macros. The design contains three components:

- `tee_service!`: initializes the `Command` enum and the `invoke_command()` function.
- `#[tee_command]`: allows a command to be written as a normal Rust function, then automatically generates a `_dispatched` wrapper for TA-side parameter parsing and a `call_` wrapper for CA-side parameter packing. Through `Codec`, compatible structs or enums can also be passed through `memref`.
- `#[derive(TeeParam)]`: checks whether the target struct or enum is compatible across the Secure world and Non-Secure world boundary, such as rejecting pointers or architecture-specific-sized types, and implements conversion between native Rust types and TEE-compatible representations.

After proc macro expansion, the boilerplate code required by Apache Teaclave is generated automatically. The final code developers write therefore resembles a normal function in a standalone crate.

We recommend that developers adopt a dedicated `ta` crate for two purposes: `lib.rs` exports the generated code for both TA and CA usage, while `main.rs` contains the TA-side entry code. In the CA crate, the `ta` crate is imported with the `ca` feature enabled. This keeps the TEE interface as a single source of truth shared by both sides.
