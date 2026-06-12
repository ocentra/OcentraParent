<!-- agent-capsule -->

> Agent Capsule
> Doc: Remote, LAN, And Mobile Platforms
> Kind: feature documentation; read only when selected by FEATURE_ROUTE_INDEX, PLAN_INDEX, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# Remote, LAN, And Mobile Platforms

## Parent Outcome

Parents can control and observe household devices locally, over LAN, and later
away from home, while Android/iOS/macOS/Linux claims remain honest about real
platform permissions and support.

## Ocentra Requirement

LAN and remote are route/custody problems, not excuses to centralize child
activity data. Mobile support must separate parent app claims from child-agent
claims.

Household AI provider mesh is a LAN/local route and custody problem, not a
central queue. Each runtime keeps its own local event bus. The Household Mesh
Bridge may export selected validated events into typed LAN messages and
republish validated incoming messages locally. Child-agent policy authority
does not move to the provider device.

## Roadmap And Expectations

- Roadmap: V0.9 LAN pairing, V2 remote access/cloud relay, V6 mobile agents,
  V8 production hardening.
- Expectations: [LAN pairing](../expectations/lan-pairing.md),
  [cloud](../expectations/cloud.md), [platforms](../expectations/platforms.md),
  [mobile agents](../roadmaps/roadmap-v6-mobile-agents.md),
  [release installer](../expectations/release-installer.md).
- Modules: `packages/parent-domain`, `packages/agent-protocol-domain`,
  `apps/parent-desktop`, `platforms/android`, `platforms/ios`,
  `crates/agent-service`.

## Competitor Pressure

See [Competitor Capability Map](../competitor-capability-map.md), especially
multi-device household, remote parent access, mobile coverage, and production
distribution.

Google, Apple, Microsoft, Bark, Qustodio, and others work across phones,
tablets, desktops, and parent apps. Ocentra must be usable beyond one Windows
PC while preserving local-first custody.

## Current Ocentra State

- LAN pairing/control proof and add-device state are in progress.
- Household AI provider mesh is now planned as a child-agent-owned AI work
  ledger plus trusted provider advertisement, claim/lease, result validation,
  custody, and mobile dormant/fallback rules. Existing degraded/unavailable LAN
  AI provider rows and screen family-hub runtime discovery do not yet prove
  full decentralized provider execution across physical household devices.
- `scripts/test/eventing-household-mesh-consumer-proof.mjs` now proves the
  Household Mesh Bridge consumer boundary: selected local events can export as
  typed authenticated LAN messages, incoming messages validate before local
  republish, unselected or mismatched event/message refs are rejected, direct
  remote publish into another runtime bus is rejected, and child-agent-only AI
  policy authority is preserved. This remains bridge evidence, not physical
  household provider execution.
- The parent service now emits a typed LAN scan summary, merges the local
  child-agent device with matching passive LAN evidence, and keeps passive
  neighbors/router rows separate from controllable child-agent targets.
- The V0.9 production household proof summary is contract-backed across
  parent-domain, agent-protocol-domain, Rust protocol, and Rust service state.
  It exposes signed hello/heartbeat, passive/router discovery, mDNS/SSDP/router
  DHCP name discovery, trusted registry, parent decisions, route custody,
  stale/offline selected-device, relay/cache, second physical child-agent,
  Android/iOS parity, and store-signing rows without upgrading manual or
  not-implemented claims.
- The V0.9 signed discovery/relay spine now adds typed adapter rows for passive
  LAN neighbor, router/infrastructure, mDNS, SSDP, router DHCP, manual direct
  address, signed child-agent hello, and signed child-agent heartbeat. It also
  exposes signed proof rejection rows, route safety rows, relay/cache decision
  rows, explicit parent-owned storage unavailability, and no Ocentra child-data
  custody claims across parent-domain, agent-protocol-domain, Rust protocol,
  Rust service state, and the focused proof harness.
- Parent portal LAN detail and Activity/Network diagnostics now consume the
  signed discovery/relay spine for route custody, signed proof, relay/cache
  unavailable state, parent decision/audit labels, route requirements, manual
  proof, and unproved claims. Devices/LAN now exposes first-class add, route
  select, rename, trust, ignore, restore, and revoke controls through existing
  add-device household-decision and route command surfaces, and portal transport
  routes LAN commands to the selected local-network child target. Canonical
  household rename/type decisions for LAN-discovered neighbors also route
  through the local-network service path and survive portal refresh without a
  portal-only second truth. Activity/Network diagnostics also show scan/evidence
  timing, evidence expiry, signed adapter proof state, and policy-target
  history.
- The LAN source-matrix follow-up now carries all 20 LAN plan workpacks and
  concrete discovery source rows through parent-domain, agent-protocol-domain,
  Rust protocol, Rust service state, and Activity/Network diagnostics. The
  focused proof script writes
  `test-results/v0-9-lan-source-matrix-plan-completion/proof.json`, and B-lane
  browser artifacts under
  `output/playwright/lan-source-matrix-plan-completion/` show Devices/LAN,
  Activity/Network source-matrix diagnostics, and Network policy target binding.
  The household identity routing proof is recorded at
  `output/lan-plan-proof/15-household-device-store/devices-identity-routing-proof.md`.
- Parent desktop Tauri proof now serializes active-controller route,
  observer-read-only, live local-network custody, relay unavailable, cache
  unavailable, and parent-owned storage unavailable states without implementing
  LAN discovery itself.
- The V0.9 parent mobile controller/observer runtime proof now records
  controller-lease visibility as read-only, separates Android degraded LAN AI
  provider handoff from iOS unavailable provider state, regenerates its
  parent-mobile/production/discovery source proof artifacts, and keeps local,
  LAN, relay, parent-cache, and parent-owned-storage route states explicit in
  `test-results/v0-9-mobile-controller-observer-runtime-proof/proof.json` and
  `test-results/parent-mobile-service-bridge-proof/proof.json`.
- The parent mobile shell runtime proof now requires per-route status reasons,
  custody, and selected-route state for Android/iOS local service, LAN service,
  cloud relay, parent cache, and parent-owned storage. It also records package
  lifecycle as manual-required, keeps Android observer/request boundary
  read-only, keeps iOS controller takeover request-first/manual-required, and
  proves degraded and unavailable LAN AI provider rows in
  `test-results/parent-mobile-shell-runtime-proof/proof.json`.
- Android/iOS package scaffolds and proof gates exist.
- Parent desktop release-support proof separates parent mobile bridge state from
  child Android/iOS agent claims and keeps signing, stores, TestFlight, Play,
  relay, and mobile child-agent parity manual-required or not implemented.
- Parent mobile Android/iOS scaffold package previews now build and smoke
  separately from child-agent Android/iOS package previews.
- Parent-owned sync/export endpoint contract proof now defines versioned
  endpoint-domain paths, headers, query params, and contract-version labels for
  parent-owned export status/sync cursor/import preview/delete status and remote
  connector status boundaries. The focused
  `sync-export-endpoint-contract-proof` harness writes
  `test-results/sync-export-endpoint-contract-proof/proof.json` and keeps
  connector OAuth, upload/download runtime, Ocentra-hosted family data custody,
  account/subscription backend, and portal UI unclaimed.
- `mobile-child-agent-capability-proof` now aggregates the Android package,
  storage/protocol, service, permission, privileged, device-gate, and iOS
  entitlement proof states into one platform matrix without upgrading any mobile
  child-agent parity, entitlement, signing, store, or external-transport claim.
- Optional remote relay and full mobile child-agent parity are not complete.

## Current Gap

Physical household proof still needs a second installed child agent, signed LAN
agent hello/heartbeat artifacts, and stronger name discovery such as
mDNS/SSDP/router DHCP integration. Browser screenshot proof now exists for the
current B-lane Devices/LAN, Activity/Network source-matrix diagnostics, and
Network policy surfaces, but that is not two-physical-child household proof.
Household AI provider mesh now has eventing bridge-boundary proof for selected
event export, validated republish, unselected or mismatched ref rejection, no
direct remote publish, no raw payload transfer, and
provider-cannot-publish-policy authority. It still needs physical provider
advertisement/heartbeat, claim/lease, duplicate prevention, child-agent result
validation over real provider output, mobile dormant/fallback, and production
runtime proof.
Optional remote relay, cache route, parent-owned sync/export transfer runtime,
connector OAuth/revocation, mobile permissions, Android Device
Owner/Accessibility/VPN/DNS/UsageStats proof, iOS Family
Controls/DeviceActivity/Network Extension proof, signing, and store
distribution remain.

## Checklist

- [ ] LAN discovery and pairing. Current proof covers Windows neighbor-table
      inventory, local child-agent hardware inventory, scan summary counts, passive
      neighbor/router separation, portal target filtering, and typed production
      proof rows. The signed discovery/relay spine now type-checks signed
      hello/heartbeat manual-required rows, stale/expired/replayed/wrong-origin/
      wrong-device/revoked/anonymous rejection rows, mDNS/SSDP/router DHCP manual
      rows, and route-safety rows; actual signed hello/heartbeat artifacts and
      second-child-agent pairing proof remain manual-required. Portal LAN detail
      and Activity/Network diagnostics consume these fields, and Devices/LAN
      exposes command-backed action controls plus service-backed household
      rename/type persistence for LAN-discovered neighbors. Browser screenshot
      proof exists under `output/playwright/lan-source-matrix-plan-completion/`
      and `output/lan-plan-proof/15-household-device-store/`; real signed
      hello/heartbeat and second-child-agent pairing proof remain
      manual-required.
- [ ] Trusted registry and revocation. Current proof covers typed registry,
      trust, parent decision, revocation, stale, and offline rows; portal
      diagnostics now show the read-model/audit state and command-backed
      Devices/LAN controls can request trust/ignore/restore/revoke. Live
      two-device household recovery proof remains.
- [ ] Controller lease and observer read-only state. Current proof covers
      parent-mobile controller-lease visibility as read-only, rejects mobile
      observer writes and approvals, keeps controller takeover manual-required,
      and proves backend lease release as local-service-owned instead of mobile
      authority. Real Android/iOS mobile controller write authority remains
      manual-required.
- [ ] Route status: local, LAN, relay, cache, stale, offline, unavailable.
      Current parent-mobile observer/service-bridge proofs cover explicit local,
      LAN, cloud-relay, parent-cache, and parent-owned-storage route states, with
      LAN AI degraded and unavailable provider states separated. The parent
      mobile shell runtime proof now adds route status reasons, custody, selected
      route ids, stale cache, offline parent-owned storage, cloud relay
      not-implemented, package lifecycle manual-required, and controller
      observer/request-first boundaries. Real cloud relay, parent-owned
      storage/cache freshness, and physical household LAN remain unimplemented
      or manual-required.
- [ ] Optional relay without default child-data custody. Current production LAN
      proof and signed discovery/relay spine explicitly mark relay route, queued
      relay, cache route, and parent-owned storage unavailable/not implemented
      while preserving no Ocentra child-data custody. The
      `sync-export-endpoint-contract-proof` adds route/header/query/version
      contracts for parent-owned sync/export and remote connector status without
      implementing relay, connector OAuth, upload/download, account/subscription
      backend, portal UI, or Ocentra-hosted family data custody.
- [ ] Android child-agent real device proof. Current aggregate proof covers
      `mobile-child-agent-capability-proof` contract/test/harness rows for
      foreground service, storage/protocol bridge, notifications, UsageStats,
      Accessibility, VPN/DNS, Device Owner, managed profile, Play signing, and
      external transport; real emulator/device behavior remains manual-required.
- [ ] iOS child-agent entitlement/device proof. Current aggregate proof covers
      `mobile-child-agent-capability-proof` contract/test/harness rows for
      simulator status, Family Controls, DeviceActivity, Screen Time, Network
      Extension, notifications/background execution, signing, TestFlight,
      device proof, App Store, and external transport; entitlement/device
      behavior remains manual-required.
- [ ] Parent mobile app proof separated from child mobile agent proof.
      Current release-support and V0.9 parent-mobile observer proofs model this
      split for handoff/reporting, route status, controller lease, LAN AI
      provider state, package lifecycle, and explicit no-child-agent-parity
      claims. CI now has separate parent mobile Android/iOS package-preview
      targets; real parent mobile UX beyond scaffold status, store signing,
      controller authority, and child mobile agent proof remain.

## Next AI Instructions

Never claim "mobile support" as one thing. Split parent mobile, child Android,
child iOS, platform packaging, store signing, route status, and remote custody
in every implementation and report.
