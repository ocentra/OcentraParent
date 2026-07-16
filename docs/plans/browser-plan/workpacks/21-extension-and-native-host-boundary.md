# 21 Extension And Native Host Boundary

<!-- agent-capsule -->

> Agent Capsule
> Plan: `browser-plan`
> Doc: `21 Extension And Native Host Boundary`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Where We Are

Current docs keep extension/native-host support optional. The default product
path is managed browser launch/profile plus browser-supported local bridge.

## Where We Want To Be

If added, extension/native-host support is managed-profile-only, permissioned,
origin-validated, schema-validated, heartbeat-monitored, and separately proved.

## Scope

- Extension install/enabled/disabled/permission-required/native-host-missing.
- Minimum permissions for URL/title/tab state.
- Optional managed-profile-only runtime signal observation such as active tab,
  canvas/WebGL/fullscreen/pointer-lock/gamepad events only after separate
  permission and proof.
- Native messaging host origin validation.
- Length-prefixed JSON schema validation.
- Managed session/profile binding.
- Service worker sleep/heartbeat stale state.
- No unmanaged personal profile capture.

## Touched Paths

- `packages/activity-domain/src/browser-extension-native-host-schemas.ts`
- `packages/activity-domain/src/browser.ts`
- `packages/activity-domain/tests/browser-extension-native-host.test.ts`
- `crates/agent-protocol/src/constants/browser.rs`
- `crates/agent-protocol/src/constants/field.rs`
- `crates/agent-core/src/browser_bridge_native_host.rs`
- `crates/agent-core/src/browser_bridge_native_host_tests.rs`
- `crates/agent-core/src/lib.rs`
- No browser extension/native host package path was created in this workpack.

## Tests And Proof

- Contract tests for extension and native-host states:
  `cmd /c npm run test --workspace @ocentra-parent/activity-domain -- browser-extension-native-host.test.ts`.
- Rust native-host frame validation tests:
  `cmd /c cargo test -p ocentra-parent-agent-core browser_bridge_native_host`.
- Security tests cover managed-profile binding, origin mismatch, schema drift,
  message length drift, stale heartbeat, native-host-missing state, and no
  personal/unmanaged profile capture claims.
- Manual browser extension install proof remains manual-required because no
  extension package or native-host registration was added.
- Runtime signal proof remains manual-required; runtime signals cannot be used
  as active-tab/browser-game evidence until the extension/native-host route is
  separately packaged, permissioned, and proved.

## AI Worker Checklist

Fill this before reporting `DONE` or PR-ready:

- [ ] Confirm source docs read: [folder README](../README.md), [source index](../source-index.md), [current snapshot](../current-browser-snapshot.md), [full scope plan](../v0-5-managed-browser-full-scope-plan.md), [test blueprint](../v0-5-managed-browser-test-blueprint.md), [UI/UX guide](../ui-ux-requirements-guide.md), [main checklist](../implementation-checklist.md), and this workpack.
- [ ] Check enhancement overlap: URL/video AI intelligence, social platform/account/feed gating, and browser games/cloud gaming.
- [ ] Hub lock covers this workpack and exact implementation/docs paths.
- [ ] Existing source layout inspected; no parallel browser truth created.
- [ ] Before-state source snapshot recorded in `output/browser-plan-proof/<workpack-id>/00-source-snapshot.md` or explicit docs-only N/A reason.
- [ ] Contracts updated first where this workpack changes behavior.
- [ ] Rust/service/portal parity updated only after contracts exist.
- [ ] Raw evidence artifacts captured where applicable: bridge/CDP payloads, managed session state, unmanaged process rows, journal entries, SQLite/read-model rows, policy decisions, and action results.
- [ ] Tests/proof listed in this workpack are implemented or explicitly marked manual-required with reason.
- [ ] Validation command outputs saved in the proof pack and summarized in [main checklist](../implementation-checklist.md).
- [ ] UI snapshots captured for every touched parent portal, child UX, block/warn, policy authoring, or dashboard state; if no UI changed, `ui-not-applicable.md` records why.
- [ ] Security/no-claim negative proof captured where applicable: no default profile attach, no unowned bridge, no unmanaged exact URL claim, no raw debugger URL exposure, and no AI direct enforcement.
- [ ] Manual platform proof captured for real browser/OS claims, including OS/browser version, command steps, screenshots/logs, and manual-required labels.
- [ ] Evidence/proof artifact paths recorded in [main checklist](../implementation-checklist.md).
- [ ] Feature/expectation/product-checklist/README update decision recorded in [main checklist](../implementation-checklist.md).
- [ ] Known gaps, deferred items, and no-claim boundaries recorded before `DONE`.

## Manual-Required Gaps

No extension security or active-tab claim until extension packaging/distribution
and native-host proof exist.
No browser-game runtime signal claim should depend on extension/native-host
events until that path is separately proved.
No product checklist update was made for WP21 because this workpack adds an
optional boundary contract and private validator only; it does not upgrade
browser capture, enforcement, OS policy, social/video/game, child UX, or mobile
capability status.
