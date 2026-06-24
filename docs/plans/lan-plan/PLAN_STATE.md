# LAN Plan State

<!-- agent-capsule -->

> Agent Capsule
> Plan: `lan-plan`
> Doc: `LAN Plan State`
> Kind: current executable status and open gaps.
> Read when: first, before opening workpacks or proof paths.
> Stop rule: do not widen into frozen follow-on workpacks from here.
> Proves: current plan model, current slice status, and next execution route only.
> Does not prove: final completion of open workpacks, physical household proof, or sibling plan completion.
> Proof rule: any status claim here must point at an existing artifact or an explicit open/manual-required gap.

<!-- /agent-capsule -->

## Current State

- Plan state: active
- Authoritative execution model: `01-20`
- Frozen follow-on only: `21-25`
- Current completed slices: `Slice A`, `B1`, `B2`
- Slice A evidence root: `output/lan-plan-proof/00-plan-model-reconciliation/`
- B1 evidence root: `output/lan-plan-proof/01-lan-b1-proof-regeneration/`
- B2 evidence root: `output/lan-plan-proof/02-lan-b2-test-truth-repair/`

## Current ownership interpretation

```text
schema-domain:
  Canonical shared LAN pairing, discovery, source-matrix, read-model, signed hello, heartbeat, assignment, revocation, and audit shapes when those shapes cross package, crate, app, or plan boundaries.

lan-domain:
  Current package-level LAN metadata and packet-local proof consumer surface. This is the current TypeScript proof surface, not the full runtime/packet owner for every LAN claim.

agent-protocol and agent-service:
  Rust protocol/service/read-model proof only when the selected workpack names those surfaces.

portal-domain and apps/portal:
  Projection and UI only. Portal renders service-backed LAN state and does not own LAN truth.

eventing-plan:
  Local event bus semantics only. Eventing does not own LAN transport, discovery, route authority, or physical topology proof.

account-identity-family-plan and device-trust-bootstrap-plan:
  Household/actor authority and trusted-device/key material owners.

remote-access-plan:
  Relay and remote-access transport owner.

parent/child runtime distribution plans:
  Package, installer, signed child package, and child-agent distribution owners.
```

## Current coupling risks

```text
- `packages/lan-domain` is the current package-level proof surface, but canonical LAN shapes live in schema-domain.
- Unit tests are not integration/e2e/security/physical/load proof.
- Single-machine proof is not real two-device household proof.
- Schema/contract proof is not packet/runtime proof.
- Source-matrix proof is not physical discovery proof.
- Portal rendering proof is not LAN truth proof.
- B1/B2 proof is not signed hello/heartbeat, service/runtime, portal, physical household, router/firewall, Android/mobile, or relay proof.
- Frozen workpacks 21-25 are not part of current completion scope.
```

## Agentless evidence-fusion adoption

As of 2026-06-23, the plan absorbs the agentless LAN design note into the
current authoritative `01-20` model instead of creating a parallel discovery
plan.

```text
W03 owns the richer interface map: local IP, subnet, default gateway, DNS server, DHCP server, broadcast address, and IPv6 prefixes.
W04 owns cross-platform neighbor normalization, including IPv6/NDP truth when exposed by the host.
W07 owns passive collector expansion: ARP, DHCP, mDNS, SSDP, WS-Discovery, LLMNR, NetBIOS, and allowed SNMP response evidence.
W09 owns bounded descriptor parsing from SSDP/UPnP metadata.
W11 owns curated TCP/UDP service probing with sanitized HTTP/TLS/banner hints, bounded WSD/SNMP identity queries where allowed, optional OS-fingerprint proof gates, and strict no-crawl behavior.
W12 owns OUI/vendor evidence plus randomized/private MAC suspicion.
W14 owns weighted evidence-fusion classification, explicit reasons/confidence, and install-eligibility honesty.
W15 owns persisted prior-scan continuity snapshots that can strengthen stale/offline and merge confidence without becoming permanent truth.
```

Hard rules adopted from that note:

```text
- agentless LAN discovery is evidence fusion, not platform certainty
- MAC vendor alone cannot claim Windows, Android, iOS, or child ownership
- open ports, banners, titles, redirects, or certificates cannot confirm child-agent identity
- ICMP reachability is optional only; ARP/NDP, neighbor tables, advertisements, and bounded service evidence remain primary
- visible classification and installability claims must carry reasons or stay unknown/manual-required
```

## Current proof interpretation

```text
Slice A proves plan-model reconciliation only.
B1 proves local lan-domain proof-regeneration only.
B2 proves LAN test-category truth only.
The only populated LAN test category is packages/lan-domain/tests/unit.
Placeholder directories do not count as test coverage.
Physical household LAN readiness needs real multi-device/manual artifacts.
Portal and downstream consumers need source/service-backed proof artifacts.
```

## Slice A Status

`Slice A` is green as of 2026-06-17 for the scope that was actually assigned:

- LAN package export/ownership repair in `packages/lan-domain`
- focused proof-schema test repair
- full `@ocentra-parent/lan-domain` test pass
- full `@ocentra-parent/lan-domain` build pass
- full `packages/lan-domain` architecture pass
- plan truth-sync for the authoritative `01-20` model
- honest proof-root bootstrap for this reconciliation slice

Exact evidence:

- `output/lan-plan-proof/00-plan-model-reconciliation/00-source-snapshot.md`
- `output/lan-plan-proof/00-plan-model-reconciliation/01-lan-domain-validation.log`
- `output/lan-plan-proof/00-plan-model-reconciliation/02-plan-truth-sync.md`
- `output/lan-plan-proof/00-plan-model-reconciliation/03-missing-proof-inventory.md`

## B1 Status

`B1` is green as of 2026-06-17 for the assigned local proof-regeneration scope only.

- Repaired proof scripts now read current `packages/lan-domain` ownership rather than stale `parent-domain`, portal, or service-backed proof paths.
- `@ocentra-parent/lan-domain` tests are green.
- `packages/lan-domain` architecture validation is green.
- The regenerated proof chain ends in `not-ready-for-product-ready-household-lan-claim`, which is the correct local mechanical result for this slice.

Exact evidence:

- `output/lan-plan-proof/01-lan-b1-proof-regeneration/01-lan-source-matrix-plan-completion-proof.json`
- `output/lan-plan-proof/01-lan-b1-proof-regeneration/02-lan-signed-discovery-relay-spine-proof.json`
- `output/lan-plan-proof/01-lan-b1-proof-regeneration/03-production-discovery-household-proof.json`
- `output/lan-plan-proof/01-lan-b1-proof-regeneration/04-household-lan-proof-readiness.json`

## B2 Status

`B2` is green as of 2026-06-17 for the assigned LAN test-truth repair scope.

- `packages/lan-domain/tests/unit` is the only populated LAN test category on this branch/worktree.
- Current real LAN test files: `18`.
- Current placeholder `.gitkeep` files outside real unit coverage: `30`.
- Placeholder category directories do not count as integration, contract, e2e, property, security, load, observability, or release coverage.
- No `packages/lan-domain/src/**` edits are part of `B2`.
- Focused validation is green:
  - `packages/lan-domain :: cmd /c npx vitest run tests/unit`

Expected evidence for `B2`:

- `packages/lan-domain/tests/README.md`
- `output/lan-plan-proof/02-lan-b2-test-truth-repair/00-b2-test-truth-note.md`

## Executable Truth

- `packages/lan-domain` is the current TypeScript package-level proof surface for executable `lan-plan` work.
- `packages/parent-domain/src/lan-*` is not the authoritative owner for current completion claims.
- The current LAN source-matrix/read-model model covers workpacks `01-20`, not `21-25`.
- Portal LAN proof still depends on source/service-backed truth; portal does not own the LAN truth model.
- Product route refresh now flows through typed Tauri host subscriptions that emit `ParentSubscriptionEvent` snapshots into the portal shell; product TSX still does not own a WebSocket transport or canonical backend event replay.
- Stored child/known-device IPs no longer leave the bounded active-refresh target list on historical truth alone; current neighbor-state MAC confirmation or the live default-gateway path is now required before suppression.

## Open Execution Buckets

- Locally executable next: `01-16`, `19`
- Open implementation but still local-slice work: `05`, `06`, `07`, `08`, `09`, `11`, `17`
- Mixed local plus physical/manual final gates: `15`, `16`, `18`, `19`, `20`
- Frozen and out of current model: `21-25`

## Remaining Gaps For Real Completion

- real second-device household proof
- router/firewall reachability proof
- manual interface override and ignored-interface reason-code proof for the now-persisted gateway/DNS/DHCP/broadcast/IPv6 interface map
- passive DHCP/WS-Discovery/SNMP evidence proof
- broader live IPv6 neighbor discovery and ff02::1/NDP proof where platform/runtime boundaries allow it beyond the current parser-level Windows/Linux normalization tests
- curated service-probe evidence proof for headers, redirects, titles, and TLS subject without crawling
- bounded active WSD/SNMP identity-query proof and optional OS-fingerprint manual gate
- weighted classification and installability proof for unknown/probable/not-installable states
- prior-scan continuity snapshot proof for weak historical hints, restart behavior, and no-override safety
- real signed child hello/heartbeat artifacts
- replay/restart/event-stream proof completion
- portal and downstream consumer proof artifacts
- Android/mobile-controller proof where the plan still keeps those claims

Household/setup/account first-run UX is not part of the current authoritative model. That work remains frozen in `21-25`.

## Next Slice

If `B2` lands cleanly with the current unit-only test truth and no remaining LAN test-placement contradiction, the next exact slice is `lan-c1-protocol-service-truth-repair`.
