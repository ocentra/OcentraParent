# Security Negative Proof

Negative contract cases covered by focused tests:

- Unknown game-like executable candidates remain weak evidence and cannot use a
  deny fallback as if the executable were a proved game.
- Reason-ref-backed child request states must cite child reason references.
- Allow-once decisions must expire.
- Replayable and replayed approvals must cite audit references.
- Storage-unavailable decisions cannot claim audit-backed replay.
- Manual-required block outcomes cannot include an enforcement result.
- Unknown app and unknown game-like proof cannot dispatch unsupported adapters.

No-claim boundary:

This proof does not grant new hard-control authority. Supported platform
blocking still requires adapter proof, rollback proof, audit proof, and service
integration in later slices.

