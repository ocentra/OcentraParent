<!-- agent-capsule -->

> Agent Capsule
> Plan: `setup-install-provisioning-plan`
> Doc: `Setup Install Provisioning Plan Execution Blueprint`
> Kind: implementation sequence and handoff protocol.
> Read when: a worker needs exact execution order, DONE rules, or handoff sequencing.
> Stop rule: choose one workpack; do not implement multiple workpacks unless explicitly assigned.
> Proves: execution routing only.
> Does not prove: implementation completion, installer readiness, deployed site, pairing readiness, or PR readiness.

<!-- /agent-capsule -->

# Setup Install Provisioning Plan Execution Blueprint

## Execution order

```text
1. WP01 Family Web Info Site
2. WP02 Registration Login Entry
3. WP03 Parent Install Journey
4. WP04 Child Install Permission Journey
5. WP05 Pairing Readiness Recovery
6. WP07 First-Run Setup UI And State Machine
7. WP06 Rollout Proof And Route Gate
```

## Codex startup prompt

```text
You are working in OcentraParent on setup-install-provisioning-plan.
Read only:
- docs/plans/setup-install-provisioning-plan/AGENTS.md
- docs/plans/setup-install-provisioning-plan/PLAN_STATE.md
- docs/plans/setup-install-provisioning-plan/NEXT_ACTIONS.md
- docs/plans/setup-install-provisioning-plan/WORKPACK_INDEX.md
Then open exactly one assigned workpack.
Do not read sibling plan folders unless the selected workpack names a handoff.
Do not claim setup readiness from website-only, installer-only, UI-only, or pairing-only proof.
Do not claim installer package/signing/update readiness without runtime-distribution proof.
Do not claim account/session readiness without account-identity proof.
```

## Pre-edit note

Before editing source or docs, write:

```text
Assigned workpack:
Implementation slice:
Expected source/doc files:
Expected tests/proof files:
Proof root:
Adjacent handoffs that are read-only:
No-claim boundaries:
```

## Source ownership map

Likely owned docs/source paths for this plan:

```text
docs/plans/setup-install-provisioning-plan/**
packages/family-domain/src/** only for setup state contract work named by workpack
packages/parent-domain/src/** only for setup/readiness read-model rows named by workpack
packages/portal-domain/src/** only for stable setup UI text/DOM ids named by workpack
apps/portal/src/** only for selected first-run/setup surface work
apps/portal/tests/** only for selected setup tests
apps/portal/e2e/** only for selected setup proof
scripts/test/** only for selected setup proof runner
```

Read-only or handoff-only paths:

```text
docs/plans/account-identity-family-plan/**
docs/plans/parent-desktop-runtime-package-plan/**
docs/plans/child-agent-runtime-distribution-plan/**
docs/plans/device-trust-bootstrap-plan/**
docs/plans/lan-plan/**
docs/plans/portal-ux-household-surfaces-plan/**
docs/plans/data-custody-storage-plan/**
docs/plans/payment-subscription-plan/**
docs/plans/policy-control-plane-plan/**
```

Do not edit handoff plans unless the user explicitly assigns route-sync work.

## Research-backed architecture constraints

```text
Cloudflare Pages is acceptable for public family information site and static/dynamic page routing.
Cloudflare Workers static assets are acceptable if the shared worker/control-plane route owns hosting.
Public family pages cannot collect child activity data.
Tauri/package signing/notarization/update/rollback belongs to runtime distribution plans.
Android/iOS/macOS/Windows install claims need platform-owner proof before production-ready wording.
```

## Focused command policy

Use relevant commands only:

```bash
npm run build --workspace @ocentra-parent/family-domain
npm run build --workspace @ocentra-parent/portal-domain
npm run test --workspace @ocentra-parent/portal -- setup
npm run test:e2e --workspace @ocentra-parent/portal -- setup
npm run lint:architecture -- --files docs/plans/setup-install-provisioning-plan packages/family-domain packages/portal-domain apps/portal
```

If a command or test path does not exist, record the missing location and keep the row open.

## Proof update rule

Each completed row needs:

```text
exact command
exit code
proof file path
test/proof id
negative case status
remaining gaps/no-claim boundary
```

Proof roots are under:

```text
output/setup-install-provisioning-plan-proof/<workpack-id>/
```

Test result roots are under:

```text
test-results/setup-install-provisioning-plan-<workpack-id>/
```

## DONE / PR_READY criteria

DONE for one workpack requires:

```text
source/docs/tests updated
focused commands run or blocker recorded
negative cases covered or explicitly open
proof artifacts written
CHECKLIST_INDEX.md rows updated
selected workpack Fill-before-DONE section updated
PLAN_STATE.md open gaps updated if state changed
```

PR_READY for the whole plan requires WP06 route gate proof and all prior workpack proof roots.

## Global no-touch rule

This plan must not update policy/eventing work while those are active in other Codex lanes.

Do not edit:

```text
docs/plans/policy-control-plane-plan/**
docs/plans/eventing-plan/**
```

unless the user explicitly assigns that route-sync after active lanes finish.
