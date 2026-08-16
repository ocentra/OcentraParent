# screen-plan Event Architecture Instruction

## Owns

- screen capture contracts, settings, evidence result models, live-view gating, retention handoff, screen queue/store/runtime boundaries.

## Must not own

- AI analysis ownership;
- tracking retention ownership where tracking-domain is the true owner;
- policy/enforcement action authority;
- portal final UX truth.

## Required chain

```text
screen capture trigger
-> screen owner validates permission/source/capture result
-> screen event/evidence record is journaled
-> tracking/activity/AI/portal consumers use typed read models or requests
```

## Logging/proof

Log trigger, permission state, capture result, redaction/custody state, queue/store result, consumer handoff, and no-claim platform boundary.

## Tests

Screen-domain tests own contracts. Rust screen tests should move to crate `tests/`. Cross-domain screen -> AI/policy/portal proof belongs in service/app/proof runner.

## First architecture slice

Pause until tracking S0/S1 if touching retention. Then run truth/proof-contract repair, stale shim removal, and Rust test relocation before platform proof.
