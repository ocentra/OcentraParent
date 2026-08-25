# Protected Capability Custody Foundation Plan Next Actions

<!-- agent-capsule -->

> Plan: `protected-capability-custody-foundation-plan`
> Doc: Protected Capability Custody Foundation Plan Next Actions
> Kind: resume queue.
> Proves: routing only; it is not a completion certificate.

<!-- /agent-capsule -->

1. Preserve the accepted CNG/TPM mechanics, private `cfg(windows)` core
   adapter, and the WP01-owned BIN-only provisioner source integrated at
   canonical `a6d7d9adf`. The graph records 114 implementation files, 0 tests,
   and no workspace requirement gaps. The provisioner is only a read-only
   preflight: it revalidates enrollment and always returns
   `ExternalProvisioningRequired`; it cannot create or publish enrollment.
   This is not READY or DONE evidence.
2. Obtain the external protected runtime authority that remains unavailable:
   OEM/firmware/MDM authorization for `TPM_RH_PLATFORM` plus NV define/undefine
   lifecycle, authenticated owner handoff, protected registry/SCM mutation,
   enrolled counter generation, independent broker/client/token observations,
   and the core monotonic provider. Startup must remain fail-closed
   `DeploymentRequired` before DB/state/listener mutation; there is no reachable
   success path in the current checkout. Parent Runtime WP12 owns only package
   invocation and lifecycle.
3. Add real broker/client transport callers only after the protected owner and
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
