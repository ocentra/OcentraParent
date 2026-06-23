<!-- agent-capsule -->

> Agent Capsule
> Plan: `setup-install-provisioning-plan`
> Doc: `Setup Install Provisioning Plan Health`
> Kind: consistency and readiness check.
> Read when: before claiming the plan is complete, stale, blocked, or PR-ready.
> Stop rule: do not use this as implementation instructions; use assigned workpacks.
> Proves: plan consistency only.
> Does not prove: source implementation or validation completion.

<!-- /agent-capsule -->

# Setup Install Provisioning Plan Health

## Current health

```text
route docs: present and upgraded
workpack index: present and upgraded
checklist index: present and upgraded
proof index: present and upgraded
research/decision map: present
workpacks: WP01, WP02, WP03, WP04, WP05, WP07, and WP06 locally closed for setup-owned proof roots
whole-plan production onboarding: blocked by sibling-owner proof gaps
PR-ready: false
```

## Status interpretation

```text
Local workpack closure means the setup-owned route/state/handoff/proof slice is closed.
Local workpack closure does not mean deployed site, live auth provider, signed package, child runtime, trusted LAN pairing, custody execution, policy baseline, payment entitlement, or production onboarding readiness.
WP06 may be locally closed as a rollout blocker pack while broad PR-ready remains false.
```

## Consistency checks

Before reporting broad progress, verify:

```text
AGENTS.md routes to PLAN_STATE.md, NEXT_ACTIONS.md, WORKPACK_INDEX.md.
WORKPACK_INDEX.md lists every executable workpack.
WORKPACK_FAMILIES.md classifies each selected workpack family without encouraging broad scans.
CHECKLIST_INDEX.md has rows for every workpack.
PROOF_INDEX.md has proof roots, required artifacts, and structured metadata for every workpack.
TEST_PROOF_EXPECTATIONS.md has focused commands, E2E tiers, logging expectations, and negative states for every workpack.
Each workpack has required inputs, expected outputs, proof root, acceptance criteria, commands or command blockers, negative states, gaps, required proof fields, and Fill-before-DONE section.
```

## Known healthy boundaries

This plan intentionally separates:

```text
public family website
registration/account handoff
parent bootstrap/install journey
child bootstrap/install/permission journey
pairing/readiness/recovery journey
first-run setup UI state machine
rollout proof gate
```

Do not collapse those boundaries.

## Known remaining blockers

The plan is not production-onboarding-ready while these sibling proofs remain open or carried as blockers:

```text
account provider/session/household/invite/recovery authority
parent package/signing/update/rollback/distribution proof
child package/runtime/platform permission proof
device-trust key/step-up/trusted-device proof
LAN discovery/signed hello/physical pairing proof
data custody/export/delete/sync proof
policy baseline production proof
payment/subscription/entitlement proof
broader portal shell/household UX proof
```

## Rejection conditions

The plan is unhealthy if:

```text
public website collects private activity data
registration/login implementation is owned here instead of account identity
parent package/signing/update readiness is claimed without runtime distribution proof
child runtime/package/permission readiness is claimed without owner proof
pairing readiness is claimed without LAN/device-trust handoff proof
setup complete is claimed without readiness matrix
missing/degraded/manual-required states are hidden
WP06 aggregation erases sibling-owner blockers
policy/eventing plan files are edited while active lanes own them
```

## PR-ready rule

The whole plan is PR-ready only when WP06 consumes or blocks every earlier proof root, preserves sibling-owner blockers, updates PLAN_STATE, and uses safe product-status wording.

A partial PR may be ready only when one selected workpack is closed with proof artifacts, command logs, no-claim boundaries, and remaining blockers listed.
