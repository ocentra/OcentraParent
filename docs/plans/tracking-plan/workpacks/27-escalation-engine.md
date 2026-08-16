# WP27 Escalation Engine

<!-- agent-capsule -->

> Agent Capsule
> Plan: `tracking-plan`
> Doc: `WP27 Escalation Engine`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Purpose

Implement rule-based escalation for unacknowledged parent alerts, missing check-ins, offline-after-alert, critical place, and left-expected-place states.

## Central schema boundary

```text
schema-domain owns escalation intent/result payloads that cross event, policy, notification, portal, or proof boundaries.
tracking-core may evaluate escalation readiness from canonical inputs.
tracking-domain may provide helper/proof adapters only.
notification/runtime owners own provider delivery and receipt behavior.
```

## Source Inputs

- `docs/expectations/notifications.md`
- `docs/expectations/policy.md`
- `docs/plans/tracking-plan/v0-5-location-test-blueprint.md`

## Target State

Escalation chains are configured by parent policy, acknowledgement-aware, provider-minimized, multi-guardian capable, and never auto-contact emergency services in MVP.

## Required proof fields

```text
canonical_schema_owner_state
policy_decision_ref_state
acknowledgement_state
check_in_resolution_state
second_guardian_state
critical_manual_state
provider_runtime_state
quiet_hours_state
durable_storage_state
audit_state
emergency_auto_contact_state
manual_required_state
no_provider_delivery_claim
no_production_worker_claim
no_product_ready_claim
no_claim
```

## Tests And Proof

Proof root: `output/tracking-plan-proof/27-escalation-engine/`

- `01-contract-proof.log`
- `09-policy-alert-proof.json`
- `10-escalation-runtime-readiness-blocker-proof.json`
- `11-escalation-runtime-artifact-gate-proof.json`
- `13-security-negative-proof.log`
- `16-validation-commands.log`

## AI Worker Checklist

- [ ] Test warning acknowledgement/check-in resolution, urgent second guardian, and critical multi-channel manual readiness.
- [ ] Parent acknowledgement cancels escalation where configured.
- [ ] Check-in resolves pending state where configured.
- [ ] AI cannot schedule escalation directly.
- [ ] No emergency auto-contact in MVP.

## Where We Are

This workpack has P1 fixture-simulation proof and runtime artifact-gate proof, but production workers, provider delivery/receipt runtime, durable escalation storage, authority proof, emergency auto-contact policy, physical-device proof, and product-ready escalation remain unclaimed until real runtime artifacts exist.

## Fill This Before Reporting DONE Or PR-ready

- [ ] Proof artifacts under `output/tracking-plan-proof/27-escalation-engine/`.
- [ ] Validation command logs for selected proof scripts.
- [ ] Known gaps/manual-required states: production escalation workers, provider delivery/receipt runtime, durable escalation storage, physical-device proof, authority proof, emergency auto-contact policy, and product-ready escalation remain proof-gated.
