# One-shot defmt host reader

This Linux host utility maps map 0 of a UIO device, drains every packet currently
available in its consortium CDBG ring, decodes the defmt stream using the 16-byte-aligned,
compile-time embedded MCU release ELF at
`examples/melt-pot/stm32mp25/mcu/target/thumbv8m.main-none-eabihf/release/melt-pot-stm32mp25-mcu_original.elf`,
prints the messages, and exits. It does not wait for interrupts or poll for later
messages.

The UIO device ID defaults to `0`. An optional argument selects another numeric suffix
of `/dev/uioN` (pass `N`, not the device path). The process needs permission to open
that device.

```sh
cargo run --manifest-path examples/utils/defmt-host/Cargo.toml
cargo run --manifest-path examples/utils/defmt-host/Cargo.toml -- 3
```

Reading advances the ring's host-owned read pointer. Calling the utility again only
prints messages written since the previous drain.

A ring may start with a truncated defmt fragment after overflow or a previous reader.
The utility reports malformed packet data to stderr and continues decoding the remaining
packets rather than abandoning the drain.
