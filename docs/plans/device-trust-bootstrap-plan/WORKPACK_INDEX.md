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
| partial / runtime-proof-present | [WP01 Device Trust Source Of Truth](workpacks/01-device-trust-source-of-truth.md) | parent-presence slice proved; broader lifecycle open | `DEVICE_TRUST_MODEL.md`, `RESEARCH_AND_UI_GUIDANCE.md` | `output/device-trust-bootstrap-plan-proof/01-device-trust-source-of-truth/` |
| partial / Windows-only merged custody slice | [WP02 Local Key Sealing](workpacks/02-local-key-sealing.md) | custody and authority-boundary code present; no desktop command-path or end-to-end sealing proof; workpack remains open | `LOCAL_KEY_SEALING_MODEL.md`, `PLATFORM_KEY_CUSTODY_MATRIX.md` | `output/device-trust-bootstrap-plan-proof/02-local-key-sealing/` |
| blocked | [WP03 Parent Step-Up Auth](workpacks/03-parent-step-up-auth.md) | five-minute receipt lifetime gate drafted; external ceremony verifier and proof remain open | `PARENT_STEP_UP_AUTH_MODEL.md`, `RESEARCH_AND_UI_GUIDANCE.md` | `output/device-trust-bootstrap-plan-proof/03-parent-step-up-auth/` |
| blocked | [WP04 Phone QR Approval Bridge](workpacks/04-phone-qr-approval-bridge.md) | audit reset | `PHONE_QR_APPROVAL_MODEL.md` | `output/device-trust-bootstrap-plan-proof/04-phone-qr-approval-bridge/` |
| partial | [WP05 Entitlement Device License](workpacks/05-entitlement-device-license.md) | audit reset | `ENTITLEMENT_DEVICE_LICENSE_MODEL.md` | `output/device-trust-bootstrap-plan-proof/05-entitlement-device-license/` |
| partial | [WP06 Recovery Reset Re-Pair](workpacks/06-recovery-reset-re-pair.md) | audit reset | `RECOVERY_RESET_MODEL.md`, `LOCAL_KEY_SEALING_MODEL.md` | `output/device-trust-bootstrap-plan-proof/06-recovery-reset-re-pair/` |
| partial | [WP07 Child Tamper Uninstall](workpacks/07-child-tamper-uninstall.md) | audit reset | `CHILD_TAMPER_UNINSTALL_MODEL.md` | `output/device-trust-bootstrap-plan-proof/07-child-tamper-uninstall/` |
| docs-only | [WP08 Open Source Dependency Adoption](workpacks/08-open-source-dependency-adoption.md) | audit reset | `DEPENDENCY_RESEARCH_AND_ADOPTION.md`, `RESEARCH_AND_UI_GUIDANCE.md` | `output/device-trust-bootstrap-plan-proof/08-open-source-dependency-adoption/` |
| partial | [WP09 Cross Plan Route Gate](workpacks/09-cross-plan-route-gate.md) | audit reset | `ROUTE_INDEX.md`, adjacent plan/feature route indexes | `output/device-trust-bootstrap-plan-proof/09-cross-plan-route-gate/` |

The previous `12/12`, `10/10`, and `complete` labels were not backed by real proof roots or runtime validation and have been reset by audit.

## Default execution order

```text
WP01 -> WP02 -> WP03 -> WP04 -> WP05 -> WP06 -> WP07 -> WP08 -> WP09
```

## Dependency rules

```text
WP01 establishes trust state/source of truth.
WP02 depends on WP01 and blocks key/trust persistence claims.
WP03 depends on WP01/WP02 and blocks high-risk action approval claims.
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
