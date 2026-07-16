<!-- agent-capsule -->

> Agent Capsule
> Plan: `screen-plan`
> Doc: `Screen Plan Workpack Families`
> Kind: owner-path classifier for selected workpacks.
> Read when: only after `WORKPACK_INDEX.md` selects or names a workpack and owner/proof family is unclear.
> Stop rule: classify the selected workpack only; do not use this file as permission to scan every workpack.
> Proves: routing and owner-path classification only.
> Does not prove: capture readiness, AI readiness, policy readiness, enforcement readiness, custody readiness, live-view readiness, remote-access readiness, or PR readiness.
> Proof rule: if this file changes route/status claims, update `AGENTS.md`, `PLAN_STATE.md`, and any affected selected workpack route.

<!-- /agent-capsule -->

# Screen Plan Workpack Families

Use this file to classify a selected workpack before opening source. This plan owns local screen evidence/capture/custody/settings/live-view-boundary work. It consumes or hands off to AI, policy, enforcement, custody, portal, remote access, and domain-source plans without owning their runtime behavior.

## Docs, route, and source reconciliation family

```text
Workpacks:
WP01 Source Index And Doc Reconciliation
WP02 Current Screen Snapshot And Gap Map
large reference workpacks: settings inventory, capability guide, schema proposal

Owners:
screen-plan for local documentation truth, route state, proof inventory, and gap map
feature/expectation docs when product claims change

Rule:
Doc reconciliation proof is routing/status proof only. It cannot close capture, AI, policy, custody, live-view, or rollout behavior by itself.
```

## Contracts, capability, scope, trigger, and platform abstraction family

```text
Workpacks:
WP03 Contract Boundary And Effect Schemas
WP04 Parent Opt-In Settings Contract
WP05 Capability Status Contract
WP06 Capture Scope Model
WP07 Capture Trigger Model
WP08 Platform Adapter Abstraction

Owners:
screen-plan and screen-domain for screen settings, capability/status, capture scope, trigger, queue, analysis result, deletion, policy evidence refs, and screen read-model contracts
schema-domain for neutral/canonical shared shapes where cross-plan reuse is required
browser/app-game/network/tracking owners for source-trigger truth when selected

Rule:
Contract proof must define typed boundaries and malformed payload rejection. Contract proof is not platform runtime proof and not AI/policy/enforcement completion.
```

## Platform capture adapter family

```text
Workpacks:
WP09 Windows Capture Adapter Plan Proof
WP10 MacOS Capture Adapter Plan Proof
WP11 Linux Capture Adapter Plan Proof
WP12 Android MediaProjection Adapter Plan Proof
WP13 iOS ReplayKit Adapter Plan Proof
WP14 Protected Surface Detector

Owners:
screen-plan for capture adapter requirements, permission/degraded/protected-surface proof, and no-cross-platform-claim boundaries
platform-specific adapter/runtime owners when selected

Rule:
One platform proof does not imply another platform. Capture permission proof is not live-view permission proof, remote-access proof, AI quality proof, or product completion.
```

## Queue, scheduler, OCR/VLM, result, validation, and journal family

```text
Workpacks:
WP15 Encrypted Temporary Image Queue
WP16 Queue Scheduler And Debouncer
WP17 Local OCR Vision Runtime Model
WP18 Screen Analysis Result Schema
WP19 Sensitive Text And Redaction Model
WP20 Result Validator And Invalid Output Handling
WP21 Journal And SQLite Ingest
WP31 Screen Intelligence Router
WP34 OCR Tesseract Baseline
WP35 OCR PaddleOCR PP-OCR Evaluation
WP36 Small VLM Guided Classifier Evaluation
WP37 Household Mesh Screen Analysis Queue
WP38 Local AI Resource Scheduler Priority Queue
WP40 Detector Prompt Packs And Schema Tests

Owners:
screen-plan and screen-domain for local screen queue, redaction, OCR/VLM route candidates, result schema, validation, journal/read-model, router, and proof datasets
screen-ai-pipeline-plan and ai-plan when the selected slice claims end-to-end AI runtime/model behavior beyond screen-local contract proof

Rule:
OCR/VLM candidate proof, router proof, or result schema proof cannot claim final AI safety, policy authority, or product model quality unless the selected proof root proves that tier.
```

## Custody, policy evidence, portal/disclosure, and enforcement handoff family

```text
Workpacks:
WP22 Deletion And Retention Proof
WP23 Policy Compiler For Screen Derived Evidence
WP24 Enforcement Handoff Guard
WP25 Parent Portal Summary UI
WP26 Child Disclosure UX
WP27 Screenshot Retention Optional Mode

Owners:
screen-plan for screen-local deletion, raw path redaction, optional raw retention gates, policy evidence refs, summary portal state, and child disclosure proof
data-custody-storage-plan for product retention/export/delete/privacy policy
policy-control-plane-plan for policy authority
enforcement plan for runtime adapter execution and rollback
portal-ux-household-surfaces-plan for broader portal UX completion

Rule:
Screen summaries/evidence refs may feed policy. Raw images and raw AI text must not become policy authority, enforcement authority, or remote upload by default.
```

## Live view, remote boundary, proof tier, and rollout family

```text
Workpacks:
WP28 Live View Optional Mode
WP29 Proof Tiers And Proof Packs
WP30 Test Suite Playwright Rollout PR Gate
WP39 Redacted Summary Only Remote Boundary

Owners:
screen-plan for local live-view preflight, local loopback/session/runtime gates, platform prompt requirements, no-frame-retention boundary, proof tier definitions, rollout proof routing, and redacted-summary-only remote boundary
remote-access-plan for relay-backed remote sessions, standing access, remote live-view authority, and remote product proof
data-custody-storage-plan for remote/export custody
privacy/legal review outside code/docs automation unless explicitly provided

Rule:
Live-view preflight, loopback, relay-cache harness, and worker-startup gates are not product live-view readiness. Redacted summaries are not raw screenshot remote upload. Rollout may aggregate only retained proof roots or exact blockers.
```
