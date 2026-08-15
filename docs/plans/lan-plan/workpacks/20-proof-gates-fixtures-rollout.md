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

Prior proof references for this workpack drifted onto stale `parent-domain`, portal, and service-backed paths that were not current proof on this branch/worktree. `B1` repaired the local proof chain back to the then-current authoritative proof slice and regenerated a fresh proof pack under:

`output/lan-plan-proof/01-lan-b1-proof-regeneration/`

That local proof pack is green for source-matrix fencing, signed discovery relay/manual boundaries, production discovery state handling, and the explicit `not-ready-for-product-ready-household-lan-claim` readiness decision. Main-lane reruns now also prove the current `01-25` source-matrix contract plus real Playwright visual proof for `/devices`, policy-target persistence, and Activity/Network evidence rendering. The current tree also has real LAN portal/live-activity tests plus named LAN fixture families in organized test folders. It still does not finish the remaining property/proof-matrix breadth or manual two-device household validation.

## Where We Want To Be

Every LAN discovery claim has a proof level: implemented, scaffold, unavailable, degraded, manual-required, or blocked. CI never depends on the user's real LAN. Real physical household claims require manual two-device artifacts.

## Ownership boundary

```text
WP20 aggregates LAN proof roots only.
Adjacent plans own their implementation and may be referenced only by typed handoff proof.
WP20 cannot convert local proof, stale proof paths, placeholder tests, or adjacent open follow-on work into broad readiness.
```

## Required rollout artifact fields

The rollout artifact must name, at minimum:

```text
rollout_id
accepted_proof_roots
missing_proof_roots
carried_blockers
manual_required_gaps
physical_topology_notes
portal_projection_state
service_runtime_state
signed_child_hello_state
heartbeat_state
router_firewall_state
android_mobile_state
relay_state
active_follow_on_workpack_boundary
product_claims_allowed
product_claims_blocked
no_claim
```

These are proof-routing fields, not implementation code prescriptions.

## B1 Truth Note - 2026-06-17

`B1` is green for the assigned local proof-regeneration scope only.

- Authoritative execution scope is `01-25`; follow-on workpacks `21-25` cannot be counted complete without their own current proof and test truth. Current row truth closes `21`, `22`, and `24` locally, while `23` and `25` remain partial/manual.
- Current proof owner for this slice is the Rust-owned LAN execution path, not historical TypeScript domain packages.
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

- [ ] Restore or replace `scripts/test/v0-9-lan-signed-discovery-relay-spine.mjs`. The workpack previously marked this runner complete, but it is absent from the current repository; historical generated output is not executable verifier source.
- [ ] Restore or replace `scripts/test/v0-9-lan-source-matrix-plan-completion.mjs`. The workpack previously marked this runner complete, but it is absent from the current repository; historical `output/` and `test-results/` files cannot close the code/test gap.
- [x] Feature docs updated by this branch keep real physical household proof, relay/cache, mobile parity, signing, and store readiness as remaining gaps.
- [x] Portal tests cover LAN UI-intent consumption and live activity parser preservation of signed-proof, route-custody, relay/cache unavailable, manual-proof, and parent-decision fields. Current live coverage exists in `apps/portal/tests/unit/activity-ui-intent.test.ts`, `apps/portal/tests/live-activity/`, `apps/portal/e2e/portal-ui.spec.ts`, and `apps/portal/e2e/lan-source-matrix-visual-proof.spec.ts`.
- [x] Keep test layout explicit: unit, integration, contract, e2e, fixtures, Playwright, security, persistence, performance, and CI validation gates. Current LAN-owned coverage now lives in explicit test roots such as `crates/lan-core/tests/{fixtures,property-based,unit}`, `crates/agent-service/tests/unit`, `apps/portal/tests/{browser,fixtures,live-activity,unit}`, and `apps/portal/e2e/`.
- [x] Add fixture names for ARP, mDNS, SSDP, child-agent, API/SQLite, and UI states. Current LAN fixture families exist under `crates/lan-core/tests/fixtures/lan-plan/` and `apps/portal/tests/fixtures/`.
- [x] Add property tests for merge scoring, evidence update, parser robustness, event ordering, and online/offline state. The organized property test source exists; the previously named generated `output/lan-plan-proof/20-proof-gates-fixtures-rollout/01-property-test-breadth.md` artifact is absent and must be regenerated in Phase 3.
- [ ] Regenerate proof matrix rows for ARP table, ARP sweep, mDNS, SSDP, IP-only merge avoidance, IP change, child confirmation, spoof rejection, offline, manual assignment, confidence explanation, and malformed packets. The previously named generated `output/lan-plan-proof/20-proof-gates-fixtures-rollout/02-proof-matrix-breadth.md` artifact is absent from the current checkout.
- [x] Add a source-matrix proof row family for all 25 workpacks and discovery sources. The matrix is intentionally status-based: weak/manual sources are visible but cannot confirm child-agent identity or assign a child profile.
- [ ] Regenerate visual snapshot proof for Devices/LAN, Activity/Network diagnostics, policy network targeting, and the current pairing/assignment/trust/ignore action surface. The previously named `test-results/v0-9-lan-source-matrix-plan-completion/` and `output/playwright/lan-source-matrix-plan-completion/` roots are absent from the current clean checkout.
- [ ] Keep manual validation artifacts for Windows, macOS, Linux, router, Windows laptop, MacBook, Linux machine, iPhone, Android phone, TV, printer, Chromecast/Google TV, console where available, and child agent. The previously named generated Windows and Linux/WSL paths are absent from the current checkout; all current manual artifacts must be regenerated and reviewed in Phase 3.

## Required Fixture Families

- ARP: `linux_proc_net_arp_basic.txt`, `linux_proc_net_arp_empty.txt`, `linux_proc_net_arp_incomplete.txt`, `macos_arp_a_basic.txt`, `windows_neighbor_table_basic.json`, `arp_reply_router.bin`, `arp_reply_phone_private_mac.bin`, `arp_malformed_short.bin`.
- mDNS: `iphone_mdns_response.bin`, `android_mdns_response.bin`, `chromecast_mdns_response.bin`, `printer_mdns_response.bin`, `ocentra_agent_mdns_response.bin`, `malformed_mdns_random.bin`, `oversized_mdns.bin`.
- SSDP: `samsung_tv_ssdp.txt`, `router_ssdp.txt`, `xbox_ssdp.txt`, `printer_ssdp.txt`, `bad_location_ssdp.txt`, `missing_location_ssdp.txt`, `device_description_tv.xml`, `device_description_router.xml`, `bad_device_description.xml`.
- Child agent: `valid_child_hello_windows.json`, `valid_child_hello_android.json`, `invalid_signature_hello.json`, `wrong_family_hello.json`, `expired_hello.json`, `replayed_nonce_hello.json`, `missing_device_id_hello.json`, `unknown_future_version_hello.json`.
- UI: `empty_devices.json`, `unknown_apple_device.json`, `confirmed_windows_child.json`, `same_ip_different_mac.json`, `same_device_new_ip.json`, `long_hostname_device.json`, `html_in_hostname_device.json`.

## Acceptance And Proof

- CI gates include format, schema-boundary checks, Rust clippy/tests, generated bridge drift checks where applicable, security parser tests, and Playwright fixture-backed UI tests.
- Source-matrix proof gates must keep weak/manual/not-implemented source rows visible in product diagnostics instead of upgrading them to implemented in docs only.
- Manual review gates include current UI screenshots or browser snapshots so visible product behavior can be checked before relying on tests alone.
- Coverage targets: core model, merge, classifier, and security logic at 90 percent or better; protocol parsers at 80 percent or better; critical UI flows covered by Playwright.
- Performance gates cover neighbor read under 100 ms, `/24` ARP packet build under 50 ms, merge/classify 256 devices under 100 ms each, load 1000 stored devices under 500 ms, and UI render of 100 devices without freezing.
- Runtime truth, read models, proof truth, and contract ownership remain Rust-owned throughout WP20. TS/UI proof only consumes those Rust-backed snapshots.

The 2026-08-15 code-first audit therefore classifies WP20 as Phase 1
incomplete. Existing fixture/property/performance/visual tests are retained,
but all six aggregate verifier commands named by the current LAN docs point to
absent scripts. Those programs must be restored or replaced before Phase 2 or
proof regeneration.

## Failure conditions

- WP20 claims readiness while upstream proof roots are absent and not carried as explicit blockers.
- B1/B2 proof is used as portal, service/runtime, signed hello/heartbeat, physical household, router/firewall, Android/mobile, or relay proof.
- Follow-on workpacks `21-25` are skipped, auto-closed, or treated as implied completion scope without their own proof.
- Placeholder test folders, `.gitkeep`, empty test trees, inline source-owned
  tests, fake coverage, or mock-only readiness claims are counted as coverage.
- Stale `docs/proof/lan-plan/` or absent Playwright output paths are cited as current proof.
- Single-machine proof is treated as real household LAN proof.

## Parallel Ownership Notes

This is the final gate workpack. It can run beside implementation as a proof owner, but it must not mark a workpack done unless the exact proof artifacts exist.
