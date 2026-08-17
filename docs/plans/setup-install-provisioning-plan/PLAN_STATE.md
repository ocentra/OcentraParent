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
Setup-plan-owned workpacks: WP01, WP02, WP03, WP04, WP05, WP07, and WP06 closed for local proof roots
Whole-plan production onboarding: blocked by sibling-owner proof gaps
PR-ready: false
```

## Status interpretation

```text
Done workpack = the setup-plan-owned state/handoff/proof slice is closed.
Done workpack != deployed website, account readiness, package readiness, child runtime readiness, trusted LAN pairing, custody readiness, policy baseline readiness, payment entitlement readiness, or production onboarding readiness.
WP06 = rollout blocker/aggregation pack; it can be locally complete while broad setup readiness stays blocked.
```

## Production reachability audit (2026-08-16)

This is a production-code audit only. The local `done`/proof wording above is
retained as historical narrow-slice state; it does not promote any setup
journey into shipped onboarding readiness.

| Workpack | Reachable production code | Missing production owner/call path | Code-pass decision |
| --- | --- | --- | --- |
| WP01 Family Web Info Site | `apps/portal/src/main.ts` and `apps/portal/src/hosted-portal-distribution.tsx` ship a parent portal/hosted distribution boundary. `infra/cloudflare/src/routes.ts` exposes health, pricing, billing, admin, and webhook routes, but no family-site/setup route. | No shipped public family-site entrypoint, deployment/custom-domain owner, or real download/register/support handoff is present in this repo. Cloudflare/deployment and runtime-distribution owners remain required. | No setup-owned slice; public-site proof is not runtime. |
| WP02 Registration Login Entry | `crates/provisioning-core/src/provisioning_install/family_context.rs` can classify an already supplied family context, but no production caller supplies account/session/household authority to it. The portal Start route is a presentation projection only. | Provider verification, session/token issuance, household membership, invite, and recovery are account-identity-owned and unresolved; no setup route can authorize a write or mint identity. | Blocked on `account-identity-family-plan`; no edit. |
| WP03 Parent Install Journey | `apps/parent-desktop/src-tauri/src/lib.rs` has shipped Tauri route commands and `crates/provisioning-core/src/provisioning_install.rs` has install-state contracts. Neither is a package/install/update entrypoint, and no caller connects them to a signed artifact. | Signed package, platform installer, integrity/checksum, update/rollback, store delivery, and publishing authority belong to `parent-desktop-runtime-package-plan`. | No setup-owned slice; installer/proof text is not runtime. |
| WP04 Child Install Permission Journey | `crates/child-runtime/src/bin/ocentra-child-agent-service.rs` starts the real child service and `crates/child-runtime/src/service.rs` durably gates tamper/recovery/removal state. `crates/child-runtime/src/runtime_gate.rs` defines provisioning preflight but `rg` found no production caller constructing or evaluating `ChildRuntimePreflightInput`. | Child package/distribution, platform permission/disclosure, and the missing preflight input owner are outside setup; no trusted account/device handoff reaches the shipped child service. | Blocked on `child-agent-runtime-distribution` plus account/device-trust inputs; no edit. |
| WP05 Pairing Readiness Recovery | LAN pairing has a real agent-service owner (`crates/agent-service/src/lan_pairing_command_entrypoints.rs`, `crates/agent-service/src/app.rs`) and a parent read path (`crates/parent-runtime-core/src/agent_service_client/loaders.rs`, `parent_ui_bridge/lan_route.rs`) with durable/fail-closed protocol state. | The Start setup snapshot does not consume that read model, and no setup-owned aggregation joins it with account, install, permission, custody, and policy readiness. Physical LAN/device-trust authority remains sibling-owned. | A pairing-only projection would not satisfy setup progression; no edit without the missing cross-owner readiness inputs. |
| WP07 First-Run Setup UI And State Machine | The route is reachable through Tauri -> `load_parent_route_snapshot` -> LAN read query -> `setup_first_run.rs` -> the generated bridge -> `SetupFirstRunRoutePanel.tsx`. Rust now owns an explicit 15-row authority matrix, reports `not-run`, `manual-required`, and `0/15` trusted inputs, and preserves LAN unavailable diagnostics. | No trusted account, package, child-runtime, pairing/device-trust, permission, custody, policy, or recovery input is bound. The evaluator/action planner is deliberately not invoked; the accepted source delta changed no tests, and the existing Rust/portal/E2E setup fixtures still describe the removed panel. | Accepted fail-closed source boundary only. Expected tests, real state-machine actions, completion guard, sibling authority inputs, and retained proof remain open. |
| WP06 Rollout Proof And Route Gate | No production entrypoint; this workpack is documentation/proof aggregation only. | It cannot create runtime authority or upgrade sibling blockers. | Proof-only; excluded from production-code closure. |

### Audit findings and stale topology

- The graph report at `f7d8f4e33` reports WP01-WP04/WP06 as `no-source` with
  `no-code-required`, and WP05/WP07 as `code-and-tests`; the plan index still
  labels all seven local proof slices `done`/`done-but-blocked`. This is a
  lifecycle/proof topology difference, not evidence of shipped setup runtime.
- The graph report has no implementation roots for the public site, account
  entry, parent-install, or rollout workpacks. Their proof roots therefore do
  not map to production callers.
- `PLAN_STATE.md` describes a `packages/setup-domain` package boundary, but
  that directory/package is absent from the current checkout. The actual
  setup contract owner found in source is
  `crates/provisioning-core/src/provisioning_install.rs`; the plan wording is
  stale and must not be used as an implementation path.
- No legal setup production slice is authorized by this audit. The smallest
  future slices are: account-owned verified session/household handoff (WP02),
  package-owner signed parent artifact/install state (WP03), child-owner
  package/permission/preflight input handoff (WP04), and a setup-owned
  aggregation that consumes those trusted states plus the existing LAN read
  model before WP07 can report anything beyond manual-required/unavailable.

## Accepted WP07 fail-closed source boundary (2026-08-17)

Commits `a8cdb5ca7` and `8922eaf50` are integrated on the source branch after
independent review. The accepted behavior removes caller-invented readiness,
keeps LAN state observation-only, separates Start-route reads from LAN command
authority, rejects Start discovery scans, and preserves typed LAN failures.
It does not implement setup progression. The next WP07 phase is expected-test
source repair for the Rust snapshot/dispatch tests, portal-domain panel tests,
portal route tests, and setup E2E fixture. Only after those tests are written
will focused execution begin.

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

## Current ownership interpretation

```text
setup-install-provisioning-plan:
  Public setup entry, setup journey state machine, readiness labels, manual-required gates, first-run projection, and rollout proof aggregation.

setup-domain:
  Setup/install/pairing/onboarding/provisioning contract boundary. Current package export is package-info only; internal source/tests prove selected slices but not public API readiness unless exports are added.

family-domain:
  Household/family helper contracts consumed by setup proofs when selected.

portal-domain/apps/portal:
  Selected first-run setup route projection and rendered proof only.

account-identity-family-plan:
  Account provider, session, token, invite, household, role, recovery, and authority truth.

parent-desktop-runtime-package-plan:
  Parent package, signing, update, rollback, and distribution truth.

child-agent-runtime-distribution-plan:
  Child package artifacts, runtime distribution, platform delivery, and child install proof.

device-trust-bootstrap-plan and lan-plan:
  Trusted-device approval, key/step-up proof, LAN discovery, signed hello, and pairing protocol truth.

data-custody-storage-plan, policy-control-plane-plan, payment-subscription-plan:
  Custody/export/delete/sync, policy baseline, and entitlement readiness truth.
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

## Current local proof roots

```text
WP01 public family-site route/data-boundary proof root exists; preview/custom-domain/public-runtime proof remains blocker-only.
WP02 registration/login handoff proof root exists; live provider/session/household implementation remains account-identity-owned.
WP03 parent install journey proof root exists; parent-domain build and direct release-support suite are green, while the workspace test wrapper still misroutes through an unrelated app-game proof harness.
WP04 child install/permission proof root exists; real child runtime/package/platform execution remains sibling-owned.
WP05 pairing/readiness proof root exists; pairing-token redaction and bootstrap audit projection are locally proved, while physical LAN/device-trust proof remains sibling-owned.
WP07 first-run setup route projection proof root exists and is green for the selected Start route; sibling readiness inputs remain blockers.
WP06 rollout blocker pack consumes WP01-WP05/WP07 and records safe wording/manual-required blockers; broad PR readiness remains false.
```

## Open gaps / sibling-owned blockers

```text
- Account provider/session/household/invite/recovery authority remains owned by account-identity-family-plan.
- Signed installers, notarization, store delivery, checksum/signature execution, updater rollback, and production publishing remain runtime-distribution-owned.
- Child package/runtime/platform execution remains child-agent-runtime-distribution/app/runtime-owned.
- Physical LAN pairing, signed hello, and trusted-device/device-trust proof remain LAN/device-trust-owned.
- Data custody execution and policy baseline production proof remain data-custody/policy-owned.
- Subscription/entitlement proof remains payment-owned.
- Broader portal shell/household UX remains portal-UX-owned beyond the selected WP07 setup route.
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
PR_READY
```

until the relevant workpack proof root proves the claim and WP06 aggregates it with safe wording.

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
