<!-- agent-capsule -->

> Agent Capsule
> Plan: `setup-install-provisioning-plan`
> Doc: `Setup Install Provisioning Workpack Families`
> Kind: owner-path classifier for selected workpacks.
> Read when: only after `WORKPACK_INDEX.md` selects or names a workpack and owner/proof family is unclear.
> Stop rule: classify the selected workpack only; do not use this file as permission to scan every workpack.
> Proves: routing and owner-path classification only.
> Does not prove: deployment, account readiness, package readiness, pairing trust, custody readiness, policy readiness, entitlement readiness, first-run completion, or PR readiness.
> Proof rule: if this file changes route/status claims, update `AGENTS.md`, `PLAN_STATE.md`, and any affected selected workpack route.

<!-- /agent-capsule -->

# Setup Install Provisioning Workpack Families

Use this file to classify a selected workpack before opening source. This plan owns setup journey/readiness/handoff proof. It consumes sibling owner proofs without owning their implementation.

## Public family site family

```text
Workpacks:
WP01 Family Web Info Site

Owners:
setup-install-provisioning-plan for public route map, public/private data boundary, privacy wording, download/register/support/status links, and deployment blocker state
Cloudflare/deployment owner plans or operators for live deploy/custom-domain proof when selected

Rule:
Public-site proof must not collect child activity data and must not imply deployed site, account readiness, installer readiness, or setup readiness.
```

## Registration and account-entry handoff family

```text
Workpacks:
WP02 Registration Login Entry

Owners:
setup-install-provisioning-plan for route-state map, account handoff labels, provider-unavailable state, invite negative states, and no-sensitive-data-before-authority proof
account-identity-family-plan for provider selection, sessions, tokens, household authority, invites, roles, and recovery truth

Rule:
Registration proof is a handoff proof only. It cannot implement auth/session logic, household membership, profile/device creation, or recovery authority.
```

## Parent install journey family

```text
Workpacks:
WP03 Parent Install Journey

Owners:
setup-install-provisioning-plan for parent-visible bootstrap/install route state, platform matrix, version/integrity display expectations, support/recovery labels, and package-owner handoff
parent-desktop-runtime-package-plan for package build, signing, notarization, store delivery, update, rollback, installer checksums, and runtime distribution truth
payment-subscription-plan for entitlement if a package route depends on subscription

Rule:
Download or install journey proof cannot claim signed package, updater, rollback, store delivery, or product install readiness without runtime distribution proof.
```

## Child install and permission journey family

```text
Workpacks:
WP04 Child Install Permission Journey

Owners:
setup-install-provisioning-plan for child bootstrap state, platform/permission/readiness labels, disclosure expectations, reinstall recovery, and runtime-owner handoffs
child-agent-runtime-distribution-plan for child package artifacts and distribution proof
app-plan or child runtime owners for local service/runtime/platform adapter behavior
device-trust-bootstrap-plan and lan-plan for trust/pairing dependencies

Rule:
Installed, running, permissioned, paired, trusted, and policy-ready must remain separate states.
```

## Pairing, readiness, and recovery family

```text
Workpacks:
WP05 Pairing Readiness Recovery

Owners:
setup-install-provisioning-plan for setup pairing journey, readiness matrix, recovery UX, redacted setup diagnostics, and no-fake-ready proof
lan-plan for local discovery, signed hello, LAN pairing protocol, and physical LAN proof
account-identity-family-plan for household/device authority
device-trust-bootstrap-plan for trusted-device approval/key proof
data-custody-storage-plan and policy-control-plane-plan for custody and policy-baseline readiness inputs

Rule:
Pairing discovered, LAN-visible, or UI-rendered is not trusted/ready until parent authority, device trust, custody, and policy-baseline states are accounted for.
```

## First-run UI state-machine family

```text
Workpacks:
WP07 First-Run Setup UI And State Machine

Owners:
setup-install-provisioning-plan for typed first-run state machine, readiness cards, setup route projection, adjacent handoff visibility, source/custody labels, and no-fake-ready proof
portal-domain/apps/portal for selected setup route rendering only
portal-ux-household-surfaces-plan for broader portal UX completion
all sibling owner plans for their readiness inputs

Rule:
First-run UI proof is not production onboarding readiness. Setup complete cannot render unless the readiness matrix proves or visibly blocks account, parent app, child agent, pairing, permissions, custody, and policy baseline.
```

## Rollout proof and route gate family

```text
Workpacks:
WP06 Rollout Proof And Route Gate

Owners:
setup-install-provisioning-plan for proof aggregation, route/index sync, public/private boundary, platform readiness matrix, manual-required gap register, and safe wording
sibling owner plans for unresolved implementation proof

Rule:
WP06 may aggregate only accepted proof roots or exact blockers. It cannot turn sibling gaps into setup readiness, and it cannot create PR-ready claims while sibling proofs remain unaccepted.
```
