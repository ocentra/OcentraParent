# Portal UX Household Surfaces Plan Test and Proof Expectations

<!-- agent-capsule -->

> Agent Capsule
> Plan: `portal-ux-household-surfaces-plan`
> Doc: `Portal UX Household Surfaces Plan Test and Proof Expectations`
> Kind: plan-local test and proof decision tree.
> Read when: After the assigned workpack/checklist row is known; use to choose required tests/proof.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: This file defines required local tests/proof; missing tests keep rows open.

<!-- /agent-capsule -->

Use this after the assigned portal UX workpack is known. Portal proof must use real service-backed state, not fake UI-only green paths.

## Where tests should live

When the portal/household implementation package exists, tests belong under its test tree and proof output under its proof folder. Until then, colocate with the owning portal package and record paths in the workpack and `PROOF_INDEX.md`.

## Decision Tree

| If the assigned work is...               | Read next                                | Expected tests or proof                                                                         |
| ---------------------------------------- | ---------------------------------------- | ----------------------------------------------------------------------------------------------- |
| Service-backed shell/navigation          | assigned workpack                        | route/DOM contract tests, real service smoke, empty/error/loading states.                       |
| First-run/profiles/household setup       | assigned workpack                        | authN/authZ, double-submit, stale session, cross-family isolation, misuse proof.                |
| Device inventory/selected device/context | assigned workpack                        | source/custody labels, offline/degraded/manual-required states, stale device negatives.         |
| Policy authoring/time budgets/templates  | assigned workpack                        | schema validation, authZ matrix, schedule/DST/clock skew, preview vs execution boundary.        |
| Reports/notifications/sync surfaces      | assigned workpack                        | read-model proof, notification state, retry/offline, audit trail, export/delete custody.        |
| AI/assistant portal surfaces             | assigned workpack; AI plan only if named | prompt/output invariant proof, redaction, human-review state, no AI authority escalation.       |
| Visual/UI polish or state rendering      | assigned workpack                        | Playwright screenshots for all touched states, accessibility-relevant assertions, no fake data. |
| PR/rollout gate                          | `PROOF_INDEX.md`                         | screenshot manifest, command logs, selected risk rows, remaining gaps.                          |

## Expected test/proof inventory

Use these names as proof intent labels in the assigned workpack/proof note. Implementers choose the actual crate/package test names after the owning implementation boundary exists.

- `portal.read-model.no-direct-source`: UI consumes approved read models and does not scan source systems directly.
- `portal.authz.visible-state-matrix`: parent/child/household/device visibility rejects cross-family or unauthorized state.
- `portal.empty-error-stale-degraded`: empty, error, stale, unsupported, and degraded states have screenshot proof.
- `portal.action.double-submit-replay`: parent actions reject duplicate, stale, refresh-abuse, and double-submit behavior.
- `portal.accessibility.keyboard-responsive`: core surfaces cover keyboard, responsive, focus, and accessible state expectations.
- `portal.logging.trace-proof`: UI actions and read-model loads produce safe logs/traces without leaking sensitive data.
- `portal.no-fake-data`: proof uses service-backed or contract-backed state, not hardcoded fake success data.

## Required proof contents

- Playwright/browser command and screenshot path for each touched state.
- Service/log/trace ref showing data came through the intended runtime path.
- Auth/custody negatives for profile, household, device, and policy surfaces.

## Failure conditions

Do not claim DONE or PR_READY if any apply:

- The expected test/proof row for the touched work type is missing.
- The implementation crate/package test folder does not exist and the missing expected location is not recorded.
- Only happy-path tests pass for a trust, policy, persistence, protocol, UI, AI, platform, security, performance, or observability boundary.
- A product/checklist row moved without command logs and proof artifact path.
- A manual-required/platform limitation was converted into a runtime capability claim.
- A proof artifact lacks negative cases, logs/traces where relevant, or exact workpack/checklist linkage.
- A sibling plan or broad source tree was read without a route reason recorded in the workpack/proof note.
