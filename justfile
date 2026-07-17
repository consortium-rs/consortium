m8_target := "thumbv8m.main-none-eabihf"
m7_target := "thumbv7em-none-eabihf"
a64_target := "aarch64-unknown-linux-gnu"
a32_target := "armv7-unknown-linux-gnueabihf"
rv32imc_target := "riscv32imc-unknown-none-elf"
rv32imac_target := "riscv32imac-unknown-none-elf"
rv32imafcp_target := "riscv32imafc-unknown-none-elf"
lint_crates := "consortium-builder consortium-cfg consortium-cli consortium-codec consortium-codec-macros consortium-codec-macros-impl consortium-dbg consortium-dts consortium-ffi consortium-hal-imx9 consortium-hal-stm32mp2 consortium-ipc consortium-ipc-doorbell-gpio consortium-ipc-doorbell-hsem consortium-ipc-doorbell-ipcc consortium-ipc-doorbell-mu consortium-ipc-macros consortium-ipc-macros-impl consortium-ipc-transport-memory consortium-ipc-transport-uart consortium-ipc-transport-uart-embedded consortium-ipc-transport-uart-unix consortium-ort consortium-runtime-mcu consortium-tee-macros consortium-tee-macros-impl melt-pot-shared"
rust_tools := "taplo-cli typos-cli cargo-deny cargo-expand cargo-insta cargo-shear"
ls_lint_version := "2.3.1"

# List available recipes
default:
    @just --list

# ── Core Commands (from CLAUDE.md) ───────────────────────────────────────────

# Install local development tools
setup: setup-uv setup-vp setup-rust-tools setup-rust-targets setup-ls-lint setup-chiptool

# Install uv
setup-uv:
    @if command -v uv >/dev/null 2>&1; then \
        echo "uv is already installed"; \
    else \
        curl -LsSf https://astral.sh/uv/install.sh | sh; \
    fi

# Install Vite+
setup-vp:
    @if command -v vp >/dev/null 2>&1; then \
        echo "vp is already installed"; \
    else \
        curl -fsSL https://vite.plus | bash; \
    fi

# Install Cargo-hosted development tools
setup-rust-tools: setup-cargo-binstall
    cargo binstall -y {{ rust_tools }}

# Install Rust targets used by cross-build, check, and test recipes
setup-rust-targets:
    rustup target add {{ a64_target }}
    rustup target add {{ a32_target }}
    rustup target add {{ m8_target }}
    rustup target add {{ m7_target }}
    rustup target add {{ rv32imc_target }}
    rustup target add {{ rv32imac_target }}
    rustup target add {{ rv32imafcp_target }}

# Install cargo-binstall, used for fast binary installs of Rust tooling
setup-cargo-binstall:
    @if command -v cargo-binstall >/dev/null 2>&1; then \
        echo "cargo-binstall is already installed"; \
    else \
        cargo install cargo-binstall; \
    fi

# Prime ls-lint through vp dlx
setup-ls-lint:
    vp dlx --yes @ls-lint/ls-lint@{{ ls_lint_version }} --version

# Install chiptool, used to validate/regenerate the peripheral access crates
setup-chiptool:
    @if command -v chiptool >/dev/null 2>&1; then \
        echo "chiptool is already installed"; \
    else \
        cargo install --git https://github.com/embassy-rs/chiptool --locked; \
    fi

# Build all crates
build:
    cargo build

# Run tests. With args: `just test ipc host`, `just test hmi host`, `just test tee host`, `just test cfg host`, `just test dbg host`, `just test codec host`.
test area="" scope="":
    @case "{{ area }}:{{ scope }}" in \
        ":") cargo test ;; \
        "ipc:"|"ipc:host") just test-ipc-host ;; \
        "ipc:a64") just test-ipc-a64 ;; \
        "ipc:a32") just test-ipc-a32 ;; \
        "ipc:m8") just test-ipc-m8 ;; \
        "ipc:m7") just test-ipc-m7 ;; \
        "hmi:"|"hmi:host") just test-hmi-host ;; \
        "hmi:a64") just test-hmi-a64 ;; \
        "hmi:a32") just test-hmi-a32 ;; \
        "tee:"|"tee:host") just test-tee-host ;; \
        "tee:a64") just test-tee-a64 ;; \
        "tee:a32") just test-tee-a32 ;; \
        "cfg:"|"cfg:host") just test-cfg-host ;; \
        "dbg:"|"dbg:host") just test-dbg-host ;; \
        "codec:"|"codec:host") just test-codec-host ;; \
        *) echo "Error: unknown test scope '{{ area }} {{ scope }}'"; echo "Try: just test ipc host | a64 | a32 | m8 | m7, just test hmi host | a64 | a32, just test tee host | a64 | a32, just test cfg host, just test dbg host, or just test codec host"; exit 1 ;; \
    esac

# Run tests for a specific crate (e.g. `just test-crate ipc`)
test-crate crate:
    @crate={{ quote(crate) }}; case "$crate" in consortium-*) ;; *) crate="consortium-$crate";; esac; cargo test -p "$crate"

# Run a single test by name (e.g. `just test-fn ipc my_test`)
test-fn crate test_name:
    @crate={{ quote(crate) }}; test_name={{ quote(test_name) }}; case "$crate" in consortium-*) ;; *) crate="consortium-$crate";; esac; cargo test -p "$crate" "$test_name"

# Lint with Clippy
clippy:
    cargo clippy

# Run all host-side linters
lint: fmt-check cargo-lint taplo-check typos ls-lint oxlint ruff-lint

# Run Rust lint checks that avoid HMI and OP-TEE CA/TA link dependencies
cargo-lint:
    @for crate in {{ lint_crates }}; do \
        cargo clippy -p "$crate" --all-targets -- -D warnings || exit 1; \
    done
    cargo clippy -p consortium-runtime-app --no-default-features --features mimx9x,stm32mp25x --all-targets -- -D warnings
    cargo clippy -p consortium-tee --no-default-features --features ta --all-targets -- -D warnings

# Lint JavaScript and TypeScript through Vite+
oxlint:
    vp lint

# Lint Python scripts
ruff-lint:
    uvx ruff check .

# Auto-fix lint and format issues
fix: cargo-fix oxfix ruff-fix fmt

# Apply Clippy autofixes, mirroring cargo-lint
cargo-fix:
    @for crate in {{ lint_crates }}; do \
        cargo clippy --fix --allow-dirty --allow-staged -p "$crate" --all-targets || exit 1; \
    done
    cargo clippy --fix --allow-dirty --allow-staged -p consortium-runtime-app --no-default-features --features mimx9x,stm32mp25x --all-targets
    cargo clippy --fix --allow-dirty --allow-staged -p consortium-tee --no-default-features --features ta --all-targets

# Apply JavaScript and TypeScript autofixes through Vite+
oxfix:
    vp lint --fix

# Apply Python autofixes
ruff-fix:
    uvx ruff check --fix .

# Format code
fmt:
    cargo fmt
    vp fmt
    uvx ruff format
    taplo format

# Check code formatting
fmt-check:
    cargo fmt --check
    vp fmt --check
    uvx ruff format --check
    taplo format --check

# Format TOML files
taplo-fmt:
    taplo format

# Check TOML validity
taplo-check:
    taplo lint

# Check spelling
typos:
    typos

# Check file and directory names
ls-lint:
    vp dlx --yes @ls-lint/ls-lint@{{ ls_lint_version }}

# ── Host (dev / CI) ──────────────────────────────────────────────────────────

# Check all crates on the host
check:
    cargo check --workspace --features consortium-ipc/std

# ── A-core (aarch64 / Linux) ─────────────────────────────────────────────────

check-ipc-a64:
    @[ "$(uname)" = "Linux" ] || { echo "Error: A-core targets require Linux"; exit 1; }
    cargo check -p consortium-ipc --target {{ a64_target }} -F std
    cargo check -p consortium-ipc-transport-memory --target {{ a64_target }} -F std
    cargo check -p consortium-runtime-app --target {{ a64_target }}
    cargo check -p consortium-ipc-doorbell-hsem --target {{ a64_target }} -F std
    cargo check -p consortium-ipc-doorbell-ipcc --target {{ a64_target }} -F std
    cargo check -p consortium-ipc-doorbell-mu --target {{ a64_target }} -F std

build-ipc-a64:
    @[ "$(uname)" = "Linux" ] || { echo "Error: A-core targets require Linux"; exit 1; }
    cargo build -p consortium-ipc --release --target {{ a64_target }} -F std
    cargo build -p consortium-ipc-transport-memory --release --target {{ a64_target }} -F std
    cargo build -p consortium-runtime-app --release --target {{ a64_target }}
    cargo build -p consortium-ipc-doorbell-hsem --release --target {{ a64_target }} -F std
    cargo build -p consortium-ipc-doorbell-ipcc --release --target {{ a64_target }} -F std
    cargo build -p consortium-ipc-doorbell-mu --release --target {{ a64_target }} -F std

# Run/compile A-core IPC tests for aarch64 Linux.
test-ipc-a64:
    @[ "$(uname)" = "Linux" ] || { echo "Error: A-core tests require Linux"; exit 1; }
    cargo test -p consortium-ipc --target {{ a64_target }} -F std
    cargo test -p consortium-ipc-transport-memory --target {{ a64_target }} -F std
    cargo test -p consortium-ipc-doorbell-hsem --target {{ a64_target }} -F std
    cargo test -p consortium-ipc-doorbell-ipcc --target {{ a64_target }} -F std
    cargo test -p consortium-ipc-doorbell-mu --target {{ a64_target }} -F std

# ── A-core (armv7 / Linux) ──────────────────────────────────────────────────

check-ipc-a32:
    @[ "$(uname)" = "Linux" ] || { echo "Error: A-core targets require Linux"; exit 1; }
    cargo check -p consortium-ipc --target {{ a32_target }} -F std
    cargo check -p consortium-ipc-transport-memory --target {{ a32_target }} -F std
    cargo check -p consortium-runtime-app --target {{ a32_target }}
    cargo check -p consortium-ipc-doorbell-hsem --target {{ a32_target }} -F std
    cargo check -p consortium-ipc-doorbell-ipcc --target {{ a32_target }} -F std
    cargo check -p consortium-ipc-doorbell-mu --target {{ a32_target }} -F std

build-ipc-a32:
    @[ "$(uname)" = "Linux" ] || { echo "Error: A-core targets require Linux"; exit 1; }
    cargo build -p consortium-ipc --release --target {{ a32_target }} -F std
    cargo build -p consortium-ipc-transport-memory --release --target {{ a32_target }} -F std
    cargo build -p consortium-runtime-app --release --target {{ a32_target }}
    cargo build -p consortium-ipc-doorbell-hsem --release --target {{ a32_target }} -F std
    cargo build -p consortium-ipc-doorbell-ipcc --release --target {{ a32_target }} -F std
    cargo build -p consortium-ipc-doorbell-mu --release --target {{ a32_target }} -F std

# Run/compile A-core IPC tests for armv7 Linux.
test-ipc-a32:
    @[ "$(uname)" = "Linux" ] || { echo "Error: A-core tests require Linux"; exit 1; }
    cargo test -p consortium-ipc --target {{ a32_target }} -F std
    cargo test -p consortium-ipc-transport-memory --target {{ a32_target }} -F std
    cargo test -p consortium-ipc-doorbell-hsem --target {{ a32_target }} -F std
    cargo test -p consortium-ipc-doorbell-ipcc --target {{ a32_target }} -F std
    cargo test -p consortium-ipc-doorbell-mu --target {{ a32_target }} -F std

# ── HMI (Cog/WPE / Linux) ──────────────────────────────────────────────────

# Run host-side HMI tests, including the raw Cog bindings and safe wrapper.
test-hmi-host:
    cargo test -p cogcore-sys
    cargo test -p cogcore
    cargo test -p consortium-hmi

# Run/compile HMI tests for aarch64 Linux.
test-hmi-a64:
    @[ "$(uname)" = "Linux" ] || { echo "Error: A-core HMI tests require Linux"; exit 1; }
    cargo test -p cogcore-sys --target {{ a64_target }}
    cargo test -p cogcore --target {{ a64_target }}
    cargo test -p consortium-hmi --target {{ a64_target }}

# Run/compile HMI tests for armv7 Linux.
test-hmi-a32:
    @[ "$(uname)" = "Linux" ] || { echo "Error: A-core HMI tests require Linux"; exit 1; }
    cargo test -p cogcore-sys --target {{ a32_target }}
    cargo test -p cogcore --target {{ a32_target }}
    cargo test -p consortium-hmi --target {{ a32_target }}

# ── TEE (Trusted Execution Environment / Linux) ──────────────────────────────

# Check TEE crates
check-tee:
    @[ "$(uname)" = "Linux" ] || { echo "Error: TEE crates require Linux"; exit 1; }
    cargo check -p consortium-tee
    cargo check -p consortium-tee --no-default-features --features ta
    cargo check -p consortium-tee-macros
    cargo check -p consortium-tee-macros-impl

# Check TEE crates with panic=abort (for TA binaries)
check-tee-test:
    @[ "$(uname)" = "Linux" ] || { echo "Error: TEE test crate requires Linux"; exit 1; }
    RUSTFLAGS="-C panic=abort" cargo check -p consortium-tee-test

# Build TEE crates
build-tee:
    @[ "$(uname)" = "Linux" ] || { echo "Error: TEE crates require Linux"; exit 1; }
    cargo build -p consortium-tee --release
    cargo build -p consortium-tee-macros --release
    cargo build -p consortium-tee-macros-impl --release

# Build TEE test crate with panic=abort
build-tee-test:
    @[ "$(uname)" = "Linux" ] || { echo "Error: TEE test crate requires Linux"; exit 1; }
    RUSTFLAGS="-C panic=abort" cargo build -p consortium-tee-test --release

# Run host TEE tests that do not require OP-TEE runtime libraries.
test-tee:
    just test-tee-host

test-tee-host:
    # `ta` is host-agnostic (no optee-teec/Linux deps) and is required for the
    # tee_command trybuild fixtures to exercise the TA codegen path. CI has no
    # optee-* bindings, so the `ca` path stays disabled.
    cargo test -p consortium-tee --no-default-features --features ta
    cargo test -p consortium-tee-macros-impl --test snap

# Run TEE tests for Linux A-core targets.
test-tee-a64:
    @[ "$(uname)" = "Linux" ] || { echo "Error: A-core TEE tests require Linux"; exit 1; }
    cargo test -p consortium-tee --no-default-features --target {{ a64_target }}
    cargo test -p consortium-tee-macros-impl --test snap

test-tee-a32:
    @[ "$(uname)" = "Linux" ] || { echo "Error: A-core TEE tests require Linux"; exit 1; }
    cargo test -p consortium-tee --no-default-features --target {{ a32_target }}
    cargo test -p consortium-tee-macros-impl --test snap

# ── M-core (thumbv8 / bare-metal) ───────────────────────────────────────────

check-ipc-m8:
    cargo check -p consortium-ipc --target {{ m8_target }}
    cargo check -p consortium-common --target {{ m8_target }}
    cargo check -p consortium-ipc-transport-memory --target {{ m8_target }} --no-default-features
    cargo check -p consortium-ipc-transport-uart --target {{ m8_target }} --no-default-features
    cargo check -p consortium-ipc-transport-uart-embedded --target {{ m8_target }} --features stm32mp25x
    cargo check -p consortium-runtime-mcu --target {{ m8_target }}
    cargo check -p consortium-runtime-mcu --target {{ m8_target }} --no-default-features --features ipc-uart
    cargo check -p consortium-ipc-doorbell-hsem --target {{ m8_target }}
    cargo check -p consortium-ipc-doorbell-ipcc --target {{ m8_target }}
    cargo check -p consortium-ipc-doorbell-mu --target {{ m8_target }}

build-ipc-m8:
    cargo build -p consortium-common --release --target {{ m8_target }}
    cargo build -p consortium-ipc --release --target {{ m8_target }}
    cargo build -p consortium-ipc-transport-memory --release --target {{ m8_target }} --no-default-features
    cargo build -p consortium-ipc-transport-uart --release --target {{ m8_target }} --no-default-features
    cargo build -p consortium-ipc-transport-uart-embedded --release --target {{ m8_target }} --features stm32mp25x
    cargo build -p consortium-runtime-mcu --release --target {{ m8_target }}
    cargo build -p consortium-ipc-doorbell-hsem --release --target {{ m8_target }}
    cargo build -p consortium-ipc-doorbell-ipcc --release --target {{ m8_target }}
    cargo build -p consortium-ipc-doorbell-mu --release --target {{ m8_target }}

# Compile M-core IPC crates for thumbv8. Unit tests are not runnable on the host.
test-ipc-m8: check-ipc-m8

# ── M-core (thumbv7 / bare-metal) ───────────────────────────────────────────

check-ipc-m7:
    cargo check -p consortium-common --target {{ m7_target }}
    cargo check -p consortium-ipc --target {{ m7_target }}
    cargo check -p consortium-ipc-transport-memory --target {{ m7_target }} --no-default-features
    cargo check -p consortium-ipc-transport-uart --target {{ m7_target }} --no-default-features
    cargo check -p consortium-ipc-transport-uart-embedded --target {{ m7_target }} --features mimx95
    cargo check -p consortium-runtime-mcu --target {{ m7_target }}
    cargo check -p consortium-runtime-mcu --target {{ m7_target }} --no-default-features --features ipc-uart
    cargo check -p consortium-ipc-doorbell-hsem --target {{ m7_target }}
    cargo check -p consortium-ipc-doorbell-ipcc --target {{ m7_target }}
    cargo check -p consortium-ipc-doorbell-mu --target {{ m7_target }}

build-ipc-m7:
    cargo build -p consortium-common --release --target {{ m7_target }}
    cargo build -p consortium-ipc --release --target {{ m7_target }}
    cargo build -p consortium-ipc-transport-memory --release --target {{ m7_target }} --no-default-features
    cargo build -p consortium-ipc-transport-uart --release --target {{ m7_target }} --no-default-features
    cargo build -p consortium-ipc-transport-uart-embedded --release --target {{ m7_target }} --features mimx95
    cargo build -p consortium-runtime-mcu --release --target {{ m7_target }}
    cargo build -p consortium-ipc-doorbell-hsem --release --target {{ m7_target }}
    cargo build -p consortium-ipc-doorbell-ipcc --release --target {{ m7_target }}
    cargo build -p consortium-ipc-doorbell-mu --release --target {{ m7_target }}

# Compile M-core IPC crates for thumbv7. Unit tests are not runnable on the host.
test-ipc-m7: check-ipc-m7

# Run host-side IPC tests, including fake-MMIO std and no-std-shaped wait-future tests.
test-ipc-host:
    cargo test -p consortium-ipc --features std
    cargo test -p consortium-ipc-transport-memory
    cargo test -p consortium-ipc-transport-memory --no-default-features --test async_atomic_waker_test
    cargo test -p consortium-ipc-doorbell-hsem
    cargo test -p consortium-ipc-doorbell-hsem --features std
    cargo test -p consortium-ipc-doorbell-ipcc
    cargo test -p consortium-ipc-doorbell-ipcc --features std
    cargo test -p consortium-ipc-doorbell-mu
    cargo test -p consortium-ipc-doorbell-mu --features std
    cargo test -p consortium-ipc-transport-uart
    cargo test -p consortium-ipc-transport-uart --no-default-features --test loopback_embedded
    cargo test -p consortium-ipc-transport-uart-unix

# ── Configuration (host-only) ───────────────────────────────────────────────

# Run host-side configuration tests: schema/validation, lowering, and the
# codegen/DTS snapshot fixtures. These crates are host-only (no target builds).
test-cfg-host:
    cargo test -p consortium-cfg-common
    cargo test -p consortium-cfg

# Run host-side debug-logging tests: descriptor/ring behavior and the logger,
# plus the defmt `decode` path (defmt-decoder host reader).
test-dbg-host:
    cargo test -p consortium-dbg
    cargo test -p consortium-dbg --features decode

# Run host-side codec tests across all families, plus the derive-macro
# snapshot/trybuild coverage.
test-codec-host:
    cargo test -p consortium-codec --features postcard,prost,rkyv,alloc
    cargo test -p consortium-codec-macros-impl

# ── RISC-V (rv32imc) ────────────────────────────────────────────────────────

check-rv32imc:
    cargo check -p consortium-ipc --target {{ rv32imc_target }}

build-rv32imc:
    cargo build -p consortium-ipc --release --target {{ rv32imc_target }}

# ── RISC-V (rv32imac) ───────────────────────────────────────────────────────

check-rv32imac:
    cargo check -p consortium-ipc --target {{ rv32imac_target }}

build-rv32imac:
    cargo build -p consortium-ipc --release --target {{ rv32imac_target }}

# ── RISC-V (rv32imafcp) ─────────────────────────────────────────────────────

check-rv32imafcp:
    cargo check -p consortium-ipc --target {{ rv32imafcp_target }}

build-rv32imafcp:
    cargo build -p consortium-ipc --release --target {{ rv32imafcp_target }}

# ── Combined ─────────────────────────────────────────────────────────────────

check-ipc-m: check-ipc-m8 check-ipc-m7

build-ipc-m: build-ipc-m8 build-ipc-m7

check-ipc-a: check-ipc-a64 check-ipc-a32

build-ipc-a: build-ipc-a64 build-ipc-a32

check-rv: check-rv32imc check-rv32imac check-rv32imafcp

build-rv: build-rv32imc build-rv32imac build-rv32imafcp

check-ipc-arm: check-ipc-a check-ipc-m

build-ipc-arm: build-ipc-a build-ipc-m

# ── Utilities ────────────────────────────────────────────────────────────────

# Add license headers (requires addlicense)
license:
    addlicense -c "Ethan Wu" -y 2026 -l apache -s -ignore "data/**/*" .

# ── PAC generation (consortium-pac-*) ────────────────────────────────────────
# Tooling for the peripheral access crates. Vendor headers live in `submodules/`,
# local patches in `scripts/pac/patches/`, generated IR under
# `crates/consortium-data/database/` (`chip/`, `peri/`). Requires
# chiptool (see `just setup-chiptool`).

# Initialize the PAC vendor submodules and apply local patches
pac-init:
    git submodule update --init submodules/imx-sm submodules/mcux-devices-imx submodules/STM32CubeMP2
    just pac-patch

# Apply all patches to all PAC submodules
pac-patch:
    #!/usr/bin/env bash
    set -e
    PATCHES_DIR="$(pwd)/scripts/pac/patches"

    for submodule_patches_dir in "$PATCHES_DIR"/*/; do
        submodule=$(basename "$submodule_patches_dir")
        target="submodules/$submodule"

        if [ ! -d "$target" ]; then
            echo "Skipping $submodule (not cloned yet)"
            continue
        fi

        patches=("$submodule_patches_dir"*.diff)
        if [ ! -e "${patches[0]}" ]; then
            echo "No patches for $submodule"
            continue
        fi

        echo "🔧 Patching $submodule..."
        for patch in "${patches[@]}"; do
            if git -C "$target" apply --check --reverse --ignore-whitespace "$patch"; then
                echo "$(basename "$patch") already applied"
                continue
            fi
            git -C "$target" apply --check --ignore-whitespace "$patch"
            git -C "$target" apply --ignore-whitespace "$patch"
        done
        echo "$submodule patched"
    done

# Apply patches for a specific PAC submodule
pac-patch-one submodule:
    #!/usr/bin/env bash
    set -e
    patches=(scripts/pac/patches/{{submodule}}/*.diff)
    if [ ! -e "${patches[0]}" ]; then
        echo "No patches found for {{submodule}}"
        exit 0
    fi
    for patch in "${patches[@]}"; do
        if git -C submodules/{{submodule}} apply --check --reverse --ignore-whitespace "$patch"; then
            echo "$(basename "$patch") already applied"
            continue
        fi
        git -C submodules/{{submodule}} apply --check --ignore-whitespace "$patch"
        git -C submodules/{{submodule}} apply --ignore-whitespace "$patch"
    done

# Reset all PAC submodules to upstream HEAD and re-apply patches
pac-reset:
    #!/usr/bin/env bash
    set -e
    echo "Resetting all PAC submodules to upstream HEAD..."
    git submodule foreach --recursive 'git am --abort 2>/dev/null || true'
    git submodule foreach --recursive 'git rebase --abort 2>/dev/null || true'
    git submodule update --init --force submodules/imx-sm submodules/mcux-devices-imx submodules/STM32CubeMP2
    just pac-patch

# Reset a specific PAC submodule and re-apply its patches
pac-reset-one submodule:
    git -C submodules/{{submodule}} reset --hard
    git -C submodules/{{submodule}} clean -fd
    just pac-patch-one {{submodule}}

# Fetch a patch from a GitHub PR and save it under scripts/pac/patches
pac-fetch-patch submodule owner repo pr:
    mkdir -p scripts/pac/patches/{{submodule}}
    curl -L https://github.com/{{owner}}/{{repo}}/pull/{{pr}}.diff \
        -o scripts/pac/patches/{{submodule}}/pr{{pr}}.diff
    echo "Saved to scripts/pac/patches/{{submodule}}/pr{{pr}}.diff"

# Check that all PAC patches apply cleanly (dry run)
pac-check:
    #!/usr/bin/env bash
    set -e
    PATCHES_DIR="$(pwd)/scripts/pac/patches"

    for submodule_patches_dir in "$PATCHES_DIR"/*/; do
        submodule=$(basename "$submodule_patches_dir")
        target="submodules/$submodule"
        [ ! -d "$target" ] && continue

        patches=("$submodule_patches_dir"*.diff)
        [ ! -e "${patches[0]}" ] && continue

        echo "🔍 Checking $submodule..."
        git -C "$target" apply --check --ignore-whitespace "${patches[@]}" \
            && echo "$submodule OK" \
            || echo "$submodule FAILED"
    done

# Show PAC submodule status
pac-status:
    git submodule status submodules/imx-sm submodules/mcux-devices-imx submodules/STM32CubeMP2

# Stage vendor CMSIS headers into ./data for the parsers
pac-prepare:
    bash scripts/pac/prepare.sh

# Parse STM32MP CMSIS headers into chiptool IR
pac-parse-stm32:
    python scripts/pac/parse_stm32.py --all

# Parse NXP MCUX CMSIS headers into chiptool IR
pac-parse-nxp:
    python scripts/pac/parse_nxp.py --all

# Parse all CMSIS headers (STM32 + NXP)
pac-parse-all: pac-parse-stm32 pac-parse-nxp
    @echo "✓ All parsers completed"

# Run custom PAC output validation tests
pac-test:
    python scripts/pac/test_output.py

# Validate PAC output with chiptool
pac-validate:
    python scripts/pac/validate_with_chiptool.py

# Show PAC validation summary
pac-summary:
    bash scripts/pac/test_summary.sh

# Full PAC pipeline: parse, test, and validate
pac-ci: pac-parse-all pac-test pac-validate
    @echo ""
    @echo "=========================================="
    @echo "PAC Pipeline Complete"
    @echo "=========================================="
    bash scripts/pac/test_summary.sh
