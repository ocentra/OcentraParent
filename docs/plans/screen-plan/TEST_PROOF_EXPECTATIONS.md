# Screen Plan Test and Proof Expectations

<!-- agent-capsule -->

> Agent Capsule
> Plan: `screen-plan`
> Doc: `Screen Plan Test and Proof Expectations`
> Kind: plan-local test and proof decision tree.
> Read when: After the assigned workpack/checklist row is known; use to choose required tests/proof.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: This file defines required local tests/proof; missing tests keep rows open.

<!-- /agent-capsule -->

Use this after the assigned screen workpack or checklist row is known. Screen proof must separate capture permission, raw retention, OCR/VLM quality, browser/live trigger, live view, and parent explanation claims.

## Where tests should live

When the screen implementation crate/package exists, tests belong under its test tree and proof output under its proof folder. Until then, colocate with the owning screen/domain/runtime package and record paths in the workpack and `PROOF_INDEX.md`.

## Decision Tree

| If the assigned work is...         | Read next                                | Expected tests or proof                                                                                    |
| ---------------------------------- | ---------------------------------------- | ---------------------------------------------------------------------------------------------------------- |
| Capture scope/permission/retention | assigned workpack or exact checklist row | permission matrix, raw retention opt-in/out, delete/export, protected-surface negatives, custody proof.    |
| Platform capture adapter           | assigned workpack                        | OS/device/version, manual permission proof, degraded/manual-required states, rollback/cleanup.             |
| OCR/VLM/detector                   | assigned workpack                        | fixture regression, quality/resource measurement, output invariants, invalid-output handling.              |
| Browser/live trigger or scheduler  | assigned workpack                        | trigger authenticity, idempotency, replay, ordering, service-started subscriber proof.                     |
| Live view/relay/cache              | assigned workpack                        | authZ, token lifecycle, origin/header, connection exhaustion, timeout/cleanup, privacy prompt screenshots. |
| Parent UI/explanation              | assigned workpack                        | Playwright screenshots, redacted summaries, empty/error/degraded states, log/trace refs.                   |
| Security/privacy/legal boundary    | assigned workpack                        | protected surface proof, no raw leakage, redaction, privacy/legal approval note.                           |

## Expected test/proof inventory

Use these names as proof intent labels in the assigned workpack/proof note. Implementers choose the actual crate/package test names after the owning implementation boundary exists.

- `screen.capture.permission-manual-required`: platform permission/enrollment gaps stay manual-required.
- `screen.capture.custody-redaction`: screenshot custody, retention, deletion, and redaction paths are proved.
- `screen.surface.inventory-boundaries`: window/surface inventory does not claim content understanding.
- `screen.trigger.clock-boundary`: capture triggers handle timing, throttling, clock skew, and stale signals.
- `screen.storage.retention-tombstone`: deletion and retention replay produce durable tombstone/read-model proof.
- `screen.platform.adapter-proof`: platform adapters record OS/version/permission/output and limitations.
- `screen.no-ai-enforcement-claim`: capture proof does not imply OCR/VLM, policy, or enforcement success.

## Required proof contents

- Screenshot or capture artifact path when visual proof is claimed.
- Redaction/custody result and retention setting.
- Platform/device limitation notes.
- Command logs and selected risk rows.

## Failure conditions

Do not claim DONE or PR_READY if any apply:

- The expected test/proof row for the touched work type is missing.
- The implementation crate/package test folder does not exist and the missing expected location is not recorded.
- Only happy-path tests pass for a trust, policy, persistence, protocol, UI, AI, platform, security, performance, or observability boundary.
- A product/checklist row moved without command logs and proof artifact path.
- A manual-required/platform limitation was converted into a runtime capability claim.
- A proof artifact lacks negative cases, logs/traces where relevant, or exact workpack/checklist linkage.
- A sibling plan or broad source tree was read without a route reason recorded in the workpack/proof note.
