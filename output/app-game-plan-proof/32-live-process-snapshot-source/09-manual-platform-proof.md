# Manual Platform Proof

WP32 uses the real local `sysinfo` process table in unit/integration-style Rust
tests and asserts the current test process is visible exactly once.

Limitations:

- This is source-boundary proof, not service subscription proof.
- No foreground window source is claimed.
- No platform authority tier is promoted.
- No manual OS setup or adapter execution was required.
