<!-- agent-capsule -->

> Agent Capsule
> Plan: `account-identity-family-plan`
> Doc: `WP06 Security Proof And Route Gate`
> Kind: final proof aggregation and route-sync workpack.
> Read when: selected by WORKPACK_INDEX.md or when making DONE/PR_READY claims for this plan.
> Stop rule: do not implement new feature scope here; consume prior proof roots, close route gaps, and record blockers.
> Proves: account identity route readiness only after prior proof roots and this gate pass.
> Does not prove: payment, policy, data custody, device trust, LAN, remote, or setup-install readiness.
> Proof rule: before DONE/PR_READY, write all WP06 proof artifacts and command log.

<!-- /agent-capsule -->

# WP06 Security Proof And Route Gate

## Goal

Aggregate security, misuse, route-sync, and no-overclaim proof before account/identity/family authority can be called ready.

## Required inputs

```text
workpacks/00-owner-boundary-proof-gate.md
PROOF_INDEX.md
TEST_PROOF_EXPECTATIONS.md
CHECKLIST_INDEX.md
output/account-identity-family-plan-proof/01-auth-provider-decision/
output/account-identity-family-plan-proof/08-rust-schema-workers-d1-runtime-migration/
output/cloudflare-control-plane-plan-proof/06-storage-do-d1-kv-r2-queue-bindings/
output/cloudflare-control-plane-plan-proof/08-testing-runner-and-test-pyramid/
output/account-identity-family-plan-proof/02-identity-household-role-model/
output/account-identity-family-plan-proof/03-session-token-lifecycle/
output/account-identity-family-plan-proof/04-invites-recovery-lifecycle/
output/account-identity-family-plan-proof/05-device-ownership-authz/
output/account-identity-family-plan-proof/07-parent-account-family-setup-ui/
```

If any prior proof root is missing, this workpack records a blocker and does not claim readiness.

## Current owner/import/proof constraints

This workpack is an aggregation gate. It must not implement new auth, setup, policy, payment, custody, LAN, remote, device-trust, Cloudflare, or portal runtime behavior.

```text
schema-domain/family-domain/family-identity-core: consumed only as prior account/family proof surfaces.
setup-install/cloudflare/payment/policy/data-custody/device-trust/LAN/remote/portal UX: adjacent consumer routes only.
WP06: proof aggregation, manual-required gap register, and route-sync evidence.
```

Allowed edits are proof/status/docs and explicitly selected route-sync notes. Do not import or modify adjacent runtime internals to make the route gate green. If a sibling route is not ready, record a blocker or manual-required entry.

## Required proof pack

```text
provider decision proof
account/household role matrix proof
session/token lifecycle proof
invite/recovery lifecycle proof
device ownership authZ proof
WP08 Rust schema/account-authority proof
Cloudflare WP06 D1/DO/KV binding, migration, and storage proof or exact blocker
Cloudflare WP08 test-runner/integration proof or exact blocker
setup UI proof or explicit UI blocker
state-changing request safety proof or blocker
logging/redaction proof
manual-required gap register
route sync proof
```

## Route sync boundaries

This plan exports authority to these plans; it does not absorb their implementation:

```text
setup-install-provisioning-plan consumes account/setup entry after provider/session/household proof.
cloudflare-control-plane-plan consumes auth adapter/worker boundary after provider decision.
payment-subscription-plan consumes parent-owner/account authority after WP05/WP06 proof.
policy-control-plane-plan consumes household/role/device/session context after WP02/WP03/WP05 proof.
data-custody-storage-plan consumes deletion/export authorization after WP04/WP05 proof.
device-trust-bootstrap-plan consumes account/household identity but owns trusted-device bootstrap.
lan-plan consumes account/household/device identities but owns LAN transport proof.
remote-access-plan consumes remote-view/control capability authorization but owns remote transport execution proof.
portal-ux-household-surfaces-plan consumes setup/role read models but owns broader portal shell UX.
```

## Required proof root

```text
output/account-identity-family-plan-proof/06-security-proof-and-route-gate/
```

Required artifacts:

```text
00-security-proof-pack.md
01-authn-negative-proof.md
02-authz-matrix-proof.md
03-token-replay-proof.md
04-recovery-abuse-proof.md
05-origin-csrf-open-redirect-proof.md
06-route-sync-proof.md
07-logging-redaction-proof.md
08-manual-required-gap-register.md
09-account-authority-cloudflare-storage-gate.md
16-validation-commands.log
```

## Acceptance criteria

- [ ] WP01 provider proof root exists or blocker recorded.
- [ ] WP08 Rust schema/account-authority proof root exists or blocker recorded.
- [ ] WP02 household/role proof root exists or blocker recorded.
- [ ] WP03 session/token proof root exists or blocker recorded.
- [ ] WP04 invite/recovery proof root exists or blocker recorded.
- [ ] WP05 device ownership proof root exists or blocker recorded.
- [ ] WP07 setup UI proof root exists or blocker recorded.
- [ ] Authentication negative proof exists.
- [ ] Authorization matrix proof exists.
- [ ] Token misuse proof exists.
- [ ] Recovery/invite misuse proof exists.
- [ ] Origin/request safety proof exists or blocker recorded.
- [ ] Logging redaction proof exists or blocker recorded.
- [ ] Cloudflare WP06 storage proof and Cloudflare WP08 runner/proof are re-aggregated after their focused validation or each exact blocker is recorded; prior WP06 completion evidence is not reused as final-gate proof.
- [ ] Route sync proof names all adjacent consumers.
- [ ] Manual-required gap register exists.
- [ ] Focused commands pass or blockers are recorded.
- [ ] PLAN_STATE and CHECKLIST_INDEX reflect actual proof state.

## Focused commands

```bash
npm run build --workspace @ocentra-parent/family-domain
npm run test --workspace @ocentra-parent/family-domain
npm run test --workspace @ocentra-parent/portal -- account
cargo test -p ocentra-parent-agent-protocol account
npm run lint:architecture -- --files packages/family-domain packages/portal-domain apps/portal crates/agent-protocol
```

If policy/eventing lanes are active, do not route-sync those plan files directly. Record a route-sync blocker or queued handoff instead.

## Negative cases

- Missing provider decision blocks auth-ready claim.
- Cross-family access denied.
- Revoked actor denied.
- Stale/reused token denied.
- Reused invite denied.
- Recovery/invite misuse denied or blocked from readiness claim.
- State-changing browser request without required safety signal denied or blocked from readiness claim.
- Support/admin cannot act as owner.
- Child profile cannot authorize child device.
- Login cannot authorize policy/payment/remote/export without role/device/freshness gates.
- Account WP08 cannot substitute Cloudflare storage/runner proof, and Cloudflare cannot redefine the Rust authority contract, for final-gate input.

## Manual-required gaps

Any adjacent plan not yet updated must be listed in `08-manual-required-gap-register.md`. A route-sync gap blocks whole-plan PR_READY but does not necessarily block a single workpack PR.

## Fill before DONE / PR_READY

```text
Workpack id and branch: WP06 Security Proof And Route Gate / codex/tracking-plan-full-continuation-a
Current branch note: this historical completion record predates the plan-harness branch. On codex/plan-harness-update, treat it as prior proof evidence only; new edits must follow workpacks/00-owner-boundary-proof-gate.md, TEST_PROOF_EXPECTATIONS.md, and PROOF_INDEX.md.
Current status: reopened / rerun required. The prior local aggregation pack remains historical evidence only. It cannot be final-gate proof until WP08's Rust authority root plus Cloudflare WP06 storage and Cloudflare WP08 runner/proof roots are consumed or precisely blocked in `09-account-authority-cloudflare-storage-gate.md`.
Prior proof roots consumed: `output/account-identity-family-plan-proof/01-auth-provider-decision/`; `output/account-identity-family-plan-proof/02-identity-household-role-model/`; `output/account-identity-family-plan-proof/03-session-token-lifecycle/`; `output/account-identity-family-plan-proof/04-invites-recovery-lifecycle/`; `output/account-identity-family-plan-proof/05-device-ownership-authz/`; `output/account-identity-family-plan-proof/07-parent-account-family-setup-ui/`. WP08 is a required missing input, not a consumed root.
Route-sync status: adjacent consumer boundaries for setup-install, Cloudflare, payment, policy, data custody, device trust, LAN, remote, and portal UX are now consumed from the account proof roots. WP07 is no longer blocked; the real setup-route proof root is part of this gate pack. Browser request-safety remains an explicit blocker because this slice still does not own a real browser request consumer.
Validation commands and results: see `output/account-identity-family-plan-proof/06-security-proof-and-route-gate/16-validation-commands.log`. This slice re-used prior focused command logs from WP01-WP05 and WP07, then ran docs/proof-slice verification only.
Manual-required gaps: browser request-safety remains blocked until a later slice owns a real browser request consumer; WP08 Rust schema/account-authority proof plus Cloudflare WP06 storage and Cloudflare WP08 runner/proof must be completed or precisely blocked before this gate reruns; Cloudflare worker/runtime proof, payment execution, policy execution, data-custody execution, device-trust bootstrap, LAN transport, remote transport, and broader portal UX/runtime remain owned by adjacent plans.
No-claim boundaries: do not claim PR_READY; do not claim product-ready account/family flow; do not claim Cloudflare runtime, payment runtime, policy runtime, data-custody execution, device-trust bootstrap, LAN transport, or remote transport readiness from this WP06 closure.
```
