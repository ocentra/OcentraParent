<!-- agent-capsule -->

> Agent Capsule
> Doc: Platform Deliverables Expectations
> Kind: expectation/acceptance documentation; read only when selected by feature doc, plan route, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# Platform Deliverables Expectations

Platform work is a product-deliverable matrix, not a single "cross-platform"
claim. CI should do as much repeatable work as possible, but real OS capture,
permission, packaging, and enforcement behavior still needs platform evidence.
Runtime proof also follows the [real evidence proof expectations](real-evidence-proof.md).

## Outcome

- Windows, macOS, Linux, Android, and iOS deliverables are tracked separately.
- Shared Rust crates, TypeScript contracts, journal format, SQLite query shape,
  and protocol events stay portable unless an explicit platform boundary says
  otherwise.
- Platform-specific adapters own OS APIs, permissions, packaging, service
  managers, foreground/background limits, and store rules.
- AI and policy layers consume typed evidence refs and capability states. They
  must not assume a platform has evidence that its adapters cannot prove.

## CI As Cross-Platform Worker

Use GitHub Actions and package-preview jobs to catch cross-platform drift early:

- Run shared TypeScript contract, schema-boundary, and source-shape checks.
- Build and test Rust crates on each available runner.
- Build/package preview artifacts for Windows, macOS, Linux, Android, and iOS
  where the repo has scaffolded lanes.
- Run install or launch smoke checks where runners can do that honestly.
- Treat runner failures as integration work for the owning platform slice.
- For parent desktop package-runtime proof, require built frontend dist,
  Rust-service boundary, service-manager launch ownership, fixed loopback
  process ownership, connect-or-degrade state, support-safe diagnostics, and
  explicit preview/manual-required non-claims.
- For V8 production release/support readiness proof, require the package-preview
  matrix to name Windows MSI, Linux DEB, macOS PKG, Android APK, and iOS
  simulator artifacts separately while keeping signing, store, real-device,
  updater rollback, and production publishing claims manual-required or
  promotion-required.
- For V8 updater rollback and release-support runbook proof, require scaffold,
  unsigned-preview, signature-required, and production update-channel rows
  separately. CI/package preview may prove mechanics, but rollback execution,
  rollback failure handling, signed channel proof, and production support
  runbook publication stay manual-required until real platform evidence exists.
- For `production-release-public-status-proof`, require public download,
  release status, update status, account status, subscription status, and
  support status surfaces to stay route-contract or manual-required until real
  public runtime, backend, signing/store, updater execution, publishing
  promotion, and support backend evidence exists.
- For `production-release-public-runtime-handoff-proof`, require route/status
  and backend adapter rows for public download, release status, update status,
  account status, subscription status, support status, and their runtime
  handoff destinations while keeping real public runtime, account/backend/
  provider execution, signing/store proof, updater execution, production
  publishing, support backend upload, and real device/store proof
  manual-required or unimplemented.

CI is not enough for:

- OS permission dialogs, entitlement behavior, device-owner policy, VPN/DNS
  permission behavior, launchd/systemd/service-manager behavior, or Windows
  service behavior that requires a real host.
- Store review, signing, notarization, TestFlight, or managed-device policy
  proof.
- LAN discovery across real machines, parent/child device pairing, or network
  visibility that depends on local router/firewall behavior.

For those cases, CI must still prove mechanics: the app/package launches, the
service reports honest capability state, typed requests route through the real
local service, and the portal renders the returned state. Real machines or
devices then prove the privileged capability with recorded evidence.

## Pre-AI Platform Checkpoint

Before adding more AI capability or enforcement behavior, pause for a platform
deliverables planning pass:

- Pull current `main` and verify the shared local contracts compile and test.
- Run the Windows local product flow first on the downstairs PC.
- Use GitHub Actions plus WSL or Docker for Linux build/package/test signal.
- Use the Mac system for macOS package, service/permission, and future iOS
  proof.
- Use Android emulator/device checks for Android package, foreground service,
  permission, and managed-device assumptions.
- Use Mac/Xcode/TestFlight or entitlement review notes for iOS claims.
- Record which evidence is implemented, scaffold-only, unavailable, or blocked
  per platform before starting the next AI/enforcement slice.
- Keep `docs/expectations/pre-ai-proof-matrix.json` current so every completed
  runtime claim has an honest CI and manual proof level.

## Windows Deliverables

- First local implementation target.
- Prove service install/start/stop/uninstall, restart survival, loopback/LAN
  portal reachability, encrypted journal writes, SQLite rebuild, process/window
  capture, browser URL/tab evidence, app/game session evidence, network/domain
  summaries, screen-analysis queue summaries, and dry-run policy preview.
- Package/runtime release-support proof may show Windows CI/package mechanics
  and support redaction, but signing, production updater rollback, store
  distribution, and production support remain manual-required until real
  artifacts and credentials prove them.
- Updater rollback/runbook status proof may show Windows package-preview and
  runbook mechanics, but it must keep signed-channel execution, rollback
  execution smoke, rollback failure smoke, and production support escalation
  manual-required.
- Run Windows local tests before product checkpoint signoff. CI may supplement
  but must not replace the local PC proof.

## Linux Deliverables

- Use Linux runners plus WSL or Docker for shared Rust/domain portability,
  package preview, install smoke, and service-manager assumptions.
- Do not claim Linux desktop capture, network visibility, or enforcement until
  Linux adapters name and test their APIs and distro/service-manager scope.
- Linux can be a strong portability gate before AI because the shared Rust and
  query layers should work there without Windows APIs.

## macOS Deliverables

- Use CI for shared build/test/package preview and the Mac system for real
  permission, launchd, signing/notarization, and OS API behavior.
- Do not claim capture, network observation, background operation, or
  enforcement parity from a scaffolded app bundle.
- macOS child-agent support requires explicit API, permission, installer, and
  user-consent evidence.

## Android Deliverables

- Track parent Android app and child Android agent claims separately.
- Use CI/emulator/device checks for package install, launch, SQLite/journal
  compatibility, foreground service behavior, notification permission,
  accessibility, VPN/DNS, device-owner, or managed-profile assumptions as each
  becomes relevant.
- `mobile-child-agent-capability-proof` may aggregate current Android
  child-agent scaffold/manual-required rows, but real Android support still
  needs emulator/device artifacts for foreground service, UsageStats,
  Accessibility, VPN/DNS, Device Owner, managed profile, Play signing, and
  transport behavior.
- Do not imply desktop-level capture or enforcement unless Android policy and
  device-management mode actually support it.

## iOS Deliverables

- Track parent iOS app and child iOS agent claims separately.
- Use the Mac system for Xcode, simulator/device, TestFlight, entitlement, and
  signing proof.
- Family Controls, Screen Time APIs, Network Extension, notifications, and
  background execution need separate entitlement and review evidence.
- `mobile-child-agent-capability-proof` may aggregate current iOS
  child-agent simulator/manual-required rows, but real iOS support still needs
  Apple entitlement, signing, TestFlight/device, App Store, notification,
  background, and Network Extension artifacts.
- If Apple-approved APIs cannot provide a capability, document the limitation
  instead of claiming parity.

## Done Signal

Every platform-facing roadmap slice must state:

- Platforms affected.
- Shared portable behavior proved by contracts/Rust tests.
- Platform adapter behavior proved by real OS evidence.
- Scaffold-only behavior.
- Known unavailable/degraded states.
- CI jobs and manual checks that support the claim.
- Proof matrix status: CI mechanical proof, manual-required, scaffold-only,
  not-yet-proven, or not-applicable.
