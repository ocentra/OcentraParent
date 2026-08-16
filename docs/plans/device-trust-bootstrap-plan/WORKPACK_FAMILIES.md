<!-- agent-capsule -->

> Agent Capsule
> Plan: `device-trust-bootstrap-plan`
> Doc: `Device Trust Bootstrap Workpack Families`
> Kind: owner-path classifier for selected workpacks.
> Read when: only after `WORKPACK_INDEX.md` selects or names a workpack and the owner/proof family is unclear.
> Stop rule: classify the selected workpack only; do not use this file as permission to scan every workpack in the family.
> Proves: routing and owner-path classification only.
> Does not prove: device trust implementation, key sealing, step-up approval, recovery, entitlement unlock, child tamper/uninstall behavior, or PR readiness.
> Proof rule: if this file changes route/status claims, update `AGENTS.md`, `PLAN_STATE.md`, and any affected selected workpack route.

<!-- /agent-capsule -->

# Device Trust Bootstrap Workpack Families

Use this file to classify a selected workpack before opening source. This plan owns the trust-bootstrap layer and proof gates. It does not own account login, setup journey, LAN transport, package install, subscription policy, data custody runtime, remote access grants, or policy authoring UX.

## Trust source-of-truth family

```text
Workpacks:
WP01 Device Trust Source Of Truth

Owners:
schema-domain for canonical trust/device/step-up/recovery shapes
family-domain for household/role/action authority helper proof when selected
device-trust-bootstrap-plan docs for source-of-truth and no-claim boundaries

Rule:
Trust source-of-truth proof must separate account login, household authority, device registration, device trust state, revocation, expiration, and child-controlled states. Login/session proof is not trust proof.
```

## Local key sealing family

```text
Workpacks:
WP02 Local Key Sealing

Owners:
platform-specific key store proof where runtime sealing is selected
schema-domain for canonical key custody state shapes
data-custody-storage-plan only after trust exists and storage/recovery artifacts are selected

Rule:
Key sealing proof must name key owner, platform store/wrapper, sealed key lifecycle, rotation/revocation, wrong user/device/key negatives, recovery interaction, unsupported/manual-required states, and no universal Ocentra decrypt key. A model doc is not platform-backed runtime proof.
```

## Parent step-up, WebAuthn, and OS approval family

```text
Workpacks:
WP03 Parent Step-Up Auth

Owners:
account-identity-family-plan for account/session/household authority
schema-domain for canonical parent step-up assertion shapes
family-domain for authority helper proof when selected
platform/WebAuthn proof only when runtime ceremony is selected

Rule:
Step-up proof must bind parent account, household, actor role, action, action device, target, nonce/challenge, expiry, and audit refs. WebAuthn/passkey contract shape is not platform ceremony proof.
```

## Phone QR approval bridge family

```text
Workpacks:
WP04 Phone QR Approval Bridge

Owners:
device-trust-bootstrap-plan for QR challenge/approval contract and replay boundary
account/family owners for actor/household authority
LAN/remote/portal only through typed handoff when selected

Rule:
QR approval proof must be action-specific, short-lived, one-time, replay-resistant, and bound to household, parent account, approving device, desktop device, target, and audit refs. QR shape alone is not approval bridge proof.
```

## Entitlement-device license binding family

```text
Workpacks:
WP05 Entitlement Device License

Owners:
payment-subscription-plan for subscription entitlement policy and billing state
device-trust-bootstrap-plan for device-bound entitlement snapshot consumption
schema-domain for shared entitlement/trust handoff shapes

Rule:
License state is not product unlock. Entitlement proof must bind signed snapshot, trusted parent device, target device, expiry, revocation, replay rejection, and no-claim boundaries. Payment remains owner of subscription semantics.
```

## Recovery reset and re-pair family

```text
Workpacks:
WP06 Recovery Reset Re-Pair

Owners:
device-trust-bootstrap-plan for recovery/reset/re-pair trust state
account plan for actor/household authority
data-custody plan for encrypted recovery bundle custody after trust exists
platform key store proof where recovery touches sealed keys

Rule:
Recovery proof must use encrypted bundle or equivalent sealed artifact, reject wrong household/device/key, prevent stale trust resurrection, preserve revocation, and distinguish account recovery from device/data recovery.
```

## Child tamper and uninstall family

```text
Workpacks:
WP07 Child Tamper Uninstall

Owners:
child-agent-runtime-distribution-plan for child package/uninstall mechanics
device-trust-bootstrap-plan for parent-authorized trust revocation and uninstall authorization boundary
account plan for actor/household authority
policy/enforcement only after typed handoff when selected

Rule:
Child tamper/uninstall proof must show child cannot control, disable, or silently remove trust. Parent-authorized removal must name actor, target, trust revocation, package uninstall handoff, audit refs, residual state, and manual-required platform gaps.
```

## Dependency adoption and security review family

```text
Workpacks:
WP08 Open Source Dependency Adoption

Owners:
docs/plans/device-trust-bootstrap-plan/DEPENDENCY_RESEARCH_AND_ADOPTION.md
security/dependency policy docs when selected
implementation owner only after a selected runtime slice adopts a dependency

Rule:
Architecture reference material is not product code adoption. A dependency can be adopted only with license, maintenance, security, supply-chain, platform, and replacement/no-claim proof.
```

## Cross-plan route and proof gate family

```text
Workpacks:
WP09 Cross Plan Route Gate

Owners:
selected proof roots under output/device-trust-bootstrap-plan-proof/<workpack>/
PLAN_STATE, ROUTE_INDEX, WORKPACK_INDEX, TEST_PROOF_EXPECTATIONS, PROOF_INDEX, PLAN_INDEX, and FEATURE_ROUTE_INDEX when route status changes
adjacent plans only when a typed handoff claim changes

Rule:
WP09 is last. It may aggregate only accepted proof roots or exact carried blockers. No PR_READY from document tests, route tests, package install, login/session, LAN pairing, license state, or one proof family standing in for another.
```
