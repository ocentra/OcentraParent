<!-- agent-capsule -->

> Agent Capsule
> Doc: Platform Expectations
> Kind: expectation/acceptance documentation; read only when selected by feature doc, plan route, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# Platform Expectations

Platform claims must match real OS capabilities. Scaffolded support is not the same as product support.

For the deliverable matrix, CI strategy, and pre-AI platform checkpoint, also
read [platform deliverables expectations](platform-deliverables.md) and
[real evidence proof expectations](real-evidence-proof.md).

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
- Hosted CI proves repeatable mechanics. Real machines and provisioned devices
  prove privileged OS/device capabilities before product claims call them fully
  working.

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
- Windows app-install package-source capture status rows may classify local
  package-source proof refs as captured, but provider/store integration and
  platform adapter execution remain separate proof gates.
- Windows app-install product-claim platform limitation fallback rows may expose
  a parent-visible fallback workflow only; they do not prove Microsoft Store
  integration, provider execution, platform interception, child delivery, app
  blocking, or product-claim approval.
- Windows app-install provider/store API execution proof rows may be
  execution-ready only as a parent-domain evidence boundary that joins
  provider/store product-claim proof with platform limitation fallback proof;
  they do not prove Microsoft Store execution, provider credentials, billing
  provider contact, store integration, platform interception, child delivery,
  app blocking, or product-claim approval.
- Windows app-install platform adapter evidence gap proof may be
  adapter-evidence-gap only when provider/store API execution proof and
  platform proof-readiness refs are attached; it does not prove real Windows
  adapter evidence, Microsoft Store execution, provider credentials, child
  delivery, app blocking, or product-claim approval.
- Windows app-install external runtime device delivery evidence proof may link
  parent-owned writer envelope, delivery result receipt, and child delivery
  envelope refs only; it does not prove external writer execution/delivery,
  Microsoft Store execution, provider credentials, platform adapter execution,
  child-device delivery, app blocking, or product-claim approval.
- Windows app-install external runtime delivery handoff proof may record
  parent-owned handoff packet and queue refs only; it does not prove external
  writer execution/delivery, Microsoft Store execution, provider credentials,
  platform adapter execution, child-device delivery, app blocking, or
  product-claim approval.
- Windows app-install external runtime writer readiness proof may classify
  parent-owned writer envelopes, delivery result receipts, target refs, audit
  refs, and report refs as handoff-ready only; it does not prove external
  writer execution/delivery, Microsoft Store execution, provider credentials,
  platform adapter execution, child-device delivery, app blocking, or
  product-claim approval.
- Windows app-install external runtime writer transport preflight proof may
  classify parent-owned transport and queue refs as ready for follow-up proof
  only; it does not prove external writer execution/delivery, Microsoft Store
  execution, provider credentials, platform adapter execution, child-device
  delivery, app blocking, or product-claim approval.
- Windows app-install external runtime writer delivery boundary proof may record
  required external writer transport, platform adapter, provider/store, and
  child-device delivery proof refs only; it does not prove external writer
  execution/delivery, Microsoft Store execution, provider credentials, platform
  adapter execution, child-device delivery, app blocking, or product-claim
  approval.
- Windows app-install external runtime writer delivery blocker proof may record
  blocked-runtime-prerequisites-missing/manual-required rows for missing
  external writer transport, platform adapter execution, provider/store
  execution, and child-device transport. It still does not prove external
  writer execution/delivery, Microsoft Store execution, provider credentials,
  platform adapter execution, child-device delivery, app blocking, or
  product-claim approval.
- Windows app-install external runtime transport queue proof may classify
  parent-owned queue and dispatch guard rows for follow-up runtime delivery
  work only. It still does not prove external writer execution/delivery,
  Microsoft Store execution, provider credentials, platform adapter execution,
  child-device delivery, app blocking, or product-claim approval.
- Windows app-install external runtime transport dispatch preflight proof may
  classify parent-owned withheld dispatch packets for follow-up runtime delivery
  work only. It still does not prove external writer execution/delivery,
  Microsoft Store execution, provider credentials, platform adapter execution,
  child-device delivery, app blocking, or product-claim approval.
- Windows app-install runtime delivery receipt boundary proof may classify
  receipt-blocked or manual-required rows from withheld dispatch packets for
  follow-up runtime delivery work only. It still does not prove external writer
  execution/delivery, Microsoft Store execution, provider credentials, platform
  adapter execution, child-device transport receipt execution, child-device
  delivery, app blocking, or product-claim approval.
- Windows app-install runtime transport delivery execution proof may classify
  parent-owned transport execution attempt, delivery result receipt, and
  child-device receipt handoff refs as withheld/manual follow-up rows only. It
  still does not prove external writer execution/delivery, Microsoft Store
  execution, provider credentials, platform adapter execution, child-device
  transport receipt execution, child-device delivery, app blocking, or
  product-claim approval.
- Windows app-install external runtime writer transport execution proof may
  classify parent-owned external writer transport packets, execution-status
  refs, and ack refs as blocked/manual follow-up rows only. It still does not
  prove external writer execution/delivery, Microsoft Store execution, provider
  credentials, platform adapter execution, child-device transport receipt
  execution, child-device delivery, app blocking, or product-claim approval.
- Windows app-install execution receipt gate proof may classify external writer
  dispatch executor, provider/store execution, platform adapter execution, and
  child-device transport receipt families as missing/manual follow-up rows
  only. It still does not prove external writer execution/delivery, Microsoft
  Store execution, provider credentials, production platform adapter execution,
  child-device transport receipt execution, child-device delivery, app
  blocking, or product-claim approval.
- Windows app-install dispatch executor receipt proof may classify parent-owned
  external writer dispatch executor handler, receipt artifact, and audit
  artifact requirements as blocked/manual follow-up rows only. It still does
  not prove external writer execution/delivery, Microsoft Store execution,
  provider credentials, production platform adapter execution, child-device
  transport receipt execution, child-device delivery, app blocking, or
  product-claim approval.
- Windows app-install package-source adapter evidence proof may attach
  sanitized local host command evidence, such as `Get-AppxPackage`
  availability and Microsoft Store package-source probe metadata, to move the
  Windows row out of pure adapter-evidence-gap. It still does not prove
  Microsoft Store execution, provider credentials, store integration, platform
  interception, production adapter implementation, child delivery, app
  blocking, or product-claim approval.
- Windows app-install package-source runtime handoff proof may project that
  sanitized command/probe status and package-source evidence refs into a typed
  parent-domain read model. It still does not prove runtime writer execution or
  delivery, Microsoft Store execution, provider credentials, store integration,
  production adapter implementation, child-device delivery, app blocking, or
  product-claim approval.
- Windows app-install provider/store platform evidence proof may combine
  provider/store execution preflight rows with package-source runtime handoff
  rows to name missing Microsoft Store credential, provider response, production
  platform adapter, platform policy, and child-device delivery receipt artifacts.
  It still does not prove provider/store execution, store integration,
  production adapter execution, runtime writer delivery, child-device delivery,
  app blocking, or product-claim approval.
- Validation needs real Windows adapter tests or guarded integration/manual evidence for each claimed OS capability, plus contract tests for every event shape crossing TypeScript and Rust.

## macOS

- Scaffold and package preview are useful early.
- Capture/enforcement claims require real permission/API proof.
- Do not assume Windows service behavior maps to launchd behavior without tests.
- Network/domain observation claims require approved macOS APIs, permission behavior, installer behavior, and user-visible consent or management requirements to be documented.
- macOS app-install package-source capture remains manual-required until host,
  signing, receipt, and store-source proof is attached behind a platform
  boundary.
- macOS app-install provider/store API execution proof must remain
  manual-required until Mac App Store credential, entitlement, provider contact,
  and platform adapter evidence are attached.
- macOS app-install platform adapter evidence gap proof must remain
  manual-adapter-evidence-required until signing, receipt, entitlement, Mac App
  Store, and adapter evidence are attached behind a platform boundary.
- Do not claim process/window, network, enforcement, or background notification parity from the package scaffold alone.

## Linux

- Useful for CI, package proof, and future desktop support.
- Do not assume Windows capture adapters apply.
- Service-manager package behavior must be tested separately.
- Network/domain observation support must name the tested distro/service-manager assumptions and kernel or desktop APIs used.
- CI package install proof is not enough to claim child-device monitoring support.
- Linux app-install package-source capture must stay unavailable until a tested
  distro/package-manager source path and limitation proof exists.
- Linux app-install provider/store API execution proof must stay unavailable
  until a tested distro/package-manager source path and provider/store evidence
  exists.
- Linux app-install platform adapter evidence gap proof must stay
  platform-unavailable until a tested distro/package-manager source path and
  platform adapter evidence exist.

## Android

- SQLite is the expected local query store.
- Use platform-approved foreground/device-management capabilities.
- Do not claim desktop-level control unless device-owner policy or equivalent is actually implemented.
- Foreground service, notification permission, accessibility, VPN, DNS, device-owner, or managed-profile behavior must be named separately; one capability does not imply the others.
- Network/domain visibility, enforcement, sync, and notifications must match Play policy and OS permission limits.
- Parent-device Android app support and child-device Android agent support are separate claims.
- Android app-install package-source capture is blocked until device-owner or
  managed-profile proof exists; generic parent-device app support is not enough.
- Android app-install provider/store API execution proof must stay
  blocked-before-claim until device-owner or managed-profile proof plus Google
  Play policy/API evidence and child-device delivery proof exist.
- Android app-install platform adapter evidence gap proof must stay
  blocked-before-claim until device-owner or managed-profile adapter proof,
  Google Play policy/API evidence, and child-device delivery proof exist.

## iOS

- Most restrictive target.
- Use Apple-approved capabilities and entitlements only.
- Do not claim background monitoring or enforcement beyond proven APIs.
- Family Controls, Screen Time APIs, Network Extension, notifications, and background execution require separate entitlement and review proof before product claims.
- Parent-device iOS app support and child-device iOS agent support are separate claims.
- If a capability cannot be implemented under approved APIs, document the limitation instead of implying parity.
- iOS app-install package-source capture is blocked until Apple entitlement and
  review proof exists; limitation rows should be explicit when APIs do not allow
  parity.
- iOS app-install provider/store API execution proof must stay
  blocked-before-claim until Apple entitlement/review proof plus approved
  App Store or Family Controls evidence and child-device delivery proof exist.
- iOS app-install platform adapter evidence gap proof must stay
  blocked-before-claim until Family Controls entitlement adapter proof, Apple
  review evidence, App Store evidence, and child-device delivery proof exist.

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
- Release-support proof must split parent desktop shell, parent mobile bridge,
  child desktop agent, child Android agent, child iOS agent, relay, signing,
  stores, and support states. Observer read-only state cannot imply policy write,
  approval, controller, capture, enforcement, timer, or local AI authority.
- `mobile-child-agent-capability-proof` is the aggregate proof hook for child
  Android/iOS mobile capability rows; it may record scaffold/manual-required/
  blocked/not-implemented states, but it must not upgrade real device,
  entitlement, signing, store, or external-transport claims without matching
  artifacts.

## Validation Gates

- Platform claim matrix in release notes or feature handoff for each completed platform-facing slice.
- Pre-AI proof matrix entry for each completed runtime claim.
- Real package install or launch smoke for scaffold claims.
- Manual platform proof records must name host/device, command or UI action,
  permissions, package version, logs/screenshots/proof JSON, and known gaps.
- Real OS capability tests, guarded integration tests, or documented manual evidence for capture, network/domain observation, pairing, enforcement, notifications, and background behavior.
- Contract and Rust parity tests for every platform event or command shape.
- Security review for LAN exposure, cloud relay, provider credentials, export/sync, device identity, and enforcement.

## Done Signal

Every platform-facing feature says exactly which platform behavior is implemented, which behavior is scaffold-only, and what validation proves the claim.
