# V0.9 Mobile Controller Discovery Runtime Proof - 2026-05-29

Branch: `codex/v0-9-mobile-controller-discovery-runtime-proof`

## Scope

This checkpoint adds a typed parent-domain read model for the current V0.9
mobile controller and household discovery runtime boundary. It does not upgrade
V0.9 to product-ready household LAN, mobile controller write authority, cloud
relay, Android child-agent parity, iOS Family Controls, signing, stores, or
mobile background behavior.

The proof command composes the existing local Rust service V0.9 LAN proof and
parent mobile shell runtime proof, then parses one structured runtime read model
for:

- explicit discovery states: discovered, pending, paired, revoked, stale,
  offline, and unavailable;
- Android parent mobile observer read-only route state;
- iOS parent mobile controller-takeover manual-required route state;
- controller takeover, release, renew, degraded provider, and failed-unpaired
  transition labels;
- stale/offline selected-device behavior labels;
- physical household LAN, cloud relay, mobile write authority, signing, stores,
  entitlements, and mobile child-agent behavior non-claims.

## Proof Command

```powershell
node scripts/test/v0-9-mobile-controller-discovery-runtime-proof.mjs
```

The command writes:

```text
test-results/v0-9-mobile-controller-discovery-runtime-proof/proof.json
```

It consumes:

```text
test-results/v0-9-production-lan-mobile-controller-proof/proof.json
test-results/v0-9-household-lan-production-discovery-proof/proof.json
test-results/parent-mobile-shell-runtime-proof/proof.json
```

## Honest Boundaries

- Local service discovery proof remains a local real-service proof, not physical
  router discovery.
- Android parent mobile remains observer read-only.
- iOS parent mobile controller takeover remains manual-required.
- Cloud relay remains not implemented.
- Mobile child-agent behavior is not claimed by parent mobile runtime proof.
- Signing, stores, entitlements, notification/background behavior, and real
  mobile device controller authority remain manual-required.

## Manual Upgrade Requirements

Before this proof can upgrade to product-ready V0.9 household LAN or mobile
controller readiness, record real artifacts for:

1. two distinct physical household devices on the same LAN,
2. router and firewall reachability,
3. origin allowlist and wrong-origin rejection on those devices,
4. route selection, takeover, release, revocation, stale/offline, and
   failed-unpaired behavior on those devices,
5. Android and iOS parent mobile package/device observer and controller
   behavior,
6. cloud relay routing, authentication, and storage if cloud relay becomes part
   of the product path.
