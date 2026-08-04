<!-- agent-capsule -->

> Agent Capsule
> Plan: `account-identity-family-plan`
> Doc: `Account Identity Family Plan Workpack Index`
> Kind: workpack selector.
> Read when: after NEXT_ACTIONS.md.
> Stop rule: open exactly one selected workpack; do not read every workpack.
> Proves: workpack routing only.
> Does not prove: implementation completion, provider choice, auth security, or PR readiness.
> Proof rule: update counts/status only after matching checklist rows and proof artifacts exist.

<!-- /agent-capsule -->

# Account Identity Family Plan Workpack Index

Use this index to select exactly one workpack.

| Status | Workpack | Boxes | Primary source docs | Proof root |
| --- | --- | ---: | --- | --- |
| partial | [WP01 Auth Provider Decision](workpacks/01-auth-provider-decision.md) | 10/10 | `RESEARCH_AND_DECISIONS.md`, `docs/expectations/cloud.md` | `output/account-identity-family-plan-proof/01-auth-provider-decision/` |
| open | [WP08 Rust Schema And Workers-D1 Runtime Migration](workpacks/08-rust-schema-workers-d1-runtime-migration.md) | 0/11 | `PLAN_STATE.md`, `docs/expectations/cloud.md`, accepted WP01 custody decision | `output/account-identity-family-plan-proof/08-rust-schema-workers-d1-runtime-migration/` |
| complete | [WP02 Identity Household Role Model](workpacks/02-identity-household-role-model.md) | 13/13 | `docs/features/family-setup-device-roles.md`, `docs/expectations/family-setup.md` | `output/account-identity-family-plan-proof/02-identity-household-role-model/` |
| complete | [WP03 Session Token Lifecycle](workpacks/03-session-token-lifecycle.md) + [current boundary addendum](workpacks/03-current-boundary-addendum.md) | 14/14 | `RESEARCH_AND_DECISIONS.md`, `packages/family-domain/src/session-lifecycle.ts` | `output/account-identity-family-plan-proof/03-session-token-lifecycle/` |
| complete | [WP04 Invites Recovery Lifecycle](workpacks/04-invites-recovery-lifecycle.md) | 13/13 | `docs/expectations/family-setup.md`, `docs/expectations/data-custody.md` | `output/account-identity-family-plan-proof/04-invites-recovery-lifecycle/` |
| complete | [WP05 Device Ownership AuthZ](workpacks/05-device-ownership-authz.md) | 13/13 | `docs/features/family-setup-device-roles.md`, `docs/expectations/platforms.md` | `output/account-identity-family-plan-proof/05-device-ownership-authz/` |
| complete | [WP07 Parent Account Family Setup UI](workpacks/07-parent-account-family-setup-ui.md) | 13/13 | `docs/expectations/portal.md`, `docs/expectations/family-setup.md` | `output/account-identity-family-plan-proof/07-parent-account-family-setup-ui/` |
| open | [WP06 Security Proof And Route Gate](workpacks/06-security-proof-and-route-gate.md) | 14/18 | all prior workpack proof roots, including WP08 | `output/account-identity-family-plan-proof/06-security-proof-and-route-gate/` |

## Default execution order

```text
WP01 -> WP08 -> WP02 -> WP03 -> WP04 -> WP05 -> WP07 -> WP06
```

## Dependency rules

```text
WP01 blocks runtime provider/session implementation.
WP08 owns the next runtime/schema slice: Rust-owned account/family contract authority, a real Workers-D1 persistence adapter, and migration proof. It is not WP01 provider-decision work and it must not use a TypeScript D1 test double as runtime proof.
WP02 blocks most authorization, UI, policy, payment, and remote-access handoffs.
WP03 blocks secure-login/session claims and must be read with workpacks/03-current-boundary-addendum.md.
WP04 may run after WP02 but must not implement data-custody side effects itself.
WP05 depends on WP02/WP03 authority and session freshness models.
WP07 depends on WP02 and enough WP03/WP04 state to render honest setup states.
WP06 must be last and is reopened until it consumes WP08's real Workers-D1 migration, redacted correlated runtime logging, and authority-operation negative proof or records a precise blocker.
```

## Module linkage by role

Use this section to decide where code belongs before opening source.

```text
Canonical shared schema owner:
  crates/schema or the owning Rust crate
  Owns shared account/family/session/device-authority shapes when those shapes cross package, crate, app, or plan boundaries.

TypeScript edge-validation migration surface:
  packages/schema-domain
  Temporary generated-validation or edge-decoder surface only where TypeScript still needs one.

TypeScript helper/projection owner:
  packages/family-domain
  Consumes Rust-owned/generated account/family contracts and exposes approved account/family helper surfaces for this plan.

Rust parity/runtime authority owner:
  crates/family-identity-core
  Mirrors account/family authority semantics in Rust without drifting field names, discriminants, nullability, or status values.

Setup/provisioning consumers:
  packages/setup-domain
  crates/provisioning-core
  Consume setup, invite, recovery, household, and readiness surfaces; they do not own family authority.

Parent UI projection/rendering consumers:
  packages/portal-domain
  apps/portal
  Consume typed setup/read-model state and render honest status; they do not own account runtime, device trust, or child activity state.

Runtime/protocol handoff targets when explicitly selected:
  crates/agent-protocol
  crates/agent-service
  Cloudflare control-plane runtime/schema work
  These are not default workpack targets unless the selected workpack names protocol, service, or Cloudflare runtime proof.

Adjacent consumer plans:
  setup-install-provisioning-plan
  cloudflare-control-plane-plan
  payment-subscription-plan
  policy-control-plane-plan
  data-custody-storage-plan
  device-trust-bootstrap-plan
  lan-plan
  remote-access-plan
  portal-ux-household-surfaces-plan
  These consume account/family authority through explicit handoff contracts, events, requests, read models, or proof routes. They must not re-own account/family authority.
```

If the selected workpack needs a shape that is useful to more than this plan, place or consume it through `crates/schema` or another neutral Rust-owned boundary. Use `schema-domain` only as a temporary generated-validation or edge-decoder surface while migration is still incomplete. Do not make a sibling feature owner package/crate the shared contract owner.

## Do not select

Do not create new workpacks unless the existing seven cannot represent the implementation slice.

Do not split proof-only rows into tiny workpacks unless WP06 explicitly needs a proof-gate follow-up.

Do not move provider/account/family authority into setup, payment, policy, remote, LAN, device-trust, or data-custody plans.
