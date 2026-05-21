# Platform Expectations

Platform claims must match real OS capabilities. Scaffolded support is not the same as product support.

For the deliverable matrix, CI strategy, and pre-AI platform checkpoint, also
read [platform deliverables expectations](platform-deliverables.md).

## Cross-Platform Claim Rule

Every feature must state the parent outcome, child-device outcome, supported platform, data scope, trust boundary, failure behavior, and validation that proves the claim. A scaffolded package, app shell, package-preview artifact, or CI launch smoke is not enough to claim capture, monitoring, enforcement, remote access, or notification support.

Shared Rust crates, TypeScript contracts, journal format, SQLite query shape, and
protocol events should stay portable by default. OS-specific capture,
permission, packaging, service-manager, foreground/background, and enforcement
behavior belongs behind platform adapters and must be proved per platform.

## CI And Manual Proof

- GitHub Actions should build, test, and package as much of Windows, macOS,
  Linux, Android, and iOS as the repo can honestly automate.
- CI failures should be treated as cross-platform integration work, not ignored
  until release time.
- CI does not replace real host proof for OS permissions, service managers,
  LAN behavior, signing/notarization, store review, device-owner policy, or
  mobile entitlements.
- Before starting more AI or enforcement work, run a cross-platform planning
  checkpoint that separates implemented, scaffold-only, unavailable, blocked,
  and degraded behavior for each deliverable platform.

## Windows

- First production-grade agent target.
- Service, MSI, process/window capture, network observation, local policy, and enforcement are expected here first.
- Windows-specific adapters must stay behind platform boundaries.
- Network/domain observation is expected to be intent-first: domain, IP, port, protocol where available, process correlation where available, timestamp, observer/source ids, attribution confidence, and unknown attribution state.
- Windows network/domain observation must not decrypt HTTPS payloads, inspect page contents, or imply packet-content monitoring unless a later explicit product/legal/security boundary approves it.
- Process-to-network correlation may be unavailable or partial. Unknown process, unknown domain, and IP-only observations must be recorded honestly instead of guessed.
- Windows LAN mode must remain explicit, origin-checked, and paired before accepting remote parent control from another device.
- Windows cloud relay, sync/export, and notifications must preserve local-first
  operation when remote services or providers fail, and must not require
  Ocentra-hosted storage of child activity data.
- Validation needs real Windows adapter tests or guarded integration/manual evidence for each claimed OS capability, plus contract tests for every event shape crossing TypeScript and Rust.

## macOS

- Scaffold and package preview are useful early.
- Capture/enforcement claims require real permission/API proof.
- Do not assume Windows service behavior maps to launchd behavior without tests.
- Network/domain observation claims require approved macOS APIs, permission behavior, installer behavior, and user-visible consent or management requirements to be documented.
- Do not claim process/window, network, enforcement, or background notification parity from the package scaffold alone.

## Linux

- Useful for CI, package proof, and future desktop support.
- Do not assume Windows capture adapters apply.
- Service-manager package behavior must be tested separately.
- Network/domain observation support must name the tested distro/service-manager assumptions and kernel or desktop APIs used.
- CI package install proof is not enough to claim child-device monitoring support.

## Android

- SQLite is the expected local query store.
- Use platform-approved foreground/device-management capabilities.
- Do not claim desktop-level control unless device-owner policy or equivalent is actually implemented.
- Foreground service, notification permission, accessibility, VPN, DNS, device-owner, or managed-profile behavior must be named separately; one capability does not imply the others.
- Network/domain visibility, enforcement, sync, and notifications must match Play policy and OS permission limits.
- Parent-device Android app support and child-device Android agent support are separate claims.

## iOS

- Most restrictive target.
- Use Apple-approved capabilities and entitlements only.
- Do not claim background monitoring or enforcement beyond proven APIs.
- Family Controls, Screen Time APIs, Network Extension, notifications, and background execution require separate entitlement and review proof before product claims.
- Parent-device iOS app support and child-device iOS agent support are separate claims.
- If a capability cannot be implemented under approved APIs, document the limitation instead of implying parity.

## Web

- Public/download/account/subscription surface first.
- Development portal scaffold only until a packaged parent portal exists.
- Does not run the child-device agent.
- Does not run child-device AI, policy evaluation, enforcement, timers, capture adapters, or scripts.
- Talks to local, LAN, or cloud-routed agents through typed service contracts.
- Web may show network/domain, LAN, cloud, sync/export, notification, and
  connector state only when returned by typed local, LAN, parent-owned storage,
  or cloud-routed services.
- Web must not claim offline child-device monitoring when no reachable child-device agent or cloud relay is connected.
- Web must not store child activity evidence, generated reports, screenshots, or
  parent rules in Ocentra-hosted infrastructure by default.

## Parent Desktop App

- Production parent portal should be packaged for parent-owned devices. Tauri is
  the preferred desktop-shell candidate until an architecture decision says
  otherwise.
- The parent desktop app may connect over loopback, LAN pairing, authenticated
  relay, local parent cache, or parent-owned storage connectors.
- The parent desktop app still does not run child-device capture, local AI
  safety evaluation, policy execution, enforcement adapters, timers, or scripts.
- It should label data source and custody clearly: live local/LAN, parent cache,
  parent-owned storage, Ocentra-hosted non-activity metadata, or unavailable.

## Validation Gates

- Platform claim matrix in release notes or feature handoff for each completed platform-facing slice.
- Real package install or launch smoke for scaffold claims.
- Real OS capability tests, guarded integration tests, or documented manual evidence for capture, network/domain observation, pairing, enforcement, notifications, and background behavior.
- Contract and Rust parity tests for every platform event or command shape.
- Security review for LAN exposure, cloud relay, provider credentials, export/sync, device identity, and enforcement.

## Done Signal

Every platform-facing feature says exactly which platform behavior is implemented, which behavior is scaffold-only, and what validation proves the claim.
