# Protected Capability Custody Foundation Plan Next Actions

<!-- agent-capsule -->

> Plan: `protected-capability-custody-foundation-plan`
> Doc: Protected Capability Custody Foundation Plan Next Actions
> Kind: resume queue.
> Proves: routing only; it is not a completion certificate.

<!-- /agent-capsule -->

1. Add the planned `ocentra-protected-capability-custody-windows-ffi` package
   as the only package-scoped unsafe module. It may contain only raw Win32,
   TBS, TPM2, and owned-handle wrappers, with `unsafe_op_in_unsafe_fn` denied
   and all other lint denies retained. It must not own custody decisions,
   enrollment, persistence, or a caller interface.
2. Add the planned safe
   `ocentra-protected-capability-custody-windows` package with a small opaque
   adapter interface. It must retain/revalidate pipe/process/token handles,
   SID/integrity/session, image+SCM identity, exact registry owner/protected
   DACL/ACE/ancestor chain, nonce/expiry/replay, and TPM2 NV/TBS monotonic
   generation. Do not widen visibility of the sealed core authority.
3. Link that adapter into the existing broker in-process at one
   dispatch/open-session seam, then add installer/SCM-owned immutable
   pre-provisioning and the first real enrolled production caller. No second
   helper process/protocol, caller-supplied identity/attestation, or disk
   generation restore is allowed.
4. Preserve fail-closed behavior before any storage, registry, journal,
   listener, bootstrap, or service-ready mutation for missing identity, wrong
   owner, revoked or stale generation, path escape, replay, restart ambiguity,
   unavailable broker, and unsupported platforms.
5. After all production source is complete, write the full expected test family
   listed in `TEST_PROOF_EXPECTATIONS.md`: retain the 11 core/protocol/broker/
   client roots, then add the absent Windows adapter custody and TPM2 NV/TBS
   monotonic-counter roots. Tests must exercise the real interface and must not
   bless a disconnected helper, fake authority, or caller assertion.
6. Run focused core/protocol/broker/client/Windows-adapter compilation,
   source-shape/Enforcer checks, and the selected tests. Then
   update the checklist and retained proof; repo-wide Enforcer, pre-commit, one
   PR, long CI, and promotion remain final gates.

## Explicit no-go actions

- Do not implement an in-process broker or a same-process DPAPI/file-lock
  substitute.
- Do not accept caller-supplied attestation, key choice, capability, lease, or
  success flags.
- Do not mark the route READY/DONE because the integrated source compiles or the
  graph observes its packages.
