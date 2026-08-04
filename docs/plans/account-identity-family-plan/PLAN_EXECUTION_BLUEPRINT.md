<!-- agent-capsule -->

> Agent Capsule
> Plan: `account-identity-family-plan`
> Doc: `Account Identity Family Plan Execution Blueprint`
> Kind: implementation sequence and handoff protocol.
> Read when: a worker needs exact execution order, DONE rules, or handoff sequencing.
> Stop rule: choose one workpack; do not implement multiple workpacks unless explicitly assigned.
> Proves: execution routing only.
> Does not prove: implementation completion, auth security, or PR readiness.

<!-- /agent-capsule -->

# Account Identity Family Plan Execution Blueprint

## Execution order

```text
1. WP01 Auth Provider Decision
2. WP08 Rust Schema And Workers-D1 Runtime Migration
3. WP02 Identity Household Role Model
4. WP03 Session Token Lifecycle
5. WP04 Invites Recovery Lifecycle
6. WP05 Device Ownership AuthZ
7. WP07 Parent Account Family Setup UI
8. WP06 Security Proof And Route Gate
```

## Codex startup prompt

```text
You are working in OcentraParent on account-identity-family-plan.
Read only:
- docs/plans/account-identity-family-plan/AGENTS.md
- docs/plans/account-identity-family-plan/PLAN_STATE.md
- docs/plans/account-identity-family-plan/NEXT_ACTIONS.md
- docs/plans/account-identity-family-plan/WORKPACK_INDEX.md
Then open exactly one assigned workpack.
Do not read sibling plan folders unless the selected workpack names a handoff.
Do not implement provider/session/account runtime before WP01 provider/custody decision is accepted.
Do not claim DONE/PR_READY without proof artifacts and focused validation commands.
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

Likely owned paths for this plan:

```text
packages/family-domain/src/**
packages/family-domain/tests/unit/**
packages/parent-domain/src/** when parent setup/read-model surface is needed
packages/portal-domain/src/** when stable UI route/text/DOM ids are needed
packages/agent-protocol-domain/src/** only for typed service/portal protocol crossings
apps/portal/src/** only for WP07 selected UI surfaces
apps/portal/tests/** only for WP07 selected UI tests
apps/portal/e2e/** only for WP07 selected Playwright proof
crates/agent-protocol/** only for cross-language contract parity
crates/agent-service/** only for selected service-backed setup/session/device boundary proof
infra/cloudflare/** only after cloudflare-control-plane-plan exposes the required worker scaffold/handoff
```

Read-only or handoff-only paths:

```text
docs/plans/setup-install-provisioning-plan/**
docs/plans/cloudflare-control-plane-plan/**
docs/plans/payment-subscription-plan/**
docs/plans/policy-control-plane-plan/**
docs/plans/data-custody-storage-plan/**
docs/plans/device-trust-bootstrap-plan/**
docs/plans/lan-plan/**
docs/plans/remote-access-plan/**
```

Do not edit handoff plans from this plan unless the user explicitly assigns route-sync work.

## Research-backed architecture constraints

Use these constraints during implementation:

```text
Cloudflare D1 is the relational account/family metadata store when Cloudflare runtime is selected.
Cloudflare Durable Objects coordinate serialized, short-lived, per-household/session/invite/recovery state where needed.
Firebase/Auth.js may be an auth/session adapter but must not own household membership, child profiles, device trust, invite/recovery state, policy authority, child evidence, or product data custody.
Firebase custom claims, if used, are access hints only and must stay minimal.
Session identifiers must be opaque, unpredictable, meaningless client-side identifiers backed by server-side session state.
Authorization is deny-by-default and validated on every request.
Recovery/invite tokens are single-use, expiring, random, stored securely, rate-limited, and enumeration-resistant.
Sensitive state-changing browser requests need CSRF/origin/fetch-metadata or equivalent proof.
```

## Focused command policy

Prefer focused commands before broad validation:

```bash
npm run build --workspace @ocentra-parent/family-domain
npm run test --workspace @ocentra-parent/family-domain
npm run test --workspace @ocentra-parent/portal -- account
npm run test:e2e --workspace @ocentra-parent/portal -- account
cargo test -p ocentra-parent-agent-protocol account
cargo test -p ocentra-parent-agent-service account
npm run lint:architecture -- --files packages/family-domain apps/portal packages/portal-domain crates/agent-protocol crates/agent-service
```

If a command does not exist or no matching tests exist, record the missing location in the proof artifact and keep the checklist row open.

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
output/account-identity-family-plan-proof/<workpack-id>/
```

Test result roots are under:

```text
test-results/account-identity-family-plan-<workpack-id>/
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

PR_READY for the whole plan requires WP06 route-gate rerun proof and all prior
workpack proof roots, including WP08's real Workers-D1 migration, redacted
correlated runtime logging, and authority-operation negative proof.

## Global no-touch rule

This plan must not update policy/eventing work while those are active in other Codex lanes.

Do not edit:

```text
docs/plans/policy-control-plane-plan/**
docs/plans/eventing-plan/**
```

unless the user explicitly assigns that route-sync after active lanes finish.
