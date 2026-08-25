# Workpack 12 - Protected Broker Provisioner Package

<!-- agent-capsule -->

> Plan: `parent-client-runtime-distribution-plan`
> Workpack: `12-protected-broker-provisioner-package`
> Kind: parent-side installer and package route.
> Proves: the package ownership boundary, expected artifact roots, and open
> contract recorded below.
> Does not prove: installer implementation, protected authority, enrollment,
> tests, proof, signing, runtime availability, PR readiness, or DONE.

<!-- /agent-capsule -->

## Purpose

Define the parent-side Windows package boundary for the protected broker and its
installer-only enrollment/provisioner handoff. This workpack is a routing
contract for the package mechanics that are currently absent; it is not an
installer implementation or a substitute for protected-custody authority.

## Ownership boundary

Parent Client Runtime Distribution owns:

- the parent-side MSI/WiX artifact and package identity;
- the elevated, installer-only custom-action/provisioner invocation boundary;
- build/release wiring and package artifact metadata;
- install, repair, upgrade, rollback, uninstall, and explicit deprovisioning
  lifecycle outcomes for this parent artifact.

Protected Capability Custody WP01 owns the private core/FFI Windows adapter,
enrollment provenance, exact registry/SCM/peer validation, TPM policy and
non-exportable-handle validation, and opaque admission/transcript outcomes.
The package may invoke an approved owner-side provisioner, but it may not mint,
parse, transport, or expose protected authority.

Setup owns the user-facing setup journey and readiness state. The child-agent
distribution plan owns child MSI/package lifecycle. Existing child-agent WiX,
service, and release files are not claimed by this workpack.

## Expected production roots

The planned parent-side package roots are:

```text
scripts/release/windows/parent-protected-custody/
scripts/release/windows/parent-protected-custody.wxs
scripts/release/windows/build-parent-protected-custody-package.ps1
```

The directory is the ownership boundary for the future custom action,
installer-only provisioner invocation, package lifecycle/build helpers, and
upgrade/rollback/uninstall coordination. Generated MSI, checksum, and signing
outputs belong under `target/release-packages/`; they are release artifacts,
not committed source. The existing child-agent
`scripts/release/windows/OcentraParentAgent.wxs` and its build script remain
outside this workpack.

## Secure provisioner contract

- Provisioning runs only from an elevated, installer-owned install/repair or
  explicitly owner-approved deprovisioning context.
- No untrusted MSI property, command-line argument, setup field, environment
  value, or parent caller may supply an `authValue`, TPM index, TPM policy, SID,
  path, image identity, SCM identity, generation, lease, capability, or success
  result.
- A raw TPM `authValue` is not a package, registry, log, command-line, or
  caller-visible field. The approved TPM policy and non-exportable handle remain
  inside the protected owner/FFI boundary; the package transports only the
  minimum installer lifecycle result needed to report failure or continuation.
- The provisioner must establish or verify only the installer-owned immutable
  enrollment expected by Protected WP01. The package cannot self-enroll a
  caller, widen an ACL, select a replacement key/index, or turn a disk/SQLite
  record into authority.
- Missing, contradictory, revoked, stale, or unavailable protected enrollment
  fails closed before the package reports a protected broker as ready.

## Upgrade, rollback, and uninstall contract

- Repair and upgrade are idempotent with respect to package identity and do not
  reset or restore the TPM-backed generation from disk state.
- A failed upgrade has a bounded rollback/teardown result; it must not silently
  restore an older broker identity, registry owner/DACL, SCM identity, or
  protected enrollment snapshot.
- Uninstall does not silently delete protected enrollment or custody state. Any
  deprovisioning requires the explicit protected owner path and records a
  manual-required/fail-closed outcome when that path is unavailable.
- Package success, MSI exit status, checksum, signing status, or service
  registration alone never upgrades protected admission, runtime readiness, or
  downstream Account/Device Trust authority.

## Required later evidence

The implementation/test wave must add and exercise the real package boundary at
these expected test roots:

```text
scripts/release/windows/parent-protected-custody/tests/
tests/repo-tooling/parent-protected-custody-package.test.mjs
```

The focused family must cover package identity and artifact wiring, elevated
custom-action input rejection, absence of raw `authValue` transport, repair /
upgrade idempotency, rollback failure, uninstall/deprovisioning safety, and
the no-claim boundary between package outcome and Protected WP01 authority.
Later retained proof belongs under
`output/parent-client-runtime-distribution-plan-proof/12-protected-broker-provisioner-package/`.

## Failure conditions

Keep WP12 open and blocked if it edits the child-agent installer, accepts raw
authority inputs, exposes TPM or registry secrets, creates a caller-mintable
enrollment API, reports protected readiness from MSI success, restores disk
generation during rollback, silently removes enrollment on uninstall, or lacks
real package/lifecycle tests and retained proof. No READY, PR_READY, or DONE
claim follows from this routing packet.
