# `#[consortium_runtime_{mcu,app}::main]` runtime entry macros

## Context

Today every melt-pot endpoint hand-writes the same boilerplate around the generated
`consortium.gen.rs`:

- **MCU** (`examples/melt-pot/*/mcu/src/main.rs`): `#[embassy_executor::main]`, then
  `ConsortiumLogger::init()`, `init_time_driver()`, `init()` (sync MPU/NVIC) →
  `.connect().await` → match/`wfi` on error, then `peripherals()`.
- **App** (`examples/melt-pot/*/app/src/main.rs`): `#[tokio::main]`, then
  `match init().await { Ok(endpoints) => …, Err(e) => … }`.

The generated code exposes several differently-named handles (`ConsortiumEndpoints`,
`IpcMemoryEndpoints`, `IpcMemoryEndpointsConnected`, `Peripherals`) and a two-phase
`init()`/`connect()` dance on MCU that every firmware repeats. The user wants a single
attribute macro that hides this and hands the body one aggregate value named **`Context`**,
reinforcing "context" as the term for the whole per-manifest resource bundle:

```rust
// MCU firmware (no_std, embassy)
#[consortium_runtime_mcu::main]
async fn main(context: Context, spawner: Spawner) { /* context.ipc_shm.sensor … */ }

// Linux app (tokio)
#[consortium_runtime_app::main]
async fn main(context: Context) { /* context.ipc_shm.sensor … */ }
```

Intended outcome: the macro wraps the runtime (`embassy_executor::main` / `tokio::main`),
calls the generated aggregate `async init()`, and binds its `Context` result into the
user's body. On init failure it dispatches to an optional user `#[…::fail]` handler,
falling back to a hard fail.

### Confirmed design decisions (from user)

1. **IPC access stays `context.ipc_shm.sensor`** — keep the current `ipc_shm` field name
   (matches AGENTS.md and today's app example). Peripherals live at `context.peripherals`.
2. **Init-failure policy**: on `Err`, the glue calls a user handler defined with a
   companion attribute `#[consortium_runtime_mcu::fail]` / `#[…app::fail]`. If the user
   defined none, hard-fail (MCU: `consortium_log::error!` + `wfi` loop; App: `tracing`
   error + `std::process::exit(1)`).
3. `Spawner` stays a **separate second parameter** on MCU (embassy muscle memory; it is
   `Copy` executor infrastructure, not a per-manifest resource). App takes only `context`.
4. **`init_time_driver()` and `defmt::timestamp!` are sealed too** — the user no longer
   hand-writes the embassy time-driver bring-up, the timer ISR, or the defmt timestamp
   source. Because these are chip-specific, they cannot live in the (chip-agnostic)
   proc-macro; they are **generated into `consortium.gen.rs`** and driven by the
   `#[main]`-called `init()`. From the user's `main.rs` they are gone — "sealed" as
   requested — the macro is just the seam that triggers them.

### Verified facts

- The descriptor handshake (`descriptor_init`/`descriptor_ack`,
  `crates/consortium-ipc-transport-memory/src/descriptor.rs:315,379`) only awaits
  `doorbell.wait().await` — **no `embassy-time`**. So folding `connect()` into the
  generated init needs no time-driver ordering; the user's `init_time_driver()` can run in
  the body. No `pre_init` attr arg is needed now.
- **Glob-import shadowing works on stable (edition 2024)** for the `#[fail]` dispatch: a
  module-scope `fn __consortium_init_fail` shadows a glob-imported default with no
  conflict; when absent the glob default is used. Verified with a standalone `rustc`
  compile. The inserted glob `use` gets `#[allow(unused_imports)]`.
- Codegen entry points: `crates/consortium-cfg/src/bridge.rs` —
  `LoweredIr::initialize` (AP path, `bridge.rs:190`) and `controller_init` (MCU path,
  `bridge.rs:265`). Endpoint structs from `src/components/ipc_shm.rs:576`
  (`ShuHan::initialize`), NVIC from `src/components/bella.rs`, `Peripherals` +
  `peripherals()` from `src/components/peripheral.rs:72` (returns `Option<TokenStream>`).
- Runtime crates re-export nothing runtime-executor related today: `consortium-runtime-mcu`
  has **no** `embassy-executor` dep; `consortium-runtime-app` has **no** `tokio::main`. The
  example crates own those deps (`embassy-executor 0.10.0`, features `executor-thread` +
  `platform-cortex-m`; `tokio` with `macros`,`rt-multi-thread`). The macro's expansion
  references `::embassy_executor` / `::tokio` which resolve in the **user** crate.
- Macro convention = thin facade (`proc-macro = true`) + `-impl` crate over
  `proc_macro2::TokenStream`; shared syn helpers in `consortium-macros-helpers` (already has
  `get_output_type`, `is_result_returning`, `extract_result_inner`). Tests = `insta`
  snapshots calling impl fns on `quote!{}` input; errors emitted as `compile_error!` tokens.
  Model: `consortium-tee-macros{,-impl}` (`tee_command` attribute + `syn::parse::Parse`
  attr-args). Root `Cargo.toml` `members = ["crates/*", …]` auto-includes new crates.
- **Time driver**: both HALs expose `timer_driver::init_timer(base: *mut (), clock_hz: u32)`
  and `timer_driver::on_timer_interrupt()`. The **ISR mechanism differs per chip**: stm32mp25
  uses cortex-m-rt `#[interrupt] fn TIM2()`; imx95 uses `#[unsafe(no_mangle)] extern "C" fn
LPIT1_IRQHandler()`. Bring-up differs too: stm32 pokes RCC (`TIM2CFGR`: RST off/EN/LPEN)
  before `init_timer`, then `nvic::set_priority`+`enable`; imx95 assumes the platform already
  clocked LPIT1 and only does `init_timer`+nvic. **No timer/time-driver metadata exists in
  `consortium-data`/`consortium-cfg` yet** — the examples hardcode base/IRQ/clock constants
  (stm32: TIM2 @ `0x4000_0000`, IRQ 105, 64 MHz; imx95: LPIT1 @ `0x442f_0000`, IRQ 15,
  24 MHz). `defmt::timestamp!` is currently a hand-written monotonic `AtomicU32` counter,
  independent of the time driver. Codegen already emits peripheral ISRs (`peripheral.rs`
  emits `#[unsafe(no_mangle)] extern "C" fn LPI2C1_IRQHandler`), so emitting a timer ISR the
  same way lands it in the app-owned `consortium.gen.rs` and respects the repo convention
  that libraries install no `#[interrupt]` handlers.

## New crates

### `crates/consortium-runtime-macros-impl`

Non-proc-macro logic crate. Deps: `syn = "2"` (full), `quote`, `proc-macro2`,
`consortium-macros-helpers` (path). Dev-deps: `insta`, `prettyplease`. Edition 2024.
Public fns operating on `proc_macro2::TokenStream`:

- `pub fn mcu_main(attr: TokenStream2, item: TokenStream2) -> TokenStream2`
- `pub fn app_main(attr: TokenStream2, item: TokenStream2) -> TokenStream2`
- `pub fn mcu_fail(attr: TokenStream2, item: TokenStream2) -> TokenStream2`
- `pub fn app_fail(attr: TokenStream2, item: TokenStream2) -> TokenStream2`

### `crates/consortium-runtime-macros`

Facade, `[lib] proc-macro = true`. Single path dep on `-impl`. Four
`#[proc_macro_attribute]` wrappers converting `.into()` both ways (mirrors
`consortium-tee-macros/src/lib.rs`).

## Macro behavior

### `mcu_main` / `app_main`

Parse the item as `syn::ItemFn`. Validate (emit `compile_error!` on failure, per convention):

- must be `async`;
- name is `main`;
- MCU: exactly two params — first `context` (any binding ident; type is written by the user
  as `Context`, but the macro **ignores the annotated type** and binds the init result to
  that ident), second is the spawner passed through to embassy;
- App: exactly one param (`context`);
- return type `()` (the aggregate flow owns error handling; a non-unit return is a
  `compile_error!` for now).

**MCU expansion** (`ident`s: user's `context` binding = `$ctx`, spawner = `$sp`, body `$body`):

```rust
#[allow(unused_imports)]
use ::consortium_runtime_mcu::__rt::__consortium_init_fail;   // glob-shadowable default
#[::embassy_executor::main]
async fn __consortium_entry($sp: ::embassy_executor::Spawner) {
    let $ctx = match init().await {
        ::core::result::Result::Ok(c) => c,
        ::core::result::Result::Err(e) => __consortium_init_fail(e),  // -> !
    };
    $body
}
```

(Use a glob `use ::consortium_runtime_mcu::__rt::*;` — not the named import above — so a
user `#[fail]`-emitted `__consortium_init_fail` shadows it. Shown named for clarity;
implement as the `*` glob with `#[allow(unused_imports)]`.)

**App expansion**:

```rust
#[allow(unused_imports)]
use ::consortium_runtime_app::__rt::*;
#[::tokio::main]
async fn __consortium_entry() {
    let $ctx = match init().await {
        ::core::result::Result::Ok(c) => c,
        ::core::result::Result::Err(e) => __consortium_init_fail(e),
    };
    $body
}
```

`init` is called unqualified — it resolves to the generated `init()` in the same module
(the crate `include!`s `consortium.gen.rs` at module scope). `::embassy_executor` /
`::tokio` resolve in the user crate (they already depend on them; `tokio` needs its
`macros` feature, which the app examples enable).

### `mcu_fail` / `app_fail`

Parse `syn::ItemFn` `fn on_fail(err: InitError) { … }` (or `-> !`). Emit the user's fn
plus a diverging wrapper at the fixed name the glue calls:

```rust
fn on_fail(err: /* user type */) { … }          // user's item, unchanged
fn __consortium_init_fail<E>(e: E) -> ! {         // shadows the runtime default
    on_fail(e);                                   // if user fn is `-> !`, this diverges
    // MCU: loop { ::cortex_m::asm::wfi() }   App: ::std::process::exit(1)
    <hard-fail tail>
}
```

Generic `<E>` so it accepts whatever concrete error the generated `init()` returns without
the impl needing to name it. If the user handler is `-> ()`, the wrapper runs the hard-fail
tail after it; if `-> !`, the tail is unreachable (allow warning).

### Runtime default fail handlers (hidden `__rt` modules)

- `consortium-runtime-mcu/src/lib.rs`: `#[doc(hidden)] pub mod __rt { pub fn
__consortium_init_fail<E>(_e: E) -> ! { crate::log::error!("consortium init failed"); loop
{ ::cortex_m::asm::wfi() } } }`. No new deps (`consortium_log`, `cortex-m` already
  present). Logs a static message — the error type is not `defmt::Format`-bounded; users who
  want detail use `#[fail]` with the concrete type.
- `consortium-runtime-app/src/lib.rs`: analogous, `tracing`-style error via `crate::log` +
  `::std::process::exit(1)`.

### Runtime re-exports

- `consortium-runtime-mcu`: `pub use consortium_runtime_macros::{mcu_main as main, mcu_fail
as fail};` behind a default `macros` feature (dep on the facade). no_std-safe: proc-macro
  deps don't affect the target binary.
- `consortium-runtime-app`: `pub use consortium_runtime_macros::{app_main as main, app_fail
as fail};` (same `macros` default feature, under the existing `cfg(target_os="linux")`).

## Codegen changes (`crates/consortium-cfg`)

Unify the aggregate under a generated `struct Context` on **both** sides and provide one
canonical `async fn init() -> Result<Context, ConsortiumInitError>` the macro calls.

- **AP** (`bridge.rs:190-244`): rename `struct ConsortiumEndpoints` → `struct Context`
  (keep fields `_consortium_dbg`, `uio`, `ipc_shm`). `init()` already `async` and returns
  `Result<_, ConsortiumInitError>`; just swap the type name. Keep `IpcMemoryEndpoints`,
  `IpcMemoryEndpointsConnected`, `ConsortiumConnectError` unchanged (from `ipc_shm.rs`).
- **MCU** (`controller_init`, `bridge.rs:265-351`): replace the sync
  `#[cfg(target_arch="arm")] fn init() -> IpcMemoryEndpoints` with an aggregate:
  ```rust
  struct Context {
      ipc_shm: IpcMemoryEndpointsConnected,
      peripherals: Peripherals,        // only when [peripheral.*] present for this core
  }
  #[cfg(target_arch = "arm")]
  async fn init() -> ::core::result::Result<Context, ConsortiumInitError> {
      // when [dbg.<core>] configured: ::consortium_dbg::logger::ConsortiumLogger::init();
      <steal + MPU carveouts + NVIC unmask, as today>
      __consortium_init_time_driver();     // sealed embassy time-driver bring-up (see below)
      let ipc_shm = IpcMemoryEndpoints::new().connect().await?;   // ConsortiumConnectError -> ConsortiumInitError
      let peripherals = peripherals();                             // if present
      ::core::result::Result::Ok(Context { ipc_shm, peripherals })
  }
  ```
  Add a small MCU `ConsortiumInitError` wrapping `ConsortiumConnectError` (derive
  `defmt::Format` under `target_os="none"`, `Debug` otherwise — mirror the
  `connect_error_derive` split in `ipc_shm.rs:645`), with a `From<ConsortiumConnectError>`.
  Keep `IpcMemoryEndpoints::new()` and `connect()` public for non-macro / two-phase users.

### Sealed time driver + defmt timestamp (new codegen, MCU only)

New component (e.g. `crates/consortium-cfg/src/components/time.rs`) driven by the core's
chip identity (`McoreName` + chip string; reuse `hal_crate_path`). It emits into
`consortium.gen.rs`, at module scope:

1. **`__consortium_init_time_driver()`** — calls a **HAL bring-up helper** so the
   chip-specific RCC/base/clock detail stays in the HAL (per AGENTS.md "chip-specific details
   inside the chip HAL"): add `consortium_hal_stm32mp2::timer_driver::bringup()` and
   `consortium_hal_imx9::timer_driver::bringup()` that encapsulate today's example bodies
   (stm32: RCC `TIM2CFGR` enable → `init_timer(TIM2_BASE, 64 MHz)` → nvic; imx95:
   `init_timer(LPIT1_BASE, 24 MHz)` → nvic). Codegen emits
   `unsafe { <hal>::timer_driver::bringup(); }` with a SAFETY comment.
2. **The timer ISR** — chip-dependent form: stm32 `#[interrupt] fn TIM2() {
<hal>::timer_driver::on_timer_interrupt(); }`; imx95 `#[unsafe(no_mangle)] extern "C" fn
LPIT1_IRQHandler() { <hal>::timer_driver::on_timer_interrupt(); }`. Reuse the emission
   style already in `peripheral.rs` (which emits `LPI2C1_IRQHandler` the same way). Gate on
   `#[cfg(target_arch = "arm")]` (needs the HAL `rt` feature for `#[interrupt]`, already
   enabled by the example crates).
3. **`defmt::timestamp!`** — emit the monotonic `AtomicU32` counter (chip-agnostic, no
   dependency on time-driver state), gated on `[dbg.<core>]` / defmt being configured, so it
   is defined exactly once. (Switching it to read `embassy_time::Instant::now()` is a future
   option once ordering guarantees are firmed up.)

Timer parameters (base, IRQ, clock_hz, ISR name/mechanism) come from a small **per-chip
default table** in the new component for the two supported chips. These are demo-board
values with a boot-handoff assumption (the platform leaves the timer clocked at the stated
rate) — surface a **config opt-out** so integrators who own their own time base can disable
the sealed driver (e.g. a `[profile]`/`[runtime]` `time_driver = false` key, or per-core);
default on for MCU cores. When disabled, `__consortium_init_time_driver()` is a no-op and no
ISR/timestamp is emitted, and the integrator supplies their own (as today).

- `struct Context` (both sides) has **no lifetime parameter** — the AP `Context` owns its
  `Vec<UioDevice>` and the connected transceivers borrow `'static` scratch buffers + the
  owned mmap windows, exactly as `ConsortiumEndpoints` does today. The user's sketched
  `Context<'static>` is not needed.
- Keep `struct Context` **ungated**; keep only `fn init()` under `#[cfg(target_arch="arm")]`
  so the host trybuild harness still compiles the module (it appends its own `fn main(){}`
  and never links `cortex-m`/embassy).

## Example updates (all four `main.rs`)

- **stm32mp25 mcu** (sketch) — note how much is _removed_:

  ```rust
  #![no_std] #![no_main]
  use embassy_executor::Spawner;
  use embassy_time::Timer;
  use melt_pot_shared::SensorReading;
  include!("consortium.gen.rs");
  // REMOVED (now sealed/generated): defmt::timestamp!, TIM2_* consts,
  //   init_time_driver(), #[interrupt] fn TIM2(), ConsortiumLogger::init().

  #[consortium_runtime_mcu::main]
  async fn main(context: Context, _spawner: Spawner) {
      let mut sensor = context.ipc_shm.sensor;   // time driver + logger already up
      let mut seq = 0u32;
      loop { … sensor.send(&reading).await …; Timer::after_secs(1).await; }
  }
  // Still user-owned: doorbell #[interrupt] fn IPCC1_RX/IPCC2_RX (forward to
  //   consortium_ipc_doorbell_ipcc::notify_*), panic_handler, HardFault.
  ```

  (imx95 mcu analogous: keep the `MU7_B` doorbell handler + `panic-halt`; `LPIT1` time
  driver, `defmt::timestamp!`, and `LPIT1_IRQHandler` become generated; use
  `context.peripherals`.)

- **app** (both): `#[consortium_runtime_app::main] async fn main(context: Context) { … }`;
  the receive loop uses `context.ipc_shm.sensor`. Drop the manual `#[tokio::main]` + match.
- **imx95 app name clash**: it imports `optee_teec::Context`. Alias it
  (`use optee_teec::Context as TeeContext;`) so the generated `Context` wins the bare name.
  Flag in the plan; fix in that file.
- Optional `#[consortium_runtime_mcu::fail]` handler can be added to one example to
  demonstrate, but keep default hard-fail elsewhere.

## Tests

- **`consortium-runtime-macros-impl/tests/snap.rs`** (new): `insta` snapshots for each of the
  four impl fns over `quote!{}` inputs — happy path (mcu 2-arg, app 1-arg, fail handler) and
  error paths (non-async, wrong arity, non-unit return) emitting `compile_error!`. Follow
  `consortium-ipc-macros-impl/tests/snap.rs` (parse → `prettyplease::unparse` →
  `assert_snapshot!`).
- **`consortium-cfg`**: regenerate with `UPDATE_SNAPSHOTS=1`:
  - `tests/snap.rs` insta: `snap__shuhan_ap_side_emits_full_endpoint_struct.snap`,
    `snap__shuhan_controller_side_emits_single_endpoint_struct.snap`, and any
    controller-init/bella snapshots that render the `Context`/`init()` shape.
  - `tests/conf.rs` per-config `tests/configs/{imx95,stm32mp257}/artifacts.snap`.
  - `tests/ui.rs` regenerates `tests/ui/linux/*__ap.rs` and `tests/ui/portable/*__<core>.rs`
    and type-checks them; confirm the new async MCU `init()` + ungated `Context` still
    compile on host (init is arm-gated, Context is not). The generated timer ISR +
    `__consortium_init_time_driver` are also arm-gated so the host fixtures skip them; the
    `defmt::timestamp!` emission is defmt-gated so it stays out of the host build.
  - New snapshot(s) for the time-driver component (ISR + `bringup` call + timestamp) per chip.
- **HAL**: `cargo test`/`cargo check -p consortium-hal-stm32mp2 -p consortium-hal-imx9` for
  the new `timer_driver::bringup()` helpers (host-buildable parts; full check under the
  chip target via `just`).

## Docs

- `AGENTS.md`: update the **IPC Core** / **Runtime Layers** sections to describe the
  `Context` aggregate, `#[consortium_runtime_{mcu,app}::main]`, and `#[…::fail]`; add the new
  macro crates to the **Macro helpers** row and Workspace Shape table.
- mdBook: optional short page under `docs/book/src/` on the runtime entry macros (defer if
  scope-limited).

## Implementation order

1. Scaffold `consortium-runtime-macros-impl` + facade; implement `mcu_main`/`app_main`
   (main path) with insta snapshots.
2. Add `mcu_fail`/`app_fail` + the hidden `__rt` default handlers in both runtime crates;
   wire the `macros` feature + re-exports.
3. HAL: add `timer_driver::bringup()` to `consortium-hal-stm32mp2` and `consortium-hal-imx9`
   (lift the example bodies; chip constants live here).
4. Codegen: AP rename → `Context`; MCU aggregate async `init()` + `Context` + MCU
   `ConsortiumInitError`; new `components/time.rs` (time-driver init call + timer ISR +
   `defmt::timestamp!`, per-chip table + opt-out); keep two-phase API. Regenerate
   consortium-cfg snapshots.
5. Update the four example `main.rs` (+ imx95 `optee_teec::Context` alias); delete the now
   hand-written timestamp/time-driver/timer-ISR from all mcu examples.
6. Docs (AGENTS.md).

## Verification

- `cargo test -p consortium-runtime-macros-impl` — macro snapshots.
- `cargo test -p consortium-cfg` (then `UPDATE_SNAPSHOTS=1 cargo test -p consortium-cfg` to
  accept intentional codegen diffs; re-run to confirm green) — snap + conf + ui fixtures,
  including the trybuild `generated_init_code_compiles` that type-checks the rendered app +
  mcu modules.
- `cargo check -p consortium-runtime-mcu` and `cargo check -p consortium-runtime-app`
  (Linux) — re-exports + default fail handlers compile.
- `just check` / `just lint` for the host bundle; `just test ipc host`.
- Full example firmware/app builds go through the builder (`csti build` generates
  `consortium.gen.rs`); the `consortium-cfg` ui fixtures are the in-repo proxy that the
  generated `Context`/`init()` compiles for both host-app and portable-mcu shapes. If a
  provisioned target is available, `just` thumbv8m/aarch64 recipes confirm the macro
  expansion links against the real embassy/tokio deps.

## Risks

- **`Context` name clash** with `optee_teec::Context` in imx95 app — resolved by aliasing
  the TEE import (plan step 4).
- **`include!` hygiene**: the macro calls `init` and `Context` unqualified; they resolve
  because `consortium.gen.rs` is `include!`d into the same module as the `#[main]` fn. Any
  crate that puts the macro and the include in different modules must re-export/`use` them —
  document in AGENTS.md.
- **`tokio::main` needs the `macros` feature** in the app crate; **embassy needs
  `executor-thread` + `platform-cortex-m`** — already satisfied by the examples; document as
  a requirement of `#[…app::main]` / `#[…mcu::main]`.
- **Attribute re-expansion**: our macro emits `#[::embassy_executor::main]` /
  `#[::tokio::main]`, which the compiler expands after ours — standard attribute stacking,
  no reentrancy issue.
- **`#[fail]` glob shadowing** relies on local-item-over-glob precedence (verified on stable
  edition 2024). If a user defines `#[fail]` in a different module than `#[main]`, the
  default is used instead — document that both must share the module (same constraint as
  `init`/`Context`).
- **Sealed timer defaults are board-specific**: the generated base/IRQ/clock and the RCC
  bring-up encode the demo-board (STM32MP257F-EV1 / FRDM-i.MX95) assumption that the boot
  handoff leaves the timer clocked at the stated rate. On a different board these are wrong —
  hence the `time_driver = false` opt-out and a follow-up to move timer selection into the
  config/chip DB rather than a hardcoded per-chip table.
- **Duplicate `defmt::timestamp!` / timer ISR**: exactly one definition is allowed per
  binary, so the examples MUST drop their hand-written versions when codegen emits them;
  leaving both is a hard compile error. Covered by plan step 5.
- **`bringup()` moves `unsafe` MMIO into the HAL**: keep each block narrow with the RCC/timer
  register invariant documented (workspace lints deny undocumented unsafe).
