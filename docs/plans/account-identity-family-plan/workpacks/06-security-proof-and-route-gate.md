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
PROOF_INDEX.md
TEST_PROOF_EXPECTATIONS.md
CHECKLIST_INDEX.md
output/account-identity-family-plan-proof/01-auth-provider-decision/
output/account-identity-family-plan-proof/02-identity-household-role-model/
output/account-identity-family-plan-proof/03-session-token-lifecycle/
output/account-identity-family-plan-proof/04-invites-recovery-lifecycle/
output/account-identity-family-plan-proof/05-device-ownership-authz/
output/account-identity-family-plan-proof/07-parent-account-family-setup-ui/
```

If any prior proof root is missing, this workpack records a blocker and does not claim readiness.

## Required proof pack

```text
provider decision proof
account/household role matrix proof
session/token lifecycle proof
invite/recovery lifecycle proof
device ownership authZ proof
setup UI proof or explicit UI blocker
origin/state-changing request safety proof or blocker
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
16-validation-commands.log
```

## Acceptance criteria

- [ ] WP01 provider proof root exists or blocker recorded.
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

## Manual-required gaps

Any adjacent plan not yet updated must be listed in `08-manual-required-gap-register.md`. A route-sync gap blocks whole-plan PR_READY but does not necessarily block a single workpack PR.

## Fill before DONE / PR_READY

```text
Workpack id and branch:
Prior proof roots consumed:
Route-sync status:
Touched files:
Validation commands and results:
Proof artifacts:
Manual-required gaps:
No-claim boundaries:
```
