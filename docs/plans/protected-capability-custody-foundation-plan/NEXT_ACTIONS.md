# Protected Capability Custody Foundation Plan Next Actions

<!-- agent-capsule -->

> Plan: `protected-capability-custody-foundation-plan`
> Doc: Protected Capability Custody Foundation Plan Next Actions
> Kind: resume queue.
> Proves: routing only; it is not a completion certificate.

<!-- /agent-capsule -->

1. Assign the implementation-only source packet for WP01 on a fresh physical
   E: worktree. Keep the existing core unchanged unless a reviewed source
   boundary requires a narrow correction.
2. Establish a real isolated Windows broker process and a client boundary with
   authenticated OS IPC. The broker, not the client or SQLite replica, must own
   ACL/path/key/watermark/write-lease decisions and the opaque admission/factory.
3. Preserve fail-closed behavior for missing identity, wrong owner, revoked or
   stale generation, path escape, replay, restart ambiguity, unavailable broker,
   and unsupported platforms.
4. After source is complete, write the full expected test family listed in
   `TEST_PROOF_EXPECTATIONS.md`: binding/storage/state, path/replica security,
   reconciliation, broker races, and Windows broker custody integration.
5. Run focused crate/source-shape/Enforcer checks and the selected tests. Then
   update the checklist and retained proof; repo-wide Enforcer, pre-commit, one
   PR, long CI, and promotion remain final gates.

## Explicit no-go actions

- Do not implement an in-process broker or a same-process DPAPI/file-lock
  substitute.
- Do not accept caller-supplied attestation, key choice, capability, lease, or
  success flags.
- Do not mark the route READY/DONE because the existing core or graph map has
  source files.
