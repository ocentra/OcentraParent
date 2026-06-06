# 08 - AI Provider Routing Contract

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
