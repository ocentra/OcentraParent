<!-- agent-capsule -->

> Agent Capsule
> Doc: V6 Mobile Agents Expectations
> Kind: expectation/acceptance documentation; read only when selected by feature doc, plan route, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# V6 Mobile Agents Expectations

This is the milestone-specific expectation file for V6 in `docs/product-roadmap.md`.

Supporting expectation files: [platforms](../expectations/platforms.md),
[platform deliverables](../expectations/platform-deliverables.md), [capture](../expectations/capture.md),
[evidence storage](../expectations/evidence-storage.md),
[location and geofence](../expectations/location-geofence.md),
[tamper and uninstall protection](../expectations/tamper-uninstall-protection.md), and
[release installer](../expectations/release-installer.md).

## Outcome

- Android, iOS, macOS, and Linux claims match real OS permissions, APIs, packaging, and store constraints.
- Mobile agents reuse shared contracts and journal/query shapes where practical.
- Platform limits are visible instead of disguised as parity.
- Parent mobile app claims and child mobile agent claims are tracked separately.
- Location, notification, integrity, permission-loss, and uninstall/tamper
  claims are tracked per platform instead of bundled under "mobile support."

## Acceptance

- Android foreground/device-admin paths and iOS approved APIs are documented and tested before product claims.
- Platform-specific capture/enforcement adapters stay behind typed boundaries.
- Location/geofence, notification, app-activity, network-filtering, and
  integrity capabilities each have their own permission/capability status.
- Mobile packaging and install/update/store-readiness evidence is available for each claimed platform.
- CI/emulator/simulator evidence is paired with real-device, entitlement, or
  managed-device notes where OS policy requires it.
- `mobile-child-agent-capability-proof` keeps Android and iOS child-agent
  package/runtime/capability state visible before full mobile parity; its proof
  is scaffold/manual-required unless real device, entitlement, signing, store,
  or transport artifacts are attached.

## Validation

- Run `npm run validate`.
- Include platform-specific smoke tests, permission/capability evidence,
  location/integrity proof where claimed, contract parity, and release/package
  checks for each supported platform.
