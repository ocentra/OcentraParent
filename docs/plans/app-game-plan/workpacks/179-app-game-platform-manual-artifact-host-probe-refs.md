# WP179 App/Game Platform Manual Artifact Host Probe Refs

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP179 App/Game Platform Manual Artifact Host Probe Refs`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Scope

Carry the WP178 parent-safe host capability probe refs into the V0.8 OS adapter
manual artifact gate read model. This keeps Windows, Android, and Linux host
visibility attached to the manual proof gate without upgrading Android/Linux
support or claiming macOS/iOS local execution from a Windows worker.

## Implementation

- Add `hostCapabilityProbeRefs` to each manual artifact gate row in
  `@ocentra-parent/parent-domain`.
- Require platform-matched opaque probe refs:
  - Windows rows use `windows-host-local-probe-ref`.
  - Android rows use `android-adb-path-probe-ref` and
    `android-adb-sdk-probe-ref`.
  - Linux rows use `linux-wsl-path-probe-ref` and
    `linux-docker-path-probe-ref`.
  - macOS and iOS rows keep an empty probe-ref list because local Windows
    execution cannot prove those platforms.
- Reject mismatched or raw-looking probe refs through the schema honesty filter.
- Update the manual artifact gate proof harness to count probe-ref rows and
  record that those refs do not expose paths, device serials, distro names, or
  private diagnostics.

## Validation

- `cmd /c npm run test --workspace @ocentra-parent/parent-domain -- v0-8-os-adapter-manual-artifact-gates app-game-adapter-execution-readiness`
- `cmd /c npm run build:contracts`
- `cmd /c "node --check scripts/test/v0-8-os-adapter-manual-artifact-gates.mjs && node scripts/test/v0-8-os-adapter-manual-artifact-gates.mjs"`

## No-Claim Boundaries

- Does not claim Android UsageStats, Accessibility, VPN/DNS, Device Owner,
  Profile Owner, hide/suspend, uninstall-block, lock-task, managed
  configuration, or Play policy proof.
- Does not claim Linux package manager, Flatpak, Snap, AppImage, procfs,
  cgroup/systemd, X11/Wayland, AppArmor, SELinux, or package restriction proof.
- Does not claim macOS or iOS runtime execution from Windows.
- Does not add broad installed-app blocking, platform enforcement, provider
  delivery, child-device delivery, raw private source rows, raw target values,
  or private diagnostics.
