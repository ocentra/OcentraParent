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

- This file is truth-synced on 2026-06-17.
- Historical checkbox counts and historical `[~]` progress markers are not authoritative.
- `01-20` are the only live execution rows.
- `21-25` are frozen follow-on material and do not drive current completion claims.

## Slice A Evidence

`Slice A` is green for the assigned reconciliation scope.

- Proof root: `output/lan-plan-proof/00-plan-model-reconciliation/`
- Source snapshot: `output/lan-plan-proof/00-plan-model-reconciliation/00-source-snapshot.md`
- Validation log: `output/lan-plan-proof/00-plan-model-reconciliation/01-lan-domain-validation.log`
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
| `01` | partial | locally closable next | `packages/lan-domain/src/lan-pairing.ts`; Slice A validation log | broader contract/read-model closure |
| `02` | partial | locally closable next | service-backed LAN add-device read model in `packages/lan-domain` | fuller evidence/device-store closure |
| `03` | partial | locally closable next | selected-interface modeling in current source matrix | full adapter/interface proof |
| `04` | partial | locally closable next on Windows/Linux | Rust/service Windows neighbor-table path exists | Linux/mac normalization or explicit manual-required proof |
| `05` | open | locally executable next | no implementation claimed | targeted ARP implementation and proof |
| `06` | open | locally executable next | no implementation claimed | bounded sweep implementation and proof |
| `07` | open | locally executable next | no implementation claimed | passive listener implementation and packet proof |
| `08` | open/manual | locally executable next | no implementation claimed | mDNS/DNS-SD implementation and proof |
| `09` | open/manual | locally executable next | no implementation claimed | SSDP/UPnP implementation and proof |
| `10` | partial | locally closable next | weak-source fencing exists in current source matrix | real query adapters if full closure is desired |
| `11` | open | locally executable next | no implementation claimed | bounded service probing implementation and proof |
| `12` | partial | locally closable next | OUI/vendor is modeled as weak/manual-required only | real vendor/proof path if full closure is desired |
| `13` | partial | locally closable next | household-device spine/source merge exists | broader dedupe closure |
| `14` | partial | locally closable next | explainable labels/status exist | fuller classifier closure |
| `15` | partial | locally closable next, then physical/manual-bound | service-backed device-store/read-model paths exist | restart durability and real household proof |
| `16` | partial | locally closable next, then physical/manual-bound | read-model and diagnostics surface exists | replay/event proof and real artifact regeneration |
| `17` | open | locally executable next | no implementation claimed | parent/child mDNS advertisements and packet proof |
| `18` | partial/manual | physical/manual-bound for final closure | signed hello/heartbeat rows exist as contracts/read-model labels | real signed artifacts and real device proof |
| `19` | partial | locally closable next, then physical/manual-bound | assignment/revoke/audit surfaces exist | restart and negative-route proof |
| `20` | partial/manual | mixed local plus physical/manual final gate | `output/lan-plan-proof/01-lan-b1-proof-regeneration/`; Workpack 20 proof note | portal/service-backed consumer proof, real signed artifacts, and remaining physical/manual proof |

## Frozen Follow-On Rows

| Workpack | State | Reason |
| --- | --- | --- |
| `21` | frozen | non-authoritative rewrite material |
| `22` | frozen | non-authoritative rewrite material |
| `23` | frozen | non-authoritative rewrite material |
| `24` | frozen | household/setup/account UI handoff work, outside current LAN execution model |
| `25` | frozen | later rollout wrapper, outside current LAN execution model |
