# Consortium Chip Database: Product Requirements Document

| Field             | Value                                                         |
| ----------------- | ------------------------------------------------------------- |
| Status            | Proposal                                                      |
| Product           | Consortium Chip Database                                      |
| Relationship      | Standalone partner project to Consortium                      |
| Initial consumers | Web application, MCP clients, Consortium CLI, code generators |
| Deployment target | Cloudflare Workers, D1, R2, and Queues                        |
| Last updated      | 2026-07-19                                                    |

## Executive summary

The Consortium Chip Database is a versioned source of hardware truth for both
humans and software. It imports vendor and community hardware descriptions,
normalizes them into a canonical intermediate representation (IR), preserves
property-level provenance, supports review and publication, and generates
consistent outputs for firmware, embedded Linux, documentation, and AI tools.

The product is broader than a peripheral access crate generator. It describes
the relationships among chips, dies, packages, processors, address spaces,
memory, peripheral IP revisions, instances, registers, interrupts, pin routes,
security domains, clocks, resets, DMA, power domains, and Linux bindings.

Published data is immutable and content-addressed. Cloudflare D1 stores
authoring state and searchable relational projections. Cloudflare R2 stores
source assets where licensing permits, canonical release bundles, and generated
artifacts. A Rust Cloudflare Worker exposes the web API and remote MCP endpoint.
A Vue and TypeScript frontend provides the human interface. The standalone
repository uses Vite+ as its unified frontend toolchain.

## Background

Consortium already contains early forms of this data:

- Chip-family memory and AMP topology descriptions.
- Per-core peripheral base-address and interrupt maps.
- Peripheral register definitions represented in chiptool-compatible YAML.
- CMSIS-header import scripts for STM32MP2 and i.MX9.
- Vendor DTS and pinctrl sources.
- Generated Rust PAC crates and interrupt metadata.

These datasets are useful but have implicit links, different schemas, and
different update paths. The new product will extract the general hardware-data
problem into its own repository while allowing Consortium to consume pinned,
verified releases.

## Product vision

A developer, silicon vendor, integrator, or AI assistant should be able to:

1. Identify an exact chip, die, package, core, and security/address-space view.
2. Find a peripheral, register, field, interrupt, memory region, or pin route.
3. Understand where every important fact came from.
4. Compare peripheral and silicon revisions without manually diffing manuals.
5. Generate mutually consistent Rust, C, SVD, chiptool, DTS, and PDF outputs.
6. Reproduce a build later using a locked canonical release.
7. Access the same released knowledge through the website, API, CLI, or MCP.

## Goals

### G1: Canonical hardware model

Define a vendor-neutral IR that is a superset of the information needed by
CMSIS-SVD, chiptool, Rust PACs, CMSIS-style C headers, Linux Device Tree,
Consortium configuration, and generated reference documentation.

### G2: Independent peripheral revisioning

Represent a peripheral IP block independently from the chips that instantiate
it. Chips refer to immutable peripheral revisions, allowing reuse and explicit
compatibility analysis.

### G3: Reproducible publication and generation

Make every published release, source input, generator, and output artifact
identifiable by a cryptographic digest. A released artifact must be reproducible
from its locked inputs.

### G4: Trust and provenance

Record source identity, version, hash, location, extraction method, confidence,
review decision, and conflicts at property level.

### G5: Human usability

Provide a fast modern interface for search, exploration, comparison, pin
planning, memory visualization, provenance review, and artifact downloads.

### G6: Tool and agent usability

Expose stable structured APIs, immutable resources, and narrowly scoped MCP
tools. Agents must receive the same release identifiers and provenance that
human users see.

### G7: Offline and Yocto-compatible consumption

Allow the Consortium CLI and BitBake to fetch a versioned archive, verify its
checksum, and perform all compilation and generation without later network
access.

## Non-goals

The initial product will not:

- Replace a vendor reference manual as the legal or safety authority.
- Claim that generated HAL drivers are functionally validated on hardware.
- Provide live register debugging or direct target programming.
- Redistribute copyrighted or confidential vendor documents without permission.
- Treat OCR or model-extracted TRM content as trusted without review.
- Model boards as if they were chip variants; board support is a later layer.
- Guarantee lossless round trips through formats that cannot represent the full
  canonical IR.
- Run live D1 or MCP queries inside BitBake compile tasks.

## Users and jobs

### Firmware developer

- Look up a field's access and side-effect behavior.
- Generate or download a Rust PAC or C header for an exact core view.
- Compare a peripheral revision used by two chips.
- Find valid interrupt and pin routes.

### Embedded Linux integrator

- Generate SoC DTSI, pinctrl, reserved-memory, and integration fragments.
- Trace `compatible`, clock, reset, DMA, power-domain, and interrupt data back
  to a source.
- Obtain pinned archives suitable for a Yocto recipe.

### Consortium user

- Resolve `[profile] chip` to an exact database release.
- Validate memory, peripheral ownership, doorbell, and address visibility.
- Lock generated endpoint code to the same hardware release across builds.

### Database maintainer

- Import multiple source formats.
- Review parser warnings and conflicting claims.
- Correct data without mutating an existing release.
- Publish a reviewed release and regenerate all supported artifacts.

### Silicon or IP partner

- Maintain private or pre-release datasets in an isolated organization.
- Reuse peripheral revisions across chip families.
- Publish selected data without exposing licensed source assets.

### AI or IDE client

- Search the catalog through MCP.
- Retrieve bounded structured context for an exact release.
- Compare revisions and trace provenance without scraping documentation.

## Product principles

1. **Published data is immutable.** Corrections create new revisions.
2. **Identity is not a name.** Renames do not create new hardware identities.
3. **Evidence is not truth.** Imports create claims that require resolution.
4. **The IR is richer than its exports.** Export limitations never constrain
   what the canonical model can preserve.
5. **Core and security views are explicit.** Base addresses, access, and IRQs
   may differ by execution context.
6. **Generation is deterministic.** No generator depends on a moving `latest`
   channel after resolution.
7. **Portable releases are authoritative.** Published D1 projections can be
   rebuilt from canonical release objects.
8. **Network access ends after fetch.** Local and Yocto builds consume locked
   bundles.

## Canonical IR

### Identity model

The IR distinguishes stable identity, immutable revision, and publication:

```text
entity_id       Stable UUID across renames and revisions
revision_id     SHA-256 of one canonical object revision
release_id      SHA-256 of a manifest selecting exact root revisions
channel         Mutable convenience pointer such as stable or nightly
```

Vendor revision strings and marketing versions are metadata, not primary keys.
Compatibility classifications are explicit and may be `documentation-only`,
`codegen-compatible`, `source-compatible`, `breaking-layout`, or `unknown`.

### Canonical graph

The published object graph is directed and acyclic:

```text
Release
└── Chip revision
    ├── Die and package revisions
    ├── Processor and execution topology
    ├── Address spaces and memory maps
    ├── Peripheral instances ──→ Peripheral revisions
    ├── Interrupt routes
    ├── Pinmux and package routes
    └── OS and security integration
```

Peripheral revisions do not point back to consuming chips. D1 derives reverse
relationships such as "used by these chips."

### Principal object types

The first stable schema is expected to contain:

- `Vendor`, `ChipFamily`, `Chip`, `DieRevision`, `PackageRevision`, and
  `MarketedPart`.
- `ProcessorCluster`, `Processor`, `Core`, `ExecutionContext`, `AddressSpace`,
  and `Bus`.
- `MemoryRegion` and `AddressMapping`.
- `Peripheral`, `PeripheralRevision`, `RegisterBlock`, `RegisterCluster`,
  `Register`, `Field`, `EnumSet`, and `EnumValue`.
- `PeripheralInstance` and `InstanceMapping`.
- `InterruptController`, `InterruptSource`, and `InterruptRoute`.
- `Pad`, `PackagePin`, `PeripheralSignal`, `PinFunction`, `PinRoute`, and
  `PinGroup`.
- Typed clock, reset, DMA, power-domain, IOMMU, and security-controller
  connections.
- `LinuxBinding` and `DeviceTreeNodeTemplate`.
- `SourceDocument`, `SourceRevision`, `SourceLocation`, and provenance edges.

### Numeric representation

Addresses, masks, and arbitrary-width register values are canonical hexadecimal
strings because JSON and JavaScript cannot safely represent every 64-bit value.
Small bounded structural values remain JSON numbers.

```yaml
base: '0x02430000'
offset: '0x0014'
width: 32
lsb: 19
irq_number: 207
reset_value: '0xc0000000'
```

The schema defines one normalized hexadecimal spelling. D1 additionally stores
indexed addresses as fixed-width big-endian blobs to preserve unsigned ordering.

### Register access semantics

Read and write permissions are separate from side effects. The IR must support:

- Read-only, write-only, and read-write access.
- Privileged and secure access restrictions.
- Read-clear, read-set, and externally modified values.
- Write-one/zero-to-clear, set, and toggle.
- Write constraints and enumerated-only writes.
- Reset value and reset mask.
- Aliased and overlapping registers.
- Register and cluster arrays.
- FIFO and atomic alias behavior.
- Implementation-defined and reserved states.

Field position and width are canonical. Masks are derived and validated rather
than independently edited.

### Provenance

Every selected property can refer to one or more source locations:

```yaml
target: /registers/7/fields/3/write_action
source_revision: sha256:...
locator:
  kind: cmsis-symbol
  value: LPUART_STAT_OR_MASK
extraction:
  method: cmsis-header-parser
  tool_version: 0.3.0
confidence: exact
```

Confidence values are `exact`, `derived`, `inferred`, `conflicting`, and
`unverified`. Rejected claims remain in the authoring audit trail.

### Canonical serialization

- JSON is the normative portable serialization for schema version 1.
- Object keys and ordered collections have deterministic canonical ordering.
- Release identity is the SHA-256 of canonical manifest content.
- Each immutable object has its own digest and may be reused by multiple
  releases.
- CBOR and compressed JSON are derived distribution formats.
- Archive timestamps, compression settings, and filenames do not affect
  semantic release identity.

## Functional requirements

### IR and releases

- **IR-001:** The system shall validate every canonical object against a
  versioned schema.
- **IR-002:** Published revisions and releases shall be immutable.
- **IR-003:** A chip revision shall refer to exact peripheral revision digests.
- **IR-004:** A release shall be reconstructable without querying D1.
- **IR-005:** Reverse references shall be derivable from forward references.
- **IR-006:** Schema migrations shall preserve the original published bytes and
  produce a new release when canonical content changes.
- **IR-007:** A release manifest shall include schema version, root objects,
  object hashes, source-manifest hash, and generator compatibility metadata.

### Import and normalization

- **IMP-001:** Import CMSIS-SVD XML.
- **IMP-002:** Import supported CMSIS C headers and macro conventions.
- **IMP-003:** Import and export chiptool YAML.
- **IMP-004:** Import selected DTS, DTSI, pinctrl, and dt-binding data.
- **IMP-005:** Preserve the raw source hash and importer version.
- **IMP-006:** Create claims rather than directly overwriting reviewed facts.
- **IMP-007:** Detect conflicts among sources and existing canonical data.
- **IMP-008:** Support deterministic importer reruns.
- **IMP-009:** Treat TRM OCR or model-assisted extraction as unverified claims.
- **IMP-010:** Record redistribution and use rights for every source asset.

### Catalog and query

- **CAT-001:** Search chips, families, peripheral types, instances, registers,
  fields, signals, and aliases.
- **CAT-002:** Filter by vendor, architecture, core, package, capability,
  release, and publication state.
- **CAT-003:** Resolve common part-number aliases without losing canonical
  identity.
- **CAT-004:** Select an execution context before returning base address,
  accessibility, or interrupt information.
- **CAT-005:** Compare two chip or peripheral revisions structurally.
- **CAT-006:** Trace a displayed value to its provenance and review state.
- **CAT-007:** Use opaque cursor pagination for large result sets.

### Authoring and review

- **AUT-001:** Create isolated drafts from an existing release or empty schema.
- **AUT-002:** Review imported claims individually or in validated groups.
- **AUT-003:** Display conflicting values and sources side by side.
- **AUT-004:** Require validation before review submission.
- **AUT-005:** Require an authorized reviewer before publication.
- **AUT-006:** Record author, reviewer, timestamps, rationale, and changed
  canonical paths.
- **AUT-007:** Prevent edits to released objects.
- **AUT-008:** Support private organization datasets and public datasets without
  cross-tenant leakage.

### Export and generation

- **EXP-001:** Export CMSIS-SVD for a selected device and address-space view.
- **EXP-002:** Export a versioned Consortium C-header profile with CMSIS
  compatibility where applicable.
- **EXP-003:** Generate Rust PAC crates, initially through a pinned chiptool
  projection where practical.
- **EXP-004:** Export chiptool-compatible YAML and a loss report.
- **EXP-005:** Generate LaTeX and a rendered PDF reference manual.
- **EXP-006:** Generate SoC DTSI, pinctrl includes, reserved-memory fragments,
  and optional board-integration templates.
- **EXP-007:** Validate DTS output with `dtc` and applicable `dt-schema`
  bindings.
- **EXP-008:** Include release and generator identity in every artifact.
- **EXP-009:** Produce byte-reproducible artifacts where the format permits it.
- **EXP-010:** Report canonical information that an export format omitted or
  approximated.

### Website

- **WEB-001:** Provide fast global search and stable release-aware URLs.
- **WEB-002:** Display interactive memory maps and peripheral-instance views.
- **WEB-003:** Render registers and bitfields with access and side effects.
- **WEB-004:** Switch among core, security, privilege, and address-space views.
- **WEB-005:** Compare chip and peripheral revisions.
- **WEB-006:** Provide pin-route search and conflict detection.
- **WEB-007:** Display provenance, confidence, and conflicting claims.
- **WEB-008:** Offer code snippets and artifact downloads for exact releases.
- **WEB-009:** Meet WCAG 2.2 AA for the core catalog and authoring workflows.
- **WEB-010:** Remain usable on desktop and tablet-width displays.

### MCP

- **MCP-001:** Expose a remote Streamable HTTP endpoint at `/mcp`.
- **MCP-002:** Provide a matching local stdio server backed by a locked bundle.
- **MCP-003:** Expose immutable resource templates for schemas, releases, chips,
  peripherals, registers, provenance, and artifacts.
- **MCP-004:** Initially expose read-only tools for search, address maps,
  peripherals, registers, IRQs, pin routes, revision comparison, provenance,
  and artifact retrieval.
- **MCP-005:** Return both concise text and structured output with stable IDs.
- **MCP-006:** Bound result sizes and paginate large collections.
- **MCP-007:** Mark tool behavior with annotations while enforcing permissions
  independently on the server.
- **MCP-008:** Treat imported descriptions and comments as untrusted content,
  never as server instructions.
- **MCP-009:** Add draft-authoring tools only after read-only authorization and
  auditing are proven.
- **MCP-010:** Keep global release publication outside MCP for the initial
  product.

### Consortium CLI and Yocto

- **CLI-001:** Resolve a chip identifier and channel to an exact release digest.
- **CLI-002:** Download and verify an immutable canonical bundle.
- **CLI-003:** Record the release, schema, URL, checksum, and generator version
  in a lockfile.
- **CLI-004:** Operate entirely from the locked bundle after fetch.
- **CLI-005:** Generate a BitBake-compatible source URI and SHA-256 pin.
- **CLI-006:** Support mirrors, pre-populated `DL_DIR`, and `BB_NO_NETWORK`
  validation.
- **CLI-007:** Preserve support for vendored bundles in source repositories.

## Database schema

D1 is divided logically into evidence, canonical metadata, projections, and
artifacts. Large binary content and canonical distribution objects reside in
R2.

### Evidence and authoring tables

```text
organization
dataset
source
source_revision
source_asset
source_location
import_run
claim
claim_conflict
review_decision
draft
draft_change
```

Claims identify a target entity and canonical path, candidate JSON value,
source location, extraction method, and confidence. Typed validation occurs
before a claim can be selected into a draft.

### Canonical metadata tables

```text
entity
entity_alias
object_revision
object_revision_edge
release
release_root
release_channel
publication
```

`object_revision` stores the entity, object kind, schema version, canonical
digest, R2 key, and compact canonical JSON when it remains below the configured
row threshold. R2 remains the source for portable published bytes.

### Search projection tables

```text
chip_index
part_index
core_index
address_space_index
memory_mapping_index
peripheral_index
instance_mapping_index
register_index
field_index
interrupt_route_index
pin_route_index
linux_binding_index
provenance_index
catalog_fts
```

Projection rows are replaceable and carry a release digest. No released bundle
depends on projection-specific row IDs.

### Generation tables

```text
generator
generation_job
artifact
artifact_variant
validation_run
```

Artifacts contain R2 key, SHA-256, size, media type, generator revision,
canonical release, options digest, validation state, and publication state.

### Indexing requirements

- Case-folded exact indexes for canonical names and aliases.
- FTS indexes for names and descriptions.
- Composite indexes beginning with dataset and release for tenant isolation.
- Fixed-width sortable address columns for memory and instance range queries.
- Explicit indexes for every foreign key used in publication and deletion
  checks.
- Query plans and measured latency included in migration review.

## Storage architecture

### D1 responsibilities

- Drafts, claims, conflicts, and review workflow.
- Published release catalog and channel pointers.
- Detailed searchable projections.
- User, organization, authorization, and audit references.
- Generation and validation job metadata.

### R2 responsibilities

- Original source assets where storage and redistribution are permitted.
- Import logs and parser diagnostics.
- Canonical objects and release manifests.
- Pre-bundled JSON/CBOR archives.
- Generated SVD, C, Rust, YAML, DTS, LaTeX, and PDF artifacts.
- Signatures, checksums, SBOMs, and source manifests.

Suggested immutable keys:

```text
sources/sha256/<digest>
objects/<schema>/<kind>/<digest>.json
releases/<schema>/<release-digest>/manifest.json
bundles/<release-digest>/canonical.tar.zst
artifacts/<release-digest>/<generator>/<version>/<options-digest>/<file>
```

Mutable channel documents are separate from immutable object paths.

### Publication transaction

1. Freeze and validate a draft.
2. Canonicalize and hash all objects.
3. Generate and validate required artifacts.
4. Upload immutable objects and artifacts to R2.
5. Verify object hashes and sizes.
6. In one D1 transaction, insert the release and optionally move a channel.
7. Retain failed publication diagnostics and later collect abandoned staging
   objects.

## System architecture

```text
Vue web application
        │ HTTPS / JSON
        ▼
Rust Cloudflare Worker ──────────────── MCP clients
        │                                   │
        │                                   ├─ Streamable HTTP /mcp
        │                                   └─ local stdio via CLI
        ├─ D1: authoring and query projections
        ├─ R2: canonical objects and artifacts
        ├─ Queues: imports, exports, validation
        └─ external build runner when a job exceeds Worker limits
```

The Worker owns authorization and application policy. The frontend and MCP
layers call the same domain services; neither calls the other as an internal
API.

Long-running PDF, PAC, and validation jobs use a queue and idempotent job key.
An external sandboxed runner may be introduced for native tools that cannot run
within the Worker Wasm environment.

## Recommended technology stack

The standalone repository should pin exact dependency versions in its lockfiles
and update deliberately. "Latest" below means the latest stable compatible
release at repository creation or an approved dependency-update change.

### Backend and shared domain

- Rust 2024 workspace.
- Cloudflare [`workers-rs`](https://developers.cloudflare.com/workers/languages/rust/)
  targeting `wasm32-unknown-unknown`.
- `serde` and JSON Schema generation for canonical types.
- D1 through Worker bindings and versioned SQL migrations.
- R2 through Worker bindings with explicit SHA-256 verification.
- Cloudflare Queues for asynchronous import and generation dispatch.
- Structured tracing with request, organization, release, import, and job IDs.
- A protocol-conformance-tested Rust MCP implementation supporting Streamable
  HTTP and stdio. The concrete MCP crate is selected after a compatibility
  spike rather than embedded in the product contract.

The canonical model, validation, diffing, import normalization, and generator
interfaces live in ordinary Rust crates without Cloudflare dependencies. The
Worker is an adapter around those crates.

### Frontend

- Vue 3 with Composition API and `<script setup lang="ts">`.
- Strict TypeScript.
- Vue Router for release-aware stable URLs.
- Pinia only for durable cross-view client state; server data uses a dedicated
  query/cache layer.
- Vue - Official language tooling, formerly known as Volar, for SFC type and
  template support. See the
  [Vue TypeScript guide](https://vuejs.org/guide/typescript/overview).
- Tailwind CSS through the official
  [`@tailwindcss/vite`](https://tailwindcss.com/docs/installation/using-vite)
  integration.
- The current stable Vite line; at the time of this PRD, Vite 8 is the stable
  major and uses Rolldown. Avoid experimental Vite APIs in product-critical
  paths.
- Accessible headless components where useful, with a small project-owned
  visual system rather than a large opaque component theme.
- Canvas or SVG visualization modules for memory maps and register bitfields;
  ordinary semantic HTML remains the accessible source representation.

### Vite+ workflow

Use Vite+ as the unified frontend runtime, package-management, development,
build, test, formatting, linting, type-checking, and task entry point:

```bash
vp install
vp dev
vp check
vp test
vp build
vp run <task>
```

The root `vite.config.ts` should enable type-aware linting and type checking.
Vite Task should orchestrate code generation and schema-client generation where
its input/output tracking is appropriate. Rust commands may be registered as
explicit tasks, but Cargo remains responsible for Rust dependency resolution
and compilation.

### Repository shape

One possible standalone layout is:

```text
apps/
  web/                       Vue application
crates/
  ir/                        canonical types and serialization
  schema/                    schema generation and compatibility checks
  database/                  repository interfaces and D1 projections
  import/                    shared import framework
  import-svd/
  import-cmsis/
  import-chiptool/
  import-devicetree/
  generate-c/
  generate-rust-pac/
  generate-svd/
  generate-chiptool/
  generate-devicetree/
  generate-latex/
  mcp/                       transport-independent MCP surface
  cli/                       local, CI, and stdio MCP entry point
  worker/                    Cloudflare Worker adapter
packages/
  api-client/                generated TypeScript API types
  ui/                        shared Vue components and visual primitives
migrations/
schemas/
fixtures/
```

## API design

- Version public HTTP APIs under `/api/v1`.
- Use release digests in URLs and responses after channel resolution.
- Return stable entity IDs and revision IDs in all entity responses.
- Use opaque cursor pagination.
- Publish machine-readable error codes with human-readable explanations.
- Accept conditional requests and emit immutable cache headers for released
  data.
- Generate OpenAPI and TypeScript client types from backend contracts.
- Do not expose generic SQL, arbitrary R2 keys, arbitrary source URLs, or shell
  options.

## MCP surface

### Remote server

The Rust Worker exposes `/mcp` using Streamable HTTP. Read-only public catalog
access may be anonymous and rate-limited. Artifact generation and all private or
authoring access require OAuth scopes.

Initial tools:

```text
search_chips
resolve_chip
get_address_map
search_peripherals
get_peripheral
search_registers
get_register
get_irq_route
find_pin_routes
compare_revisions
trace_provenance
get_artifact
```

Initial resource URI forms:

```text
consortium://schema/{version}
consortium://releases/{release_digest}
consortium://chips/{entity_id}@{revision_id}
consortium://peripherals/{entity_id}@{revision_id}
consortium://registers/{entity_id}@{revision_id}
consortium://artifacts/{release_digest}/{format}/{variant}
```

### Local server

The CLI exposes the same read-only surface over stdio using a locked local
bundle. It performs no remote channel resolution and requires an exact release.

### Authorization

Initial scopes:

```text
catalog:read
sources:read
artifacts:generate
drafts:write
imports:write
reviews:approve
releases:publish
```

The server enforces scopes at the domain-service boundary. Tool annotations are
descriptive hints and never replace authorization checks.

## Security and trust requirements

- Validate OAuth audience, issuer, expiry, scopes, and organization membership.
- Apply tenant predicates before every private D1 and R2 lookup.
- Use separate public and private R2 prefixes or buckets where appropriate.
- Reject arbitrary network fetches from import tools; use uploads or approved
  source connectors.
- Sandbox parsers and native generators with CPU, memory, time, and output
  limits.
- Treat source comments, descriptions, OCR, and generated text as untrusted
  content.
- Escape all rendered descriptions and sanitize the supported documentation
  markup subset.
- Require explicit confirmation and audit for publication and destructive draft
  actions.
- Use immutable artifact keys and conditional writes.
- Keep secrets out of bundles, logs, MCP results, and generated outputs.
- Generate an SBOM for deployable services and generator containers.

## Licensing requirements

Every source revision records:

- Owner and publisher.
- Source URL or acquisition record.
- Document or package version.
- Cryptographic hash.
- Declared license.
- Permission to store privately.
- Permission to redistribute the original.
- Permission to publish extracted facts.
- Required attribution.
- Access classification: public, organization-private, or restricted.

The publication validator blocks releases that would include unauthorized source
assets. Published facts retain citations even when the original document cannot
be redistributed.

Generated artifacts include the database project's license plus applicable
source attribution and generated-code notices. License policy must be reviewed
before claiming that an exported vendor-derived PAC or header is freely
redistributable.

## Non-functional requirements

### Correctness

- Validate register overlap, bit overlap, reset widths, enum ranges, array
  bounds, address ranges, IRQ targets, and dangling references.
- Preserve source evidence for every automatic correction or normalization.
- Maintain golden fixtures and round-trip tests for supported imports.
- Compare generated PAC layouts against canonical offsets and widths.

### Performance

- Public exact-ID reads: p95 server latency below 150 ms excluding client
  network latency and uncached R2 transfer.
- Catalog search: p95 below 300 ms for expected production dataset size.
- Initial application shell: usable within 2.5 seconds on a representative
  mid-range connection and device.
- Register and field pages must virtualize large collections without hiding
  semantic table access from assistive technology.

### Reliability

- Idempotent imports and generation jobs.
- No channel pointer moves until all required release objects are verified.
- Restore and rebuild procedures tested from R2 canonical bundles.
- Publication failures leave the prior channel unchanged.

### Observability

- Structured logs and traces across HTTP, MCP, D1, R2, queue, and runner work.
- Metrics for search latency, cache hit rate, import conflicts, validation
  failures, generation duration, publication failures, and MCP tool errors.
- Audit events for private reads, draft writes, reviews, and publication.

### Compatibility

- Schema compatibility tests for every change.
- At least the current and previous stable canonical schema readers supported by
  the CLI.
- Explicit exporter support matrix by schema and generator version.

## Success metrics

### Initial technical success

- Import the existing Consortium STM32MP2 and i.MX9 datasets.
- Represent all currently used memory, peripheral, IRQ, and register data
  without unreported loss.
- Reproduce the current PAC input data and interrupt definitions.
- Resolve existing Consortium example chips from a locked release.
- Answer the initial MCP read tool suite from both remote and local servers.

### Product success

- A user can find any known register field from a part number in fewer than
  three search interactions.
- Every published register offset and field position has provenance or an
  explicit unverified status.
- A maintainer can understand and resolve a conflicting import without reading
  raw database rows.
- Generated artifacts from the same release agree on names, addresses, widths,
  and interrupt numbers.
- At least one Yocto build completes with networking disabled after fetch.

## Delivery phases

### Phase 0: Schema RFC and extraction plan

- Finalize stable identity and canonicalization rules.
- Specify register semantics and execution-context addressing.
- Define source, claim, provenance, and licensing models.
- Inventory reusable Consortium importers and fixtures.
- Decide the standalone repository name and governance.

**Exit criterion:** reviewed schema RFC with representative STM32MP2 and i.MX9
examples.

### Phase 1: Local canonical core

- Create the Rust IR, schema, validation, diff, and bundle crates.
- Import existing Consortium chip, memory, and peripheral data.
- Publish local content-addressed release bundles.
- Implement a read-only local CLI.

**Exit criterion:** current data converts deterministically and validates without
unexplained loss.

### Phase 2: Export parity

- Implement chiptool YAML, Rust PAC, C, SVD, and interrupt exports.
- Compare generated layout and API fixtures with current Consortium outputs.
- Add generator manifests and reproducibility tests.

**Exit criterion:** selected current PAC and configuration consumers can use the
new bundle.

### Phase 3: Cloud catalog and read-only web

- Deploy Rust Worker, D1 migrations, R2 release storage, and public API.
- Build Vue search, chip, peripheral, register, memory, and provenance views.
- Add public artifact downloads and release-aware URLs.

**Exit criterion:** a published release is searchable and browsable end to end.

### Phase 4: MCP

- Implement remote Streamable HTTP and local stdio servers.
- Add read-only resources and tool suite.
- Add authorization, rate limits, conformance tests, and prompt-injection
  handling.

**Exit criterion:** supported MCP clients can retrieve the same exact released
facts as the web API and local CLI.

### Phase 5: Authoring and publication

- Add source upload, import runs, conflict review, drafts, validation, approval,
  and publication UI.
- Add private organization datasets and audit logs.

**Exit criterion:** a maintainer can import, review, and publish a correction
without direct D1 or R2 access.

### Phase 6: Consortium and Yocto integration

- Add release resolution and lockfile support to Consortium CLI.
- Generate BitBake pins and validate offline builds.
- Replace bundled legacy data only after output parity and migration review.

**Exit criterion:** Consortium examples build from locked database releases and
one supported Yocto flow passes with post-fetch networking disabled.

### Phase 7: Extended data

- Add pinmux planning, Linux bindings, clocks, resets, DMA, and power domains.
- Add reviewed TRM-assisted extraction.
- Add LaTeX/PDF reference publication.
- Evaluate board and module databases as separate layers.

## MVP acceptance criteria

The MVP is complete when all of the following are true:

1. A versioned canonical schema and deterministic serializer are published.
2. Existing i.MX93, i.MX95, and STM32MP2 controller-core data imports into it.
3. Peripheral revisions are independent and reused by chip instances.
4. Core/address-space-specific base addresses and IRQ routes are queryable.
5. Published releases are content-addressed and downloadable from R2.
6. D1 search projections can be rebuilt from a published bundle.
7. The Vue website supports search, chip, peripheral, register, and provenance
   views.
8. At least chiptool YAML, Rust PAC input, C header, SVD, and canonical bundle
   exports are available.
9. Remote and local MCP servers implement the initial read-only tools.
10. The Consortium CLI can lock, fetch, verify, and consume an exact release.
11. `vp check`, `vp test`, and `vp build` pass for the frontend workspace.
12. Rust formatting, linting, unit tests, schema compatibility tests, importer
    fixtures, and generator golden tests pass in CI.

## Risks and mitigations

| Risk                              | Mitigation                                                                |
| --------------------------------- | ------------------------------------------------------------------------- |
| Conflicting vendor sources        | Preserve claims, display conflicts, require resolution                    |
| Copyright restrictions            | Separate facts from assets and enforce publication rights                 |
| IR constrained by SVD/chiptool    | Define the IR independently and publish export loss reports               |
| D1 growth or contention           | Keep blobs in R2, index measured queries, design shard-safe IDs           |
| Worker Wasm limitations           | Keep generators portable and use sandboxed external runners when required |
| Parser silently invents semantics | Confidence levels, provenance, golden fixtures, review gates              |
| Peripheral revision explosion     | Content-addressed reuse and compatibility classification                  |
| MCP data exfiltration             | Scoped authorization, tenant filters, bounded tools, audit logs           |
| Prompt injection in source text   | Treat all imported prose as data and separate it from instructions        |
| Generated-code incompatibility    | Version generators and test layouts and public API fixtures               |
| Moving frontend toolchain         | Exact lockfiles, stable APIs, Vite+ validation, scheduled upgrades        |

## Open decisions

- Final product and repository name.
- Public contribution and governance model.
- Exact canonical JSON normalization profile.
- UUID representation in D1 and public APIs.
- Whether private datasets share a D1 database or use database-per-tenant
  isolation.
- Authentication provider and reviewer-role model.
- Initial Rust MCP implementation after the compatibility spike.
- Generator execution environment for LaTeX, `dt-schema`, and native chiptool.
- Legal policy for derived facts from restricted vendor manuals.
- Boundary among die revisions, package variants, and marketed part numbers for
  each vendor family.

## Relationship to Consortium

The new repository becomes the upstream publisher of general chip data.
Consortium remains responsible for AMP-specific configuration, validation,
lowering, endpoint generation, build planning, and deployment artifacts.

Migration should be incremental:

1. Import current Consortium datasets into the new IR.
2. Demonstrate output and validation parity.
3. Teach Consortium to consume an optional locked external bundle.
4. Make the external bundle the default only after representative examples and
   CI pass.
5. Retain a vendored or cached release for bootstrapping and offline builds.

The database must remain useful without Consortium, while Consortium should gain
a richer and more reproducible hardware foundation by adopting it.
