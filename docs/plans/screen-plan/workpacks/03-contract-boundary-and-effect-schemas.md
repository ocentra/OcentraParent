# 03 Contract Boundary And Effect Schemas

## Target State

Settings, capability, queue, capture job, analysis result, model status, deletion, policy target, and read-model contracts are schema-backed.

## Current State

Partial foundation exists in `packages/activity-domain/src/screen-evidence*.ts` and `crates/agent-protocol/src/screen_evidence.rs`.

## Checklist

- [ ] Reconcile existing schemas with expectation docs.
- [ ] Add missing settings contract.
- [ ] Add missing capability/status contract.
- [ ] Add missing capture queue job contract.
- [ ] Add missing analysis result contract.
- [ ] Add missing deletion/custody contract.
- [ ] Add policy evidence ref contract.
- [ ] Add strict malformed payload tests.

## Proof

- Activity-domain tests.
- Agent-protocol tests.
- No raw app/runtime strings or manual brands.
