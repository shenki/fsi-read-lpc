## Reading LPC firmware space over FSI

This is a small libpdbg application that reads LPC FW space on a Power9/Power10
using scoms. It can run on the BMC or the host.

It is implemented as a Rust application calling in to the C libpdbg API.

https://github.com/open-power/pdbg

### Cross compiling

The project includes copies of libfdt and libpdbg built for ARMv7 hard float,
and PowerPC 64bit little endian.

In `~/.cargo/config.toml`:
```
[target.arm-unknown-linux-gnueabihf]
linker = "arm-linux-gnueabihf-gcc"

[target.powerpc64le-unknown-linux-gnu]
linker = "powerpc64le-linux-gnu-gcc"
```

```
cargo build --target=arm-unknown-linux-gnueabihf
cargo build --target=powerpc64le-unknown-linux-gnu
```
