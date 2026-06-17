# 20 Proof Gates, Fixtures, And Rollout

<!-- agent-capsule -->

> Agent Capsule
> Plan: `lan-plan`
> Doc: `20 Proof Gates, Fixtures, And Rollout`
> Kind: proof reference; read only when validating matching claim.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

Sources: [20-step plan](../v0-9-lan-discovery-20-step-plan.md),
[test blueprint](../v0-9-lan-discovery-test-blueprint.md),
[UI/UX guide](../ui-ux-requirements-guide.md), and
[folder README](../README.md).

## Where We Are

Prior proof references for this workpack drifted onto stale `parent-domain`,
portal, and service-backed paths that were not current proof on this
branch/worktree. `B1` repaired the local proof chain back to authoritative
`packages/lan-domain` ownership and regenerated a fresh proof pack under:

`output/lan-plan-proof/01-lan-b1-proof-regeneration/`

That local proof pack is green for source-matrix fencing, signed discovery
relay/manual boundaries, production discovery state handling, and the explicit
`not-ready-for-product-ready-household-lan-claim` readiness decision. It does
not finish portal proof, service/runtime proof, the full fixture family, visual
snapshot proof, or manual two-device household validation.

## Where We Want To Be

Every LAN discovery claim has a proof level: implemented, scaffold,
unavailable, degraded, manual-required, or blocked. CI never depends on the
user's real LAN. Real physical household claims require manual two-device
artifacts.

## B1 Truth Note - 2026-06-17

`B1` is green for the assigned local proof-regeneration scope only.

- Authoritative execution scope remains `01-20`; frozen follow-on `21-25` is
  still out of completion scope.
- Current proof owner for this slice is `packages/lan-domain`, not
  `packages/parent-domain/src/lan-*`.
- Exact regenerated artifacts:
  - `output/lan-plan-proof/01-lan-b1-proof-regeneration/01-lan-source-matrix-plan-completion-proof.json`
  - `output/lan-plan-proof/01-lan-b1-proof-regeneration/02-lan-signed-discovery-relay-spine-proof.json`
  - `output/lan-plan-proof/01-lan-b1-proof-regeneration/03-production-discovery-household-proof.json`
  - `output/lan-plan-proof/01-lan-b1-proof-regeneration/04-household-lan-proof-readiness.json`
- Exact remaining non-claims:
  - portal screenshot/render proof
  - service/runtime-backed LAN source proof
  - real signed child hello and heartbeat artifacts
  - real two-device household LAN proof
  - router/firewall/local-network permission proof
  - cloud relay routing/storage/auth proof

## Requirement Checklist

- [x] `scripts/test/v0-9-lan-signed-discovery-relay-spine.mjs` proves the
      signed-discovery relay spine remains coherent for the local `B1`
      `lan-domain` slice. Latest local evidence is
      `output/lan-plan-proof/01-lan-b1-proof-regeneration/02-lan-signed-discovery-relay-spine-proof.json`.
- [x] `scripts/test/v0-9-lan-source-matrix-plan-completion.mjs` proves the
      authoritative `01-20` source-matrix/read-model contract remains coherent
      for the local `B1` slice. Latest local evidence is
      `output/lan-plan-proof/01-lan-b1-proof-regeneration/01-lan-source-matrix-plan-completion-proof.json`.
- [x] Feature docs updated by this branch keep real physical household proof,
      relay/cache, mobile parity, signing, and store readiness as remaining
      gaps.
- [ ] Portal tests cover LAN UI-intent consumption and live activity parser
      preservation of signed-proof, route-custody, relay/cache unavailable,
      manual-proof, and parent-decision fields.
- [ ] Keep test layout explicit: unit, integration, contract, e2e, fixtures,
      Playwright, security, persistence, performance, and CI validation gates.
- [ ] Add fixture names for ARP, mDNS, SSDP, child-agent, API/SQLite, and UI
      states.
- [ ] Add property tests for merge scoring, evidence update, parser robustness,
      event ordering, and online/offline state.
- [ ] Add proof matrix rows for ARP table, ARP sweep, mDNS, SSDP, IP-only
      merge avoidance, IP change, child confirmation, spoof rejection,
      offline, manual assignment, confidence explanation, and malformed packets.
- [ ] Add a source-matrix proof row family for all 20 workpacks and discovery
      sources. The matrix is intentionally status-based: weak/manual sources
      are visible but cannot confirm child-agent identity or assign a child
      profile.
- [ ] Add visual snapshot proof for Devices/LAN, Activity/Network diagnostics,
      policy network targeting, and the current pairing/assignment/trust/ignore
      action surface. Previously cited Playwright paths are absent on this
      branch/worktree and are not current proof; see `../PROOF_INDEX.md`.
- [ ] Keep manual validation artifacts for Windows, macOS, Linux, router,
      Windows laptop, MacBook, Linux machine, iPhone, Android phone, TV,
      printer, Chromecast/Google TV, console where available, and child agent.

## Required Fixture Families

- ARP: `linux_proc_net_arp_basic.txt`, `linux_proc_net_arp_empty.txt`,
  `linux_proc_net_arp_incomplete.txt`, `macos_arp_a_basic.txt`,
  `windows_neighbor_table_basic.json`, `arp_reply_router.bin`,
  `arp_reply_phone_private_mac.bin`, `arp_malformed_short.bin`.
- mDNS: `iphone_mdns_response.bin`, `android_mdns_response.bin`,
  `chromecast_mdns_response.bin`, `printer_mdns_response.bin`,
  `ocentra_agent_mdns_response.bin`, `malformed_mdns_random.bin`,
  `oversized_mdns.bin`.
- SSDP: `samsung_tv_ssdp.txt`, `router_ssdp.txt`, `xbox_ssdp.txt`,
  `printer_ssdp.txt`, `bad_location_ssdp.txt`, `missing_location_ssdp.txt`,
  `device_description_tv.xml`, `device_description_router.xml`,
  `bad_device_description.xml`.
- Child agent: `valid_child_hello_windows.json`,
  `valid_child_hello_android.json`, `invalid_signature_hello.json`,
  `wrong_family_hello.json`, `expired_hello.json`,
  `replayed_nonce_hello.json`, `missing_device_id_hello.json`,
  `unknown_future_version_hello.json`.
- UI: `empty_devices.json`, `unknown_apple_device.json`,
  `confirmed_windows_child.json`, `same_ip_different_mac.json`,
  `same_device_new_ip.json`, `long_hostname_device.json`,
  `html_in_hostname_device.json`.

## Acceptance And Proof

- CI gates include format, schema-boundary checks, Rust clippy/tests,
  TypeScript unit/integration/contract tests, security parser tests, and
  Playwright fixture-backed UI tests.
- Source-matrix proof gates must keep weak/manual/not-implemented source rows
  visible in product diagnostics instead of upgrading them to implemented in
  docs only.
- Manual review gates include current UI screenshots or browser snapshots so
  visible product behavior can be checked before relying on tests alone.
- Coverage targets: core model, merge, classifier, and security logic at 90
  percent or better; protocol parsers at 80 percent or better; critical UI
  flows covered by Playwright.
- Performance gates cover neighbor read under 100 ms, `/24` ARP packet build
  under 50 ms, merge/classify 256 devices under 100 ms each, load 1000 stored
  devices under 500 ms, and UI render of 100 devices without freezing.

## Parallel Ownership Notes

This is the final gate workpack. It can run beside implementation as a proof
owner, but it must not mark a workpack done unless the exact proof artifacts
exist.
