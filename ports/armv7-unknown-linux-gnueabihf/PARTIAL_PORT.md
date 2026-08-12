# Partial-port notice

The crate builds on stable Rust and its QEMU suite passes one closure test plus six end-to-end `ffi_call` tests. It is **not production-complete**:

1. translated `ffi_call` retains a 64 KiB backing-stack allocation per call;
2. the ARM VFP homogeneous-float-aggregate return regression test is ignored after reproducing a zero result;
3. the assigned EC2 instance was already shutting down before access, so work and tests ran in an ARM64 Ubuntu Docker fallback rather than EC2.

Exact commands, diagnostics, and logs are in `PORTING.md` and `logs/`.
