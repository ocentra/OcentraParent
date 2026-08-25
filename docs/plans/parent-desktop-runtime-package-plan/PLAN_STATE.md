# Parent Client Runtime Distribution Plan State

Status: WP03 parent desktop shell and WP06 parent local-service route bridge have independently accepted refreshed production source but are active until their complete expected-test wave, focused execution, and proof revalidation finish. WP01, WP02, WP04, WP08, and WP09 retain their earlier bounded closure; remaining implementation and proof for later workpacks remain open. Protected WP01's CNG/TPM mechanics, private FFI/core boundary, and read-only BIN provisioner preflight are integrated at canonical `a6d7d9adf`; the preflight always fails closed with `ExternalProvisioningRequired`, and WP12 remains blocked as a package-only contract until its real installer/package evidence exists.

Research status: aligned against the current repo parent client surfaces, the existing desktop/mobile proof scripts, and the runtime-distribution guidance in the pasted apply set. The historical `parent-desktop-runtime-package-plan` path is retained for compatibility only.

## Current ownership interpretation

```text
apps/portal:
  Parent web portal source/projection surface and web distribution target when selected.

portal-domain:
  Public portal contracts/projections when selected.

parent-domain:
  Parent client package/handoff contracts only where public exports exist and the selected workpack names them.

scripts/dev:
  Parent desktop dev launch helpers and local launch proof anchors.

scripts/release:
  Build/package/proof helpers for selected parent desktop/mobile artifacts.

setup-install-provisioning-plan:
  Setup journey, install readiness, first-run state, and setup-side handoff owner.

child-agent-runtime-distribution-plan:
  Child package/runtime, child package lifecycle, tamper/uninstall, and child-specific artifact owner.

protected-capability-custody-foundation-plan:
  Protected WP01 owns private core/FFI enrollment acceptance, authority
  creation, TPM policy and non-exportable-handle validation, exact registry/
  SCM/peer authority, and opaque broker outcomes. Parent WP12 owns only the
  parent-side MSI/WiX, fixed BIN-only provisioner invocation, build wiring, and
  package lifecycle contract; it never accepts raw `authValue` or caller
  authority.

device-trust-bootstrap-plan:
  Trusted-device bootstrap, local sealed trust, and parent presence/approval owner.

account-identity-family-plan, payment-subscription-plan, policy-control-plane-plan, remote-access-plan, and data-custody-storage-plan:
  Sibling owners for account, payment, policy, remote access, and custody behavior.
```

## Current coupling risks

```text
- The folder path still says desktop, but the canonical scope is parent client distribution.
- Web build proof is not production account portal readiness.
- Desktop launch smoke is not desktop product readiness.
- Mobile scaffold or source artifact proof is not Android/iOS platform support.
- Package artifact proof is not setup completion.
- Route bridge proof is not child-agent runtime authority.
- Signing, notarization, store, update, rollback, SBOM, and launch claims must remain per artifact/platform.
```

## Current proof interpretation

```text
output/parent-client-runtime-distribution-plan-proof/<workpack>/ is the canonical proof root.
docs/proof/parent-desktop-runtime-package-plan/ is compatibility-only for old references and should not become the active proof root.
WP01 is now closed against `output/parent-client-runtime-distribution-plan-proof/01-parent-client-scope-and-route-boundary/`.
WP02 is now closed against `output/parent-client-runtime-distribution-plan-proof/02-parent-web-portal-distribution/`.
WP03 retains historical proof under `output/parent-client-runtime-distribution-plan-proof/03-parent-desktop-shell-package/`, but the 2026-08-17 source refresh requires new tests and proof revalidation.
WP04 is now closed against `output/parent-client-runtime-distribution-plan-proof/04-parent-android-package/`.
WP06 retains historical proof under `output/parent-client-runtime-distribution-plan-proof/06-parent-local-service-route-bridge/`, but the 2026-08-17 source refresh requires new tests and proof revalidation.
WP08 is now closed against `output/parent-client-runtime-distribution-plan-proof/08-parent-client-update-rollback/`.
WP09 is now closed against `output/parent-client-runtime-distribution-plan-proof/09-parent-client-launch-smoke-matrix/`.
Later workpacks remain open until their proof artifacts and checklist rows align.
Manual-required states are expected for unsupported or unavailable platform/store proof.
```

Evidence from the repo:

- `apps/portal` is the parent web portal surface.
- `dev:desktop` and `dev:desktop:lan` already exist for the parent desktop shell.
- `release:package:parent-android` and `release:package:parent-ios` already exist for parent mobile packaging.
- `test:parent-android-package-proof`, `test:parent-mobile-shell-runtime-proof`, `test:parent-mobile-package-source-artifact-proof`, `test:parent-desktop-release-support-proof`, `test:parent-mobile-service-bridge`, and `test:parent-mobile-controller-observer-handoff` already exist as proof anchors.
- The repo already differentiates parent client surfaces from child runtime work, but the plan naming still read as desktop-only before this correction.
- The plan-local route docs and the WP01 proof root now keep canonical parent-client scope, route bridge, setup handoff, portal UX handoff, and child-runtime exclusions separate under a Rust-first ownership rule.
- The hosted parent web portal route now has real build, route, auth, cache, and preview/staging/production separation proof under `output/parent-client-runtime-distribution-plan-proof/02-parent-web-portal-distribution/` without upgrading preview/build presence into production readiness.
- The parent local-service route bridge now has Rust-owned contract and runtime proof under `output/parent-client-runtime-distribution-plan-proof/06-parent-local-service-route-bridge/`, including canonical `ParentRouteSnapshot` shape checks plus explicit unavailable/timeout/local-target boundary coverage for Devices-route local-service state.
- The parent launch smoke matrix now has focused proof under `output/parent-client-runtime-distribution-plan-proof/09-parent-client-launch-smoke-matrix/`, with web blocked, desktop manual-required, and Android/iOS blocked rows recorded as smoke-only state instead of readiness.
- The desktop production health path now uses the configured local agent's typed `AgentHealthCheck` / `AgentHealthReported` WebSocket response. Readiness requires the command correlation, protocol schema, expected service/portal peers, `online=true`, and `transport=websocket`; the reported timeout is sourced from the same core health-command timeout. Raw TCP listener acceptance is no longer treated as runtime readiness. This phase does not rerun or promote validation/proof.

Current parent direction:

- Parent web portal is a distribution target with build, route, auth, and cache proof.
- Parent web portal now also has explicit hosted env-separation proof for preview, staging, and production labels, plus wrong-route, missing-auth, and stale-cache negative coverage.
- Parent local-service route bridge now has explicit Rust-owned route snapshot, local-service reachability, timeout degradation, and setup-separation proof without turning the bridge into setup or child-runtime ownership.
- Parent desktop shell/package now has a real Windows Tauri package proof with MSI and NSIS artifacts, explicit dry-run launch anchors, local artifact-hash evidence, and ready/degraded Rust service-bridge proof; signing, production update, production rollback, setup readiness, child runtime authority, and mobile parity remain explicit manual-required or no-claim boundaries.
- Parent client launch smoke now has an explicit cross-platform matrix: web blocked with a recorded local blocker, desktop manual-required after passing dry-run launch and Rust bridge proof, and Android/iOS blocked with current parent-mobile proof blockers recorded rather than hidden.
- WP04 parent Android package is complete with real build/artifact proof; install on a real Android target remains manual-required in this checkout because no attached device or local emulator command was available, and signing/store proof remains manual-required.
- Parent iOS package remains manual-required until simulator/device and store proof exists.
- Parent client route bridge contracts must be separate from setup journey and child runtime claims.
- Child agent runtime/package distribution belongs to `child-agent-runtime-distribution-plan`; this plan may only reference its handoff boundary.

## Production-code reachability audit (2026-08-16)

This table records source reachability and ownership only. It does not promote
graph, proof, test, CI, package, store, or release status.

| Workpack | Shipped production path | Code state and exact blocker | Slice decision |
| --- | --- | --- | --- |
| WP01 scope/route | Documentation route only; no runtime entrypoint | No production code is required. Scope separation remains the blocker for adjacent owners, not a missing desktop caller. | No code; audit recorded here. |
| WP02 web portal | `apps/portal/src/main.ts` -> hosted portal distribution surface | Web distribution/projection code is reachable in the web target. Publishing, account backend/runtime, setup, and child authority are outside this plan. | No code; external ownership. |
| WP03 desktop shell | `apps/parent-desktop/src-tauri/src/main.rs` -> `lib::run` -> Tauri commands | Shell is reachable. Service readiness now requires protocol-owned response kind plus nonce/request/correlation/source/target/event/digest/freshness binding and one hard transport deadline; the legacy raw TCP helper is compatibility/test support only. Signing, production update/rollback, setup, and child authority remain open. | Accepted source; expected tests and proof revalidation open. |
| WP04 Android parent | `platforms/android/parent/.../MainActivity.java` | Reachable APK scaffold only; no Rust/HostBridge/service caller or live parent read-model input. Device/install/store authority is external/manual-required. | No code; platform bridge blocked. |
| WP05 iOS parent | `platforms/ios/OcentraParentMobile/AppDelegate.swift` -> `ParentMobileStatusViewController` | Reachable iOS scaffold only; controller renders static status and has no Rust bridge/service caller. Provisioning/device/store authority is external/manual-required. | No code; platform bridge blocked. |
| WP06 local-service bridge | Tauri/dev-web route commands -> `parent-runtime-core::load_parent_route_snapshot` / action dispatch -> typed agent-service WebSocket loaders | Real service-owned read-model and command paths exist. Every response is identity/freshness bound, and required dependency/LAN failures fail closed into unavailable/degraded snapshots and rejected actions instead of stale connected state. Setup and child runtime remain separate owners. | Accepted source; expected tests and proof revalidation open. |
| WP07 signing/store matrix | `scripts/release/parent-desktop-release-support-proof.mjs` and status fields in `apps/parent-desktop/src-tauri/src/lib.rs` | These are release/status surfaces, not signer, notarizer, or store authority. Certificates, provisioning, notarization, and store submission remain external. | No code; external release ownership. |
| WP08 update/rollback | Desktop proof/status fields plus release-support scripts | No shipped parent updater/rollback executor is reachable from the Tauri runtime; current states remain scaffold/manual-required. SBOM and signed-channel authority remain open. | No code; external release/runtime owner. |
| WP09 launch smoke | `scripts/dev/dev-parent-desktop.mjs`, platform smoke scripts, and Tauri launch anchors | Launch scripts and package smoke are reachable validation paths only; they do not add product runtime behavior or readiness authority. | No code; validation-only. |
| WP10 setup handoff | `parent-runtime-core` Start-route panel projection and `apps/portal/src/SetupFirstRunRoutePanel.tsx` | The surface explicitly reports setup runtime unavailable; no setup producer or install-state handoff caller is present in this plan. Setup journey owner remains external. | No code; setup owner blocked. |
| WP11 proof/CI gate | CI workflows and release-support proof scripts | Aggregates evidence only; no product runtime caller or authority. CI/proof work is deferred by phase. | No code; proof/CI-only. |
| WP12 protected broker provisioner | Planned parent-side MSI/WiX/custom-action/build/lifecycle roots under `scripts/release/windows/parent-protected-custody/` and its package test roots; it invokes the WP01-owned BIN-only provisioner | No real custom action, package invocation, protected enrollment, or lifecycle test family is present. The WP01-owned BIN is read-only preflight with no library/public API; it always fails closed and accepts no caller/MSI-provided path, index, policy, auth, identity, or success input. External platform authority/owner handoff remains unavailable. Existing child-agent WiX is not a substitute. | Routing-only; blocked on real package invocation/test/proof evidence after consuming the accepted Protected WP01 owner boundary. |

Open gaps:

- Parent client artifact matrix is missing from the old desktop-only plan.
- Signing/store/notarization states are not explicit per artifact.
- Focused parent-client SBOM artifact proof is still blocked/manual-required in WP08 until a bounded artifact owner path is routed.
- Parent launch smoke rows now exist, but web local launch and parent-mobile artifact launch remain blocked or manual-required until their owning portal/mobile surfaces close the recorded blockers.
- Setup handoff contracts are not explicit in a single source.
- Parent-side protected broker/provisioner MSI/WiX, fixed BIN-only provisioner
  invocation, custom-action, build, and upgrade/rollback/uninstall contract is
  routed as WP12; its package source, installer ceremony, tests, and proof
  remain absent. WP01's current preflight does not establish enrollment or
  protected readiness.
- Child-agent distribution proof is owned by `child-agent-runtime-distribution-plan`, not this plan.
- WP01 scope and route-boundary reconciliation is complete.
- WP02 hosted parent web portal distribution is complete, but production publishing, account backend/runtime, setup readiness, and child runtime authority remain explicit non-claims outside this workpack.
- WP03 parent desktop shell/package is complete, but signing, production update, production rollback, setup readiness, child runtime authority, and mobile parity remain explicit manual-required or non-claims outside this workpack.
- WP04 parent Android package is complete, but device/emulator install proof, release signing, Google Play/store distribution, iOS parity, desktop parity, and child-runtime distribution remain explicit manual-required or non-claims outside this workpack.
- WP06 parent local-service route bridge is complete, but setup readiness, child runtime distribution, and package/release readiness remain explicit non-claims outside this workpack.
- WP08 parent client update rollback is complete, but SBOM remains an explicit blocker/manual-required gap and the packet does not claim release readiness.
- WP09 parent client launch smoke matrix is complete, but smoke stays downstream of runtime truth and does not convert blocked/manual-required rows into readiness claims.
- Later workpacks still need their own proof roots and focused validation.

## HID execution guard

- Follow `PLAN_EXECUTION_BLUEPRINT.md`, then `WORKPACK_INDEX.md`, then `NEXT_ACTIONS.md`.
- Use `WORKPACK_FAMILIES.md` only when owner/proof family is unclear.
- Do not mark this plan complete from checklist deltas alone.
- Before any checked update, attach a real test run log or explicit blocker from the assigned boundary and a proof manifest under `output/parent-client-runtime-distribution-plan-proof/<workpack>/`.
- Failure rule: no PR-ready claim until replay/idempotency, authZ/replay, and rollback/teardown proofs are present for the assigned slice.

## HID execution blueprint

Continue execution from: [PLAN_EXECUTION_BLUEPRINT.md](PLAN_EXECUTION_BLUEPRINT.md).
Update this plan only via the blueprint and matching workpack checklist.
