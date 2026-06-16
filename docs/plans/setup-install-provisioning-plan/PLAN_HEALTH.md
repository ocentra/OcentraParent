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
workpacks: mostly upgraded; WP05 retained from existing detailed version after safety-filter write block
implementation: not started by this plan route
source proof: not generated yet
PR-ready: false
```

## Consistency checks

Before reporting broad progress, verify:

```text
AGENTS.md routes to PLAN_STATE.md, NEXT_ACTIONS.md, WORKPACK_INDEX.md.
WORKPACK_INDEX.md lists every executable workpack.
CHECKLIST_INDEX.md has rows for every workpack.
PROOF_INDEX.md has proof roots and required artifacts for every workpack.
TEST_PROOF_EXPECTATIONS.md has focused commands and negative states for every workpack.
Each workpack has required inputs, expected outputs, proof root, acceptance criteria, commands or command blockers, negative states, gaps, and Fill-before-DONE section.
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

## Known incomplete areas

The plan is not implementation-complete until these are done:

```text
WP01 public site boundary proof
WP02 registration/account handoff proof
WP03 parent install journey proof
WP04 child install/permission journey proof
WP05 pairing/readiness/recovery proof
WP07 first-run setup UI/state proof
WP06 rollout proof and route gate
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
policy/eventing plan files are edited while active lanes own them
```

## PR-ready rule

The whole plan is PR-ready only when WP06 consumes or blocks every earlier proof root and updates PLAN_STATE.

A partial PR may be ready only when one selected workpack is closed with proof artifacts, command logs, and remaining open workpacks listed.
