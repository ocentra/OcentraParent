# Platforms

The `platforms/` workspace contains mobile/platform package scaffolds and proof
paths. A platform package is not the same as platform product support.

```mermaid
flowchart LR
  Contracts["packages/* domain contracts"]
  Android["platforms/android"]
  IOS["platforms/ios"]
  Rust["Rust child-agent runtime"]
  Proof["manual/CI proof artifacts"]

  Contracts --> Android
  Contracts --> IOS
  Rust --> Android
  Rust --> IOS
  Android --> Proof
  IOS --> Proof
```

## Platform Claim Rule

Every platform capability must say whether it is `done`, `in progress`,
`scaffold-only`, `manual-required`, `degraded`, `blocked`, or `not started`.

Package build or simulator launch proof can support a scaffold claim. It cannot
claim child-device monitoring, background execution, enforcement, app blocking,
network filtering, location, notification delivery, or store readiness.

## Connected Docs

- [Platform expectations](../docs/expectations/platforms.md)
- [Platform deliverables expectations](../docs/expectations/platform-deliverables.md)
- [Mobile agents roadmap expectations](../docs/expectations/roadmap-v6-mobile-agents.md)
- [Product capability checklist](../docs/product-capability-checklist.md)

## Current Gaps

- Android needs real device/emulator proof for privileged behavior.
- iOS needs entitlement, signing, and approved API proof before child-agent
  claims.
- Parent mobile and child mobile are separate products and must not be merged
  into one vague "mobile support" claim.
- Parent mobile shell route-status proof is contract/CI proof only: it keeps
  local service, LAN service, relay, cache, storage, package lifecycle, degraded
  and unavailable LAN AI, and observer/request-first boundaries explicit, but
  real mobile UX, controller authority, signing, stores, and child mobile agent
  runtime support remain unclaimed.
- Parent mobile now has separate Android and iOS scaffold apps under
  `platforms/android/parent` and `platforms/ios/OcentraParentMobile`, with
  package-preview CI kept separate from child-agent package previews.
