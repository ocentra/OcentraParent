# Tracking Plan State

<!-- agent-capsule -->

> Agent Capsule
> Plan: `tracking-plan`
> Doc: `Tracking Plan State`
> Kind: current state and open gaps.
> Read when: Immediately after plan AGENTS.md; use for current state and no-claim boundaries.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.

<!-- /agent-capsule -->

Generated from the existing `tracking-plan` docs. This is the default resume/status file; large historical docs are linked, not embedded.

## Scope

This folder is the single working plan location for location evidence, geofence rules, expected-place schedules, device status, nearby-place intelligence, AI safety analysis, parent acknowledgements, alerts, escalation, check-ins, temporary live tracking, missing-device mode, and tracking UI/UX requirements.

## Current ownership interpretation

```text
tracking-plan: tracking semantics, local proof routing, read-model requirements, and no-claim boundaries.
crates/schema: canonical cross-boundary tracking schemas and shared event/protocol/read-model/proof shapes.
tracking-domain: helper/projection/proof adapter package; public exports are helpers, not automatic schema authority.
tracking-core: Rust runtime/evaluator/projection helper crate; mirrors canonical contracts for runtime use.
ocentra-eventing/eventing-plan: generic event envelope, idempotency, journal/replay, dead-letter, and topology mechanics.
policy, notification, custody, AI, portal, platform, network/browser/app/LAN plans: sibling owners for their own authority/runtime/proof tiers.
```

## Current snapshot source

- Snapshot: [current-tracking-snapshot.md](current-tracking-snapshot.md)

## What is already present / proved

- Feature, expectation, capability, schema proposal, and settings inventory docs exist.
- Real TypeScript tracking surfaces now mostly live in `packages/tracking-domain`.
- Real Rust tracking runtime exists in `crates/tracking-core`.
- Local/CI proof exists for many contract, fixture, hosted-route, service read-model, provider-preflight, retention-preflight, and rollout accounting slices.
- Product-readiness closure proof exists as blocker accounting, not product-ready approval.

## Open gaps / missing product runtime

```text
- Architecture gate still flags pre-existing bypass-guard issues in packages/tracking-domain/src/tracking-control-catalog-data.ts.
- False-green workpacks remain reopened until their selected proof tiers are rerun or carried as blockers.
- Physical mobile background/geofence proof remains manual-required.
- Provider delivery/receipt runtime remains manual-required.
- Production workers, durable outbox/history/storage, escalation runtime, and full runtime UI proof remain manual-required.
- Event-driven tracking chains WP34-WP39 are active scope and must use centralized schemas.
```

## Centralized schema status

```text
Cross-boundary tracking schemas must live in crates/schema or an approved neutral Rust protocol/event/evidence boundary.
tracking-domain may expose helpers/proof adapters but must not silently become canonical schema owner.
tracking-core Rust types must cite the canonical schema/protocol/event contract they mirror when they escape the crate.
```

## Checklist summary

- Full checklist: [implementation-checklist.md](implementation-checklist.md) (not default context).
- Checkbox rows detected: 111 total, 79 checked, 32 unchecked.
- Checklist index: [CHECKLIST_INDEX.md](CHECKLIST_INDEX.md).

## Workpack summary

- Workpacks on disk: 39.
- Workpacks previously indexed by generated docs: 33.
- Checkbox-closed workpacks requiring audit reopen: `WP25`, `WP27`, `WP28`, `WP29`, `WP33`.
- On-disk workpacks omitted by the earlier generated index: `WP34`, `WP35`, `WP36`, `WP37`, `WP38`, `WP39`.

## Audit-priority workpacks

- [WP33 Proof Gates Fixtures Rollout And PR Gate](workpacks/33-proof-gates-fixtures-rollout-and-pr-gate.md) - 65/65 checked, but proof rerun is blocked and the checked state is not trustworthy.
- [WP25 Policy Compiler For Tracking Rules](workpacks/25-policy-compiler-for-tracking-rules.md) - 11/11 checked, but runtime compiler/evaluator completion is not yet proved.
- [WP27 Escalation Engine](workpacks/27-escalation-engine.md) - 11/11 checked, but runtime escalation proof is incomplete.
- [WP28 Temporary Live Tracking Mode](workpacks/28-temporary-live-tracking-mode.md) - 11/11 checked, but runtime/UI proof is incomplete.
- [WP29 Missing-Device Mode](workpacks/29-missing-device-mode.md) - 11/11 checked, but runtime/device proof is incomplete.
- [WP34-WP39](WORKPACK_INDEX.md) - event-contract and event-flow workpacks are on disk and active.

## Product-ready no-claim boundaries

Do not claim product-ready tracking, physical-device proof, background platform behavior, provider delivery, notification receipt runtime, durable production worker execution, full product UI runtime, policy authority, custody execution, or event-runtime completion unless the selected workpack explicitly proves it.

## Default no-read list

- `README_FULL_ORIGINAL.md` unless you need historical full README context.
- Full `implementation-checklist.md` unless `CHECKLIST_INDEX.md` names exact section/row.
- All workpacks; use `WORKPACK_INDEX.md`.
- `WORKPACK_FAMILIES.md` unless owner/proof family is unclear.
- Source inventories and pasted-content audits unless source ownership is unclear.
- Historical checkpoint/proof docs unless `PROOF_INDEX.md` or the assigned workpack names them.

## Health / consistency

- See `PLAN_HEALTH.md` before claiming the whole plan is complete or stale.

## HID Execution Guard

- Follow [PLAN_HID_MATRIX.md](../../PLAN_HID_MATRIX.md), then this plan's assigned WORKPACK_INDEX.md and NEXT_ACTIONS.md.
- Do not mark this plan complete from checklist deltas alone.
- Before any checked update, attach a real test run log or explicit known blocker and a proof manifest under docs/proof/tracking-plan/.
- Failure rule: no PR-ready claim until replay/idempotency, authZ/replay, rollback/teardown, stale/degraded states, schema authority, and no-claim proofs are present for the assigned slice.

## HID execution blueprint

Continue execution from: [PLAN_EXECUTION_BLUEPRINT.md](PLAN_EXECUTION_BLUEPRINT.md). Update this plan only via the blueprint and matching workpack checklist.
