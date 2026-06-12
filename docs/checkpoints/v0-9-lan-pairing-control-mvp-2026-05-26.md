<!-- agent-capsule -->

> Agent Capsule
> Doc: V0.9 LAN Pairing And Control MVP Proof
> Kind: historical checkpoint/proof documentation; read only when CHECKPOINT_INDEX or PROOF_INDEX names it.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

# V0.9 LAN Pairing And Control MVP Proof

Date: 2026-05-26
Branch: `codex/v0.9-lan-pairing-control-mvp`
Base: `c351dc19b9cc1a90a7b650cb2e8329bcb9618d3e`

## Scope

This branch advances the V0.9 LAN pairing/control implementation from proof
spines to a reviewable MVP service path. It keeps the proof boundary narrow:
real Rust service processes, real WebSocket routes, typed contracts, persistent
local trusted-device registry behavior, and explicit negative checks. It does
not claim production LAN discovery, a full portal selector UX, household router
or firewall behavior, mobile background behavior, or a cryptographic pairing
ceremony beyond the current direct proof-submit contract.

## Implementation Summary

- Added parent-domain parity for `offline` LAN rejection and explicit
  `discoveryStatus`, `challengeStatus`, and `proofPreviewStatus` fields on the
  planned-unsupported LAN runtime surfaces.
- Added agent-protocol-domain parity for `local-network-disabled` rejection so
  TypeScript protocol contracts can represent the Rust rejection enum.
- Added `OCENTRA_PARENT_AGENT_LAN_CHILD_DEVICE_ID` as a LAN runtime identity
  guard. When set, a child agent rejects pairing proof or control commands whose
  command target is for a different child device before child-side execution.
- Kept LAN HTTP discovery, challenge, proof, control, and registry endpoint
  surfaces explicitly `planned-unsupported`; the MVP path remains WebSocket
  with direct proof submit, route select/revoke/status, and signed parent
  intents.
- Added a two-service proof harness at
  `scripts/test/v0-9-lan-pairing-control-mvp.mjs`.

## Mechanical Proof Run

Command:

```powershell
cmd /c npm run build:contracts
cargo build -p ocentra-parent-agent-service
cmd /c node scripts/test/v0-9-lan-pairing-control-mvp.mjs
```

Observed proof summary:

```text
v0-9-lan-pairing-control-mvp-ok:wrong-origin-websocket-rejected-before-upgrade,first-child-agent:anonymous-rejected,first-child-agent:pairing-proof-accepted-unselected,first-child-agent:unselected-control-rejected,first-child-agent:route-selected,first-child-agent:rule-query-accepted,first-child-agent:replay-rejected,first-child-agent:route-revoked,first-child-agent:revoked-control-rejected,second-child-agent:anonymous-rejected,second-child-agent:pairing-proof-accepted-unselected,second-child-agent:unselected-control-rejected,second-child-agent:route-selected,second-child-agent:rule-query-accepted,second-child-agent:replay-rejected,wrong-agent-port-rejected-as-wrong-device,second-child-agent:restart-restores-trusted-unselected,second-child-agent:restart-unselected-control-rejected,second-child-agent:restart-route-reselected,second-child-agent:restart-approval-accepted
```

Ignored local artifact:

- `test-results/v0-9-lan-pairing-control-mvp/proof.json`

The proof starts two real `ocentra-parent-agent-service` processes on
`0.0.0.0:4492` and `0.0.0.0:4493`, each with its own
`OCENTRA_PARENT_AGENT_LAN_CHILD_DEVICE_ID` and local JSON registry path.

## Proof Labels

| Area                                           | Evidence                                                                                                                                                                                               | Label                                      |
| ---------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ------------------------------------------ |
| Typed LAN pairing/control contracts            | Parent-domain and agent-protocol-domain tests parse paired, unselected, offline, local-network-disabled, proof-preview, and runtime-support surfaces.                                                  | `implemented` for contracts                |
| Child-agent local identity guard               | Rust service rejects wrong child target for proof submit and control before execution when `OCENTRA_PARENT_AGENT_LAN_CHILD_DEVICE_ID` is set.                                                          | `implemented` for service guard            |
| Anonymous and unselected control               | Harness rejects anonymous LAN control and rejects trusted-but-unselected control.                                                                                                                      | `implemented` for service path             |
| Route selection and accepted child-side query  | Harness selects each child route and accepts a signed `rule-query` through the real WebSocket service.                                                                                                 | `implemented` for MVP service path         |
| Replay, revocation, and wrong-origin negatives | Harness rejects replayed intent, persists revocation, rejects post-revocation control, and rejects wrong-origin WebSocket upgrade.                                                                     | `implemented` for service path             |
| Wrong-port / wrong-agent negative              | Harness sends the second child target to the first child service port and receives `wrong-device`.                                                                                                     | `implemented` for two-instance local proof |
| Restart persistence                            | Harness restarts the second service against the same registry: trusted registry restores, selected route is cleared, unselected control rejects, route can be reselected, and approval intent accepts. | `implemented` for local JSON registry MVP  |
| HTTP discovery/challenge/control endpoints     | Runtime support surfaces still mark these endpoints `planned-unsupported`.                                                                                                                             | `planned-unsupported`                      |
| Household two-device LAN                       | This run used two local service processes on one Windows host, not two physical devices across a router/firewall.                                                                                      | `manual-required`                          |
| Portal multi-device selector UX                | This branch avoids C-owned portal paths; service/protocol can distinguish child agents, but portal UX proof is not in this branch.                                                                     | `not-yet-proven`                           |
| macOS/Linux/Android/iOS LAN behavior           | Not run in this local proof pass.                                                                                                                                                                      | `manual-required`                          |

## Remaining Owner-Ready Proof

Run this after the branch lands or from the PR branch on the actual devices:

```powershell
set OCENTRA_PARENT_AGENT_LOCAL_NETWORK_ENABLED=true
set OCENTRA_PARENT_AGENT_LAN_CHILD_DEVICE_ID=<child-device-id>
set OCENTRA_PARENT_AGENT_LAN_PAIRING_REGISTRY_PATH=<child-local-registry-json>
set OCENTRA_PARENT_AGENT_ALLOWED_ORIGINS=http://<parent-host>:4478
set OCENTRA_PARENT_AGENT_ADDR=0.0.0.0:4477
cargo build -p ocentra-parent-agent-service
target\debug\ocentra-parent-agent-service.exe
```

Record:

- child and parent OS versions, IPs, subnet, and firewall state;
- successful health reachability from parent to child service;
- failed wrong-origin WebSocket upgrade;
- proof submit, route select, accepted signed query, replay rejection, revoke,
  and post-revoke rejection;
- service restart with the same registry path and the expected
  trusted-but-unselected state;
- portal selector behavior once C-owned portal UI work exposes multi-device
  selection.

Do not upgrade V0.9 to product-complete until the physical two-device LAN,
router/firewall, and portal selector proof records exist.
