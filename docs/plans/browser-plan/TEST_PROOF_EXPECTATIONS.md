# Browser Plan Test and Proof Expectations

<!-- agent-capsule -->

> Agent Capsule
> Plan: `browser-plan`
> Doc: `Browser Plan Test and Proof Expectations`
> Kind: plan-local test and proof decision tree.
> Read when: After the assigned workpack/checklist row is known; use to choose required tests/proof.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: This file defines required local tests/proof; missing tests keep rows open.

<!-- /agent-capsule -->

Use this after the assigned browser workpack is known. Browser proof must separate installed/running browser state, managed profile custody, active tab evidence, policy authoring, and intervention authority.

## Where tests should live

When the browser implementation crate/package exists, tests belong under its test tree and proof output under its proof folder. Until then, colocate with the owning browser/domain/runtime package and record paths in the workpack and `PROOF_INDEX.md`.

## Decision Tree

| If the assigned work is...              | Read next                                | Expected tests or proof                                                                                     |
| --------------------------------------- | ---------------------------------------- | ----------------------------------------------------------------------------------------------------------- |
| Browser inventory/read model            | assigned workpack                        | installed/running/supported/unsupported/managed/unmanaged states, portable/package cases, stale rows.       |
| Managed profile/custody/redaction       | assigned workpack                        | custody invariants, profile repair/restart, redaction proof, no private payload leakage.                    |
| Active tab/URL evidence                 | assigned workpack                        | active-state proof, unknown active-state negatives, URL normalization, redirect/URL-hijack, custody labels. |
| Policy authoring/settings manifest      | assigned workpack                        | schema fuzzing, value/update contracts, invalid setting negatives, source authority proof.                  |
| Browser intervention/control            | assigned workpack                        | authZ, manual-required, rollback, rate-limit/abuse, idempotency, audit trail.                               |
| Browser UI                              | assigned workpack                        | Playwright/e2e screenshots for managed/unmanaged, unsupported, degraded, empty/error states.                |
| Network/request surface                 | assigned workpack; validation matrix     | CORS/origin/header/host/redirect, request smuggling/desync/cache poisoning where request paths are touched. |
| Social/feed/cloud-game browser AI lanes | assigned workpack; AI plan only if named | prompt/output invariants, redaction, no AI authority proof, fixture regression.                             |

## Expected test/proof inventory

Use these names as proof intent labels in the assigned workpack/proof note. Implementers choose the actual crate/package test names after the owning implementation boundary exists.

- `browser.inventory.managed-unmanaged-states`: installed/running/supported/unsupported/managed/unmanaged states stay distinct.
- `browser.profile.custody-redaction`: profile custody and repair/restart proof does not leak private payloads.
- `browser.active-tab.url-normalization`: active tab evidence handles redirects, malformed URLs, and unknown active state.
- `browser.origin.header-security`: origin/header/host/redirect boundaries reject injection, splitting, hijack, and cache-poisoning cases where touched.
- `browser.policy.authz-idempotency`: browser interventions reject unauthorized, stale, replayed, and double-submit actions.
- `browser.rate-limit.abuse`: repeated intervention/settings calls are bounded and auditable.
- `browser.ui.managed-state-proof`: Playwright/screenshots cover managed, unmanaged, unsupported, empty, error, and degraded states.
- `browser.ai.evidence-only`: browser AI/social/video classifiers remain evidence-only and redacted.

## Required proof contents

- URL/tab evidence source and custody state.
- Negative tests for unknown/unsupported/manual-required cases.
- Browser screenshots/logs for UI behavior.
- Security proof for origin/header/redirect/request changes.

## Failure conditions

Do not claim DONE or PR_READY if any apply:

- The expected test/proof row for the touched work type is missing.
- The implementation crate/package test folder does not exist and the missing expected location is not recorded.
- Only happy-path tests pass for a trust, policy, persistence, protocol, UI, AI, platform, security, performance, or observability boundary.
- A product/checklist row moved without command logs and proof artifact path.
- A manual-required/platform limitation was converted into a runtime capability claim.
- A proof artifact lacks negative cases, logs/traces where relevant, or exact workpack/checklist linkage.
- A sibling plan or broad source tree was read without a route reason recorded in the workpack/proof note.
