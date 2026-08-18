# Workpack 08 - Child Parent Authorized Uninstall

<!-- agent-capsule -->

> Agent Capsule
> Plan: `child-agent-runtime-distribution-plan`
> Workpack: `08-child-parent-authorized-uninstall`
> Kind: workpack route and proof contract.

<!-- /agent-capsule -->

Purpose: define parent-authorized uninstall, revocation, and removal proof for the child agent.

Status: source partial / production caller missing. Durable revocation source exists; Account-authority composition, platform cleanup callbacks, receipts, tests, validation, and proof remain open.

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

The public revoke, reauthorize, and tamper APIs have no production caller outside `child-runtime`. No Windows/macOS/Linux/Android package or device-owner callback consumes the result, performs bounded cleanup, and writes a durable idempotent cleanup receipt. Revocation state therefore blocks the in-process runtime boundary but does not execute platform removal.

## Required production source outcome

- consume Account WP08 current verified household authority without copying or minting it;
- invoke the existing child removal boundary through an authenticated parent/operator route;
- dispatch platform-specific cleanup callbacks only after durable revocation and retain idempotent cleanup receipts/residual-custody truth;
- integrate with WP07 lifecycle stop/disable/remove states and WP10 trusted runtime identity;
- keep unsupported/device-proof-required platforms explicit.

Implementation dependencies: Account WP08, Child WP10, and Child WP07 reviewed implementation. Normal READY/DONE remains strict.

## Expected test-source gap

- correct actor/household/child/device/action acceptance and wrong-target/action rejection;
- nonce/generation replay, stale authority, revoked trust, restart durability, and reauthorization;
- platform callback success/failure/retry/idempotency and crash-between-revocation-and-cleanup;
- residual custody and cleanup receipt truth per platform;
- proof that child/local signal input cannot self-authorize revocation or cleanup.

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
- [ ] production Account-authority caller invokes the child boundary
- [ ] platform cleanup callbacks and idempotent receipts are implemented
- [ ] platform uninstall and device-owner cleanup artifacts are validated
- [ ] contract tests and focused runtime tests are run
- [ ] proof artifacts are refreshed under the declared output root
- [ ] focused architecture validation is run for the touched Rust files
