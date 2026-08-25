# Protected Capability Custody Foundation Plan Next Actions

<!-- agent-capsule -->

> Plan: `protected-capability-custody-foundation-plan`
> Doc: Protected Capability Custody Foundation Plan Next Actions
> Kind: resume queue.
> Proves: routing only; it is not a completion certificate.

<!-- /agent-capsule -->

1. Implement the dependency-owned safe Windows adapters required by the
   integrated broker source: pinned `OpenProcess` identity plus impersonated
   token SID/integrity/session observation, exact registry owner/DACL and parent
   chain validation, and a real non-restorable monotonic provider. Do not widen
   visibility of the sealed core authority to obtain those observations.
2. Add installer/SCM-owned immutable enrollment for the broker identity, client
   SID/image, registry roots, and service configuration, then compose the first
   real enrolled production caller. The broker, not the client or SQLite
   replica, must own protected decisions and opaque admission.
3. Preserve fail-closed behavior before any storage, registry, journal,
   listener, bootstrap, or service-ready mutation for missing identity, wrong
   owner, revoked or stale generation, path escape, replay, restart ambiguity,
   unavailable broker, and unsupported platforms.
4. After source is complete, write the full expected test family listed in
   `TEST_PROOF_EXPECTATIONS.md`: core-owned binding/storage/state/path/
   reconciliation units, protocol wire-contract tests, broker authority/race/
   Windows process tests, and client admission/IPC-authentication tests.
5. Run focused core/protocol/broker/client compilation, source-shape/Enforcer
   checks, and the selected tests. Then
   update the checklist and retained proof; repo-wide Enforcer, pre-commit, one
   PR, long CI, and promotion remain final gates.

## Explicit no-go actions

- Do not implement an in-process broker or a same-process DPAPI/file-lock
  substitute.
- Do not accept caller-supplied attestation, key choice, capability, lease, or
  success flags.
- Do not mark the route READY/DONE because the integrated source compiles or the
  graph observes its packages.
