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
| validation / bounded producer source accepted / tests deferred | [WP08 Rust Schema And Account Authority](workpacks/08-rust-schema-workers-d1-runtime-migration.md) | Prior packet plus crate-private Account producer transport; current test/proof rows open | `PLAN_STATE.md`, accepted WP01 custody decision, canonical Rust contract boundary | `docs/proof/account-identity-family-plan/08-rust-schema-workers-d1-runtime-migration/` |
| target-aware source reviewed / tests and composition open | [WP02 Identity Household Role Model](workpacks/02-identity-household-role-model.md) | 13/13 historical proof plus reviewed target-aware resolver/storage-custody consumer source; expected tests, Cloudflare provider composition, and current proof open | `docs/features/family-setup-device-roles.md`, `docs/expectations/family-setup.md` | `output/account-identity-family-plan-proof/02-identity-household-role-model/` |
| validation / production source reviewed / route-store tests and proof open | [WP03 Session Token Lifecycle](workpacks/03-session-token-lifecycle.md) + [current boundary addendum](workpacks/03-current-boundary-addendum.md) | 14/14 historical proof; reviewed Cloudflare browser-session migrations/store/routes now mapped with trusted-time, forward schema sentinel, complete support provenance, and runtime row validation; expected route/store/request-safety tests and proof remain open | `RESEARCH_AND_DECISIONS.md`, `crates/family-identity-core/src/session_lifecycle.rs`, `infra/cloudflare/migrations/account-identity/0007_account_browser_session_custody_hardening.sql`, `infra/cloudflare/src/storage/account-identity-authority-store.ts`, `infra/cloudflare/src/storage/account-browser-session-store.ts`, `infra/cloudflare/src/auth/browser-session-routes.ts` | `output/account-identity-family-plan-proof/03-session-token-lifecycle/` |
| source reviewed / owner adapters and six expected test roots open | [WP04 Invites Recovery Lifecycle](workpacks/04-invites-recovery-lifecycle.md) | 13/13 historical proof; strict durable repository, rate/replay custody, private owner receipts, and typed handoff source reviewed; shipped identity/membership/support/Data owners, tests, and current proof open | `docs/expectations/family-setup.md`, `docs/expectations/data-custody.md` | `output/account-identity-family-plan-proof/04-invites-recovery-lifecycle/` |
| blocked / coordinator and owner participants missing / tests and handoffs open | [WP05 Device Ownership AuthZ](workpacks/05-device-ownership-authz.md) | 13/13 historical proof; current Account-bound billing/support source is bounded, but capability/lease reservations and the multi-owner fence handoff remain open | `docs/features/family-setup-device-roles.md`, `docs/expectations/platforms.md`, `MULTI_OWNER_EFFECT_FENCING_DECISION.md` | `output/account-identity-family-plan-proof/05-device-ownership-authz/` |
| blocked / planned source route / owner-specific fencing and recovery absent | [WP05A Runtime Effect Fencing Coordinator](workpacks/05-runtime-effect-fencing-coordinator.md) | New routing workpack; coordinator/recovery plus private Account participant and capability/lease reservation adapters are absent and must not duplicate Account, Device Trust, or step-up truth | `MULTI_OWNER_EFFECT_FENCING_DECISION.md`, `PLAN_STATE.md` | `output/account-identity-family-plan-proof/05-runtime-effect-fencing-coordinator/` |
| complete | [WP07 Parent Account Family Setup UI](workpacks/07-parent-account-family-setup-ui.md) | 13/13 | `docs/expectations/portal.md`, `docs/expectations/family-setup.md` | `output/account-identity-family-plan-proof/07-parent-account-family-setup-ui/` |
| open | [WP06 Security Proof And Route Gate](workpacks/06-security-proof-and-route-gate.md) | 14/18 | all prior workpack proof roots, including WP08 | `output/account-identity-family-plan-proof/06-security-proof-and-route-gate/` |
| partial source reviewed / adapters, caller, tests, and Cloudflare consumer open | [WP09 Account Issuer Key Custody And Cloudflare Handoff](workpacks/09-account-issuer-key-custody-and-cloudflare-handoff.md) | Canonical `4f6245e51` integrates durable issuer/key lineage, startup validation, and receipt/wire outbox core; no protected signer, binding authenticator, delivery owner, production caller, Cloudflare outer-wire/key-registry consumer, or seven expected test roots exist | `PLAN_STATE.md`, WP08 sealed transport contract, `docs/engineering-graph/code-map.json` | `docs/proof/account-identity-family-plan/09-account-issuer-key-custody-and-cloudflare-handoff/` |

## Default execution order

```text
WP01 -> Account WP08 -> (Account WP09 issuer/key custody || Account WP02 target authority) -> Cloudflare WP06 writer/caller -> Device Trust WP03 ceremony -> Account WP05A multi-owner fence -> Cloudflare WP08 -> WP03 -> WP04 -> WP05 -> WP07 -> Account WP06
```

## WP09 route state

WP09's Account-owned durable issuer core is independently reviewed and integrated through canonical `4f6245e51`. It owns monotonic public-key registry/revocation, strict startup validation/recovery, household-scoped receipt/wire outbox custody, and the typed handoff over WP08's existing sealed contract. Its only current direct dependency is Account WP08 as a reviewed-implementation prerequisite; normal completion remains gated on WP08 DONE. The crate-private signer/binding/delivery seams have no implementation or production caller, seven expected test roots are absent, and Cloudflare has no outer-wire/key-registry consumer. This route does not claim a complete producer adapter, Cloudflare runtime readiness, Device Trust, Account WP02, Account WP05A, tests, proof, READY, or DONE.

## WP08 verification state

WP08 has a tracked, hand-authored durable manifest under
`docs/proof/account-identity-family-plan/08-rust-schema-workers-d1-runtime-migration/`.
Its prior `9/9` status proves only the earlier Rust authority/generated-edge
packet; the 2026-08-16 account-identity handoff began as a code-drafted packet
and is not covered by that manifest. Raw or generated output remains ignored.
Independent P0/P1 review accepts the 2026-08-17 bounded source packet: canonical
household/child/device binding with pairing, installation, selected route,
lifecycle, revocation, bounded generation, guarded identifiers, active provider
mapping, exact account consistency, and a crate-private fail-closed read port.
It does not authorize completion, tests, proof, Cloudflare runtime, or a
production authority claim. WP02's target-aware action source is reviewed, but
its expected tests and normal completion remain open. Cloudflare WP06's
authoritative writer/provider caller, Device Trust WP03's live binding, and
Cloudflare WP08 proof remain open.

## WP02-WP05 live production correction

The checked WP02-WP05 rows preserve an earlier local contract/proof slice and
do not close the replacement source. Current reviewed source adds the sealed
Account capability, durable authority/session custody, target-aware action
resolution, and WP04 strict invite/recovery repository with private owner
receipts. WP05's current runtime composer is only a fail-closed boundary: its
manual-required CAS fence has no durable Account-owned opaque-effect repository,
schema, or crash/replay recovery owner. Data Custody WP08 confirmation
staging/consume depends on that handoff. Cloudflare provider composition, WP04
production owner adapters, remaining runtime handoffs, the complete
expected-test wave, validation, and current proof are still open.

The rejected `codex/account-wp02-wp05-source-wave` packet at `ac03afee3a` is
quarantined remote evidence only. It added public deserializable lifecycle
records with caller-mintable proof/replay/freshness state and no production
caller or durable compare-and-swap owner. Do not revive those DTOs. The first
legal replacement is the accepted `35edb2830` packet now integrated through
`e69acf279`; the rejected DTOs remain quarantined and are not implementation.

## Dependency rules

```text
WP01 blocks runtime provider/session implementation.
WP08 owns only the Rust-owned account/family contract authority and account-authority parity. It is not WP01 provider-decision work and it does not own any Cloudflare binding, adapter, migration, or worker test runner.
WP02 consumes WP08. Its target-aware actor/action source now rejects caller-supplied same-family, capability, lease, and step-up trust, but expected tests and normal completion remain open. WP05 consumes the new WP05A multi-owner fence and owns Account authorization only; it must not copy Device Trust or parent-step-up state. WP05A owns coordinator/recovery, the private Account participant adapter, and Account-side capability/lease reservations, while Account WP02/WP08 retain Account source truth and Device Trust WP01/WP03 retain their own source-of-truth participants. Data Custody WP08/WP09/WP10/WP11 confirmation and runtime composition remain blocked behind those typed opaque handoffs. Cloudflare WP06 still owns authoritative provider composition and remains the next source dependency before most downstream authorization, UI, policy, payment, remote-access, and Device Trust handoffs.
Cloudflare WP06 consumes WP02 and owns the authoritative D1 writer/update/revocation/CAS surface, provider-to-sealed-authority caller, migration execution, and storage-proof packet. Its existing D1 read adapter is not sufficient. Cloudflare WP08 owns the Cloudflare runner/pyramid proof after WP06. Neither packet redefines account/family authority.
WP03 blocks secure-login/session claims and must be read with workpacks/03-current-boundary-addendum.md.
WP04 may run after WP02 but must not implement data-custody side effects itself.
WP05 depends on WP02/WP03 authority and session freshness models.
WP07 depends on WP02 and enough WP03/WP04 state to render honest setup states.
WP06 must be last and is reopened until it consumes green WP08 Rust authority proof plus the exact Cloudflare WP06 storage and Cloudflare WP08 runner/proof handoffs. A recorded blocker keeps this gate and dependent payment/policy/remote/device-trust scheduling blocked; it is not a release condition.
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
