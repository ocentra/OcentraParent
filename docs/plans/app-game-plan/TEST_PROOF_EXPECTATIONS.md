# App + Game Plan Test and Proof Expectations

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `App + Game Plan Test and Proof Expectations`
> Kind: plan-local test and proof decision tree.
> Read when: After the assigned workpack/checklist row is known; use to choose required tests/proof.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: This file defines required local tests/proof; missing tests keep rows open.

<!-- /agent-capsule -->

Use this after the assigned app/game workpack is known. App/game proof must separate inventory, runtime, foreground, launcher, identity, policy, approval, timer, and UI claims.

## Where tests should live

When the app/game implementation crate/package exists, tests belong under its test tree and proof output under its proof folder. Until then, colocate with the owning domain/runtime package and record paths in the workpack and `PROOF_INDEX.md`.

## Decision Tree

| If the assigned work is...                                   | Read next                                | Expected tests or proof                                                                                                  |
| ------------------------------------------------------------ | ---------------------------------------- | ------------------------------------------------------------------------------------------------------------------------ |
| Source/snapshot/gap or doc routing                           | `DOC_INDEX.md`, exact rows               | doc link sanity, source coverage proof, no product status move without runtime evidence.                                 |
| Identity, inventory, runtime, foreground, launcher contracts | assigned workpack; source-boundary flow  | schema negatives, branded ids, Rust parity, inventory-is-not-use, runtime-is-not-foreground, foreground-is-not-content.  |
| Journal/SQLite/read models                                   | assigned workpack; `PROOF_INDEX.md`      | ingest/replay, migration/rollback, ordering, idempotency, read-model differential proof.                                 |
| Unknown approval, parent actions, app/game authority         | assigned workpack; policy expectations   | authZ matrix, replay, stale candidate, expiry, manual-required blocks, privilege escalation negatives.                   |
| Budget/timer/policy compiler                                 | assigned workpack                        | schedule/DST/clock skew, restart recovery, bonus-time audit refs, dry-run-only/manual-required proof.                    |
| Child-facing or parent portal UI                             | assigned workpack; UI guide only         | Playwright/e2e screenshots for warning, approval, limit, denied, manual-required, unavailable, error/empty states.       |
| Platform adapter/execution                                   | assigned workpack                        | capability matrix, platform manual proof, rollback/unblock/unshield proof, no execution without authority-tier evidence. |
| AI classifier/digest                                         | assigned workpack; AI plan only if named | output schema invariants, weak candidate negatives, no AI enforcement authority, redaction proof.                        |
| Notification/local outbox/audit                              | assigned workpack                        | local outbox payload-minimization, delivery eligibility, audit history, retry/idempotency.                               |
| Rollout/PR gate                                              | `PROOF_INDEX.md`                         | complete proof pack 00-12 where required by checklist, selected risk rows, validation commands.                          |

## Expected test/proof inventory

Use these names as proof intent labels in the assigned workpack/proof note. Implementers choose the actual crate/package test names after the owning implementation boundary exists.

- `app-game.identity.schema-negative-decode`: app/game ids, aliases, launchers, and catalog refs reject malformed or ambiguous input.
- `app-game.inventory.not-usage`: installed inventory is not counted as runtime, foreground, or usage.
- `app-game.runtime.not-foreground`: process/runtime evidence does not claim active use without foreground proof.
- `app-game.launcher.ambiguity`: launcher and child game candidates preserve uncertainty until stronger evidence exists.
- `app-game.session.ordering-idempotency`: session transitions handle duplicate, stale, missing, and out-of-order events.
- `app-game.policy.authz-replay`: parent policy updates reject unauthorized, stale, replayed, or cross-child actions.
- `app-game.platform.manual-required`: unsupported or unproven platform capabilities stay manual-required.
- `app-game.ui.degraded-proof`: parent/child surfaces show empty, stale, unsupported, and limitation states with screenshot/log proof.
- `app-game.no-fake-green`: proof uses real contracts/adapters/read models, not mocked scanner or policy success.

## Required proof contents

- Negative proof for every boundary claim: inventory/use, runtime/foreground, foreground/content, AI/enforcement.
- Journal/read-model rows where persistence is touched.
- Screenshots and Playwright logs for UI states.
- Authority tier, rollback, and manual-required proof for adapter behavior.

## Failure conditions

Do not claim DONE or PR_READY if any apply:

- The expected test/proof row for the touched work type is missing.
- The implementation crate/package test folder does not exist and the missing expected location is not recorded.
- Only happy-path tests pass for a trust, policy, persistence, protocol, UI, AI, platform, security, performance, or observability boundary.
- A product/checklist row moved without command logs and proof artifact path.
- A manual-required/platform limitation was converted into a runtime capability claim.
- A proof artifact lacks negative cases, logs/traces where relevant, or exact workpack/checklist linkage.
- A sibling plan or broad source tree was read without a route reason recorded in the workpack/proof note.
