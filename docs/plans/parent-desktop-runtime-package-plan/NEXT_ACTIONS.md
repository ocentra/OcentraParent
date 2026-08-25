# Parent Client Runtime Distribution Next Actions

## Scope and ownership

- Plan owner: `parent-client-runtime-distribution-plan` in the historical `parent-desktop-runtime-package-plan` path.
- Ownership domain: parent web portal distribution, parent desktop shell/package, parent Android package, parent iOS package, route bridge, signing/store matrix, update/rollback, and launch smoke.
- Scope boundary: parent client artifacts only. Child agent distribution, setup journey, account provider choice, pairing protocol internals, policy behavior, billing provider behavior, remote access, data custody, and child capture/enforcement adapters are out of scope.
- Use `WORKPACK_FAMILIES.md` only when the selected workpack owner/proof family is unclear.

## Decision routes and failure conditions

- If a package artifact or signing state is missing, keep the workpack open.
- If the setup handoff is being treated as package proof, block the row.
- If the mobile surface is scaffold-only, keep the row manual-required.
- If the route bridge is not explicitly defined, do not claim parent client readiness.
- If child-agent, setup, device-trust, account, payment, policy, remote, or custody behavior is needed, route through that owning plan instead of widening this one.

## Closed in completed workpacks

- [x] Confirm canonical scope and route bridge separation. Proof: `output/parent-client-runtime-distribution-plan-proof/01-parent-client-scope-and-route-boundary/`.
- [x] Define the parent web portal distribution contract and proof boundary. Proof: `output/parent-client-runtime-distribution-plan-proof/02-parent-web-portal-distribution/`.
- [x] Define the parent local-service route bridge boundary and no-claim separation. Proof: `output/parent-client-runtime-distribution-plan-proof/06-parent-local-service-route-bridge/`.
- [x] Define the parent client launch smoke matrix and its no-claim boundaries. Proof: `output/parent-client-runtime-distribution-plan-proof/09-parent-client-launch-smoke-matrix/`.

## Remaining plan-level follow-ons

- [ ] Define the parent client artifact matrix.
- [x] Define the parent desktop shell/package contract.
- [ ] Define the remaining iOS distribution state and the cross-artifact signing/store matrix.
- [ ] Define signing/store/notarization states per artifact.
- [x] Define update/rollback and launch-smoke proof expectations.
- [ ] Define setup handoff inputs and outputs.
- [ ] Define the proof matrix and external artifact root.

## WP12 protected broker provisioner route

- [ ] Define the parent-side MSI/WiX artifact, elevated custom-action/provisioner
  invocation, and build/release wiring under
  `scripts/release/windows/parent-protected-custody/`, with the manifest at
  `scripts/release/windows/parent-protected-custody.wxs` and the package build
  entry at `scripts/release/windows/build-parent-protected-custody-package.ps1`.
- [ ] Invoke and package Protected WP01's owner-approved BIN-only provisioner
  binary. The Cargo manifest, `src/main.rs`, and private `src/provisioning/`
  source are WP01-owned; WP12 owns no library/public API and accepts no
  caller/MSI-provided path, TPM index/policy, `authValue`, identity, generation,
  lease, capability, or success input.
- [ ] Consume Protected WP01's private core/FFI enrollment contract without
  exposing or accepting raw `authValue`, TPM index/policy, SID, path, image,
  generation, lease, capability, or caller-supplied success.
- [ ] Keep Protected WP01 as the sole owner of the provisioner source,
  authority creation, TPM policy, enrollment format, and non-exportable-handle
  acceptance; WP12 owns only binary invocation and
  install/repair/upgrade/rollback/uninstall lifecycle.
- [ ] Define real install/repair/upgrade/rollback/uninstall and explicit
  deprovisioning outcomes; preserve TPM generation and fail closed on missing
  or contradictory enrollment.
- [ ] Add the package/lifecycle tests and retained proof under the WP12 roots;
  no package success, signing/checksum, or service registration may claim
  protected authority or runtime readiness.
- [ ] Keep WP12 blocked until the accepted Protected WP01 owner boundary is
  consumed by real package invocation/lifecycle implementation, expected
  tests, focused execution, and proof.

## WP01 closeout

- WP01 is complete. The canonical scope, Rust-first route-bridge boundary, setup handoff separation, child-runtime exclusion, portal UX handoff, and compatibility-only historical folder path are now proved under `output/parent-client-runtime-distribution-plan-proof/01-parent-client-scope-and-route-boundary/`.

## WP02 closeout

- WP02 is complete. The real portal build, hosted route boundary, auth gating, stale-cache honesty, and preview/staging/production separation are now proved under `output/parent-client-runtime-distribution-plan-proof/02-parent-web-portal-distribution/`.
- The first Playwright attempt hit a foreign process on default port `4490`; the focused hosted spec passed on explicit free ports `4590`-`4592`, and the proof log records that reroute.
- WP02 does not claim production publishing, setup readiness, desktop/mobile package readiness, or child runtime authority.

## WP06 source acceptance and open test wave

- WP06 has independently accepted refreshed production source. The prior proof root remains historical until the complete response-identity, freshness, deadline, dependency/LAN failure, and no-stale-connected test family is refreshed, run, and re-proved.
- Focused cargo tests passed for Devices-route unavailable, timeout, passive-local-target, and setup-first-run separation behavior, and the schema contract test passed for the canonical route bridge shape and generated thin TS artifacts.
- WP06 does not claim setup readiness, child runtime distribution ownership, desktop/mobile/web package readiness, or portal UX ownership.

## WP03 source acceptance and open test wave

- WP03 retains its real Tauri package artifacts and historical proof, while the refreshed all-command response binding and hard transport deadline source is independently accepted. Desktop-shell/bridge tests and proof must be refreshed before closure.
- WP03 does not claim signed release readiness, production update or rollback readiness, setup completion, child runtime authority, or Android/iOS parity.

## WP04 closeout

- WP04 is complete. The parent Android package packet now has a real proof root under `output/parent-client-runtime-distribution-plan-proof/04-parent-android-package/`, with the `release:package:parent-android` build anchor, APK/checksum artifact evidence, explicit `ca.ocentra.parent.mobile/.MainActivity` launch target, and an Android install/store/manual-required register.
- WP04 records the current install truth honestly: this checkout had no attached device in `adb devices` and no local `emulator` command, so install/launch remains manual-required instead of being promoted from the debug APK artifact.
- WP04 does not claim child-runtime distribution, Google Play release readiness, iOS readiness, desktop readiness, or setup completion.

## WP09 closeout

- WP09 is complete. The launch smoke matrix now has a real proof root under `output/parent-client-runtime-distribution-plan-proof/09-parent-client-launch-smoke-matrix/`, with explicit web, desktop, Android, and iOS rows plus degraded, unavailable, and manual-required visibility.
- WP09 records the current smoke truth honestly: web is blocked, desktop is manual-required after passing dry-run launch and Rust bridge proof, and Android/iOS are blocked before artifact launch in the current parent-mobile proof path.
- WP09 does not claim setup completion, child runtime ownership, desktop/mobile parity, or product readiness from smoke alone.

- Next smallest open workpack is WP05 parent iOS package.
