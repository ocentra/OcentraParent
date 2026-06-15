<!-- agent-capsule -->

> Agent Capsule
> Plan: `setup-install-provisioning-plan`
> Doc: `Setup Install Provisioning Plan State`
> Kind: current state and open gaps.
> Read when: immediately after AGENTS.md.
> Stop rule: use this file to choose route state, then continue only to NEXT_ACTIONS.md and WORKPACK_INDEX.md.
> Proves: current plan state and open-gap accounting only.
> Does not prove: implementation completion, deployed website, installer readiness, or PR readiness.
> Proof rule: if state changes, update the assigned workpack, CHECKLIST_INDEX.md, and PROOF_INDEX.md proof path.

<!-- /agent-capsule -->

# Setup Install Provisioning Plan State

## Current status

```text
Plan route: upgraded
Execution-grade workpacks: in progress
Implementation: not started by this plan route
Proof artifacts: none recorded by this plan route yet
PR-ready: false
```

## Current product direction

```text
family.ocentra.ca is a public information/download/account-entry surface by default.
It must not collect child activity data.
Registration/login and household authority are account-identity handoffs.
Parent bootstrap/install and child bootstrap/install are separate flows.
Installer build/signing/update artifacts are owned by runtime distribution plans.
Pairing protocol internals are owned by LAN/device-trust plans.
This plan owns the setup journey/state machine/readiness labels/proof manifest across those handoffs.
```

## Current repo facts already read

- `docs/features/family-setup-device-roles.md` says family setup is product foundation and first-run setup is not product-complete.
- `docs/expectations/family-setup.md` requires household creation/join, child profiles, device roles/status, co-parent/observer removal, recovery, and source-state labels.
- `docs/expectations/portal.md` says portal sends typed requests to the child-device agent and must show live/stale/degraded/unavailable states honestly.
- `docs/expectations/platforms.md` says platform claims must match real OS capabilities and scaffold/package preview does not prove production capability.

## External research anchors

- Cloudflare Pages can host full-stack apps on Cloudflare's network and supports Git/direct upload/C3 deploy modes, Pages Functions, rollbacks, redirects, and custom domains.
- Cloudflare Workers static assets can serve static application assets behind Workers when a Worker-owned route is preferred.
- Tauri updater/signing/notarization/package behavior is owned by runtime distribution plans, not by this setup plan.
- Android package/permission visibility and iOS/macOS distribution/notarization constraints must be handled by platform/package owner plans before production install claims.

## Open gaps

```text
- No final public family-site route/deploy shape proof exists.
- No single setup state machine ties account, parent app, child agent, permissions, pairing, recovery, custody, and policy baseline.
- No platform-specific parent/child install journey matrix has been proven.
- No parent bootstrap code state machine proof exists.
- No child pairing/bootstrap code state machine proof exists.
- No first-run setup UI proof exists.
- No rollout proof manifest exists under output/setup-install-provisioning-plan-proof/.
- No route-sync proof exists for account identity, runtime distribution, device trust, LAN, portal UX, data custody, policy, or payment handoffs.
```

## No-claim boundaries

Do not claim:

```text
public family site deployed
registration/login implemented
parent installer ready
child installer ready
pairing ready
first-run setup ready
platform support ready
production onboarding ready
```

until the relevant workpack proof root and checklist rows prove the claim.

## Default execution order

```text
WP01 family web info site
WP02 registration login entry
WP03 parent install journey
WP04 child install permission journey
WP05 pairing readiness recovery
WP07 first-run setup UI and state machine
WP06 rollout proof and route gate
```

WP06 is last because it consumes the earlier proof roots.

## Health rules

- Do not implement account identity/session logic here.
- Do not implement package signing/update/installer generation here.
- Do not implement LAN protocol internals here.
- Do not implement device trust/key sealing here.
- Do not implement data export/delete/custody side effects here.
- Do not mark setup complete from website/installer/UI-only proof.
- Do not edit policy/eventing plan files while active lanes own them.
