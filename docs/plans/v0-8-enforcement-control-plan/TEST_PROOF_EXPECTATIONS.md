# V0.8 Enforcement Control Plan Test and Proof Expectations

<!-- agent-capsule -->

> Agent Capsule
> Plan: `v0-8-enforcement-control-plan`
> Doc: `V0.8 Enforcement Control Plan Test and Proof Expectations`
> Kind: plan-local test and proof decision tree.
> Read when: After the assigned workpack/checklist row is known; use to choose required tests/proof.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: This file defines required local tests/proof; missing tests keep rows open.

<!-- /agent-capsule -->

Use this after the assigned enforcement workpack is known. Enforcement proof must separate policy decision, adapter capability, execution authority, rollback, and manual-required states.

## Where tests should live

When the enforcement implementation crate/package exists, tests belong under its test tree and proof output under its proof folder. Until then, colocate with the owning enforcement/domain/runtime package and record paths in the workpack and `PROOF_INDEX.md`.

## Decision Tree

| If the assigned work is...                                  | Read next                                       | Expected tests or proof                                                                               |
| ----------------------------------------------------------- | ----------------------------------------------- | ----------------------------------------------------------------------------------------------------- |
| WP01-WP03 contracts, evidence refs, adapter capability      | assigned workpack                               | schema negatives, branded ids, capability matrix, no execution without authority proof.               |
| WP04-WP08 app/game/browser/network/screen handoffs          | assigned workpack and owning plan only if named | consumer contract proof, dry-run/manual-required states, replay/idempotency, source custody.          |
| WP09-WP12 execution/rollback/audit paths                    | assigned workpack                               | privilege escalation negatives, rollback/unblock, audit trail, stale target, double-submit.           |
| WP13-WP16 platform authority and package/release boundaries | assigned workpack                               | platform manual proof, version/tag/package smoke, capability limitation notes.                        |
| WP17-WP20 proof/PR/rollout gates                            | `PROOF_INDEX.md`                                | complete proof manifest, selected risk rows, validation command logs, remaining manual-required gaps. |

## Expected test/proof inventory

Use these names as proof intent labels in the assigned workpack/proof note. Implementers choose the actual crate/package test names after the owning implementation boundary exists.

- `enforcement.policy-input.schema-negative`: policy/action inputs reject malformed, stale, unauthorized, and cross-child data.
- `enforcement.authz.privilege-escalation`: role and device authority prevent privilege escalation and cross-family action.
- `enforcement.adapter.capability-matrix`: adapter capability, unsupported, degraded, and manual-required states are explicit.
- `enforcement.execute.rollback-unblock`: execution proof includes rollback, unblock, cleanup, and stale-target handling.
- `enforcement.replay.idempotency-race`: duplicate, replayed, concurrent, and out-of-order actions are safe.
- `enforcement.audit.log-trace`: action, result, rollback, and parent-visible status emit safe audit/log/trace refs.
- `enforcement.canary.rollback-validation`: rollout/canary proof includes failure-mode and rollback validation.
- `enforcement.no-ai-direct-action`: AI/classification evidence cannot execute enforcement without deterministic policy authority.

## Required proof contents

- Policy input, authority tier, adapter capability, dry-run/execution result.
- Rollback/unblock cleanup evidence.
- Audit/log/trace refs for parent-visible action.
- Explicit manual-required state when proof is missing.

## Failure conditions

Do not claim DONE or PR_READY if any apply:

- The expected test/proof row for the touched work type is missing.
- The implementation crate/package test folder does not exist and the missing expected location is not recorded.
- Only happy-path tests pass for a trust, policy, persistence, protocol, UI, AI, platform, security, performance, or observability boundary.
- A product/checklist row moved without command logs and proof artifact path.
- A manual-required/platform limitation was converted into a runtime capability claim.
- A proof artifact lacks negative cases, logs/traces where relevant, or exact workpack/checklist linkage.
- A sibling plan or broad source tree was read without a route reason recorded in the workpack/proof note.
