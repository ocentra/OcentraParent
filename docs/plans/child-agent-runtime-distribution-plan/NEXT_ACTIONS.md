# Next Actions

## Scope and ownership

- Plan owner: `child-agent-runtime-distribution-plan`.
- Ownership domain: child Windows, macOS, Linux, Android, and iOS package distribution, respawn, tamper/uninstall, signing/device-owner matrix, and setup-device-trust handoff.
- Scope boundary: child runtime artifacts only. Parent client distribution, setup journey ownership, account identity, policy behavior, and billing behavior are out of scope.

## Decision routes and failure conditions

- If a package artifact or signing state is missing, keep the workpack open.
- If parent client distribution is being claimed here, block the row.
- If the platform cannot support respawn or device-owner behavior, keep the row manual-required.
- If setup-device-trust handoff is not explicit, do not claim package readiness.

## Immediate audit-derived next actions

- [ ] Re-run WP02 from an elevated Windows shell so the built MSI can exercise install, start, stop, restart, uninstall, authority cleanup, and installed service-manager respawn proof instead of the current `admin-required` blocked state.
  - latest non-elevated blocked proof rerun: `test-results/windows-package-lifecycle-proof/2026-06-28T20-18-36-351Z/proof.json`
- [ ] Validate WP03's child macOS package draft with a real proof pack under `output/child-agent-runtime-distribution-plan-proof/03-child-macos-service-package/`; keep launchd, signing, and install/runtime boundaries explicit.
- [ ] Validate WP04's child Linux package draft with a real proof pack under `output/child-agent-runtime-distribution-plan-proof/04-child-linux-service-package/`; keep distro, package-lifecycle, service-health, and crash-recovery boundaries explicit.
- [ ] Validate WP05's child Android identity, foreground composition, and Rust/JNI bridge with a real proof pack under `output/child-agent-runtime-distribution-plan-proof/05-child-android-agent-package/`; keep native-library packaging, `debug-apk-built`, `debug-apk-sideload`, install/launch/removal manual-required, and device-owner/managed-profile manual-required truth explicit.
- [x] Close WP06 with the Rust-owned iOS capability proof pack under `output/child-agent-runtime-distribution-plan-proof/06-ios-entitlement-capability-proof/` and keep capability-only, provisioning-limit, supervision-limit, manual-required, no-daemon, and no-parity boundaries explicit.
- [x] Close WP08 with a real uninstall/revocation proof pack under `output/child-agent-runtime-distribution-plan-proof/08-child-parent-authorized-uninstall/` and keep parent-authorization, revocation-audit, teardown, residual-state, and no-self-authorize truth explicit.
- [x] Close WP09 with a Rust-owned shared matrix contract under `crates/schema/`, a checked-in generated TS contract, and a thin `schema-domain` adapter/proof pack under `output/child-agent-runtime-distribution-plan-proof/09-child-signing-store-device-owner-matrix/`.
- [ ] Draft and later validate the WP10 package/update consumer against a setup-owned response producer; keep the Rust-owned typed handoff, external artifact pointer, route-sync boundaries, and non-claims explicit under `output/child-agent-runtime-distribution-plan-proof/10-setup-device-trust-handoff/`.
- [x] Close WP01 with a real proof pack under `output/child-agent-runtime-distribution-plan-proof/01-child-agent-scope-and-route-boundary/` and keep the Rust-first scope route, historical parent-client compatibility note, and package/install/runtime/setup/release no-claim boundaries explicit.
- [x] Close WP11 with a real aggregate proof gate under `output/child-agent-runtime-distribution-plan-proof/11-proof-ci-release-gate/` and keep rejected/open workpacks visible so PR-ready or release-ready is not falsely claimed.
- [ ] Remove the stale legacy proof route from this plan and keep every proof reference on the `output/...` root.

## Actioned completion tracker

- [x] Confirm canonical child scope and parent/child separation.
- [x] Define the child artifact matrix.
- [ ] Define the Windows, macOS, Linux, Android, and iOS distribution contracts.
- [x] Define signing, store, and device-owner states per artifact.
- [x] Define managed respawn and uninstall/tamper proof expectations.
- [x] Define setup-device-trust handoff inputs and outputs.
- [ ] Define the proof matrix and external artifact root.
