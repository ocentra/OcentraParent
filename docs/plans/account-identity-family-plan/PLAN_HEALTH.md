<!-- agent-capsule -->

> Agent Capsule
> Plan: `account-identity-family-plan`
> Doc: `Account Identity Family Plan Health`
> Kind: consistency and readiness check.
> Read when: before claiming the plan is complete, stale, blocked, or PR-ready.
> Stop rule: do not use this as implementation instructions; use assigned workpacks.
> Proves: plan consistency only.
> Does not prove: source implementation or validation completion.

<!-- /agent-capsule -->

# Account Identity Family Plan Health

## Current health

```text
route docs: present and upgraded
workpack index: present and upgraded
checklist index: present and upgraded
proof index: present and upgraded
research/decision map: present
workpacks: present and upgraded
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
TEST_PROOF_EXPECTATIONS.md has focused commands and negative cases for every workpack.
Each workpack has required inputs, expected paths, proof root, acceptance criteria, commands, negative cases, gaps, and Fill-before-DONE section.
```

## Known healthy boundaries

This plan intentionally separates:

```text
authentication
household membership
role authorization
device authority
session freshness
invite/recovery lifecycle
support/admin actor class
setup UI state
adjacent plan handoffs
```

Do not collapse those boundaries.

## Known incomplete areas

The plan is not implementation-complete until these are done:

```text
WP01 provider decision proof
WP02 household/role model proof
WP03 session/token lifecycle proof
WP04 invite/recovery lifecycle proof
WP05 device ownership authZ proof
WP07 setup UI proof or explicit blocker
WP06 security proof and route gate
```

## Rejection conditions

The plan is unhealthy if:

```text
provider/session/role/device claims are made without proof roots
Firebase/Auth.js owns family product data
custom claims contain household/member/child/device product data
login is treated as policy/payment/remote/export authority
child profile is treated as child device trust
support/admin can act as parent owner
setup UI implies protected/trusted state without device proof
proof/checklist changed before source/tests for implementation work
policy/eventing plan files are edited while active lanes own them
```

## PR-ready rule

The whole plan is PR-ready only when WP06 consumes or blocks every earlier proof root and updates PLAN_STATE.

A partial PR may be ready only when one selected workpack is closed with proof artifacts, command logs, and remaining open workpacks listed.
