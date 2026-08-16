# 08 - AI Provider Routing Contract

<!-- agent-capsule -->

> Agent Capsule
> Plan: `ai-plan`
> Doc: `08 - AI Provider Routing Contract`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Target State

Every AI task explicitly routes to deterministic, same-device local worker,
trusted household AI provider, limited/dormant mobile fallback, or
parent-approved remote assistant. Routing selects execution only; it does not
move policy authority.

## Where We Are

Local provider routing proof exists for parent assistant states and same-device
provider scheduler states. The product router must cover all AI task families,
household provider classes, custody compatibility, and stale/offline/revoked
provider rejection.

## Checklist

- [ ] Define provider route contract.
- [ ] Add task-to-route matrix.
- [ ] Add same-device vs household provider vs mobile fallback route classes.
- [ ] Make deterministic route first.
- [ ] Make desktop/laptop preferred for heavy jobs.
- [ ] Make mobile dormant unless no better provider exists and policy/resource
      state allows.
- [ ] Make remote route unavailable for normal child safety.
- [ ] Record selected route in result journal.
- [ ] Reject unsupported/stale/offline/revoked providers.
- [ ] Reject custody-incompatible provider route.

## Proof

- Route selection tests.
- Unsupported task tests.
- Remote safety-path rejection tests.
- Desktop beats mobile for heavy job.
- Mobile dormant with desktop available.
- Stale/offline/revoked provider rejection.
- Custody mismatch rejection.
