# Workpack 08 - Child Parent Authorized Uninstall

<!-- agent-capsule -->

> Agent Capsule
> Plan: `child-agent-runtime-distribution-plan`
> Workpack: `08-child-parent-authorized-uninstall`
> Kind: workpack route and proof contract.

<!-- /agent-capsule -->

Purpose: define parent-authorized uninstall, revocation, and removal proof for the child agent.

Status: complete. WP08 now has canonical contract, thin consumer read-model wiring, real tests, focused validation, and a proof pack under `output/child-agent-runtime-distribution-plan-proof/08-child-parent-authorized-uninstall/`.

## Owns

- parent-authorized uninstall flow
- revocation and removal state
- no-child-self-authorize removal rule
- uninstall cleanup and audit trail

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

## Proof Root

- `output/child-agent-runtime-distribution-plan-proof/08-child-parent-authorized-uninstall/`
- runtime evidence: `test-results/tamper-uninstall-artifact-status-proof/proof.json`

## Proved States

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

## Validations

- `cmd /c npm exec --workspace @ocentra-parent/schema-domain -- vitest run tests/proof/tamper-uninstall-artifact-status.test.ts`
- `cmd /c npm exec --workspace @ocentra-parent/enforcement-domain -- vitest run tests/unit/tamper-uninstall-artifact-status.test.ts`
- `cmd /c node scripts/test/tamper-uninstall-artifact-status-proof.mjs`
- `cmd /c npm run lint:architecture -- --files packages/schema-domain/src/tamper-uninstall-artifact-status.ts packages/schema-domain/tests/proof/tamper-uninstall-artifact-status.test.ts packages/enforcement-domain/src/tamper-uninstall-artifact-status-read-model.ts packages/enforcement-domain/tests/unit/tamper-uninstall-artifact-status.test.ts packages/enforcement-domain/vitest.config.ts scripts/test/tamper-uninstall-artifact-status-proof.mjs`

## Completion Checklist

- [x] parent-authorized uninstall flow is represented in the canonical contract
- [x] revocation state is explicit and leaves trust inactive until parent reauthorization
- [x] no-child-self-authorize removal rule is explicit
- [x] uninstall cleanup remains visible until cleanup proof exists
- [x] teardown proof ends child authority cleanly
- [x] thin `enforcement-domain` consumer reflects the contract without becoming the owner
- [x] real schema-domain proof test exists in `tests/proof`
- [x] real enforcement-domain unit test exists in `tests/unit`
- [x] focused proof runner emits proof JSON and proof labels
- [x] proof artifacts exist under the declared output root
- [x] no-claim boundaries stay explicit and honest
- [x] focused architecture validation passed for the touched files
