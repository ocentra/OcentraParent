# Parent Desktop Runtime Package Test Blueprint

<!-- agent-capsule -->

> Agent Capsule
> Plan: `parent-desktop-runtime-package-plan`
> Doc: `Parent Desktop Runtime Package Test Blueprint`
> Kind: test blueprint reference; read only when local expectations route here.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.

<!-- /agent-capsule -->

This is the companion requirement blueprint for the
[Parent Desktop Runtime Package 20-Step Plan](parent-desktop-runtime-package-20-step-plan.md).

## Scope

Included:

- Tauri parent desktop shell boundary.
- Local service connection command proof.
- Controller/observer/route/custody source states.
- Package preview and launch smoke.
- Platform capability matrix.
- Update/rollback scaffold states.
- Signing/store/manual-required states.
- Support diagnostics and redaction.
- Feature doc/checklist synchronization.

Excluded until separately proved:

- Child-device capture inside the parent shell.
- Local AI execution inside the parent shell.
- Policy evaluation or enforcement inside the parent shell.
- Production publishing from `main`.
- Store distribution, notarization, signing, and mobile entitlements without
  credentials/artifacts.

## Core Test Principle

### PKG-TEST-001: Desktop shell is not child authority

Requirement: Tauri commands can connect, query, and display state, but cannot
run capture, AI, policy evaluation, timers, or enforcement.

Proof: Contract/script tests and README checks keep command output scoped to
parent shell and service state.

Acceptance: Parent desktop package proof cannot be mistaken for child-agent
runtime proof.

### PKG-TEST-002: Service connection is real

Requirement: Desktop runtime proof must reach the real Rust service path or show
unavailable state.

Proof: Script tests launch/check the local service or use managed unavailable
state; no hardcoded success-shaped response counts.

Acceptance: The shell reports what it can actually reach.

### PKG-TEST-003: Route and custody states are visible

Requirement: Local, LAN, relay, cache, parent-owned storage, unavailable, stale,
controller, and observer states are distinct.

Proof: Tests check serialized state and visible labels where UI is involved.

Acceptance: Parents know whether the desktop app is live or viewing cached or
unavailable state.

### PKG-TEST-004: Package preview is not release claim

Requirement: CI/package preview artifacts prove mechanics only.

Proof: Matrix output and docs separate preview, unsigned, signed, store,
notarized, scaffold, and manual-required states.

Acceptance: Main branch package work does not claim production release.

### PKG-TEST-005: Platform states are split

Requirement: Parent desktop, parent mobile, child Windows, child Android, child
iOS, macOS, Linux, signing, stores, and relay are separate rows.

Proof: Platform matrix tests fail if one platform claim upgrades another.

Acceptance: "Mobile support" never hides parent app versus child-agent proof.

### PKG-TEST-006: Support output is redacted

Requirement: Support diagnostics include useful runtime/package state without
secrets or private child activity.

Proof: Tests check redaction and allowed fields.

Acceptance: Debug output can be shared safely for manual support.

### PKG-TEST-007: Release boundary is guarded

Requirement: `main` builds previews; production release publishing requires
explicit promotion.

Proof: Workflow/docs checks and PR review verify branch/publish boundaries.

Acceptance: Merging to `main` cannot silently ship a production release.

### PKG-TEST-008: Checklist locks are reported, not ignored

Requirement: If product checklist updates are blocked by another lane, D reports
the exact desired update and continues non-overlapping proof.

Proof: Hub report names blocked row and current lock owner.

Acceptance: Package/runtime proof does not get lost behind one doc conflict.

## Required Fixture Families

- Service connection: available local service, unavailable service, stale route,
  controller, observer, LAN route, relay unavailable, cache unavailable.
- Package state: dev preview, CI preview, unsigned artifact, signing required,
  signed unavailable, update scaffold, rollback unavailable.
- Platform matrix: Windows desktop parent, macOS package preview, Linux preview,
  Android parent scaffold, iOS parent scaffold, Android child manual-required,
  iOS child entitlement-required.
- Support output: normal state, degraded service, missing package metadata,
  redacted local path, redacted secret/token-like value.

## Acceptance And Proof

- Focused package/runtime proof scripts pass.
- Desktop README and feature docs match current capability state.
- Platform matrix output records implemented/scaffold/manual-required states.
- `npm run validate` passes before PR-ready integration unless primary accepts a
  documented omission.
- Checklist update is committed or explicitly blocked with exact desired row
  language.
