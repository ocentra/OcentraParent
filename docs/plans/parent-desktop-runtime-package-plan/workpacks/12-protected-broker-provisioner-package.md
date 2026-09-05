# Workpack 12 - Protected Broker Provisioner Package

<!-- agent-capsule -->

> Plan: `parent-desktop-runtime-package-plan`
> Workpack: `12-protected-broker-provisioner-package`
> Kind: parent-side installer and package route.
> Proves: the package ownership boundary, expected artifact roots, and open
> contract recorded below.
> Does not prove: installer implementation, protected authority, enrollment,
> tests, proof, signing, runtime availability, PR readiness, or DONE.

<!-- /agent-capsule -->

## Purpose

Define the parent-side Windows package boundary for the protected broker and its
installer-only invocation/lifecycle handoff. This is a planned/source-authorable
route for the bounded package mechanics; normal derived completion remains
blocked while package source, tests, proof, signing, and runtime evidence are
absent. Protected WP01 is the accepted
neutral core/FFI/protocol foundation; WP02 owns the fixed Enrollment/SCM/TPM
owner transaction, WP03 owns monotonic/anti-rollback currentness, and WP04
owns the client anchor/fixed-pipe transport. Those owners and their external
prerequisites remain separate from this package route. WP12 may invoke the
fixed owner-approved binary but cannot establish protected authority or make a
package ready.

WP12 produces the installed broker/provisioner artifact and package boundary
that Protected WP04 later consumes for client anchor and fixed-pipe transport.
WP12 does not consume or use WP04 source and has no reverse dependency.

## Ownership boundary

Parent Client Runtime Distribution owns:

- the parent-side MSI/WiX artifact and package identity;
- the elevated, installer-only custom-action/provisioner invocation boundary;
- build/release wiring and package artifact metadata;
- install, repair, upgrade, rollback, uninstall, and explicit deprovisioning
  lifecycle outcomes for this parent artifact.

Protected Capability Custody WP01 owns the neutral private core/FFI mechanics,
neutral protocol, sealed admission/transcript boundary, and fail-closed
foundation. WP02 owns the fixed protected Enrollment/SCM/TPM owner transaction,
enrollment provenance, and owner-approved provisioner handoff; WP03 owns the
monotonic/anti-rollback provider; WP04 owns the private client anchor and fixed
pipe transport/session. The package may invoke the fixed owner-approved
provisioner, but WP12 may not mint, parse, transport, or expose protected
authority and may not substitute package success for any of those owner
boundaries.

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

The BIN-only provisioner source is owned and mapped by Protected WP02. WP12
does not own its Cargo manifest, `src/main.rs`, or private
`src/provisioning/` directory; it invokes and packages the fixed owner-approved
binary. It has no `src/lib.rs` and no public API. Its operation is fixed by the
Protected WP02/package boundary and may depend on the Windows FFI only as the
approved owner-side consumer alongside the protected core. The provisioner
does not accept a caller/MSI-provided path, TPM index or policy, `authValue`,
SID, image identity, generation, lease, capability, or success assertion.

The parent package scripts directory is the ownership boundary for the future
custom action, installer-only binary invocation, package lifecycle/build
helpers, and upgrade/rollback/uninstall coordination. Generated MSI, checksum,
and signing outputs belong under `target/release-packages/`; they are release
artifacts, not committed source. The existing child-agent
`scripts/release/windows/OcentraParentAgent.wxs` and its build script remain
outside this workpack.

## Secure provisioner contract

- Provisioning runs only from an elevated, installer-owned install/repair or
  explicitly owner-approved deprovisioning context.
- The BIN-only provisioner performs one fixed installer-owned operation through
  private `src/provisioning/` modules; it does not expose a library, public
  callable API, or general-purpose FFI wrapper to broker/client/other callers.
- No untrusted MSI property, command-line argument, setup field, environment
  value, or parent caller may supply an `authValue`, TPM index, TPM policy, SID,
  path, image identity, SCM identity, generation, lease, capability, or success
  result.
- A raw TPM `authValue` is not a package, registry, log, command-line, or
  caller-visible field. The approved TPM policy and non-exportable handle remain
  inside the protected owner/FFI boundary; the package transports only the
  minimum installer lifecycle result needed to report failure or continuation.
- The provisioner must establish or verify only the installer-owned immutable
  enrollment owned by Protected WP02. The package cannot self-enroll a
  caller, widen an ACL, select a replacement key/index, or turn a disk/SQLite
  record into authority.
- Missing, contradictory, revoked, stale, or unavailable protected enrollment
  fails closed before the package reports a protected broker as ready.
- The fixed TPM NV index/policy is not `TPM_RH_PLATFORM` hierarchy authority;
  LocalSystem, elevation, TBS, PCP signing, or an OS account cannot substitute
  for the external OEM/firmware/MDM owner ceremony. Empty or caller-supplied
  authorization is forbidden, and the current protected preflight therefore has
  no reachable success path.

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
the no-claim boundary between package outcome and Protected WP02/Protected
authority.
Later retained proof belongs under
`output/parent-client-runtime-distribution-plan-proof/12-protected-broker-provisioner-package/`.

## Failure conditions

Keep WP12 open and blocked if it edits the child-agent installer, accepts raw
authority inputs, exposes TPM or registry secrets, creates a caller-mintable
enrollment API, reports protected readiness from MSI success, restores disk
generation during rollback, silently removes enrollment on uninstall, or lacks
real package/lifecycle tests and retained proof. No READY, PR_READY, or DONE
claim follows from this routing packet.

## Routing refresh — 2026-08-25

WP12 remains the installer-only package and invocation lifecycle owner. Its
bounded WiX/build/package source slice is authorizable now, while normal
derived completion remains blocked. Its
complete expected production boundary is:

```text
scripts/release/windows/parent-protected-custody/
scripts/release/windows/parent-protected-custody.wxs
scripts/release/windows/build-parent-protected-custody-package.ps1
```

Its expected tests are the package test directory and
`tests/repo-tooling/parent-protected-custody-package.test.mjs`. The package
may invoke the fixed Protected owner-approved BIN and report bounded package
failure/continuation outcomes, but it cannot mint or transport protected
authority, provide a signer/store lease, or accept raw `authValue`, TPM
index/policy, SID, path, image, generation, lease, capability, or success
inputs. No package artifact, signing result, checksum, or service registration
promotes Protected, Account, setup, or runtime readiness. Source, tests, proof,
signing, and DONE remain open. The zero-argument provisioner must preserve its
current `ExternalProvisioningRequired`/manual-required behavior; no service
start or readiness claim is added.
