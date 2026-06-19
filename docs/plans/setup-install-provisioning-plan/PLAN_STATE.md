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
Implementation: scoped WP01, WP02, WP04, and WP05 typed source/test/proof validation is green in owned slices, with WP05 now backed by explicit pairing-token redaction contracts in `@ocentra-parent/family-domain` plus a focused provisioning bootstrap audit projection test in `ocentra-provisioning-core`; WP03 proof and handoff coverage now exists with setup-domain plus production-domain validation green, a repaired `@ocentra-parent/parent-domain` build path, and a direct green `parent-desktop-release-support` suite, while the package test wrapper still detours into an unrelated app-game proof harness; WP07 portal/state-machine implementation, portal-domain tests, portal render tests, and Playwright proof are now green on this branch; broader plan completion remains partial
Proof artifacts: WP01 proof root is green for the owned public-site/data-boundary slice; WP02 proof root is green for the owned account-entry/handoff slice; WP03 proof root exists for the owned parent-install/handoff slice and now records the repaired parent-domain build plus the narrower workspace test-wrapper blocker; WP04 proof root is green for the owned child install/permission slice; WP05 proof root is green for the owned pairing slice, including pairing-token redaction and bootstrap audit projection proof; WP07 proof root is now present and green for the owned first-run route projection; WP06 blocker-pack proof root is refreshed and now consumes WP07 plus the repaired WP03 build state, while whole-plan rollout remains blocked by sibling-owner proof gaps
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
- WP01 public family-site route/data-boundary proof root now exists, but preview/custom-domain/public-runtime proof is still blocker-only.
- WP02 registration/login proof root now exists, but live provider/session/household implementation remains owned by `account-identity-family-plan`.
- WP03 parent install journey proof root now exists, and the repaired export surface clears `npm run build --workspace @ocentra-parent/parent-domain`; the direct suite `Push-Location packages/parent-domain; npx vitest run tests/unit/parent-desktop-release-support.test.ts; Pop-Location` passes, while `npm run test --workspace @ocentra-parent/parent-domain -- parent-desktop-release-support` still detours through `scripts/test/app-game-source-gated-policy-preview-read-model-proof.mjs` and fails on nonexistent `app-game-*` filters before proving the workspace wrapper path; signed installers, notarization, store delivery, update/rollback execution, and production publishing remain owned by `parent-desktop-runtime-package-plan`.
- WP04 child install/permission journey proof root now exists, and the rendered first-run setup proof is now green in WP07, but real child runtime/package/platform execution remains owned by sibling plans.
- WP05 pairing/readiness/recovery proof root now exists and its pairing/bootstrap redaction ownership gap is locally closed, but real LAN/device-trust proof remains sibling-owned.
- WP06 rollout blocker pack now consumes WP07, the repaired WP03 build state, and the closed WP05 redaction proof while whole-plan rollout remains blocked by sibling owner proofs only.
- Sibling owner plans still hold unmet account/provider/session, runtime distribution, device trust, LAN, portal UX beyond the owned Start route, data custody, policy, and payment proof.
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
