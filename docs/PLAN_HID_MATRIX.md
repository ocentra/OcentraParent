# 19-Plan High-Information-Density Execution Matrix

## Objective

The goal is to make every plan executable by an AI or developer without context debt:

1. Read the minimum documents.
2. Choose the exact slice.
3. Run only the tests needed for that slice.
4. Record proof artifacts before any state/checklist moves.
5. Repeat until every failure path is resolved.

This matrix is the execution reference for all 19 plans in this branch.

## Research-backed test guardrails used in this branch

- RFC 9110 HTTP Semantics for method safety/idempotency and replay behavior: https://www.rfc-editor.org/rfc/rfc9110.html
- OWASP API Security Top 10 (2023) for API risk prioritization and authorization/rate-limiting expectations: https://owasp.org/API-Security/editions/2023/en/
- OWASP Fuzzing guidance for malformed-input and parser hardening: https://owasp.org/www-community/Fuzzing
- OWASP Web Security Testing Guide for test structuring and proof evidence style: https://owasp.org/www-project-web-security-testing-guide/
- NIST SP 800-218 Secure Software Development Framework for governance and security-depth expectations: https://csrc.nist.gov/pubs/sp/800/218/final
- Playwright testing artifact guidance for reproducible UI proof: https://playwright.dev/docs/test-configuration
- Playwright tracing and screenshot guidance: https://playwright.dev/docs/api/class-tracing
- Hypothesis property-based testing patterns for contract/property validation: https://hypothesis.readthedocs.io/
- Stryker mutation testing workflow for mutation score checks when test depth is weak: https://stryker-mutator.io/docs/

## Global execution invariants

- No plan checkbox may move to checked without proof evidence linked from `docs/proof/<plan>/`.
- No checklist claims green if only happy-path tests are documented.
- Every slice must include:
  - unit/integration boundary where applicable
  - at least one negative path test (authz/replay/abuse)
  - one non-functional assertion (logging, tracing, alert, or rate boundary)
  - one persistence/rollback or teardown check
- No `TODO` or hand-wavy acceptance text may block execution.

## Plan-by-plan precision matrix

1. account-identity-family-plan

- Current posture: first-pass, not execution-closed.
- Missing precision: provider migration criteria, household/domain authority matrix, token replay teardown proofs.
- Required test families: authN/authZ matrix, token lifecycle, rate-limit/replay, abuse, observability.
- Required proof bundle:
  - `docs/proof/account-identity-family-plan/slice-01-*.md`
  - `docs/proof/account-identity-family-plan/slice-02-*.md`
  - `docs/proof/account-identity-family-plan/slice-03-*.md`
  - `docs/proof/account-identity-family-plan/slice-04-*.md`
  - `docs/proof/account-identity-family-plan/PLAN_PROOF_MANIFEST.md`

2. ai-plan

- Current posture: documented but not fully execution-closed.
- Missing precision: explicit output invariants and safety boundary tests for every AI path.
- Required test families: contract-parse/property-based, prompt-injection, hallucination regression, result invariants, rate-abuse.
- Required proof bundle:
  - `docs/plans/ai-plan/PLAN_EXECUTION_BLUEPRINT.md`
  - `docs/proof/ai-plan/PLAN_PROOF_MANIFEST.md`

3. app-plan

- Current posture: scope docs exist, requires hardened boundary proof.
- Missing precision: app-family boundary contract proofs and state replay tests.
- Required test families: contract/schema negative, authZ matrix, replay/idempotency, observability.
- Required proof bundle:
  - `docs/plans/app-plan/PLAN_EXECUTION_BLUEPRINT.md`
  - `docs/proof/app-plan/PLAN_PROOF_MANIFEST.md`

4. app-game-plan

- Current posture: high checklist coverage but still implementation gaps.
- Missing precision: policy authority split and execution audit proofs.
- Required test families: authZ replay, launcher/integration, no fake-green, rollback.
- Required proof bundle:
  - `docs/plans/app-game-plan/PLAN_EXECUTION_BLUEPRINT.md`
  - `docs/proof/app-game-plan/PLAN_PROOF_MANIFEST.md`

5. browser-plan

- Current posture: progress present with open gaps.
- Missing precision: managed/unmanaged profile distinction, request/headers boundary proof, policy rollback scope.
- Required test families: authZ, idempotency/replay, schema negative, request security, rate-limit.
- Required proof bundle:
  - `docs/plans/browser-plan/PLAN_EXECUTION_BLUEPRINT.md`
  - `docs/proof/browser-plan/PLAN_PROOF_MANIFEST.md`

6. data-custody-storage-plan

- Current posture: first-pass and missing full custody boundaries.
- Missing precision: schema boundary split, retention/tombstone semantics, export/import custody model.
- Required test families: schema parse negative, retention/delete, encryption boundary, sync consistency.
- Required proof bundle:
  - `docs/plans/data-custody-storage-plan/PLAN_EXECUTION_BLUEPRINT.md`
  - `docs/proof/data-custody-storage-plan/PLAN_PROOF_MANIFEST.md`

7. eventing-plan

- Current posture: near-complete but one/one open risk rows.
- Missing precision: consumer product claims and version-skew guards.
- Required test families: replay/idempotency, ordering/dead-letter, schema drift/compatibility.
- Required proof bundle:
  - `docs/plans/eventing-plan/PLAN_EXECUTION_BLUEPRINT.md`
  - `docs/proof/eventing-plan/PLAN_PROOF_MANIFEST.md`

8. lan-plan

- Current posture: execution-gap heavy, many workpacks open.
- Missing precision: zero-trust discovery and signed peer lifecycle evidence.
- Required test families: authN/authZ, discovery partial-outage, lease lifecycle, audit.
- Required proof bundle:
  - `docs/plans/lan-plan/PLAN_EXECUTION_BLUEPRINT.md`
  - `docs/proof/lan-plan/PLAN_PROOF_MANIFEST.md`

9. network-plan

- Current posture: checklist dense, proof flow still open.
- Missing precision: parser boundary proof, policy-service decoupling proof, request security.
- Required test families: schema fuzzing, request smuggling/desync, authZ/replay, rate-limit DoS.
- Required proof bundle:
  - `docs/plans/network-plan/PLAN_EXECUTION_BLUEPRINT.md`
  - `docs/proof/network-plan/PLAN_PROOF_MANIFEST.md`

10. parent-client-runtime-distribution-plan

- Current posture: no implementation checklist baseline.
- Missing precision: service/launcher contracts, signed artifact evidence, rollback path.
- Required test families: local-service smoke, signing/rollback, origin/header security.
- Required proof bundle:
  - `docs/plans/parent-desktop-runtime-package-plan/PLAN_EXECUTION_BLUEPRINT.md`
  - `docs/proof/parent-desktop-runtime-package-plan/PLAN_PROOF_MANIFEST.md`

11. payment-subscription-plan

- Current posture: first-pass with pricing/provider decisions pending.
- Missing precision: subscription lifecycle matrix, webhook idempotency, recovery states.
- Required test families: checkout auth/session matrix, webhook replay, rate-limit/abuse, failure and fallback.
- Required proof bundle:
  - `docs/plans/payment-subscription-plan/PLAN_EXECUTION_BLUEPRINT.md`
  - `docs/proof/payment-subscription-plan/PLAN_PROOF_MANIFEST.md`

12. policy-control-plane-plan

- Current posture: first-pass, no unified compiler path yet.
- Missing precision: policy source-of-truth, conflict resolution, delivery/replay boundaries.
- Required test families: property tests for policy compilation rules, authZ boundaries, alert/observability, rollback.
- Required proof bundle:
  - `docs/plans/policy-control-plane-plan/PLAN_EXECUTION_BLUEPRINT.md`
  - `docs/proof/policy-control-plane-plan/PLAN_PROOF_MANIFEST.md`

13. portal-ux-household-surfaces-plan

- Current posture: planning-heavy and UI-facing.
- Missing precision: service-backed data dependencies, visible state matrix, error-state visual proof.
- Required test families: authZ visible-state matrix, e2e state transitions, double-submit/refresh abuse.
- Required proof bundle:
  - `docs/plans/portal-ux-household-surfaces-plan/PLAN_EXECUTION_BLUEPRINT.md`
  - `docs/proof/portal-ux-household-surfaces-plan/PLAN_PROOF_MANIFEST.md`

14. remote-access-plan

- Current posture: first-pass and safety-critical split.
- Missing precision: relay reliability, control vs view separation, session teardown proof.
- Required test families: authZ, replay-idempotency, retry storm/partial outage, audit/log redaction.
- Required proof bundle:
  - `docs/plans/remote-access-plan/PLAN_EXECUTION_BLUEPRINT.md`
  - `docs/proof/remote-access-plan/PLAN_PROOF_MANIFEST.md`

15. screen-ai-pipeline-plan

- Current posture: high checklist count, few workpacks still open.
- Missing precision: trigger-to-capture boundary and OCR/VLM output invariants.
- Required test families: schema/invariant tests, prompt injection, hallucination regression, redaction/PII.
- Required proof bundle:
  - `docs/plans/screen-ai-pipeline-plan/PLAN_EXECUTION_BLUEPRINT.md`
  - `docs/proof/screen-ai-pipeline-plan/PLAN_PROOF_MANIFEST.md`

16. screen-plan

- Current posture: implementation workpacks remain while checklist lines read mostly complete.
- Missing precision: capture/custody chain, delete/tombstone policy, policy handoff readiness.
- Required test families: permission/auth lifecycle, OCR invariants, retention-delete, event sequencing.
- Required proof bundle:
  - `docs/plans/screen-plan/PLAN_EXECUTION_BLUEPRINT.md`
  - `docs/proof/screen-plan/PLAN_PROOF_MANIFEST.md`

17. setup-install-provisioning-plan

- Current posture: first-pass state machine and first-run flow.
- Missing precision: family-site to install-to-runtime handoff and recovery/fallback proofs.
- Required test families: onboarding auth flow, platform matrix install tests, first-run recovery, manual fallback.
- Required proof bundle:
  - `docs/plans/setup-install-provisioning-plan/PLAN_EXECUTION_BLUEPRINT.md`
  - `docs/proof/setup-install-provisioning-plan/PLAN_PROOF_MANIFEST.md`

18. tracking-plan

- Current posture: partial implementation and many open workpacks.
- Missing precision: consent boundaries, platform adapter parity, geofence and ordering invariants.
- Required test families: geofence invariants, session replay/ordering, alert/escalation, canary rollback.
- Required proof bundle:
  - `docs/plans/tracking-plan/PLAN_EXECUTION_BLUEPRINT.md`
  - `docs/proof/tracking-plan/PLAN_PROOF_MANIFEST.md`

19. v0-8-enforcement-control-plan

- Current posture: first-pass and execution-checklist missing.
- Missing precision: policy input contracts, adapter authority matrix, canary + rollback evidence.
- Required test families: contract schema negative, privilege boundary, replay/race, fallback execution proof.
- Required proof bundle:
  - `docs/plans/v0-8-enforcement-control-plan/PLAN_EXECUTION_BLUEPRINT.md`
  - `docs/proof/v0-8-enforcement-control-plan/PLAN_PROOF_MANIFEST.md`

## Cross-plan execution order (risk-minimizing)

1. account-identity-family-plan
2. policy-control-plane-plan
3. portal-ux-household-surfaces-plan
4. parent-client-runtime-distribution-plan
5. remote-access-plan
6. network-plan
7. lan-plan
8. data-custody-storage-plan
9. setup-install-provisioning-plan
10. payment-subscription-plan
11. tracking-plan
12. browser-plan
13. app-plan
14. app-game-plan
15. screen-plan
16. screen-ai-pipeline-plan
17. ai-plan
18. eventing-plan
19. v0-8-enforcement-control-plan

## Exit gate before PR-ready in any plan

- `PLAN_STATE.md` + `PLAN_EXECUTION_BLUEPRINT.md` read and updated for the assigned slice.
- `PLAN_PROOF_MANIFEST.md` updated with:
  - run log entry
  - negative-case evidence
  - test bundle location
- At least one negative-path proof and one rollback/recovery proof captured per slice.
  - If a failure was observed, the failure and correction is documented and re-run evidence is attached.

## Plan-level execution floor (non-negotiable before slice close)

Each closed slice must include at least these proof blocks:

1. `docs/proof/<plan>/slice-XX-<scope>.md` — command/result log with pass and fail state.
2. `docs/proof/<plan>/PLAN_PROOF_MANIFEST.md` — manifest includes:
   - command transcript
   - evidence artifact paths (test logs, screenshots, traces, exports)
   - negative-case coverage index
3. Explicit teardown/rollback note for every tested negative or abuse path.

Per-plan minimum families that must be present in slice evidence:

1. account-identity-family-plan

- unit: session/token contract/unit parser
- integration: household-authz and role boundary flows
- e2e: registration/login + recovery UI/flow handoff
- security: authN/authZ matrix, replay, rate-limit/retry abuse
- non-functional: observability and audit trail assertions

2. ai-plan

- unit: parser/schema + prompt boundary invariant tests
- integration: provider runtime routing + queue/dead-letter paths
- e2e: policy handoff and evidence route
- security: prompt-injection, output invariants, hallucination regression
- non-functional: fallback/timeout and resource behavior

3. app-plan

- unit: domain contract decode/encode + input safety
- integration: state ordering and family boundary transitions
- e2e: app setup/app-family visibility
- security: privilege and authZ boundaries
- non-functional: logs/traces and retry behavior

4. app-game-plan

- unit: contract/schema and launcher-state transitions
- integration: policy engine + service read model updates
- e2e: approval/parent-action journeys
- security: replay/race, privilege boundary, unavailable states
- non-functional: platform capability and rollback checks

5. browser-plan

- unit: setting schema and request profile validation
- integration: policy and intervention lifecycle
- e2e: managed/unmanaged runtime paths
- security: header/hop/hijack, request smuggling/ desync, open-redirect
- non-functional: rate-limit and retry resilience

6. data-custody-storage-plan

- unit: schema-negative and ownership decoding
- integration: sync/export/import pipelines
- e2e: retention/delete and recovery workflow proof
- security: data corruption/replay and sync boundary checks
- non-functional: integrity checks and cleanup windows

7. eventing-plan

- unit: envelope parse and version schema
- integration: replay/dead-letter consumption and consumer parity
- e2e: event bridge recovery path under restart
- security: ordering/replay abuse and version skew
- non-functional: throughput and drift/error budget proof

8. lan-plan

- unit: discovery metadata and lease contract validation
- integration: pairing, trust tokens, heartbeat lifecycle
- e2e: physical two-device claim/revoke run
- security: signed hello, anti-replay, authz abuse
- non-functional: partial outage and retry behavior

9. network-plan

- unit: metadata schema and parser contracts
- integration: policy-service decoupled consumption
- e2e: network enforcement and route fallback checks
- security: request security, host/origin header tests, smuggling probes
- non-functional: request latency, memory/FD and backlog pressure

10. parent-client-runtime-distribution-plan

- unit: route and bootstrap contracts
- integration: service smoke + launcher readiness
- e2e: install/provision + restart + rollback
- security: origin/header/policy boundary
- non-functional: artifact integrity and startup stability

11. payment-subscription-plan

- unit: product/pricing config and webhook event schema
- integration: subscription state machine + entitlement persistence
- e2e: checkout/login lifecycle and manual recovery flow
- security: webhook replay, fraud/retry abuse, privilege separation
- non-functional: pricing/error budget and settlement consistency

12. policy-control-plane-plan

- unit: compiler/decision schemas
- integration: policy delivery and override surfaces
- e2e: policy authoring/parent override journeys
- security: conflict resolution, authZ boundaries, replay
- non-functional: rollback and override audit proof

13. portal-ux-household-surfaces-plan

- unit: domain contract decode and route state
- integration: live portal data consumption and polling/state transitions
- e2e: household-facing flows including loading/error paths
- security: visible state authZ and sensitive-data redaction
- non-functional: UI rendering and accessibility smoke

14. remote-access-plan

- unit: relay and grant command schema checks
- integration: control/view separation and session lifecycle
- e2e: remote control/session teardown with failure recovery
- security: authN/authZ replay, logging/redaction, abuse paths
- non-functional: retry storm behavior and partial outage handling

15. screen-ai-pipeline-plan

- unit: capture trigger schema + OCR/model output invariants
- integration: result queue and policy handoff contract
- e2e: capture→analysis→decision journey
- security: prompt/input injection, PII redaction, output invariants
- non-functional: throughput and queue backpressure

16. screen-plan

- unit: policy route and OCR parser contracts
- integration: capture, retention, and handoff services
- e2e: child→parent visibility and deletion workflows
- security: permission lifecycle and replay ordering
- non-functional: event sequencing and storage lifecycle

17. setup-install-provisioning-plan

- unit: account/install state machine contracts
- integration: web→runtime handoff and platform detection
- e2e: registration, install, permission, and recovery path
- security: onboarding abuse and unauthorized state entry
- non-functional: install matrix and manual fallback evidence

18. tracking-plan

- unit: location/session schema and geofence model validation
- integration: adapter capability and permission matrices
- e2e: consent flow, alert/escalation, missing-device path
- security: geofence/enforcement replay, role/tenant separation
- non-functional: event ordering and migration/rollback checks

19. v0-8-enforcement-control-plan

- unit: enforcement input/adapter schema boundaries
- integration: policy execution and portal state consumption
- e2e: parent-visible status + service fallback path
- security: privilege boundary, replay/race, manual-required modes
- non-functional: canary rollout, rollback and observability
