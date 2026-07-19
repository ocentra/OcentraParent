# LAN Plan State

<!-- agent-capsule -->

> Agent Capsule
> Plan: `lan-plan`
> Doc: `LAN Plan State`
> Kind: current executable status and open gaps.
> Read when: first, before opening workpacks or proof paths.
> Stop rule: do not widen beyond the selected workpack from here; `21-25` are active LAN follow-on scope, with `23` and `25` currently open.
> Proves: current plan model, current slice status, and next execution route only.
> Does not prove: final completion of open workpacks, physical household proof, or sibling plan completion.
> Proof rule: any status claim here must point at an existing artifact or an explicit open/manual-required gap.

<!-- /agent-capsule -->

## Current State

- Plan state: active
- Authoritative execution model: `01-25`
- Active open follow-on workpacks: `23`, `25`
- Current completed slices: `Slice A`, `B1`, `B2`, `01`, `02`, `03`, `05`, `06`, `10`, `12`, `13`, `14`, `15`, `21`, `22`, `24`
- Slice A evidence root: `output/lan-plan-proof/00-plan-model-reconciliation/`
- B1 evidence root: `output/lan-plan-proof/01-lan-b1-proof-regeneration/`
- B2 evidence root: `output/lan-plan-proof/02-lan-b2-test-truth-repair/`
- WP25 tracked evidence root:
  `docs/proof/lan-plan/25-rollout-checklist-and-pr-gate/`

## Current ownership interpretation

```text
Rust shared schema/protocol crates:
  Canonical LAN pairing, discovery, source-matrix, route-snapshot, signed
  hello, heartbeat, assignment, revocation, audit, and read-model shapes when
  those shapes cross package, crate, app, or plan boundaries. Rust owns
  contracts and runtime truth.

lan-core, agent-protocol, agent-service, and parent-runtime-core:
  Own LAN business logic, protocol/service/runtime behavior, read models, route
  snapshots, and proof when the selected workpack names those surfaces.

apps/portal and thin TS bridge/presentation code:
  Projection and UI only. TS consumes Rust-backed snapshots, generated DTOs,
  and host-bridge events; it does not own LAN truth, contract authority, or
  runtime proof.

eventing-plan:
  Local event bus semantics only. Eventing does not own LAN transport,
  discovery, route authority, or physical topology proof.

account-identity-family-plan and device-trust-bootstrap-plan:
  Household/actor authority and trusted-device/key material owners.

remote-access-plan:
  Relay and remote-access transport owner.

parent/child runtime distribution plans:
  Package, installer, signed child package, and child-agent distribution
  owners.
```

## Current coupling risks

```text
- stale references to `packages/lan-domain`, `schema-domain`,
  `parent-domain`, or other TS contract catalogs as authoritative LAN owners
- unit tests are not integration/e2e/security/physical/load proof
- placeholder test folders and `.gitkeep` files do not count as coverage
- single-machine proof is not real two-device household proof
- schema/contract proof is not packet/runtime proof
- source-matrix proof is not physical discovery proof
- portal rendering proof is not LAN truth proof
- B1/B2 proof is not signed hello/heartbeat, service/runtime, portal, physical
  household, router/firewall, Android/mobile, or relay proof
- active workpacks 23-25 still need explicit truth-sync and must not be skipped,
  auto-closed, or treated as off-plan
```

## Agentless evidence-fusion adoption

As of 2026-06-23, the plan absorbs the agentless LAN design note into the
current authoritative `01-25` model instead of creating a parallel discovery
plan.

```text
W03 owns the richer interface map: local IP, subnet, default gateway, DNS
server, DHCP server, broadcast address, and IPv6 prefixes.
W04 owns cross-platform neighbor normalization, including IPv6/NDP truth when
exposed by the host.
W07 owns passive collector expansion: ARP, DHCP, mDNS, SSDP, WS-Discovery,
LLMNR, NetBIOS, and allowed SNMP response evidence.
W09 owns bounded descriptor parsing from SSDP/UPnP metadata.
W11 owns curated TCP/UDP service probing with sanitized HTTP/TLS/banner hints,
bounded WSD/SNMP identity queries where allowed, optional OS-fingerprint proof
gates, and strict no-crawl behavior.
W12 owns OUI/vendor evidence plus randomized/private MAC suspicion.
W14 owns weighted evidence-fusion classification, explicit reasons/confidence,
and install-eligibility honesty.
W15 owns persisted prior-scan continuity snapshots that can strengthen
stale/offline and merge confidence without becoming permanent truth.
```

Hard rules adopted from that note:

```text
- agentless LAN discovery is evidence fusion, not platform certainty
- MAC vendor alone cannot claim Windows, Android, iOS, or child ownership
- open ports, banners, titles, redirects, or certificates cannot confirm
  child-agent identity
- ICMP reachability is optional only; ARP/NDP, neighbor tables,
  advertisements, and bounded service evidence remain primary
- visible classification and installability claims must carry reasons or stay
  unknown/manual-required
```

## Current proof interpretation

```text
Slice A proves plan-model reconciliation only.
B1 proves a historical local proof-regeneration slice only. Its legacy
TS-package artifacts are not current contract/runtime authority.
B2 proves LAN test-category truth only. It rejects placeholder coverage claims
and does not bless TS packages as forward test owners.
The only populated legacy LAN test category tracked by B2 is
`packages/lan-domain/tests/unit`.
Authoritative forward LAN logic tests belong in proper Rust crate test groups,
with TS tests limited to presentation consumers.
Physical household LAN readiness needs real multi-device/manual artifacts.
Portal and downstream consumers need source/service-backed proof artifacts.
```

## Slice A Status

`Slice A` is green as of 2026-06-17 for the scope that was actually assigned:

- legacy `packages/lan-domain` export/ownership repair for the reconciliation
  slice only
- focused proof-schema test repair
- legacy `@ocentra-parent/lan-domain` test pass for the assigned slice only
- legacy `@ocentra-parent/lan-domain` build pass for the assigned slice only
- legacy `packages/lan-domain` architecture pass for the assigned slice only
- plan truth-sync for the then-authoritative `01-20` reconciliation slice,
  now carried forward inside the active `01-25` Rust-first model
- honest proof-root bootstrap for this reconciliation slice

Exact evidence:

- `output/lan-plan-proof/00-plan-model-reconciliation/00-source-snapshot.md`
- `output/lan-plan-proof/00-plan-model-reconciliation/01-lan-domain-validation.log`
- `output/lan-plan-proof/00-plan-model-reconciliation/02-plan-truth-sync.md`
- `output/lan-plan-proof/00-plan-model-reconciliation/03-missing-proof-inventory.md`

## B1 Status

`B1` is green as of 2026-06-17 for the assigned local proof-regeneration scope
only.

- Repaired proof scripts now treat legacy `packages/lan-domain` inputs as
  historical slice evidence instead of letting stale `parent-domain`, portal,
  or service-backed paths stand in as current proof.
- Legacy `@ocentra-parent/lan-domain` tests are green for the assigned slice
  only.
- Legacy `packages/lan-domain` architecture validation is green for the
  assigned slice only.
- The regenerated proof chain ends in
  `not-ready-for-product-ready-household-lan-claim`, which is the correct
  local mechanical result for this slice.

Exact evidence:

- `output/lan-plan-proof/01-lan-b1-proof-regeneration/01-lan-source-matrix-plan-completion-proof.json`
- `output/lan-plan-proof/01-lan-b1-proof-regeneration/02-lan-signed-discovery-relay-spine-proof.json`
- `output/lan-plan-proof/01-lan-b1-proof-regeneration/03-production-discovery-household-proof.json`
- `output/lan-plan-proof/01-lan-b1-proof-regeneration/04-household-lan-proof-readiness.json`

## B2 Status

`B2` is green as of 2026-06-17 for the assigned LAN test-truth repair scope.

- `packages/lan-domain/tests/unit` is the only populated legacy LAN test
  category on this branch/worktree.
- Current real LAN test files: `18`.
- Current placeholder `.gitkeep` files outside real unit coverage: `30`.
- Placeholder category directories do not count as integration, contract, e2e,
  property, security, load, observability, or release coverage.
- Future authoritative LAN logic coverage belongs in proper Rust crate test
  groups; TS tests stay presentation-only.
- No `packages/lan-domain/src/**` edits are part of `B2`.
- Historical B2 validation record is green for the assigned residue audit only:
  - `packages/lan-domain :: cmd /c npx vitest run tests/unit`
- That historical command is not part of the forward LAN execution path.

Expected evidence for `B2`:

- `packages/lan-domain/tests/README.md`
- `output/lan-plan-proof/02-lan-b2-test-truth-repair/00-b2-test-truth-note.md`

## Executable Truth

- Rust-owned schema/protocol/service/runtime crates are authoritative for
  executable `lan-plan` work.
- Legacy `packages/lan-domain` or other TS package artifacts may remain as
  historical proof inputs or presentation adapters only; they are not
  authoritative owners.
- Forward LAN code/test closure should keep moving logic coverage into
  organized Rust crate `tests/` groups and keep TS limited to presentation/UI
  surfaces.
- `packages/parent-domain/src/lan-*` is not the authoritative owner for current
  completion claims.
- The current LAN source-matrix/read-model model already drives workpacks
  `01-20`; workpacks `21-25` are also active scope, with `21`, `22`, and `24`
  locally closed by their own proof and `23`/`25` still open.
- Portal LAN proof still depends on source/service-backed truth; portal does
  not own the LAN truth model.
- Canonical backend LAN stream replay is loaded, validated, ordered, and bound
  to the independently loaded status history state/latest ID/latest time before
  Rust constructs `ParentSubscriptionEvent`. Rejection emits a safe host-owned
  warning identity. The Tauri host delivery decision and isolated portal state
  edge separately prove unseen-ID delivery, replay-only snapshot binding, and
  stable newest-128 buffering. No current proof observes the same replay batch
  through the real Tauri `AppHandle` emitter and portal listener. The complete
  backend-to-host-to-portal chain and manual runtime proof therefore remain
  open; product TSX still does not own replay business logic.
- The current WP25 rollout truth and validation history are tracked under
  `docs/proof/lan-plan/25-rollout-checklist-and-pr-gate/`; generated `output/`
  files are not accepted as tracked source proof.
- Stored child/known-device IPs no longer leave the bounded active-refresh
  target list on historical truth alone; current neighbor-state MAC
  confirmation or the live default-gateway path is now required before
  suppression.

## Open Execution Buckets

- Local Rust implementation complete; manual/packet proof remains: `04`, `07`,
  `08`, `09`, `11`, `17`
- Partial implemented slices still needing local proof/gap closure: none after
  the accepted 2026-06-28 LAN packet waves; the remaining open work is now
  integration/runtime parity or explicit manual/packet proof
- Mixed local plus physical/manual final gates: `16`, `18`, `19`, `20`, `23`,
  `25`
- Locally closed rows and truth-synced summaries: `01`, `02`, `03`, `05`,
  `06`, `10`, `12`, `13`, `14`, `15`, `21`, `22`, `24`

## Remaining Gaps For Real Completion

- real second-device household proof
- router/firewall reachability proof
- long-running passive DHCP listener proof plus broader passive
  trigger/platform/manual cross-checks
- live macOS/manual neighbor-table proof for the now locally complete W04
  parser path
- curated service-probe evidence proof for headers, redirects, titles, and TLS
  subject without crawling
- live household/manual proof for bounded WSD and SNMP identity queries, plus
  the optional OS-fingerprint manual gate
- weighted classification and installability proof for
  unknown/probable/not-installable states
- real signed child hello/heartbeat artifacts
- restart and physical cross-session event-stream proof completion
- real Tauri `AppHandle` emission plus portal-listener observation of a
  backend replay batch; backend validation, bridge construction, the host
  delivery decision, and an isolated portal state edge are locally covered,
  but they do not yet prove the complete chain
- additional downstream consumer proof artifacts beyond the current Rust-backed
  `/devices`, policy-target, and Start-route first-run portal snapshots
- Android/mobile-controller proof where the plan still keeps those claims
- broader `build:contracts` and source-matrix wrapper gates are green in this
  lane on 2026-06-28:
  `cmd /c npm run build:contracts` and
  `node scripts/test/v0-9-lan-source-matrix-plan-completion.mjs`

Household/setup/account first-run UX is no longer a broad unvalidated LAN gap.
The remaining open truth is now concentrated in route/runtime integration
(`16`, `19`), signed-child/manual proof (`18`, `23`), the proof-gate wrapper
(`20`, `25`), and the explicit packet/manual proof tails for locally
code-complete workpacks (`04`, `07`, `08`, `09`, `11`, `17`).

## Next Slice

With the accepted 2026-06-28 packet waves landed locally, the next exact slice
is the main-lane integration and truth-sync pass across `16`, `19`, and the
top-level `lan-plan` summary/checklist surfaces.
`lan-c1-protocol-service-truth-repair`.
