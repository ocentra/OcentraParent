# Workpack 08 - Child Parent Authorized Uninstall

<!-- agent-capsule -->

> Agent Capsule
> Plan: `child-agent-runtime-distribution-plan`
> Workpack: `08-child-parent-authorized-uninstall`
> Kind: workpack route and proof contract.

<!-- /agent-capsule -->

Purpose: define parent-authorized uninstall, revocation, and removal proof for the child agent.

Status: production code drafted / test-deferred. The child service now owns a durable parent-authorized revocation boundary and preserves removal audit state; platform uninstall/device-owner cleanup remains manual-required. Contract tests, validation, and proof are deferred to the later global phase.

## Owns

- parent-authorized uninstall flow
- revocation and removal state
- no-child-self-authorize removal rule
- uninstall cleanup and audit trail

## Production code boundary

- `ocentra-child-runtime` persists `removal-state.json` under the service-owned durable root.
- Revocation and reauthorization require `VerifiedParentRemovalAuthorization`, which can only be constructed from the existing verified household-authority contract; its reference is retained as evidence and is not authority by itself.
- The service must be configured with the matching household, child-profile, and target-device identity; unbound proofs and proofs for another child are rejected before state mutation.
- Revocation changes typed service readiness to `Revoked`, closes observed-event ingress, and retains the audit trail across restart.
- Windows/Linux/macOS package managers own service stop/remove hooks; Android package/device-owner removal remains manual-required. No platform uninstall proof is claimed.
- Windows MSI removes the child service/files while retaining the service-owned ProgramData custody root so `removal-state.json` and its audit history are not silently discarded; Linux and macOS package hooks likewise do not claim to delete durable custody.
- The deferred Windows lifecycle harness still expects ProgramData absence and uses legacy parent service/path labels; that proof-only mismatch must be reconciled before validation.

## Must prove

- the child cannot self-authorize trust removal
- parent authorization is required where the platform allows uninstall control
- revocation leaves an auditable removal trail
- teardown proof shows child authority ends cleanly

## Failure conditions

- stealth persistence is treated as success
- child self-uninstall authority is implied
- revoked trust remains active
- removal proof is kept only in the plan folder

## Deferred proof root

- `output/child-agent-runtime-distribution-plan-proof/08-child-parent-authorized-uninstall/`
- runtime evidence: `test-results/tamper-uninstall-artifact-status-proof/proof.json`

## Contract/proof states to validate later

- `child-self-authorize-forbidden`
- `required-where-platform-allows`
- `inactive-until-parent-reauthorizes`
- `audit-trail-required`
- `authority-ends-cleanly-when-removal-is-proved`
- `reported-until-cleanup-proof`

## No-Claim Boundary

- no stealth persistence claim
- no anti-tamper resistance claim
- no privilege-escalation claim
- no admin-removal blocking claim
- no provider-delivery claim
- no raw child-data custody claim
- no parent-client parity claim
- no uninstall-control parity claim where the platform still remains manual-required or device-proof-required

## Deferred validations (not run in this production pass)

- `cmd /c npm exec --workspace @ocentra-parent/schema-domain -- vitest run tests/proof/tamper-uninstall-artifact-status.test.ts`
- `cmd /c npm exec --workspace @ocentra-parent/enforcement-domain -- vitest run tests/unit/tamper-uninstall-artifact-status.test.ts`
- `cmd /c node scripts/test/tamper-uninstall-artifact-status-proof.mjs`
- `cmd /c npm run lint:architecture -- --files packages/schema-domain/src/tamper-uninstall-artifact-status.ts packages/schema-domain/tests/proof/tamper-uninstall-artifact-status.test.ts packages/enforcement-domain/src/tamper-uninstall-artifact-status-read-model.ts packages/enforcement-domain/tests/unit/tamper-uninstall-artifact-status.test.ts packages/enforcement-domain/vitest.config.ts scripts/test/tamper-uninstall-artifact-status-proof.mjs`

## Production-pass checklist

- [x] child service owns durable parent-authorized revocation and reauthorization state
- [x] revoked trust is represented as typed service readiness and blocks runtime ingress
- [x] revocation audit entries remain durable until a parent reauthorization decision
- [x] platform package/device removal remains explicit manual-required state
- [ ] platform uninstall and device-owner cleanup artifacts are validated
- [ ] contract tests and focused runtime tests are run
- [ ] proof artifacts are refreshed under the declared output root
- [ ] focused architecture validation is run for the touched Rust files
