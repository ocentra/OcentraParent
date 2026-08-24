# LAN Plan Workpack Index

<!-- agent-capsule -->

> Agent Capsule
> Plan: `lan-plan`
> Doc: `LAN Plan Workpack Index`
> Kind: authoritative workpack selector.
> Read when: before opening any LAN workpack.
> Stop rule: use only one authoritative workpack from `01-26`.
> Proves: current execution model and workpack routing only.
> Does not prove: row completion or physical proof by itself.
> Proof rule: if a workpack state changes here, the checklist and proof index must match it.

<!-- /agent-capsule -->

Use `WORKPACK_FAMILIES.md` only when the selected workpack owner/proof family
is unclear. Do not use it as permission to scan multiple workpacks.

## Authoritative Execution Workpacks

| ID | State | Next class | Current truth |
| --- | --- | --- | --- |
| `01` | complete | local proof | Rust-owned LAN contract families, fail-closed schema-version guards, explicit signed-child and mDNS contract surfaces, discovery-evidence and route-snapshot payload coverage, and focused protocol contract drift validation are locally green without reintroducing TS-owned contract truth. |
| `02` | complete | local proof | Rust-owned evidence/device-record closure is locally green: the service-backed add-device read model persists canonical known-household records in trusted-registry JSON, preserves merged `firstSeenAt`/`lastSeenAt` timing plus distinct source-backed evidence rows, enriches existing paired-child/router scan-truth without duplicate suppression rows, and reran green on 2026-06-28 across focused `agent-core`, `agent-service`, and scoped architecture validation. |
| `03` | complete | local proof | Shared Rust local-identity selection now captures gateway, DNS, DHCP, broadcast, and IPv6-prefix fields, supports explicit manual interface selection, preserves explicit ignored-interface reasons, and persists selected-interface attribution through scan-plan metadata, discovery evidence, and scan-history/runtime serialization under focused Rust proof. |
| `04` | partial/manual | local proof plus platform/manual proof | The local Rust neighbor-ingestion slice is now code-complete: Windows/Linux/macOS normalization covers IPv4/IPv6 rows, malformed/incomplete/duplicate handling, and normalized `observed_at` propagation into downstream read-model state under green 2026-06-28 reruns. The honest remaining gap is live macOS/manual platform proof, not another local Rust implementation hole. |
| `05` | complete | local proof | Targeted ARP refresh is locally complete as a bounded Rust read-model capability with selected-interface/local-subnet scoping, persisted scan-metadata evidence, focused throttling proof, packet-IO reply handling, and no-false-no-response behavior; it remains weak evidence only and does not create child authority. |
| `06` | implementation gap / source repair authorized | implementation then local proof | The bounded active IPv4 path, gateway/router suppression, current-MAC-confirmed durable suppression, service-probe suppression reuse, and packet-IO coverage exist, but the stimulation-owned targeted-ARP result is discarded before `physical_lan_scan` recollects it. The second collection can be throttled, so persisted scan metadata can miss the actual observation. Implement one owner-correct capture -> normalize/merge -> persist handoff without bypassing rate limits or fabricating evidence; tests, proof, and checklist review remain open. |
| `07` | partial/manual | local proof plus packet/manual proof | Passive ARP, DHCP, mDNS, SSDP, WS-Discovery, LLMNR, NetBIOS, and allowed SNMP-response evidence are locally code-complete as hint-only Rust read-model sources with green 2026-06-28 reruns across LAN-core and agent-service scan-history proof. The exact remaining gap is long-running DHCP listener proof plus broader packet/platform/manual artifacts. |
| `08` | partial/manual | local implementation plus packet proof | Rust-owned mDNS/DNS-SD discovery is locally complete with selected service enumeration/types, PTR/SRV/TXT/A/AAAA parsing, hostile-name sanitization, and hint-only agent handling covered by focused LAN-core tests; only broader packet/manual proof remains open. |
| `09` | partial/manual | local implementation plus packet/manual proof | Rust-owned SSDP/UPnP discovery is locally complete with bounded `M-SEARCH`, safe response parsing, private-only descriptor fetch rules, router-visible/non-enrollable handling, malformed/timeout/oversize rejection, SSDP-backed read-model coverage, and green 2026-06-28 LAN-core plus architecture reruns; only broader packet/manual proof remains open. |
| `10` | complete | local proof | NetBIOS, LLMNR, and reverse DNS are locally complete as weak name-only evidence: malformed/unsafe values are rejected, duplicate-name fixtures stay below auto-merge thresholds, hostname signals never confirm child identity or assignment, the exact W10 source-matrix filter target is restored as a real test entry point, and focused reruns plus scoped architecture validation were green on 2026-06-28. |
| `11` | partial/manual | local proof plus manual gate | Bounded HTTP/HTTPS identity probing, sanitized title/header/redirect/descriptor parsing, TLS subject capture, no-crawl weak evidence, bounded WSD/SNMP identity queries, and passive-history bridging for allowed SNMP replies are locally green in Rust and surface through route snapshots. The one exact remaining blocker is the intentionally unimplemented optional/manual-gated OS-fingerprint proof, plus broader live household/manual proof for the bounded query paths. |
| `12` | complete | local proof | Rust MAC parsing, local OUI/vendor evidence, randomized/private warning, multicast/malformed rejection, read-model warning surfacing, and weak-only confidence downgrade are locally green; richer cross-source classifier closure remains owned by later classification work rather than this vendor-evidence slice. |
| `13` | complete | local proof | Household merge/dedupe is locally complete: install-id and pairing-id carry through protocol, service, inventory, and canonical merge paths as strong evidence; the merge path already emits explicit `dedupe-decision`, `score`, and `reasons` output; and focused unit/property reruns plus scoped architecture validation were green on 2026-06-28. |
| `14` | complete | local proof | Weighted Rust classification, explicit reasons and confidence, router or unsupported or unknown states, non-child scanner-only boundaries, focused portal label rendering, and refreshed `/devices` Rust-snapshot proof are locally green; broader installability or physical/manual claims remain owned by later workpacks. |
| `15` | complete | local proof | Durable household device store/read-model behavior is locally green: trusted-registry JSON persists canonical known-household records, prior-scan continuity and durable scan-suppression truth reuse are wired, active refresh refuses stale IP-only suppression for reused addresses, migration/fail-closed recovery is tested, stale/offline restart read-model recovery is tested, routers stay visible but non-enrollable, and scan-plan sidecar metadata persists. |
| `16` | partial/code-test gap | integrated delivery test then physical/manual proof | LAN read-model and diagnostics code exists; the backend stream, parent replay validator, desktop host-delivery decision, and portal listener/state edge are each covered separately. No test drives a backend replay through a real Tauri `AppHandle` emission into the portal listener, so the complete production delivery chain is not yet an expected-test-complete Phase 1 slice. Richer network-flow evidence and physical/manual artifacts remain later gates. |
| `17` | partial/manual | local proof plus packet/manual proof | Parent/child mDNS advertisement contracts, packet encoding, lifecycle evaluation, goodbye handling, and agent-service sync tests are locally complete and green as hint-only discovery. Signed-child confirmation remains owned by W18, and the only remaining W17 blocker is broader Android/iOS/macOS manual multicast/background proof. |
| `18` | partial/manual | physical/manual proof bound | Local Rust/core signed hello and heartbeat verification is green: fail-closed schema or message drift, replay or expiry or wrong-family or wrong-device rejection, future-safe capability passthrough, unpaired runtime rejection, stale or offline or manual-required read-model projection, and focused portal/manual-required label rendering are proven; remaining open proof is explicit iOS/manual platform evidence plus second-device artifacts. |
| `19` | partial/manual | local proof plus physical/manual proof | Rust-owned assignment/revoke/audit behavior is locally green across route trust, rename, ignore/restore, restart recovery, restart readback of the live child canonical id, route select, revoke audit evidence, portal LAN-target routing, selected-route local-network command dispatch, and the current Windows `/devices` Playwright path; remaining open proof is broader physical/manual topology artifacts, not a local LAN command gap. |
| `20` | partial/code-test gap | executable verifier repair then physical/manual proof | Fixture, property, performance, and portal visual test families exist, but all six LAN aggregate verifier commands named by the current plan docs point to absent scripts. The signed-relay and source-matrix runners are also incorrectly checked complete in WP20. Historical generated output cannot substitute for missing programs, so Phase 1 remains open before physical/manual proof. |

## Follow-On Workpack Truth

These rows are active `lan-plan` follow-on scope. Current row truth is mixed:
`21`, `22`, and `24` are locally complete with their own proof; `23`, `25`, and
`26` remain partial/open. No follow-on row can support PR-ready or completion
claims without its own current Rust-first proof, organized tests where
applicable, and explicit no-claim boundaries.

| ID | State | Current truth |
| --- | --- | --- |
| `21` | complete | `crates/family-identity-core` now carries the Rust-owned household/setup contract family, including the record-shaped `family_identity::RecoveryState`, with organized contract coverage green in `tests/unit/family_identity_contracts.rs` and crate-wide validation green. |
| `22` | complete | The current-state and gap map is now synchronized to live Rust/runtime/UI truth: local code-complete slices, manual-required LAN proofs, and the exact Windows first-run portal proof result are reflected from current validation instead of stale TS-first assumptions. |
| `23` | partial/manual | Rust-owned route-custody, stale/offline, rejection, revoke, and read-model projection proof is locally green across protocol and agent-service tests; remaining gaps are physical two-device/manual topology proof and broader restart/manual artifacts, not local Rust code gaps. |
| `24` | complete | Rust-owned `setupFirstRunPanel` snapshot shape, parent-runtime snapshot builder, portal route wiring, focused unit/runtime tests, portal build, and the exact Windows `setup-first-run-ui-proof` Playwright command are green; broader physical/manual LAN closure remains owned by sibling workpacks, not this portal slice. |
| `25` | partial/code-test gap | Rust replay parsing/status binding, the host delivery decision, and isolated portal-state consumption are separately covered by focused protocol, service, runtime, desktop, and portal tests. Phase 1 remains open because the real backend-to-Tauri-`AppHandle`-to-portal-listener regression and WP20's aggregate verifier programs are missing. Physical multi-device, router/firewall, signed-artifact, restart, and other manual topology/runtime artifacts remain later gates. |
| `26` | blocked — partial code draft / test/proof gap | A bounded custody/registry/transport draft exists, but no shipped signer-registration route or completion claim exists. WP26 must compose W15 household custody, W18 signed hello/heartbeat trust and transport authority, and W19 route/revocation/lease authority, then provide only a private typed Eventing WP10 authorization handoff after Device Trust WP01 persistent registration and WP03 one-time `RegisterLanSignerAnchor` authorization. The canonical proof root is `output/lan-plan-proof/26-signed-child-beacon-ingress-and-household-mesh-authority-handoff/`; no portal authority, fake transport, or synthetic receiver is accepted. |

## Selection Rules

- Select exactly one authoritative workpack from `01-26`.
- If the selected workpack owner/proof family is unclear, classify it through
  `WORKPACK_FAMILIES.md`; do not scan every family.
- Do not raise status from B1/B2 proof outside their explicit non-claims.
- Do not use legacy TypeScript package proof surfaces as current contract,
  read-model, or runtime ownership.
- Do not treat `21-26` as parked/future scope. They are active follow-on scope
  and must use the current row truth above.
- Do not mark any `21-26` row complete or PR_READY without that row's own
  current Rust-first row truth, organized tests where applicable, and proof
  artifacts.
- Real LAN closure must live in organized Rust crate `tests/` folders. UI
  presentation tests may exist only as supporting checks and never as LAN proof
  authority. Inline source-owned tests, placeholder directories, `.gitkeep`
  trees, fake coverage, and mock-only readiness do not count as closure.
- Do not claim physical household, portal, service/runtime, signed
  hello/heartbeat, router/firewall, Android/mobile, or relay readiness from
  schema, unit, source-matrix, B1, or B2 proof alone.
