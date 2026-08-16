<!-- agent-capsule -->

> Agent Capsule
> Plan: `v0-8-enforcement-control-plan`
> Doc: `V0.8 Enforcement Control Workpack Families`
> Kind: owner-path classifier for selected workpacks.
> Read when: only after `WORKPACK_INDEX.md` selects or names a workpack and owner/handoff boundaries are unclear.
> Stop rule: classify the selected workpack only; do not use this file as permission to scan every workpack.
> Proves: routing and owner-path classification only.
> Does not prove: enforcement runtime readiness, feature completeness, or PR readiness.
> Proof rule: if this file changes route/status claims, update `AGENTS.md`, `PLAN_STATE.md`, and the affected selected workpack route.

<!-- /agent-capsule -->

# V0.8 Enforcement Control Workpack Families

Use this file to classify a selected workpack before opening source. This plan
owns enforcement action authority and proof boundaries. It consumes policy,
evidence, protocol, UI, and runtime handoffs from sibling plans; it does not
silently absorb those owners.

## Contract, evidence-ref, and capability family

```text
Workpacks:
WP01 Contract Boundary And Effect Schemas
WP02 Policy Decision Evidence References
WP03 Adapter Capability Matrix

Owners:
schema-domain for canonical cross-boundary enforcement contracts
policy-control-plane-plan for policy authority and decision refs
enforcement-domain for TypeScript helper/proof/read-model consumer surfaces
agent-protocol and agent-protocol-domain for protocol parity only

Rule:
Typed contracts, decision refs, and capability rows do not prove runtime action
by themselves. Dry-run, observe-only, report-only, rejected, and
manual-required states remain non-executing.
```

## App/game execution family

```text
Workpacks:
WP04 Owned-Process Time Limit
WP05 App And Game Session Handoff

Owners:
v0-8-enforcement-control-plan for action authority, execution result, rollback,
and audit linkage
app-game-plan for app/game session evidence, launcher/process identity, and
duration facts
policy-control-plane-plan for upstream policy authority

Rule:
App/game evidence is consumed here; it is not authored here. Broad installed-app
blocking remains manual-required until a separate adapter and proof path exists.
```

## Browser and network action-boundary family

```text
Workpacks:
WP06 Managed Browser Session Control
WP07 Unmanaged Browser Fallback
WP08 Network/Domain Report-Only Boundary

Owners:
browser-plan for browser profile/session/URL evidence and unmanaged-browser fact
surfaces
network-plan for network/domain evidence and any future blocking adapter
v0-8-enforcement-control-plan for managed-session action states, unmanaged
fallback state, and report-only/manual-required boundaries

Rule:
Managed profile control, unmanaged browser fallback, and network/domain
visibility remain distinct. Exact URL, decrypted-content, or network-blocking
claims need their own proof and must not be borrowed from visibility evidence.
```

## Approval, audit, read-model, and surface family

```text
Workpacks:
WP09 Timer Recovery And Rollback
WP10 Parent Approval And Override
WP11 Audit And Journal Events
WP12 Child-Facing Status And Reasons
WP13 Service Read Models And API
WP14 Portal Control State Consumption

Owners:
policy-control-plane-plan for ask-parent, override, expiry, and approval
authority semantics
eventing-plan for journal, replay, and idempotency mechanics
data-custody-storage-plan for durable export/report/sync handoffs when selected
portal-ux-household-surfaces-plan for rendered UI implementation
v0-8-enforcement-control-plan for rollback, action history, read-model shape,
manual-required state, and no-claim boundaries

Rule:
Read models and surfaces consume audited action state. They do not create
authority, hide degraded/manual-required gaps, or upgrade preview data into
execution truth.
```

## Integrity and non-claim family

```text
Workpacks:
WP15 Integrity Heartbeat And Permission Loss
WP16 Tamper/Uninstall Non-Claim Design
WP17 Cross-Platform Unavailable States

Owners:
child-agent-runtime-distribution-plan and device-trust-bootstrap-plan for
runtime presence, install, trust, and future hardening surfaces when selected
v0-8-enforcement-control-plan for health visibility, degraded/manual-required
state, and anti-claim boundaries

Rule:
Heartbeat, stale/offline status, install state, or permission loss is not
anti-tamper proof. Unsupported or unproved platform states must stay explicit.
```

## Proof, UI closeout, and rollout-gate family

```text
Workpacks:
WP18 Proof Command And Matrix
WP19 Playwright And UI Proof
WP20 Rollout Docs And CI/PR Gate

Owners:
v0-8-enforcement-control-plan docs and proof artifacts under
output/v0-8-enforcement-control-plan-proof/ and
docs/proof/v0-8-enforcement-control-plan/
portal-ux-household-surfaces-plan for rendered UI behavior only when selected
primary/coordinator lane for PR creation, review, merge, and branch integration

Rule:
Aggregated proof may include only accepted proof roots or precise carried
blockers. Partial route/docs/UI proof cannot claim full enforcement readiness.
```
