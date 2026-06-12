# Tracking Plan Test and Proof Expectations

<!-- agent-capsule -->

> Agent Capsule
> Plan: `tracking-plan`
> Doc: `Tracking Plan Test and Proof Expectations`
> Kind: plan-local test and proof decision tree.
> Read when: After the assigned workpack/checklist row is known; use to choose required tests/proof.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: This file defines required local tests/proof; missing tests keep rows open.

<!-- /agent-capsule -->

Use this file only after `AGENTS.md`, `PLAN_STATE.md`, `WORKPACK_INDEX.md`, and the assigned tracking workpack are known. It is the local decision tree for tests and proof. Do not open full checklists or checkpoints unless this file or `PROOF_INDEX.md` points there.

## Where tests should live

When the tracking implementation crate/package exists, tracking tests belong under that plan/feature implementation test tree, with proof output under that implementation's proof/output folder. Until that lands, colocate tests with the owning domain package/crate and record the path in the workpack and `PROOF_INDEX.md`.

## Decision Tree

| If the assigned work is...                                           | Read next                                             | Expected tests or proof                                                                                                                                                                  |
| -------------------------------------------------------------------- | ----------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| WP01-WP02 source, snapshot, gap map, routing docs                    | `DOC_INDEX.md`, exact source rows only                | `tracking-doc-link-sanity`, `tracking-source-coverage-gap-proof`; no product status move without evidence.                                                                               |
| WP03-WP07 evidence, status, capability, retention, custody contracts | `CHECKLIST_INDEX.md` exact rows; owning domain README | schema decode/encode negatives, branded id tests, invariant tests for no raw location/device strings, retention/delete/export contract negatives.                                        |
| WP08-WP13 Android/iOS/desktop adapters                               | assigned workpack; platform expectation docs only     | platform adapter integration smoke, permission/capability matrix, degraded/manual-required states, offline/last-known-only negatives, battery/low-power behavior, manual artifact proof. |
| WP14-WP18 geofence, expected-place, acknowledgement, child check-in  | assigned workpack; expectation docs named there       | transition ordering, clock skew/DST, replay/idempotency, double-submit, stale location, parent override/exception, child misuse proof.                                                   |
| WP19-WP22 nearby-place/POI/taxonomy/local place DB                   | assigned workpack; provider docs named there          | provider schema fuzzing, ambiguity/risk taxonomy invariants, cache/custody proof, quota/rate-limit, no direct enforcement from weak POI evidence.                                        |
| WP23-WP24 AI location safety/provider routing                        | assigned workpack; AI expectation only if named       | prompt-injection, hallucination regression, output schema invariants, redaction, provider fallback, temperature sensitivity, no AI authority-to-enforce proof.                           |
| WP25-WP29 policy, alerts, escalation, temporary live, missing-device | assigned workpack; notification/policy expectations   | authZ matrix, replay/idempotency, alert severity invariants, escalation ordering, token/session lifecycle, rate limit/abuse, rollback/expiry proof.                                      |
| WP30 parent/child UI                                                 | assigned workpack; UI guide only                      | Playwright/e2e real state proof, empty/error/offline/degraded states, permission states, screenshots, logging/trace refs for clicked/acknowledged flows.                                 |
| WP31-WP33 platform proof, journal/SQLite, rollout gate               | `PROOF_INDEX.md`, exact checklist rows                | migration/rollback/schema drift, journal replay, read-model differential proof, proof artifact completeness, PR gate validation list.                                                    |

## Expected test/proof inventory

Use these names as proof intent labels in the assigned workpack/proof note. Implementers choose the actual crate/package test names after the owning implementation boundary exists.

- `tracking.location.schema-negative-decode`: location/device/geofence contracts reject malformed or ambiguous inputs.
- `tracking.permission.manual-required`: missing permission, background restriction, battery saver, and platform limitation states remain explicit.
- `tracking.geofence.transition-invariants`: enter/exit/dwell transitions handle jitter, duplicates, stale samples, and ordering.
- `tracking.schedule.clock-dst-boundary`: expected-place schedules handle timezone, DST, expiry, and clock skew.
- `tracking.session.idempotency-replay`: location/session/read-model ingestion is idempotent under replay and partial outage.
- `tracking.authz.family-isolation`: parent/child/device access rejects cross-family and stale-token cases.
- `tracking.alert.rate-limit-escalation`: alerts are rate-limited, severity-bounded, and auditable.
- `tracking.ui.proof-screenshot-log`: parent/child surfaces show empty, stale, degraded, and manual-required states with screenshots/logs.

## Required proof contents

- Command logs for every validation command.
- Negative-case output, not only passing happy path.
- Proof artifact path and exact workpack/checklist row updated.
- For UI proof: screenshot path plus Playwright/browser command and relevant log/trace reference.
- For platform proof: OS/device/version, permission/enrollment state, adapter output, limitation/manual-required note.
- For AI proof: input fixture, prompt/output fixture, schema validation result, safety/redaction result.

## Failure conditions

Do not claim DONE or PR_READY if any apply:

- The expected test/proof row for the touched work type is missing.
- The implementation crate/package test folder does not exist and the missing expected location is not recorded.
- Only happy-path tests pass for a trust, policy, persistence, protocol, UI, AI, platform, security, performance, or observability boundary.
- A product/checklist row moved without command logs and proof artifact path.
- A manual-required/platform limitation was converted into a runtime capability claim.
- A proof artifact lacks negative cases, logs/traces where relevant, or exact workpack/checklist linkage.
- A sibling plan or broad source tree was read without a route reason recorded in the workpack/proof note.
