# WP25 Policy Compiler For Tracking Rules

<!-- agent-capsule -->

> Agent Capsule
> Plan: `tracking-plan`
> Doc: `WP25 Policy Compiler For Tracking Rules`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Purpose

Compile location, geofence, expected-place, nearby-place, stale/offline,
battery, check-in, and escalation targets into parent policy decisions.

## Source Inputs

- `docs/expectations/policy.md`
- `docs/expectations/location-geofence.md`
- `docs/plans/tracking-plan/v0-5-location-ai-safety-analysis-plan.md`

## Target State

Policy decisions cite evidence, rule refs, schedule refs, AI candidate refs
when used, capability state, exception state, and audit refs.

## Tests And Proof

Proof root: `output/tracking-plan-proof/25-policy-compiler-for-tracking-rules/`

- `01-contract-proof.log`
- `09-policy-alert-proof.json`
- `13-security-negative-proof.log`
- `16-validation-commands.log`

## AI Worker Checklist

- [x] Parent policy is final action authority.
- [x] Compile observe, notify, ask child, ask parent ack, live tracking,
      escalate, critical alert, suppress, and manual-required.
- [x] Add deterministic conflict tests.
- [x] Prevent AI-only alert/escalation.
- [x] Preserve dry-run/preview where applicable.

## Where We Are

This workpack has focused contract proof from `codex/tracking-plan-full-scope`
and P1 compiler/evaluator runtime proof from
`codex/tracking-policy-compiler-runtime-proof-refresh` under the proof root
below. Runtime enforcement, platform adapters, provider delivery, production
workers, physical devices, and UI behavior are not claimed beyond the proof
state recorded in `proof.json` and the implementation checklist.

## Where We Want To Be

This workpack can be assigned independently, implemented against the owning domain boundaries, validated with real contracts or platform proof, and reported without leaving unclear tracking claims behind.

## Scope

- Re-read only the source inputs listed in this workpack and the shared source index.
- Update the owned contracts, runtime paths, UI paths, or docs named by the implementation worker before editing.
- Keep no-claim boundaries explicit for LAN/IP hints, nearby-place ambiguity, AI evidence, background behavior, and remote sync.
- Update the feature doc and product capability checklist only when status, proof, or gaps change.

## Touched Paths

- docs/plans/tracking-plan/workpacks/25-policy-compiler-for-tracking-rules.md
- docs/plans/tracking-plan/implementation-checklist.md
- `output/tracking-plan-proof/25-policy-compiler-for-tracking-rules/`
- Implementation paths listed by the worker before editing.

## Manual-Required Gaps

- Platform, provider, UI, retention, or runtime claims remain manual-required until the assigned proof artifacts exist.
- Any unsupported platform or provider failure must surface as degraded/manual-required state, not as a silent success.

## Fill This Before Reporting DONE Or PR-ready

- [x] Workpack id and branch:
      `codex/tracking-policy-compiler-runtime-proof-refresh`.
- [x] Touched files: tracking policy compiler proof source/test/script, proof
      artifacts, product docs, checklist, and this workpack doc.
- [x] Validation commands and results:
      `node scripts/test/tracking-policy-compiler-runtime-proof.mjs` passed.
- [x] Proof artifacts under `output/tracking-plan-proof/25-policy-compiler-for-tracking-rules/`.
- [x] Product doc/checklist updates: owning feature doc, capability checklist,
      implementation checklist, and this workpack doc updated.
- [x] Known gaps/manual-required states: runtime enforcement, platform adapters,
      provider delivery, notification receipt ingestion, production workers,
      physical-device behavior, full UI/report/policy consumers, and child
      delivery remain proof-gated as applicable.
