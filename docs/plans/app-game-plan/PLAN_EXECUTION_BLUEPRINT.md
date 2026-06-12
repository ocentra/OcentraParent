# App + Game Plan � HID Execution Blueprint

## Execution objective

Split game/app behavior into explicit runtime, policy, and approval boundaries with enforceable audit proof.

## Slice 01 � Game Runtime and Inventory Contracts

### Acceptance

- Runtime model, foreground states, and launcher ownership are contract-defined.

### Tests

- `app-game.policy.authz-replay`
- `app-game.no-fake-green`

### Proof

- `docs/proof/app-game-plan/slice-01-runtime-contract.md`

## Slice 02 � Approval and Family Authority

### Acceptance

- Parent actions, parent/co-parent overrides, and stale actions are handled by policy boundaries.

### Tests

- `app-game.policy.authz-replay`
- `app-game.approval-state-machine`

### Proof

- `docs/proof/app-game-plan/slice-02-approval-boundary.md`

## Slice 03 � Platform Execution Capability Matrix

### Acceptance

- Platform adapters are capability-typed with safe unavailable/manual-required states.

### Tests

- `app-game.platform.capability-matrix`
- `app-game.platform.rollback`

### Proof

- `docs/proof/app-game-plan/slice-03-platform-capability.md`

## Slice 04 � Journal and State Replay

### Acceptance

- Replays and journal reads do not duplicate or reorder visible game action state incorrectly.

### Tests

- `app-game.journal.replay-ordering`

### Proof

- `docs/proof/app-game-plan/slice-04-journal.md`

## Workpacks (execution lane)

### Slice-to-workpack binding

- Slice 01: docs/plans/app-game-plan/workpacks/01-contract-boundary-and-effect-schemas.md
- Slice 02: docs/plans/app-game-plan/workpacks/02-source-index-and-doc-reconciliation.md
- Slice 03: docs/plans/app-game-plan/workpacks/03-current-app-game-snapshot-and-gap-map.md
- Slice 04: docs/plans/app-game-plan/workpacks/04-app-game-identity-model.md

## PR-ready gate

- No execution path can claim enforcement authority until policy-authority and audit trail proofs are linked.

## HID test floor (this plan)

### Required test families for closed slice

- Unit: runtime/action contracts and authority model checks
- Integration: policy authoring + service/portal read model updates
- E2E: approval and parent action journeys
- Security: privilege boundary and anti-replay behavior
- Non-functional: platform capability matrix and rollback

### Mandatory slice evidence checks

- negative cases documented (at least one per slice)
- rollback/teardown proof recorded
- proof manifest references command output, artifacts, and manual review notes
