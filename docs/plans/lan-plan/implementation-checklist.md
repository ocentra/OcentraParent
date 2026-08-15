# LAN Plan Implementation Checklist

<!-- agent-capsule -->

> Agent Capsule
> Plan: `lan-plan`
> Doc: `LAN Plan Implementation Checklist`
> Kind: workpack truth table.
> Read when: after `PLAN_STATE.md` and `WORKPACK_INDEX.md`, or when a worker must record exact row truth.
> Stop rule: do not treat historical checkbox math as authoritative progress.
> Proves: current row truth, current slice evidence, and remaining gates only.
> Does not prove: final completion of open workpacks or physical household readiness.
> Proof rule: rows may move only when code, tests, and proof artifacts exist or when an explicit open/manual-required note is recorded.

<!-- /agent-capsule -->

## Checklist Interpretation

- This file is truth-synced on 2026-06-28.
- Historical checkbox counts and historical `[~]` progress markers are not
  authoritative.
- `01-25` are the live execution rows for the active LAN plan scope.
- `21-25` are active follow-on rows with mixed current state. Rows `21`, `22`,
  and `24` are locally complete with their own proof; rows `23` and `25`
  remain partial/manual. No row counts as PR-ready without its own current
  Rust-first ownership, organized tests where applicable, and proof.
- Rust-owned schema/protocol/service/runtime crates are authoritative for
  contracts, business logic, read models, and route snapshots. TS is
  presentation-only.
- Legacy `packages/lan-domain` evidence is historical reconciliation input
  only; it does not re-own current LAN truth and must not be used as a forward
  execution target.
- Placeholder test folders and `.gitkeep` files never count as coverage; use
  real crate/package test groups.
- Forward LAN closure requires organized Rust crate `tests/` groups for logic
  and proof, with TS test ownership limited to real UI/presentation surfaces.

## Slice A Evidence

`Slice A` is green for the assigned reconciliation scope.

- Proof root: `output/lan-plan-proof/00-plan-model-reconciliation/`
- Source snapshot: `output/lan-plan-proof/00-plan-model-reconciliation/00-source-snapshot.md`
- Validation log: `output/lan-plan-proof/00-plan-model-reconciliation/01-lan-domain-validation.log` (legacy package validation; not current ownership authority)
- Truth-sync notes: `output/lan-plan-proof/00-plan-model-reconciliation/02-plan-truth-sync.md`
- Missing proof inventory: `output/lan-plan-proof/00-plan-model-reconciliation/03-missing-proof-inventory.md`

## B1 Evidence

`B1` is green for the assigned local proof-regeneration scope only.

- Proof root: `output/lan-plan-proof/01-lan-b1-proof-regeneration/`
- Source matrix proof: `output/lan-plan-proof/01-lan-b1-proof-regeneration/01-lan-source-matrix-plan-completion-proof.json`
- Signed relay proof: `output/lan-plan-proof/01-lan-b1-proof-regeneration/02-lan-signed-discovery-relay-spine-proof.json`
- Production discovery proof: `output/lan-plan-proof/01-lan-b1-proof-regeneration/03-production-discovery-household-proof.json`
- Readiness decision proof: `output/lan-plan-proof/01-lan-b1-proof-regeneration/04-household-lan-proof-readiness.json`

## Authoritative Workpack Truth

| Workpack | Current state | Closability after Slice A | Evidence now | Remaining gate |
| --- | --- | --- | --- | --- |
| `01` | complete | local proof | Rust-owned LAN contract families, fail-closed schema-version guards, explicit signed-child and mDNS contract surfaces, discovery-evidence and route-snapshot payload coverage, and focused protocol contract drift validation are locally green without reintroducing TS-owned contract truth. | no remaining local Rust/test blocker in the owned W01 slice |
| `02` | complete | local proof | service-backed LAN add-device read model, canonical known-device persistence, merged evidence timing, distinct source-backed evidence rows, and paired-child/router scan-truth enrichment reran green on 2026-06-28 across focused `agent-core`, `agent-service`, and scoped architecture validation | no remaining local Rust/test blocker in the owned W02 slice |
| `03` | complete | local proof | shared Rust local-identity selection now captures gateway/DNS/DHCP/broadcast/IPv6-prefix fields, manual interface choice, ignored-interface reasons, and selected-interface attribution through scan-plan and history serialization under focused proof | no local code gap; later work owns broader household/manual proof |
| `04` | partial/manual | local proof plus platform/manual proof | Windows/Linux/macOS neighbor normalization is now locally code-complete with IPv4/IPv6 parsing, malformed/incomplete/duplicate handling, and downstream `observed_at` propagation under green 2026-06-28 reruns | live macOS/manual platform proof |
| `05` | complete | local proof | targeted ARP refresh is locally complete as bounded Rust host-refresh with selected-interface/local-subnet scoping, persisted scan metadata, throttling proof, and packet-IO reply handling | no local code gap; later work owns broader household/manual proof |
| `06` | complete | local proof | bounded active IPv4 stimulation is locally complete with gateway/router suppression, durable active-refresh and service-probe suppression truth reuse, scan-plan metadata persistence, and duplicate-reply proof | no local code gap; later work owns broader household/manual proof |
| `07` | partial/manual | local proof plus packet/manual proof | passive ARP/DHCP/mDNS/SSDP/WS-Discovery/LLMNR/NetBIOS/SNMP-response hint evidence now surfaces through Rust read models and passive history, and the local code path reran green on 2026-06-28 | long-running DHCP listener proof plus broader packet/platform/manual artifacts |
| `08` | partial/manual | local implementation plus packet proof | Rust-owned mDNS/DNS-SD discovery is locally complete with bounded service enumeration, PTR/SRV/TXT/A/AAAA parsing, hostile-name sanitization, and hint-only agent handling covered by focused tests | broader packet/manual proof |
| `09` | partial/manual | local implementation plus packet proof | Rust-owned SSDP/UPnP discovery is locally complete with bounded `M-SEARCH`, private-only descriptor fetch rules, router-visible/non-enrollable handling, and malformed/timeout/oversize rejection | broader packet/manual proof |
| `10` | complete | local proof | NetBIOS/LLMNR/reverse-DNS name recovery is locally proven as weak name-only evidence, does not upgrade child identity, and now has a real scoped source-matrix validation target again under green 2026-06-28 reruns | no remaining local weak-name Rust/test blocker in the owned W10 slice |
| `11` | partial/manual | local proof plus manual gate | bounded HTTP/HTTPS identity probing, TLS subject capture, bounded WSD/SNMP identity queries, and passive-history bridging for allowed SNMP replies exist in Rust and reran green on 2026-06-28 across the owned Rust/protocol surfaces | optional/manual-gated OS-fingerprint proof plus broader live household/manual proof |
| `12` | complete | local proof | Rust MAC parsing, OUI/vendor evidence, randomized/private warnings, multicast/malformed rejection, and weak-only confidence downgrade are locally green | no local code gap; later classifier work owns richer cross-source closure |
| `13` | complete | local proof | household-device spine/source merge now carries install-id and pairing-id as strong evidence through protocol, service, inventory, and canonical merge paths, and the current merge path already emits explicit `dedupe-decision`, `score`, and `reasons` outputs under green 2026-06-28 reruns | no remaining local Rust/test blocker in the owned W13 slice |
| `14` | complete | local proof | weighted Rust classification, explicit reasons/confidence, router/unsupported/unknown states, non-child boundaries, focused portal labels, and refreshed `/devices` Rust-snapshot proof are locally green | broader installability or physical/manual claims remain later work |
| `15` | complete | local proof | durable device-store/read-model behavior is locally green with trusted-registry persistence, continuity and scan-suppression truth reuse, stale/offline restart recovery, router visibility without enrollability, and scan-plan sidecar metadata persistence | no local code gap; real household/manual proof remains later work |
| `16` | partial/code-test gap | integrated delivery test then physical/manual proof | Backend stream, parent replay validation, desktop host delivery decision, and portal listener/state seams are separately implemented and tested | add one backend replay -> real Tauri `AppHandle` emit -> portal listener integration regression; richer network-flow and physical/manual proof follows later |
| `17` | partial/manual | local proof plus packet/manual proof | parent/child mDNS advertisement contracts, packet encoding, lifecycle evaluation, goodbye handling, and agent-service sync tests are locally complete and green as hint-only discovery | signed-child confirmation plus broader Android/iOS/macOS manual multicast/background proof |
| `18` | partial/manual | physical/manual-bound for final closure | local Rust/core signed hello and heartbeat verification is green across fail-closed drift, replay/expiry/wrong-family rejection, manual-required projections, focused portal rendering, and green 2026-06-28 reruns across `lan-core`, `agent-service`, `agent-protocol`, and scoped architecture validation | explicit iOS/manual platform evidence plus second-device artifacts |
| `19` | partial/manual | local proof plus physical/manual proof | assignment/revoke/audit behavior is locally green across route trust, rename, ignore/restore, restart recovery, restart readback of the live child canonical id, route select, revoke evidence, selected-route LAN command dispatch, and the current `/devices` Playwright path | broader physical/manual topology artifacts |
| `20` | partial/code-test gap | executable verifier repair then physical/manual proof | fixture/property/performance/visual test families exist, but historical generated roots are not verifier source | restore or replace all six absent aggregate runner programs named by current LAN docs, then continue to physical/manual artifacts |

## Active Follow-On Rows

| Workpack | State | Reason |
| --- | --- | --- |
| `21` | complete | Rust-first LAN contract boundary truth is synced to the current family/schema owner and focused validation |
| `22` | complete | LAN state/gap map is synced to current Rust/runtime/UI truth instead of stale TS-first assumptions |
| `23` | partial/manual | current pairing/route proof truth is locally green across Rust/service/read-model tests; physical/manual topology artifacts remain open |
| `24` | complete | household/setup/account handoff truth is green on the current Windows portal/runtime proof path; TS remains presentation-only |
| `25` | partial/code-test gap | Rust-owned backend replay validation, bridge projection, redacted rejection diagnostics, host delivery decision, and portal-state seams exist separately. Phase 1 remains open on the integrated `AppHandle`-to-listener regression and WP20's absent aggregate verifiers; tracked proof and physical/manual topology remain later gates. |

Rows `21-25` remain active scope. `23` is manual-proof open; `25` has a Phase 1
code/test gap plus later manual proof.
Keep all follow-on rows aligned to the current Rust-first ownership and
organized-test model before using them for implementation, completion, or PR
gates.
