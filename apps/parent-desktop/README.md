# Ocentra Parent Desktop

This workspace is the production desktop shell for the parent portal. The Vite portal remains the fast HMR surface for development, while this Tauri app embeds the built UI for desktop distribution and talks to the Rust parent runtime through the Tauri bridge.

The desktop shell does not execute child-device capture, policy enforcement, or AI model work. It uses Rust-owned bridge commands and displays only the read models and snapshots the service exposes.

Parent mobile runtime proof uses the same contract boundary as a shell wrapper:
Android and iOS package mechanics may prove launch scaffolds, but parent mobile
remains observer/request-first, routes assistant/report work to LAN service
providers when available, exposes stale/offline cache and parent-owned storage
states as unavailable custody paths, records per-route status reasons and
selected route state, keeps package lifecycle manual-required until real
install/update/uninstall proof exists, and never runs local model execution by
default.

Useful commands:

```powershell
cmd /c npm run build --workspace @ocentra-parent/parent-desktop
cmd /c npm run tauri:dev --workspace @ocentra-parent/parent-desktop
cmd /c npm run tauri:build --workspace @ocentra-parent/parent-desktop
```

For parallel worker demos, keep running the lane-specific Rust agent and portal dev services from the hub assignment, then use the portal dev URL for visual HMR validation.

## Runtime Package Proof

The desktop Tauri command `parent_platform_proof_state` reports the package
runtime boundary used by smoke tests:

- built portal frontend source: packaged builds use `apps/portal/dist`;
- backend kind: the shell expects the Rust agent service, not Vite as a backend;
- service launch owner: installer/package service manager starts the Rust
  service; Tauri connects to that service or reports degraded readiness;
- service health endpoint, connect timeout, and fixed loopback agent address
  ownership;
- runtime readiness: connected when the Rust service socket accepts, degraded
  when unavailable;
- route/source/custody states: active-controller route, observer read-only,
  live local-network custody, relay not implemented, parent cache stale, and
  parent-owned storage offline are serialized for route and support proof;
- package service proof: Windows service install starts `OcentraParentAgent`,
  probes `http://127.0.0.1:4477/health`, and fails the lifecycle proof when
  health is unavailable;
- process ownership: the parent desktop shell does not run child-agent capture,
  policy enforcement, timers, or local model execution;
- preview/release state: package preview is unsigned, update channel is
  scaffold-only, rollback is unavailable, and signing/notarization/store
  distribution remain manual-required;
- support/platform proof: support diagnostics expose only redacted
  version/commit/platform/package/service/route fields, and the platform matrix
  keeps parent desktop, Android parent mobile, iOS parent mobile, Android child
  agent, iOS child agent, signing, store, cache/storage, and relay rows split;
- parent mobile route-status proof: Android/iOS shell rows keep local service,
  LAN service, cloud relay, parent cache, parent-owned storage, degraded LAN AI,
  unavailable LAN AI, package lifecycle, and observer/request-first boundaries
  explicit without upgrading parent mobile UX, controller authority, cloud relay,
  store signing, or child mobile agent claims;
- port conflict policy: package/runtime proof records fixed Ocentra Parent ports
  and does not reclaim unrelated processes.

`cmd /c npm run test:parent-desktop-release-support-proof` adds the release
support proof around that runtime boundary. It builds and tests
`@ocentra-parent/parent-domain`, writes
`test-results/parent-desktop-release-support-proof/proof.json`, and validates a
typed read model with built portal dist, Rust-service boundary, package
service-manager launch ownership, fixed loopback/process ownership,
connect-or-degrade posture, signed-channel update posture, manual-required CI
artifact state, and support-safe diagnostic redaction. The diagnostic contract
rejects tokens, child activity, raw URLs, screenshots, journals, SQLite
snapshots, private paths, command lines, keystrokes, clipboard data, and message
contents.

This is CI-mechanical package/runtime proof. It is not signing, installer
release, update-channel, store, notarization, mobile, or child-device authority
proof.

## Ownership

- Packages the parent portal as a desktop app for parent-owned devices.
- Connects to local, LAN, relay, cache, or parent-owned storage paths through
  typed contracts as those paths become available, with cache/storage allowed to
  stay stale/offline instead of silently replacing the selected route.
- Presents parent-controller and parent-observer status without taking
  child-agent authority.

## Must Not Own

- Child-device capture, policy evaluation, enforcement, timers, or local model
  execution.
- Silent local storage of child evidence outside the documented custody model.
- Platform claims that are only true for the Vite dev portal.

## Connected Docs

- [Platform expectations](../../docs/expectations/platforms.md)
- [Release installer expectations](../../docs/expectations/release-installer.md)
- [Remote access expectations](../../docs/expectations/roadmap-v2-parent-owned-remote-access-cloud-relay.md)

## Gaps To Fill

- Production packaging and signing.
- Route-status UX for local, LAN, relay, cache, parent-owned storage, and
  unavailable sources.
- Parent assistant/report workflows that call real service/provider paths.
