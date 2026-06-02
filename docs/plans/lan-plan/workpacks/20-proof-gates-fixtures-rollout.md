# 20 Proof Gates, Fixtures, And Rollout

Sources: [20-step plan](../v0-9-lan-discovery-20-step-plan.md),
[test blueprint](../v0-9-lan-discovery-test-blueprint.md),
[UI/UX guide](../ui-ux-requirements-guide.md), and
[folder README](../README.md).

## Where We Are

Existing proof commands cover important local V0.9 LAN behavior, but production
household LAN readiness is still not proven. The pasted test blueprint requires
fixtures, property tests, Playwright coverage, performance gates, manual
validation, and explicit CI gates.

## Where We Want To Be

Every LAN discovery claim has a proof level: implemented, scaffold,
unavailable, degraded, manual-required, or blocked. CI never depends on the
user's real LAN. Real physical household claims require manual two-device
artifacts.

## Requirement Checklist

- [ ] Keep test layout explicit: unit, integration, contract, e2e, fixtures,
      Playwright, security, persistence, performance, and CI validation gates.
- [ ] Add fixture names for ARP, mDNS, SSDP, child-agent, API/SQLite, and UI
      states.
- [ ] Add property tests for merge scoring, evidence update, parser robustness,
      event ordering, and online/offline state.
- [ ] Add proof matrix rows for ARP table, ARP sweep, mDNS, SSDP, IP-only
      merge avoidance, IP change, child confirmation, spoof rejection,
      offline, manual assignment, confidence explanation, and malformed packets.
- [ ] Add visual snapshot proof for Devices/LAN, Activity/Network diagnostics,
      policy network targeting, and any pairing/assignment/trust/ignore surface
      before claiming UI readiness.
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
  Playwright mocked UI tests.
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
