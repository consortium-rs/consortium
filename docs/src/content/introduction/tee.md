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

##
