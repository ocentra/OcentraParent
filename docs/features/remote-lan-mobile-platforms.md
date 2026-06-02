# Remote, LAN, And Mobile Platforms

## Parent Outcome

Parents can control and observe household devices locally, over LAN, and later
away from home, while Android/iOS/macOS/Linux claims remain honest about real
platform permissions and support.

## Ocentra Requirement

LAN and remote are route/custody problems, not excuses to centralize child
activity data. Mobile support must separate parent app claims from child-agent
claims.

## Roadmap And Expectations

- Roadmap: V0.9 LAN pairing, V2 remote access/cloud relay, V6 mobile agents,
  V8 production hardening.
- Expectations: [LAN pairing](../expectations/lan-pairing.md),
  [cloud](../expectations/cloud.md), [platforms](../expectations/platforms.md),
  [mobile agents](../expectations/roadmap-v6-mobile-agents.md),
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
- The parent service now emits a typed LAN scan summary, merges the local
  child-agent device with matching passive LAN evidence, and keeps passive
  neighbors/router rows separate from controllable child-agent targets.
- Parent desktop Tauri proof now serializes active-controller route,
  observer-read-only, live local-network custody, relay unavailable, cache
  unavailable, and parent-owned storage unavailable states without implementing
  LAN discovery itself.
- Android/iOS package scaffolds and proof gates exist.
- Parent desktop release-support proof separates parent mobile bridge state from
  child Android/iOS agent claims and keeps signing, stores, TestFlight, Play,
  relay, and mobile child-agent parity manual-required or not implemented.
- `mobile-child-agent-capability-proof` now aggregates the Android package,
  storage/protocol, service, permission, privileged, device-gate, and iOS
  entitlement proof states into one platform matrix without upgrading any mobile
  child-agent parity, entitlement, signing, store, or external-transport claim.
- Optional remote relay and full mobile child-agent parity are not complete.

## Current Gap

Physical household proof still needs a second installed child agent, signed LAN
agent hello/heartbeat, assign/rename/ignore flows, and stronger name discovery
such as mDNS/SSDP/router DHCP integration. Optional remote relay, mobile
permissions, Android Device Owner/Accessibility/VPN/DNS/UsageStats proof, iOS
Family Controls/DeviceActivity/Network Extension proof, signing, and store
distribution remain.

## Checklist

- [ ] LAN discovery and pairing. Current proof covers Windows neighbor-table
      inventory, local child-agent hardware inventory, scan summary counts, passive
      neighbor/router separation, and portal target filtering; mDNS/SSDP/router DHCP
      names and second-child-agent pairing proof remain.
- [ ] Trusted registry and revocation.
- [ ] Controller lease and observer read-only state.
- [ ] Route status: local, LAN, relay, cache, stale, offline, unavailable.
- [ ] Optional relay without default child-data custody.
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
      Current release-support proof models this split for handoff/reporting, but
      real parent mobile UX, store signing, and child mobile agent proof remain.

## Next AI Instructions

Never claim "mobile support" as one thing. Split parent mobile, child Android,
child iOS, platform packaging, store signing, route status, and remote custody
in every implementation and report.
