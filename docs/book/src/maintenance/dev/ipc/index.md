# Inter-processor Communication (IPC) Subsystem

Design notes for the typed IPC stack. The current trait stack, transports, and doorbells
are documented in the [IPC introduction](../../../content/introduction/ipc.md) and in each
crate's README; these chapters record the reasoning and proposals behind them.

- [IPC QoS Design](ipc-qos-design.md) — a proposal for priority and back-pressure across
  channels. Not implemented.
- [IPC Design](ipc-design.md) — the original vision for the doorbell → transport →
  codec → channel layering. Crate names and trait signatures have since changed; the
  layering has not.

The register model of each doorbell — IPCC, MU, and HSEM — is documented on its
`consortium-ipc-transport-memory::bell` module, next to the shared wait protocol.
