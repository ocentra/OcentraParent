# WP33 Proof Gates Fixtures Rollout And PR Gate

<!-- agent-capsule -->

> Agent Capsule
> Plan: `tracking-plan`
> Doc: `WP33 Proof Gates Fixtures Rollout And PR Gate`
> Kind: proof reference; read only when validating matching claim.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Purpose

Define final test fixtures, proof packs, Playwright/manual proof, docs updates, and merge blockers for tracking work.

## Central schema and claim gate

```text
WP33 aggregates proof; it does not own canonical schemas.
Every accepted public tracking contract must cite schema-domain or a neutral protocol/event/evidence boundary as canonical owner.
tracking-domain/tracking-core proof helpers cannot become schema authority.
Proof-file presence is not product readiness.
```

## Source Inputs

- `docs/plans/tracking-plan/v0-5-location-test-blueprint.md`
- `docs/plans/tracking-plan/implementation-checklist.md`
- `docs/plans/tracking-plan/pasted-content-coverage-audit.md`
- `.ocentra-ai/rules/ocentra-parent-validation.mdc`

## Target State

No tracking implementation can report `DONE` or PR-ready without proof packs, validation commands, docs/checklist updates, schema-owner notes, and explicit known gaps.

## Required rollout fields

```text
accepted_proof_roots
missing_proof_roots
audit_reopened_workpacks
schema_owner_state
tracking_domain_public_contract_state
tracking_core_parity_state
event_contract_state
claim_audit_state
physical_or_external_state
provider_runtime_state
production_worker_state
portal_runtime_state
manual_required_state
claims_allowed
claims_blocked
pr_ready_state
no_claim
```

## Tests And Proof

Proof root: `output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/`

The selected proof must cite exact artifact paths and must preserve no-claim boundaries for every proof tier. Existing long artifact lists remain historical inputs; current acceptance must be based on selected workpack proof roots and current claim-audit state.

## Merge Blockers

- LAN/IP displayed as GPS.
- Location missing accuracy/source/timestamp/freshness.
- Stale displayed as live.
- Nearby POI displayed as exact place with low accuracy.
- AI triggers notification without policy decision.
- Critical alert suppressed by generic exception.
- Parent acknowledgement ignored.
- Retention delete fails.
- Remote sync runs by default.
- Remote AI runs by default.
- Background tracking claimed without Android/iOS proof.
- Cross-boundary schema exists only inside tracking-domain or tracking-core.

## AI Worker Checklist

- [ ] Run the smallest useful validation while working.
- [ ] Run requested focused tests before handoff.
- [ ] Update feature docs and queue central capability checklist delta when proof changes.
- [ ] Include touched files, validation, product-doc updates, known gaps, platform proof state, schema-owner state, and no-claim boundaries in `DONE`.
- [ ] Do not mark product-complete from planning-only docs, proof-file presence, fixtures, hosted screenshots, or checked boxes.

## Where We Are

This workpack has many retained proof artifacts and claim-audit rows, but it remains an aggregation gate. It must not convert local/CI proof, hosted screenshots, simulator/emulator artifacts, manual runbooks, or fixture proof into product-ready tracking.

## Fill This Before Reporting DONE Or PR-ready

- [ ] Workpack id and branch.
- [ ] Touched files.
- [ ] Validation commands and results.
- [ ] Proof artifacts under `output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/`.
- [ ] Schema-owner state for any public tracking contract used by accepted proof.
- [ ] Known gaps/manual-required states.
- [ ] Claim-audit state and PR-ready state.
