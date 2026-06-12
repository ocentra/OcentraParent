<!-- agent-capsule -->

> Agent Capsule
> Plan: `data-custody-storage-plan`
> Doc: `TEST_PROOF_EXPECTATIONS.md`
> Kind: plan-local test and proof decision tree.
> Read when: After the assigned workpack is known; use to choose required tests/proof.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: This file defines required local tests/proof; missing tests keep rows open.

<!-- /agent-capsule -->

# Data Custody Storage Plan Test and Proof Expectations

Use this after one workpack is selected. These are proof intents and expected behavior names, not implementation recipes.

## Where Tests Should Live

When the implementation crate/package exists, tests belong under that feature or product-domain test tree and proof output belongs under that feature's proof/output folder. Until then, colocate with the owning domain package/crate and record the path in the workpack and PROOF_INDEX.md.

## Decision Tree

| If the assigned work is... | Expected tests or proof                                                                   |
| -------------------------- | ----------------------------------------------------------------------------------------- |
| Source/acceptance map      | feature-route consistency, expectation coverage, status no-overclaim proof.               |
| Current state/gap map      | gap-state proof, manual-required proof, no product completion claim.                      |
| Contract/boundary shape    | schema negatives, branded ids, protocol/read-model compatibility, no duplicate truth.     |
| Runtime/user flow handoff  | real service or UI path proof, degraded states, logs/traces/screenshots where applicable. |
| Rollout/PR gate            | proof manifest, selected risk rows, validation command logs, skipped-risk notes.          |

## Expected Test/Proof Inventory

- data-custody.source.acceptance-route: feature, expectation, plan, checklist, and proof rows agree.
- data-custody.contract.schema-negative: invalid or ambiguous domain/protocol/read-model inputs fail closed.
- data-custody.authz.family-device-boundary: parent/child/device role and cross-family access are rejected where applicable.
- data-custody.replay.idempotency-ordering: duplicate, stale, replayed, and out-of-order actions or evidence are safe.
- data-custody.degraded.manual-required: unsupported, missing permission, unavailable provider, or platform gaps remain explicit.
- data-custody.ui.state-proof: user-visible states have empty/error/stale/degraded proof and screenshots when UI is touched.
- data-custody.observability.redaction: logs, metrics, traces, and alerts are present and redact sensitive data where applicable.
- data-custody.rollout.pr-gate: DONE/PR_READY includes proof artifacts, commands, updated rows, known gaps, and adjacent-plan handoffs.

## Failure Conditions

Do not claim DONE or PR_READY if any apply:

- Expected proof intent is missing for a touched risk.
- Only happy-path evidence exists for trust, policy, persistence, protocol, UI, AI, platform, security, performance, or observability.
- Product status moved without feature/checklist/proof updates.
- Adjacent implementation completion is claimed from this plan alone.
- A code recipe was added instead of expected outcome, boundary, shape, validation, or proof language.
