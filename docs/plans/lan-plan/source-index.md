# LAN Source Index

<!-- agent-capsule -->

> Agent Capsule
> Plan: `lan-plan`
> Doc: `LAN Source Index`
> Kind: source ownership index; read only when source ownership is unclear.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not inspect broad source from here; use only the named package/crate path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

This index keeps the LAN plan tied to existing source documents and code. It is
not a replacement for feature, expectation, roadmap, checklist, package, or
crate ownership docs.

## Product Source Docs

- Owning feature:
  [Remote, LAN, and mobile platforms](../../features/remote-lan-mobile-platforms.md)
- Owning adjacent feature:
  [Family setup and device roles](../../features/family-setup-device-roles.md)
- Main expectation: [LAN pairing](../../expectations/lan-pairing.md)
- Platform expectation: [Platforms](../../expectations/platforms.md)
- Family setup expectation: [Family setup](../../expectations/family-setup.md)
- Network/domain adjacent expectation:
  [Network flow evidence](../../expectations/network-flow-evidence.md)
- Real evidence adjacent expectation:
  [Real evidence proof](../../expectations/real-evidence-proof.md)
- Product status table:
  [Product capability checklist](../../product-capability-checklist.md)
- Roadmap source: [Product roadmap](../../product-roadmap.md)
- Implementation tracking:
  [LAN plan implementation checklist](implementation-checklist.md)
- Current state snapshot: [Current LAN snapshot](current-lan-snapshot.md)
- Pasted-content reconciliation:
  [Pasted content coverage audit](pasted-content-coverage-audit.md)
- Source plan:
  [V0.9 LAN discovery 20-step plan](v0-9-lan-discovery-20-step-plan.md)
- Test blueprint:
  [V0.9 LAN discovery test blueprint](v0-9-lan-discovery-test-blueprint.md)
- UI/UX guide: [LAN discovery UI/UX requirements guide](ui-ux-requirements-guide.md)

## Routing: Move Here Or Point Here

LAN implementation planning belongs in this folder when it is about:

- household LAN discovery;
- local child-agent identity and hardware inventory;
- passive LAN, router, infrastructure, and unsupported-device visibility;
- child-agent signed hello and heartbeat proof;
- LAN pairing, route custody, trusted registry, revocation, and recovery;
- canonical household device records and source-matrix proof;
- Devices/LAN, Activity/Network, and Policy Network target UI proof;
- direct-address/manual proof flows;
- optional relay/cache custody labels when they affect LAN route truth;
- physical two-device proof and no-claim boundaries.

Shared source docs stay where they are and are pointed to from this folder:

- feature docs stay under `docs/features`;
- expectation docs stay under `docs/expectations`;
- architecture docs stay under `docs/architecture`;
- product checklist and roadmap stay at the docs root;
- package/crate/app ownership docs stay next to their source.

Do not move those shared docs into this plan folder. Link them here and update
them only when implementation status, acceptance contract, or proof changes.

## Source Truth Rule

```text
LAN scan discovers.
Child agent confirms.
Parent assigns.
```

Do not create a second truth for household devices. Portal rows, policy targets,
activity diagnostics, and proof JSON must derive from the typed read model and
household device registry, not from hand-built portal fixtures that disagree
with the service.

## Feature Routing

| Feature doc                          | LAN-plan relationship                                                                                                                                      |
| ------------------------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `remote-lan-mobile-platforms.md`     | Owning feature. LAN route, custody, remote/relay, Android/iOS child-agent parity, and platform non-claims feed status/proof back here.                     |
| `family-setup-device-roles.md`       | Owning adjacent feature. Household, child profile, parent assignment, trust, ignore, revoke, selected device, and setup UX feed status/proof back here.    |
| `network-domain-control.md`          | Adjacent weaker evidence source. Network/domain activity can support diagnostics but cannot prove child identity or parent assignment.                     |
| `policy-schedules-approvals.md`      | Shared policy boundary. LAN device target binding must use typed target/read-model state, not portal-only shortcuts.                                       |
| `reports-notifications-sync.md`      | Adjacent reporting boundary. Activity/Network diagnostics and reports must show custody/source labels without implying finished cloud sync.                |
| `enforcement-integrity-tamper.md`    | Shared control boundary. LAN route trust must not bypass enforcement, integrity, or stale/revoked route checks.                                            |
| `evidence-store-query.md`            | Shared evidence boundary. LAN scan/source evidence should journal/query through shared evidence/read-model paths where product evidence changes.           |
| `production-distribution-support.md` | Release/support boundary. LAN support bundles must redact raw local paths, private network details, secrets, and child evidence unless explicitly allowed. |

## Adjacent Plan Docs

- Browser plan: [Browser plan README](../browser-plan/README.md)
- Portal UX household surfaces plan:
  [Portal UX household surfaces plan](../portal-ux-household-surfaces-plan/README.md)
- V0.8 enforcement control plan:
  [V0.8 enforcement control plan](../v0-8-enforcement-control-plan/README.md)

## TypeScript Ownership

- `packages/parent-domain/src/lan-discovery-source-matrix.ts`
- `packages/parent-domain/src/lan-discovery-evidence.ts`
- `packages/parent-domain/src/lan-device-parent-actions.ts`
- `packages/parent-domain/src/lan-pairing.ts`
- `packages/parent-domain/src/lan-pairing-device.ts`
- `packages/parent-domain/src/lan-pairing-control.ts`
- `packages/parent-domain/src/lan-pairing-browser-runtime.ts`
- `packages/parent-domain/src/lan-pairing-product-proof.ts`
- `packages/parent-domain/src/lan-production-household-proof.ts`
- `packages/parent-domain/src/lan-signed-discovery-relay-spine.ts`
- `packages/parent-domain/src/v0-9-household-lan-pairing-proof.ts`
- `packages/agent-protocol-domain/src/lan-discovery-source-matrix.ts`
- `packages/agent-protocol-domain/src/lan-pairing-browser-add-device-state.ts`
- `packages/agent-protocol-domain/src/lan-pairing-browser-runtime.ts`
- `packages/agent-protocol-domain/src/lan-pairing-challenge.ts`
- `packages/agent-protocol-domain/src/lan-signed-discovery-relay-spine.ts`

TypeScript rule: enhance these existing LAN paths. Do not create a parallel LAN
domain package unless an ownership boundary genuinely changes. Weak network/name
sources must stay typed as weak/manual-required until child-agent proof and
parent decisions make them controllable.

## Rust Ownership

- `crates/agent-protocol/src/lan_pairing.rs`
- `crates/agent-protocol/src/lan_pairing_browser_add_device_state.rs`
- `crates/agent-protocol/src/lan_pairing_browser_add_device_state/source_matrix.rs`
- `crates/agent-protocol/src/lan_pairing_browser_add_device_state/signed_discovery_relay_spine.rs`
- `crates/agent-protocol/src/lan_pairing_browser_add_device_state/production_household_proof.rs`
- `crates/agent-protocol/src/lan_pairing/discovery_states.rs`
- `crates/agent-protocol/src/lan_pairing/device_roles.rs`
- `crates/agent-protocol/src/lan_pairing/household_proof.rs`
- `crates/agent-protocol/src/lan_pairing_authority.rs`
- `crates/agent-protocol/src/constants/lan_pairing.rs`
- `crates/agent-service/src/lan_network_inventory.rs`
- `crates/agent-service/src/lan_network_inventory_command.rs`
- `crates/agent-service/src/lan_network_inventory_hardware.rs`
- `crates/agent-service/src/lan_pairing.rs`
- `crates/agent-service/src/lan_pairing_browser_add_device_state.rs`
- `crates/agent-service/src/lan_pairing_browser_add_device_state/source_matrix.rs`
- `crates/agent-service/src/lan_pairing_browser_add_device_state/source_matrix/source_rows.rs`
- `crates/agent-service/src/lan_pairing_browser_add_device_state/signed_discovery_relay_spine.rs`
- `crates/agent-service/src/lan_pairing_browser_add_device_state/production_household_proof.rs`
- `crates/agent-service/src/lan_pairing_household_device_spine.rs`
- `crates/agent-service/src/lan_pairing_household_device_spine/merge.rs`
- `crates/agent-service/src/lan_pairing_runtime_state.rs`
- `crates/agent-service/src/lan_pairing_status.rs`

Rust rule: TypeScript contracts come first, Rust protocol parity second, Rust
service state/adapters third, and portal/read-model consumption fourth.

## Portal Ownership

- `vendor/ocentra-parent-core-ui/AppPages/ParentPortal/ParentPortalSvgSurface.tsx`
- `vendor/ocentra-parent-core-ui/AppPages/ParentPortal/activity-ui-intent.ts`
- `vendor/ocentra-parent-core-ui/AppPages/ParentPortal/DeviceChoiceGrid/`
- `apps/portal/src/live-activity-state.ts`
- `apps/portal/src/live-activity-panel.ts`
- `apps/portal/tests/activity-ui-intent.test.ts`
- `apps/portal/tests/live-activity-state.test.ts`
- `apps/portal/tests/live-activity-network-flow.test.ts`
- `apps/portal/tests/transport-lan-target.test.ts`
- `apps/portal/e2e/portal-ui.spec.ts`

Portal rule: render service-backed LAN state and typed source-matrix rows.
Portal must not infer a child profile from hostnames, vendor strings, IP
addresses, router labels, or network activity. Unsupported/router/passive rows
stay visible-only until a child-agent route exists.

## Proof Scripts

- `node scripts/test/v0-9-lan-source-matrix-plan-completion.mjs`
- `node scripts/test/v0-9-lan-signed-discovery-relay-spine.mjs`
- `node scripts/test/v0-9-production-lan-household-proof.mjs`
- `node scripts/test/v0-9-production-lan-multidevice-hardening.mjs`
- `node scripts/test/v0-9-household-lan-production-discovery-proof.mjs`
- `node scripts/test/v0-9-household-lan-proof-readiness.mjs`
- `node scripts/test/v0-9-household-lan-product-proof.mjs`
- `node scripts/test/v0-9-household-lan-pairing-proof.mjs`
- `node scripts/test/browser-first-lan-discovery-add-device-state.mjs`
- `node scripts/test/lan-browser-discovery-pairing-runtime.mjs`
- `node scripts/test/websocket-lan-smoke.mjs`
- `npm run test:e2e`
- `npm run validate`

## Current Test Files

- `packages/parent-domain/tests/lan-discovery-source-matrix.test.ts`
- `packages/parent-domain/tests/lan-signed-discovery-relay-spine.test.ts`
- `packages/parent-domain/tests/lan-production-household-proof.test.ts`
- `packages/agent-protocol-domain/tests/lan-discovery-source-matrix.test.ts`
- `packages/agent-protocol-domain/tests/lan-pairing-browser-add-device-state.test.ts`
- `crates/agent-protocol/src/lan_pairing_browser_add_device_state_tests.rs`
- `crates/agent-protocol/src/lan_pairing_tests.rs`
- `crates/agent-service/src/lan_pairing_browser_add_device_state_tests.rs`
- `crates/agent-service/src/lan_pairing_tests.rs`
- `crates/agent-service/src/lan_pairing_household_device_spine_tests.rs`
- `apps/portal/tests/transport-lan-target.test.ts`
- `apps/portal/tests/live-activity-network-flow.test.ts`
- `apps/portal/e2e/portal-ui.spec.ts`

## Source Truth Rule For Status Updates

When LAN work changes product state, update the owning feature docs, matching
expectation docs, product capability checklist row, and touched module README.
If the work only adds planning detail inside this folder, no product status
update is required.
