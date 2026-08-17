<!-- agent-capsule -->

> Agent Capsule
> Plan: `device-trust-bootstrap-plan`
> Doc: `Device Trust Bootstrap Plan Workpack Index`
> Kind: workpack selector.
> Read when: after PLAN_STATE.md and NEXT_ACTIONS.md.
> Stop rule: open exactly one selected workpack; do not read every workpack.
> Proves: workpack routing only.
> Does not prove: device trust implementation, recovery readiness, entitlement readiness, or PR readiness.
> Proof rule: update counts/status only after matching checklist rows and proof artifacts exist.

<!-- /agent-capsule -->

# Device Trust Bootstrap Plan Workpack Index

Choose one workpack. Do not open all workpacks.

Use `WORKPACK_FAMILIES.md` only when the selected workpack owner/proof family is unclear. Do not use it as permission to scan multiple workpacks.

| Status | Workpack | Boxes | Primary source docs | Proof root |
| --- | --- | ---: | --- | --- |
| ready / implementation route | [WP01 Device Trust Source Of Truth](workpacks/01-device-trust-source-of-truth.md) | parent-presence slice proved; broader lifecycle remains partial; READY route is drafted and independently static-reviewed for the missing persistent trusted-device/signer-key registration implementation; tests, proof, validation, caller integration, and completion remain open; not a completion claim | `DEVICE_TRUST_MODEL.md`, `RESEARCH_AND_UI_GUIDANCE.md` | `output/device-trust-bootstrap-plan-proof/01-device-trust-source-of-truth/` |
| partial / Windows-only merged custody slice | [WP02 Local Key Sealing](workpacks/02-local-key-sealing.md) | custody and authority-boundary code present; no desktop command-path or end-to-end sealing proof; workpack remains open | `LOCAL_KEY_SEALING_MODEL.md`, `PLATFORM_KEY_CUSTODY_MATRIX.md` | `output/device-trust-bootstrap-plan-proof/02-local-key-sealing/` |
| blocked / implementation-only source authorized | [WP03 Parent Step-Up Auth](workpacks/03-parent-step-up-auth.md) | depends on Device Trust WP01, Account Identity WP08, and Cloudflare WP06; reviewed-implementation gates authorize only the bounded source packet; atomic ceremony custody/recovery and linked-challenge lifecycle validation are independently static-reviewed with no remaining internal P0/P1; authoritative target resolution, platform/passkey provider, durable sign counter, tests, proof, runtime, LAN handoff, and completion remain open | `PARENT_STEP_UP_AUTH_MODEL.md`, `RESEARCH_AND_UI_GUIDANCE.md` | `output/device-trust-bootstrap-plan-proof/03-parent-step-up-auth/` |
| blocked | [WP04 Phone QR Approval Bridge](workpacks/04-phone-qr-approval-bridge.md) | typed challenge/response boundary drafted; issuer, ceremony, transport, and proof remain open | `PHONE_QR_APPROVAL_MODEL.md` | `output/device-trust-bootstrap-plan-proof/04-phone-qr-approval-bridge/` |
| partial | [WP05 Entitlement Device License](workpacks/05-entitlement-device-license.md) | device-bound verifier boundary drafted; signature, revocation, and proof remain open | `ENTITLEMENT_DEVICE_LICENSE_MODEL.md` | `output/device-trust-bootstrap-plan-proof/05-entitlement-device-license/` |
| partial | [WP06 Recovery Reset Re-Pair](workpacks/06-recovery-reset-re-pair.md) | confirmation-only restore blocked; verified parent and execution-receipt gates drafted; encryption, revocation, and proof remain open | `RECOVERY_RESET_MODEL.md`, `LOCAL_KEY_SEALING_MODEL.md` | `output/device-trust-bootstrap-plan-proof/06-recovery-reset-re-pair/` |
| partial / code-drafted | [WP07 Child Tamper Uninstall](workpacks/07-child-tamper-uninstall.md) | durable child tamper evidence and parent-authorized revocation boundary drafted; platform removal and validation deferred | `CHILD_TAMPER_UNINSTALL_MODEL.md` | `output/device-trust-bootstrap-plan-proof/07-child-tamper-uninstall/` |
| docs-only | [WP08 Open Source Dependency Adoption](workpacks/08-open-source-dependency-adoption.md) | audit reset | `DEPENDENCY_RESEARCH_AND_ADOPTION.md`, `RESEARCH_AND_UI_GUIDANCE.md` | `output/device-trust-bootstrap-plan-proof/08-open-source-dependency-adoption/` |
| partial | [WP09 Cross Plan Route Gate](workpacks/09-cross-plan-route-gate.md) | audit reset | `ROUTE_INDEX.md`, adjacent plan/feature route indexes | `output/device-trust-bootstrap-plan-proof/09-cross-plan-route-gate/` |

The previous `12/12`, `10/10`, and `complete` labels were not backed by real proof roots or runtime validation and have been reset by audit.

## Production reachability audit (2026-08-16)

The current source map was checked against the dependency order above. WP01,
WP02, WP03, WP04, WP05, WP06, and WP07 have bounded production contracts or
fail-closed local state, but none has a complete shipped cryptographic/device
authority path for the missing behavior. WP08 and WP09 remain research/route
work only. In particular, `ParentDeviceTrustCommandFacade` and the Windows
custody implementation have no registered external production caller; the
entitlement verifier and restore executor are unavailable-by-default ports;
the QR and step-up paths have no ceremony issuer/nonce consumer; and child
removal still stops at durable evidence/manual-required platform cleanup.

This audit records source reachability only. It does not treat tests, proof,
static status, synthetic challenges/receipts, generic JSON, or public DTOs as
authority and does not change any workpack to complete. No target-authority
edit is legal without Account WP08's canonical binding and Cloudflare WP06's
durable repository/caller; the platform/passkey ceremony composition follows
those owners. The graph validator reported checked-in graph/source drift during
the prior audit; the 2026-08-17 coordinator updates record bounded reviewed
implementation and dependency evidence without changing any DONE state.

## Current implementation-phase disposition — 2026-08-17

The pushed WP01 integration source packet is drafted and independently
static-reviewed with no P0/P1 findings. The graph records only its four reviewed
`family-identity-core` source paths as implementation evidence; WP01 remains
READY, not DONE. Tests, focused validation, proof, production caller
integration, global Enforcer/architecture acceptance, platform custody, and
broader lifecycle composition remain open.

WP03 remains BLOCKED in the default graph on WP01, Account WP08, and Cloudflare
WP06. Reviewed-implementation gates now authorize only the bounded WP03 source
packet against all three reviewed source owners; the default dependency state
does not change and the route does not provide ceremony authority, provider
authority, tests, proof, runtime reachability, or completion. No WP26 edge is
opted into it.

## Default execution order

```text
WP01 ---------------------------+
Account WP08 -> Cloudflare WP06 +-> WP03 -> WP04 -> WP05 -> WP06 -> WP07 -> WP08 -> WP09
WP02 remains a conditional custody dependency where the selected ceremony needs it.
```

## Dependency rules

```text
WP01 establishes trust state/source of truth.
WP02 depends on WP01 and blocks key/trust persistence claims.
WP03 depends on WP01, Account Identity WP08, and Cloudflare WP06 and blocks
high-risk action approval claims. WP02 is conditional only for a demonstrated
private-key/install custody requirement.
WP04 depends on WP03 and blocks phone/QR approval claims.
WP05 depends on WP01/WP02 and payment handoff; license never unlocks behavior alone.
WP06 depends on WP02/WP03/WP04 and blocks recovery/reset claims.
WP07 depends on WP01/WP02/WP06 and blocks child uninstall/tamper claims.
WP08 can run in parallel as research but cannot approve adoption without proof.
WP09 is last and consumes all previous proof roots.
```

## Do not select

Do not move account identity, package distribution, LAN transport, remote access, payment entitlement, data custody implementation, portal UX, setup journey, or policy authoring into this plan.

Do not raise status from document tests, route tests, copied binaries, copied config, package install, login/session proof, LAN pairing, license state, entitlement snapshot presence, mock proof, or a proof root for another workpack.
