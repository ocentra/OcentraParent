# Protected Capability Custody Foundation Plan Next Actions

<!-- agent-capsule -->

> Plan: `protected-capability-custody-foundation-plan`
> Doc: Protected Capability Custody Foundation Plan Next Actions
> Kind: resume queue.
> Proves: routing only; it is not a completion certificate.

<!-- /agent-capsule -->

1. The FFI package and private `cfg(windows)` core adapter source is accepted
   and integrated at canonical `9375b0e10` from reviewed branch `8df832f2d`.
   Preserve the package-local unsafe boundary, private enrollment/peer/SCM/
   monotonic policy, and sealed core constructors. The graph records 99
   implementation files, 0 tests, and no workspace requirement gaps; this is
   not READY or DONE evidence.
2. Complete the protected runtime authority that remains unavailable:
   installer-owned TPM policy and non-exportable handle enrollment, with
   fail-closed `DeploymentRequired` before DB/state/listener mutation when the
   record is absent, contradictory, revoked, stale, or unavailable. WP01 owns
   the fixed BIN-only provisioner source and protected acceptance/opaque
   outcomes; Parent Runtime WP12 owns only package invocation and lifecycle.
3. Add the real enrolled production caller only after the protected owner and
   WP12 package boundaries exist. No second helper process/protocol,
   caller-supplied identity/attestation, raw `authValue`, disk generation
   restore, or caller-minted authority is allowed.
4. After production source is stable, write and execute all 13 expected test
   roots listed in `TEST_PROOF_EXPECTATIONS.md`, including the core-private
   Windows adapter and TPM2 NV/TBS monotonic-counter roots. Tests must exercise
   the real private seams and must not bless a disconnected helper, fake
   authority, or caller assertion.
5. Run the selected focused source/tests and Enforcer/architecture checks,
   then update checklist and retained proof. Repo-wide Enforcer, pre-commit,
   one PR, long CI, and promotion remain final gates.

## Explicit no-go actions

- Do not implement an in-process broker or a same-process DPAPI/file-lock
  substitute.
- Do not accept caller-supplied attestation, key choice, capability, lease, or
  success flags.
- Do not mark the route READY/DONE because the integrated source compiles or the
  graph observes its planned roots.
