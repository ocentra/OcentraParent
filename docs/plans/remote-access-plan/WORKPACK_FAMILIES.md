<!-- agent-capsule -->

> Agent Capsule
> Plan: `remote-access-plan`
> Doc: `Remote Access Workpack Families`
> Kind: owner-path classifier for selected workpacks.
> Read when: only after `WORKPACK_INDEX.md` selects or names a workpack and owner/proof family is unclear.
> Stop rule: classify the selected workpack only; do not use this file as permission to scan every workpack.
> Proves: routing and owner-path classification only.
> Does not prove: remote runtime readiness, relay readiness, live-view readiness, control readiness, custody readiness, or PR readiness.
> Proof rule: if this file changes route/status claims, update `AGENTS.md`, `PLAN_STATE.md`, and any affected selected workpack route.

<!-- /agent-capsule -->

# Remote Access Workpack Families

Use this file to classify a selected workpack before opening source. This plan owns remote live-access authority and proof. It consumes screen, LAN, account, device-trust, data-custody, portal, protocol, and eventing handoffs; it does not own those sibling runtime surfaces.

## Capability fabric and standing grant family

```text
Workpacks:
WP01 Remote Capability Fabric

Owners:
remote-access-plan for capability types, standing access semantics, route/session model, grant lifecycle, and no-claim boundaries
account-identity-family-plan for account/household/role/session/device authority
device-trust-bootstrap-plan for parent presence and trusted-device step-up

Rule:
Remote capability proof must separate live view, screenshot/diagnostic, and deferred control capabilities. One generic remote flag cannot authorize unrelated actions.
```

## Live screen relay family

```text
Workpacks:
WP02 Live Screen Relay

Owners:
remote-access-plan for relay session, standing paired viewing authority, unavailable/degraded states, and remote custody boundary
screen-plan for capture primitives, protected-surface behavior, screenshot custody, and local screen retention settings
data-custody-storage-plan for raw frame/screenshot/recording retention/export/delete rules

Rule:
Live-view proof is not local screenshot proof and not remote control proof. Raw frames are not retained unless explicitly authorized by custody and screen settings.
```

## Deferred remote control family

```text
Workpacks:
WP03 Remote Input Control Authority

Owners:
future control slice only; not current live-view pass
remote-access-plan for high-risk control capability model when this workpack is explicitly opened
enforcement/platform/account/device-trust owners for action authority, platform permission, and step-up proof

Rule:
Remote input/control is off by default and deferred. No current live-view claim may imply keyboard, pointer, app focus, admin, or child-device control.
```

## Pairing, disclosure, revocation, and standing access family

```text
Workpacks:
WP04 Session Pairing Grants

Owners:
remote-access-plan for pairing/grant lifecycle, standing-access visibility, revoke/remove-device behavior, reconnect/crash recovery semantics, and audit proof
account-identity-family-plan for actor/household/session/device authority
portal/child surfaces for visible disclosure state when selected

Rule:
Standing access remains until revoke or device removal. Revocation and device removal win over reconnect, relay, cache, and stale grant reuse.
```

## Relay security, abuse, and availability family

```text
Workpacks:
WP05 Relay Security Abuse Controls

Owners:
remote-access-plan for authenticated/scoped relay sessions, rate limits, backpressure, replay/cross-household isolation, outage/degraded states, and redacted diagnostics
data-custody-storage-plan for diagnostic retention and private payload boundaries
account-identity-family-plan for token/session authority

Rule:
Relay availability is not permission to retain raw screen, input, or child-private payloads. Relay production claims require abuse, load, replay, and cross-household proof.
```

## Rollout proof and route gate family

```text
Workpacks:
WP06 Rollout Proof And Route Gate

Owners:
selected proof roots under `output/remote-access-plan-proof/<workpack>/`
PLAN_STATE, WORKPACK_INDEX, NEXT_ACTIONS, PROOF_INDEX, TEST_PROOF_EXPECTATIONS, PLAN_HEALTH, and selected workpacks when state changes

Rule:
Rollout proof may aggregate only accepted proof roots or exact carried blockers. Local capture proof, LAN proof, UI-only proof, relay route presence, or legacy docs cannot become remote readiness.
```
