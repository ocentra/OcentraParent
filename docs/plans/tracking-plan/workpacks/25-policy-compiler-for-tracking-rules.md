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

Compile tracking evidence, rule refs, schedule refs, exception refs, and audit refs into parent policy decisions.

## Central schema boundary

```text
schema-domain owns public tracking policy input/ref schemas.
policy-control-plane-plan owns policy authority and final decision semantics.
tracking-domain/tracking-core may prepare evidence-derived inputs and proof adapters only.
```

## Source Inputs

- `docs/expectations/policy.md`
- `docs/expectations/location-geofence.md`
- `docs/plans/tracking-plan/v0-5-location-ai-safety-analysis-plan.md`

## Target State

Policy decisions cite evidence, rule refs, schedule refs, AI candidate refs when used, capability state, exception state, and audit refs.

## Required proof fields

```text
canonical_schema_owner_state
evidence_ref_state
policy_rule_ref_state
schedule_ref_state
ai_candidate_ref_state
capability_state
exception_state
audit_ref_state
conflict_resolution_state
dry_run_state
manual_required_state
no_ai_authority_claim
no_enforcement_claim
no_provider_claim
no_product_ready_claim
no_claim
```

## Tests And Proof

Proof root: `output/tracking-plan-proof/25-policy-compiler-for-tracking-rules/`

- `01-contract-proof.log`
- `09-policy-alert-proof.json`
- `13-security-negative-proof.log`
- `16-validation-commands.log`

## AI Worker Checklist

- [ ] Parent policy is final action authority.
- [ ] Compile observe, notify, acknowledgement, live tracking, escalate, critical alert, suppress, and manual-required.
- [ ] Add deterministic conflict tests.
- [ ] Prevent AI-only alert/escalation.
- [ ] Preserve dry-run/preview where applicable.

## Where We Are

This workpack has focused contract proof from `codex/tracking-plan-full-scope` and P1 compiler/evaluator runtime proof from `codex/tracking-policy-compiler-runtime-proof-refresh` under the proof root below. Runtime enforcement, platform adapters, provider delivery, production workers, physical devices, and UI behavior are not claimed beyond the proof state recorded in `proof.json` and the implementation checklist.

## Manual-Required Gaps

- Platform, provider, UI, retention, or runtime claims remain manual-required until the assigned proof artifacts exist.
- Any unsupported platform or provider failure must surface as degraded/manual-required state, not as a silent success.

## Fill This Before Reporting DONE Or PR-ready

- [ ] Workpack id and branch: `codex/tracking-policy-compiler-runtime-proof-refresh`.
- [ ] Validation commands and results: `node scripts/test/tracking-policy-compiler-runtime-proof.mjs` passed.
- [ ] Proof artifacts under `output/tracking-plan-proof/25-policy-compiler-for-tracking-rules/`.
- [ ] Known gaps/manual-required states: runtime enforcement, platform adapters, provider delivery, notification receipt ingestion, production workers, physical-device behavior, and full UI/report/policy consumers remain proof-gated as applicable.
