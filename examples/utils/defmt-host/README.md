# One-shot defmt host reader

This Linux host utility maps map 0 of a UIO device, drains every packet currently
available in its consortium CDBG ring, decodes the defmt stream using the firmware ELF
given on the command line, prints the messages, and exits. It does not wait for
interrupts or poll for later messages.

Pass the firmware's `*_original.elf` (the one carrying the `.defmt` table; `csti build`
stages it as `dist/firmware/<core>_original.elf`). The optional second argument selects
the UIO device: a bare number is `/dev/uioN`; anything else is matched against
`/sys/class/uio/*/name`, exactly or as `<name>@<unit-address>`. It defaults to `dbg-shm`,
the `linux,uio-name` `consortium-cfg` gives the debug-ring node. The process needs
permission to open that device.

```sh
cargo build --release --manifest-path examples/utils/defmt-host/Cargo.toml
consortium-example-defmt-host cm33_original.elf            # the `dbg-shm` device
consortium-example-defmt-host cm33_original.elf 2          # /dev/uio2
consortium-example-defmt-host cm33_original.elf dbg-shm-uio
```

Reading advances the ring's host-owned read pointer. Calling the utility again only
prints messages written since the previous drain.

A ring may start with a truncated defmt fragment after overflow or a previous reader.
The utility reports malformed packet data to stderr and continues decoding the remaining
packets rather than abandoning the drain.
