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
| partial / foundation-source only; expected tests, authority bridge, and caller open | [WP01 Device Trust Source Of Truth](workpacks/01-device-trust-source-of-truth.md) | accepted continuation is integrated through `68717b5b7`: durable lifecycle/current-binding source is present, but no shipped authority issuer, platform ceremony, production caller, expected-test wave, proof, or completion exists | `DEVICE_TRUST_MODEL.md`, `RESEARCH_AND_UI_GUIDANCE.md` | `output/device-trust-bootstrap-plan-proof/01-device-trust-source-of-truth/` |
| partial / Windows custody source only; downstream of WP01 | [WP02 Local Key Sealing](workpacks/02-local-key-sealing.md) | DPAPI/registry and opaque runtime seams exist, but `require_authenticated_parent_authority()` is permanently unavailable; no ceremony issuer, desktop mount, custody-to-lifecycle startup composition, current tests, or end-to-end sealing proof is available | `LOCAL_KEY_SEALING_MODEL.md`, `PLATFORM_KEY_CUSTODY_MATRIX.md` | `output/device-trust-bootstrap-plan-proof/02-local-key-sealing/` |
| blocked / bounded custody retained / target authority and runtime missing | [WP03 Parent Step-Up Auth](workpacks/03-parent-step-up-auth.md) | depends on Device Trust WP01, Account Identity WP08, and Cloudflare WP06; target-aware Account WP02 is transitive through WP06. Atomic ceremony custody/recovery and linked-challenge lifecycle source are retained, but planned target-authority and parent-runtime owners, platform/passkey provider, durable sign counter, expected tests, proof, LAN handoff, and completion remain open | `PARENT_STEP_UP_AUTH_MODEL.md`, `RESEARCH_AND_UI_GUIDANCE.md` | `output/device-trust-bootstrap-plan-proof/03-parent-step-up-auth/` |
| blocked | [WP04 Phone QR Approval Bridge](workpacks/04-phone-qr-approval-bridge.md) | typed challenge/response boundary drafted; issuer, ceremony, transport, and proof remain open | `PHONE_QR_APPROVAL_MODEL.md` | `output/device-trust-bootstrap-plan-proof/04-phone-qr-approval-bridge/` |
| partial / source accepted, expected tests open | [WP05 Entitlement Device License](workpacks/05-entitlement-device-license.md) | unsigned projection and fail-closed wire/context boundary are integrated; real issuer/signature/revocation authority, callers, expected tests, and proof remain open | `ENTITLEMENT_DEVICE_LICENSE_MODEL.md` | `output/device-trust-bootstrap-plan-proof/05-entitlement-device-license/` |
| partial / source accepted, expected tests open | [WP06 Recovery Reset Re-Pair](workpacks/06-recovery-reset-re-pair.md) | caller-minted restore authority is removed and verified-parent/executor boundaries fail closed; encryption, real executor/custody, expected tests, revocation proof, and callers remain open | `RECOVERY_RESET_MODEL.md`, `LOCAL_KEY_SEALING_MODEL.md` | `output/device-trust-bootstrap-plan-proof/06-recovery-reset-re-pair/` |
| partial / source accepted, expected tests open | [WP07 Child Tamper Uninstall](workpacks/07-child-tamper-uninstall.md) | durable removal/tamper custody and trust-bound readiness are integrated; parent transport, platform removal/attestation, expected tests, and proof remain open | `CHILD_TAMPER_UNINSTALL_MODEL.md` | `output/device-trust-bootstrap-plan-proof/07-child-tamper-uninstall/` |
| docs-only | [WP08 Open Source Dependency Adoption](workpacks/08-open-source-dependency-adoption.md) | audit reset | `DEPENDENCY_RESEARCH_AND_ADOPTION.md`, `RESEARCH_AND_UI_GUIDANCE.md` | `output/device-trust-bootstrap-plan-proof/08-open-source-dependency-adoption/` |
| partial | [WP09 Cross Plan Route Gate](workpacks/09-cross-plan-route-gate.md) | audit reset | `ROUTE_INDEX.md`, adjacent plan/feature route indexes | `output/device-trust-bootstrap-plan-proof/09-cross-plan-route-gate/` |

The previous `12/12`, `10/10`, and `complete` labels were not backed by real proof roots or runtime validation and have been reset by audit.

## Production reachability audit (2026-08-16)

The current source map was checked against the dependency order above. WP01,
WP02, WP03, WP04, WP05, WP06, and WP07 have bounded production contracts or
fail-closed local state, but none has a complete shipped cryptographic/device
authority path for the missing behavior. WP08 and WP09 remain research/route
work only. In particular, `ParentDeviceTrustCommandFacade` and the Windows
custody implementation have no registered external production caller, and
`require_authenticated_parent_authority()` is permanently unavailable before
custody mutation; the entitlement verifier and restore executor are
unavailable-by-default ports;
the QR and step-up paths have no ceremony issuer/nonce consumer; and child
removal still stops at durable evidence/manual-required platform cleanup.

This audit records source reachability only. It does not treat stale lifecycle
tests, synthetic probes, proof, static status, synthetic challenges/receipts,
generic JSON, or public DTOs as authority and does not change any workpack to
complete. No target-authority
edit is legal without Account WP08's canonical binding and Cloudflare WP06's
durable repository/caller; the platform/passkey ceremony composition follows
those owners. The graph validator reported checked-in graph/source drift during
the prior audit; the 2026-08-17 coordinator updates record bounded reviewed
implementation and dependency evidence without changing any DONE state.

## Current implementation-phase disposition — 2026-08-17

The accepted Device Trust continuation is integrated through `68717b5b7` and independently reviewed
with no P0/P1 findings. The graph records its five reviewed
`family-identity-core` source paths as implementation evidence; WP01 remains a
foundation/source-only validation route, not a shipped authority or production
caller. Focused source-format, architecture, Enforcer, diff, and guard checks
passed. Expected-test migration, functional validation, proof, production caller
integration, repo-wide Enforcer/architecture acceptance, platform custody, and
broader lifecycle composition remain open.

WP05's unsigned entitlement projection, WP06's fail-closed restore boundary,
and WP07's durable removal/readiness boundary are also accepted source, not
test/proof/completion claims. WP03 remains BLOCKED in the default graph on
WP01, Account WP08, and Cloudflare WP06. The reviewed-implementation edges do
not authorize the bounded WP03 source packet yet: Cloudflare WP06 still lacks
its planned authoritative caller/writer source evidence. Account WP02 is the
only authority-chain workpack currently eligible for implementation-only work;
the default dependency state does not change, and the route does not provide
ceremony authority, provider authority, tests, proof, runtime reachability, or
completion. WP02 is not a default WP26 dependency. If the
platform sealing/lifecycle-revocation path is selected, the reviewed WP26 ->
WP02 gate must be added and completed before the LAN/child consumer route is
assigned; the non-sealing route remains free of that optional dependency.

## Default execution order

```text
WP01 foundation/source ------------------------------+
Account WP08 -> Account WP02 target authority --------+
Cloudflare WP06 authoritative writer/provider caller -+-> WP03 parent ceremony
                                                        +-> LAN WP26 / child current-binding consumer
WP03 -> WP04 -> WP05 -> WP06 -> WP07 -> WP08 -> WP09
WP02 parent-runtime/platform sealing + revocation is a conditional gate on the
LAN/child consumer route only when a private-key/install custody path is selected.
The default non-sealing route does not force WP02; a selected route carries the
reviewed WP26 -> WP02 edge and waits for WP02 completion. WP02 cannot create
ceremony authority.
```

### Conditional WP02 graph gate

The graph has one reviewed dependency shape: `depends_on` edges are hard by
default, and `implementationGate: "reviewed-implementation"` permits only a
separately reviewed source phase. It has no always-on optional edge toggle, so
the default graph intentionally keeps WP02 out of WP26's hard dependency list.

When a platform sealing/lifecycle-revocation path is selected, promote the
following reviewed edge in `docs/engineering-graph/overrides.json`, add WP02 to
the matching WP26 `hardDependencies`, and regenerate/validate the graph before
assigning the consumer:

```text
from = WP-lan-plan-26-signed-child-beacon-ingress-and-household-mesh-authority-handoff
to = WP-device-trust-bootstrap-plan-02-local-key-sealing
kind = depends_on; confidence = reviewed
implementationGate = reviewed-implementation (source phase only)
```

That selected route cannot proceed until WP02's sealing, lifecycle-generation,
and revocation handoff is complete. The edge points downstream from WP26 to
WP02 and does not point back to WP03, so it cannot create a cycle. If the
platform path is not selected, the edge remains absent and the Account WP08 ->
Account WP02 target authority -> Cloudflare WP06 -> WP03 -> LAN/child route
does not force Device Trust WP02.

## Dependency rules

```text
WP01 establishes trust state/source of truth.
WP02 owns only the downstream parent-runtime/platform sealing, lifecycle
composition, and revocation bridge; it cannot mint or substitute parent
ceremony authority. Its WP01 foundation edge is a reviewed conditional gate for
the selected platform-custody route, not a prerequisite for the default Account
-> Cloudflare -> WP03 -> LAN/child route.
WP03 depends on WP01, Account Identity WP08, and Cloudflare WP06 and blocks
high-risk action approval claims. Target-aware Account WP02 is consumed
transitively through WP06; it is not duplicated as a direct WP03 edge. Device
Trust WP02 is conditional only for a demonstrated private-key/install custody
requirement.
WP04 depends on WP03 and blocks phone/QR approval claims.
WP05 depends on WP01/WP02 and payment handoff; license never unlocks behavior alone.
WP06 depends on WP02/WP03/WP04 and blocks recovery/reset claims.
WP07 depends on WP01/WP02/WP06 and blocks child uninstall/tamper claims.
LAN WP26 and any child current-binding consumer are ordered after WP03's
one-time `RegisterLanSignerAnchor` ceremony; they consume the current binding
and revocation state and do not register signer authority locally. If the
platform sealing/lifecycle-revocation path is selected, the reviewed WP02 gate
must also be complete before this consumer route proceeds; the default route
does not force WP02.
WP08 can run in parallel as research but cannot approve adoption without proof.
WP09 is last and consumes all previous proof roots.
```

## Do not select

Do not move account identity, package distribution, LAN transport, remote access, payment entitlement, data custody implementation, portal UX, setup journey, or policy authoring into this plan.

Do not raise status from document tests, route tests, copied binaries, copied config, package install, login/session proof, LAN pairing, license state, entitlement snapshot presence, mock proof, or a proof root for another workpack.
