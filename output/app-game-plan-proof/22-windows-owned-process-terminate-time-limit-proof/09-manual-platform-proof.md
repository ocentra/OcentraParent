# Manual Platform Proof

Platform: Windows (`win32`)

Proof command:

```text
node scripts/test/v0-8-windows-app-time-limit-adapter-mvp.mjs
```

Observed on 2026-06-03:

- Real Rust service built and launched on loopback.
- WebSocket commands created, recovered, cancelled, rejected, and expired
  app time-limit timer state.
- Owned/current child process expiry returned `expired/process-terminated`.
- Dry-run returned `would-enforce/dry-run-no-action`.
- Stale action mismatch returned `enforcement-active-timer-state-mismatch`.

Manual-required/no-claim boundary:

- This is not package-wide app blocking, AppLocker/App Control proof, install
  blocking, launch allowlisting, or cross-platform enforcement proof.
