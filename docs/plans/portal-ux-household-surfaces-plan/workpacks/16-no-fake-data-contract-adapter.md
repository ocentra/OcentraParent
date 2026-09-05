# 16 No-Fake-Data Contract Adapter

> **Current status (2026-09-05): DONE FOR THE BOUNDED NO-FAKE-DATA
> PROJECTION / EXECUTION OWNERS REMAIN OPEN.** Typed Rust parent-host distribution
> state, fixture separation, invalid and missing payload handling, read-only
> Platforms and Install Updates routes, Remote Access honesty, focused desktop
> and compact E2E, and retained proof are complete. Installer, updater, rollback,
> signing, store, authenticated remote-session, transport, and sibling runtime
> authority remain explicit non-claims.

<!-- agent-capsule -->

> Agent Capsule
> Plan: `portal-ux-household-surfaces-plan`
> Doc: `16 No-Fake-Data Contract Adapter`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

Sources: [20-step plan](../portal-ux-household-surfaces-20-step-plan.md), [test blueprint](../portal-ux-household-surfaces-test-blueprint.md), [UI/UX guide](../ui-ux-requirements-guide.md), and [folder README](../README.md).

## Ownership boundary

```text
portal UX owns visible fixture/runtime labels, invalid payload display, schema-decode display, and no-product-claim boundary.
domain/schema/protocol owners own the actual service read-model contracts.
data-custody owns custody/retention/export/delete semantics.
logging/evidence owners own proof artifact and diagnostic storage.
```

## Where We Are

Design and test states sometimes need fixtures. Product routes must not mistake fixtures for runtime state.

The rendered-route audit found bounded product mismatches:

- Remote Access already fails closed without an authenticated session, but the shell labels that route as Data instead of its dedicated Remote destination. The nested Remote Screen policy has no typed service payload and therefore must remain an explicit unavailable surface rather than presenting fixture-shaped modes or actions as current state.
- Platforms and Install Updates render a LAN device command preview with checked controls even though those routes do not own LAN commands. The desktop host already exposes typed package, update, rollback, signing, store-distribution, and artifact-proof status, but the parent route snapshot does not carry that state to Portal.

## Where We Want To Be

Fixtures, demos, and runtime service payloads are separated and labeled; runtime payloads are decoded before rendering. Platforms and Install Updates consume a route-specific, read-only desktop distribution snapshot through the existing parent route load/subscription boundary, including when the local agent is unavailable. They do not create a second Portal invoke path or imply that an installer, updater, rollback executor, signing owner, or store publisher exists.

## Required proof fields

The selected proof must name, at minimum:

```text
route
payload_source
fixture_state
runtime_state
schema_decode_state
invalid_payload_state
missing_payload_state
portal_local_replacement_state
source_label_state
custody_label_state
product_claim_state
no_claim
```

These are proof-routing fields, not implementation code prescriptions.

## Requirement Checklist

- [x] Decode service payloads with domain/protocol schemas.
- [x] Label demo/fixture states explicitly.
- [x] Avoid portal-local replacements for service read models.
- [x] Add tests for invalid/missing payload states.
- [x] Keep product claims tied to real-service proof.
- [x] Carry desktop package/update status through the existing typed parent route snapshot.
- [x] Keep Platforms and Install Updates read-only when execution owners are unavailable.

## Acceptance And Proof

UI tests can tell whether they are exercising service state or explicit fixtures. Platforms and Install Updates render typed host-owned status, do not render the LAN command preview, and expose no install/update/rollback action without a real owner.

Proof must include fixture-mode, service-mode, invalid-payload, missing-payload, and no-product-claim cases for the selected route.

The retained proof is [WP16 No-Fake-Data Contract Adapter Proof](../../../proof/portal-ux-household-surfaces-plan/WP16_NO_FAKE_DATA_CONTRACT_ADAPTER_PROOF.md).
Its generated command logs and detailed negative-state records are under
`output/portal-ux-household-surfaces-plan-proof/16-no-fake-data-contract-adapter/`.
The verified packet passed Portal and Portal-domain builds and complete unit
suites, Rust bridge contract and parent-route projection tests, the focused real
Platforms/Install Updates and Remote Access E2E scenarios, and the scoped
architecture/generated-artifact gate. The broader Portal route E2E still has
open sibling-route failures and is not used as a plan-wide or PR-ready claim.

## Failure conditions

- Do not render fixtures as runtime state.
- Do not replace service read models with portal-local truth.
- Do not claim product readiness from demo/fixture state.
- Do not hide source/custody labels when they are required for the selected view.

This source slice does not authorize a remote-session runtime, child capability, transport route, custody claim, policy mutation, command draft, LAN command, package installer, updater, rollback executor, signing/notarization owner, store publisher, or owner authority. Those remain with the Remote Access, Parent Desktop Runtime Package, and owning runtime plans.

## Parallel Ownership Notes

This protects C from accidentally making visual-only work look product-complete.
