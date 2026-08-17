# LAN Plan Next Actions

<!-- agent-capsule -->

> Agent Capsule
> Plan: `lan-plan`
> Doc: `LAN Plan Next Actions`
> Kind: resume queue and highest-open work.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.

<!-- /agent-capsule -->

This file is the short resume list for the next worker. It is derived from open
workpack/checklist status and does not replace the assigned workpack.

## How to use

1. Confirm the hub assignment and lane.
2. Pick only the assigned workpack from the list below.
3. Open that workpack and exact checklist rows only.
4. Do not claim `DONE` unless the assigned workpack's acceptance/proof rows are
   updated and validation is listed.

## Highest-open workpacks by current remaining work

### WP26 authority gate

WP26 is a partial code draft, not a READY or completion packet. Do not add a
LAN service route or treat local custody/transport code as signer authority.
Route the missing authority through Device Trust WP01's persistent
trusted-device/signer-key registration source, then WP03's one-time parent
`RegisterLanSignerAnchor` ceremony. WP26 remains blocked until both owners
exist and W15/W18/W19 composition is legally reachable. WP02 is conditional
only on a demonstrated private-key/install custody requirement.

- [26 Signed Child Beacon Ingress And Household Mesh Authority Handoff](workpacks/26-signed-child-beacon-ingress-and-household-mesh-authority-handoff.md): open production packet for the real child/runtime peer ingress, W15 household custody, W18 signed hello/heartbeat and transport authority, W19 route/revocation/lease composition, atomic message/idempotency persistence, and the private Eventing WP10 authorization handoff. Code, organized real-ingress tests, and proof are all open; no portal authority, fake transport, or synthetic receiver is allowed.
- [16 Read Models And LAN Events](workpacks/16-read-models-and-lan-events.md): backend stream, parent replay validation, desktop delivery decision, and portal listener/state seams exist and are separately tested. The next exact Phase 1 slice is one integrated backend replay -> real Tauri `AppHandle` emit -> portal-listener regression; richer network-flow and physical/manual evidence follow later.
- [19 Assignment, Revocation, And Audit](workpacks/19-assignment-revocation-audit.md): main-lane route/runtime/UI verification bucket now reduced to broader physical/manual topology artifacts after the restart/readback Rust proof closed locally.
- [20 Proof Gates, Fixtures, And Rollout](workpacks/20-proof-gates-fixtures-rollout.md): restore or replace all six aggregate verifier programs named by current LAN docs; every referenced `v0-9-*lan*`/production-discovery runner is absent from the current tree.
- [23 Pairing And Route Proof](workpacks/23-pairing-and-route-proof.md): locally green Rust route/revoke proof with real two-device/manual topology artifacts still open.
- [25 Rollout Checklist And PR Gate](workpacks/25-rollout-checklist-and-pr-gate.md): final wrapper depends first on the WP16 integrated delivery regression and WP20 executable verifier repair, then on the remaining manual-required LAN proofs.
- [18 Signed Child Hello And Heartbeat](workpacks/18-signed-child-hello-heartbeat.md): local Rust/core closure is green; explicit iOS/manual and second-device physical proof remain open.

## Active note

- Current LAN-plan truth stays Rust-owned: contracts, business logic, read
  models, route snapshots, and runtime proof belong to Rust crates; TS remains
  presentation-only and must not be reintroduced as a contract or test owner
  because of older plan wording.
- `01 Rust-Owned Contract Boundary And Bridge Validation` is now locally proven:
  Rust-owned LAN protocol contracts reject wrong schema versions, reject future
  enum drift, keep signed-child and mDNS contract families explicit, and stay
  covered by focused `agent-protocol` contract tests plus scoped architecture
  lint without adding TS-owned LAN contract truth.
- `16 Read Models And LAN Events` now has a real parent-Rust inventory-backed
  `LanBrowserAddDeviceReadModel` for the product host bridge, forwards
  Devices-route scan actions into `agent-service`, auto-scans the real Tauri
  Devices route into visible LAN inventory, and emits typed Tauri
  host-subscription updates plus subscribed route events into the portal shell
  without a UI WebSocket. The portal now applies subscribed
  `ParentSubscriptionEvent.events` through the same event-buffer path used by
  command responses, and now rejects stale subscribed route snapshots or event
  batches once a newer Rust-backed view is buffered, so subscribed LAN route
  updates no longer drop or regress Rust-owned event history while refreshing
  the latest snapshot.
  Existing focused tests cover `parent-runtime-core` route snapshots, schema
  bridge contracts, portal consumers, desktop delivery decisions, and the
  backend runtime stream separately. They do not exercise a real `AppHandle`
  emission into the portal listener. That integrated regression is the exact
  remaining Phase 1 test gap; broader runtime/manual proof follows later.
- The latest `03` slice lifts gateway, DNS server, DHCP server,
  broadcast-address, and IPv6-prefix capture into the shared Rust
  local-interface shape and persists that interface map into
  `LanDiscoveryScanPlan` plus scan-history metadata under focused `lan-core`
  and `agent-service` proof. Explicit manual interface selection,
  ignored-interface reason-code proof, and selected-interface attribution
  through the relevant discovery-evidence and runtime serialization paths are
  now locally proven.
- The latest `04` slice is now locally code-complete: Windows/Linux/macOS
  neighbor parsing covers IPv4/IPv6 rows, malformed/incomplete/duplicate-row
  normalization, and downstream `observed_at` propagation under green reruns.
  The remaining `04` gap is live macOS/manual platform proof, not another
  local Rust implementation hole.
- The latest `06`/`15` slice keeps passive LAN inventory honest while
  suppressing costly service-identity probes for durable paired,
  child-agent-backed, ignored/revoked, and router truth reconstructed from
  registry JSON plus previous canonical household state. Bounded active refresh
  now suppresses a stored child/paired IP only when live neighbor state still
  confirms the same MAC, so stale IP-only truth cannot hide a reused address.
  Durable truth still feeds passive identity hints for matching neighbors. This
  is not closure for Workpack `11`; safe-port probing and broader bounded
  service-proof remain open.
- The latest `02`/`15` slice now persists canonical `knownHouseholdDevices`
  into trusted-registry JSON, merges evidence `firstSeenAt`/`lastSeenAt`
  across updates, restores that known-device store into the add-device read
  model as stale restart truth under explicit runtime proof, and uses it for
  later scan suppression before falling back to scan-history reconstruction.
  The latest `02` packet also preserves distinct source-backed evidence rows in
  the durable registry and enriches an existing paired-child/router scan-truth
  row instead of emitting a duplicate suppression candidate for the same
  device. `02` is now locally closed for its owned Rust scope after the
  focused `agent-core`, `agent-service`, and scoped architecture reruns.
- `07 Passive Discovery Listeners` now has focused Rust proof for passive ARP
  weak hints, DHCP, mDNS, SSDP, WS-Discovery, LLMNR, NetBIOS, Ocentra beacon
  observations through the signed-child hello or heartbeat path, and allowed
  SNMP response history bridging. The local code path is now fully rerun green;
  the exact remaining gap is real long-running DHCP listener proof plus
  broader packet/platform/manual artifacts.
- `08 mDNS And DNS-SD Discovery` now has focused Rust proof for selected
  service enumeration/types, PTR/SRV/TXT/A/AAAA parsing, hostile-name
  sanitization, and hint-only agent handling. Remaining work is broader
  packet/manual proof only; mDNS alone still does not confirm signed-child
  identity or physical household topology.
- `09 SSDP And UPnP Discovery` now has focused Rust proof for bounded
  `M-SEARCH`, private-only descriptor fetch rules, router-visible or
  non-enrollable handling, malformed or timeout or oversize rejection, and
  safe descriptor parsing. Remaining work is broader packet/manual proof only.
- `10 NetBIOS, LLMNR, And Reverse DNS` is now locally closed for the owned
  Rust slice: hostname evidence stays weak and name-only, malformed or unsafe
  values are rejected, duplicate names stay below auto-merge thresholds, child
  identity is never inferred, and the exact W10 source-matrix validation
  target now exists as a real test entry point.
- `11 Light Service Probing` now has focused Rust proof for bounded
  HTTP/HTTPS/TLS identity probing, selected-interface gating, trusted/router
  suppression, bounded WSD/SNMP queries, and weak-only probe evidence. The one
  honest open box in `11` is optional/manual-gated OS fingerprint proof.
- `13 Merge And De-Duplication Engine` is now locally closed for the owned
  Rust slice: install-id and pairing-id carry through as strong merge
  evidence, the canonical path already emits explicit
  `dedupe-decision/score/reasons` output, and the old local blocker was stale.
- `14 Explainable Classification` is now locally complete for current LAN
  scope: weighted Rust classification, explicit reasons and confidence, router
  or unsupported or unknown states, scanner-only non-child boundaries, focused
  portal label rendering, and the refreshed `/devices` Rust-snapshot proof are
  all green. Broader installability or physical/manual claims remain owned by
  later workpacks.
- `17 Parent And Child mDNS Advertisements` now has focused Rust proof for
  parent/child advertisement contracts, opaque metadata, packet encoding, and
  lifecycle evaluation through agent-service sync tests. Remaining work is
  signed-child confirmation plus broader Android/iOS/macOS manual multicast
  proof.
- `18 Signed Child Hello And Heartbeat` now has focused Rust/core proof for
  verifier rejection states, future-safe capability passthrough, unpaired
  runtime rejection, stale/offline/manual-required projection without deleting
  the device record, and focused portal/manual-required label rendering.
  Remaining work is explicit iOS/manual platform proof plus second-device
  physical proof.
- `19 Assignment, Revocation, And Audit` now has focused Rust/service/portal
  proof for route trust, rename, ignore/restore, restart recovery, route
  select, revoke audit evidence, rename/type refresh persistence, and
  selected-route LAN command routing into the real local-network child target.
  The current Windows `/devices` Playwright route is green. Remaining work is
  broader physical/manual topology artifacts.
- `26 Signed Child Beacon Ingress And Household Mesh Authority Handoff` is now
  the explicit open LAN route for the missing real child/runtime peer ingress.
  It depends on W15/W18/W19, must retain atomic message/idempotency custody and
  fail-closed provider-policy and authority negatives, and unlocks only the
  private typed Eventing WP10 authorization handoff after its own proof exists.
  Eventing does not own LAN transport, and portal/UI does not own authority.
- `05 Targeted ARP Checks` now has real bounded Rust host-refresh support with
  selected-interface/local-subnet gating, response/no-response evidence,
  throttling, packet-IO abstraction coverage, and scan-plan metadata
  persistence. Remaining `05` work is proof/checklist truth closure only.

## PR readiness guard

A PR-ready slice should close a named workpack or clearly explain the exact
remaining rows. Do not create a tiny PR that only updates one proof note while
leaving the assigned workpack/checklist state stale.

Before reporting `DONE` or `PR_READY`, update the workpack, checklist row(s),
proof reference(s), and feature/product docs if product status changed.

## Actioned completion tracker

- [ ] Re-check this plan route from AGENTS/PLAN_STATE and confirm the assigned
      workpack path.
- [ ] Update one assigned workpack and matching checklist/proof rows before
      reporting progress.
- [ ] Record failure conditions, skipped checks, and evidence path in
      PLAN_STATE/TEST_PROOF_EXPECTATIONS for every claimed progress.
- [ ] Confirm every claimed test lives in a real test folder/group; placeholder
      directories, `.gitkeep`, and inline source-owned tests do not count as
      coverage.
