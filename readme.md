# lrOS — Minimal AArch64 hobby kernel

lrOS is a minimal no_std, no_main hobby kernel for AArch64 (QEMU `virt`) that demonstrates:
- Bare-metal entry from assembly (src/boot.S)
- A small PL011 UART driver in Rust (src/lib.rs)
- Custom linker script (linker.ld)
- Build configured for `aarch64-unknown-none` (Cargo.toml, .cargo/config.toml)

Quickstart
1. Ensure toolchain and QEMU:
   - Install Rust (stable) and the `aarch64-unknown-none` target or configure your cross toolchain per project requirements.
   - Install QEMU (with aarch64 support).

2. Build:
   ./run.sh
   (script runs `cargo build --release` and then starts QEMU)

3. Run:
   The script starts QEMU with `-nographic` and `-serial stdio`. Kernel output (UART) appears in the terminal.

Key files
- src/boot.S — early assembly boot code, sets up stack and calls `kmain`.
- src/lib.rs — Rust entry (kmain), simple UART driver and panic handler.
- linker.ld — link layout; entry symbol `_start`.
- .cargo/config.toml — custom target and linker arguments.
- run.sh — convenience script to build and run under QEMU.

Notes and troubleshooting
- The project targets bare metal; debugging messages appear through the PL011 UART.
- If you see no output, verify QEMU invocation and that the UART base address matches your QEMU machine.
- For formatted panic messages the code implements `core::fmt::Write`; to change behavior edit the panic handler in src/lib.rs.
- If linking fails, ensure the linker script path is correct in .cargo config and that your linker supports the script.

License
- No license specified. Add a LICENSE file if needed.

Contributing
- Make changes in the src directory and update the linker script or cargo config as needed.
