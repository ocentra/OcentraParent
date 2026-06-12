<!-- agent-capsule -->

> Agent Capsule
> Doc: V0.9 LAN Discovery Challenge MVP Proof
> Kind: historical checkpoint/proof documentation; read only when CHECKPOINT_INDEX or PROOF_INDEX names it.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

# V0.9 LAN Discovery Challenge MVP Proof

Date: 2026-05-26
Branch: `codex/v0.9-lan-discovery-challenge-mvp`
Base: `0f61746a6513cb7368905b3e10839954930d0da5`

## Scope

This branch advances the B-owned V0.9 LAN pairing slice from direct proof submit
only to a reviewable direct WebSocket discovery/challenge/proof-preview
ceremony. It keeps the product claim narrow: the Rust service can expose a
schema-backed challenge preview through the existing LAN WebSocket status
surface, validate a submitted pairing proof against that issued challenge when
one exists, and continue to reject anonymous, wrong-origin, stale, malformed,
replayed, wrong-device, and unselected control paths.

It does not claim production LAN discovery, mDNS/SSDP broadcast discovery,
cryptographic device auth, physical two-device household LAN proof, firewall
behavior, portal selector UX, mobile behavior, or production installer
permissions.

## Implementation Summary

- Added `websocket-direct` as the honest runtime support label for
  discovery/challenge/proof-preview surfaces while leaving HTTP discovery,
  challenge, proof, control, and registry endpoints `planned-unsupported`.
- Added TypeScript and Rust protocol parity for a LAN challenge request shape
  and direct WebSocket challenge/proof-preview status.
- Extended the Rust LAN runtime with in-memory issued-challenge state. Proof
  submit remains backward compatible with the direct proof-submit MVP when no
  challenge has been issued, but validates against the issued challenge once
  the service has previewed one.
- Added challenge-issued audit payload fields without raw proof secret material.
- Hardened proof submit rejection for wrong-origin, stale, malformed, replayed,
  and wrong-device cases.
- Added `scripts/test/v0-9-lan-discovery-challenge-mvp.mjs`, a two-service
  real WebSocket proof harness.

## Mechanical Proof Run

Commands:

```powershell
cmd /c cargo fmt
cmd /c cargo test -p ocentra-parent-agent-protocol lan_pairing -- --nocapture
cmd /c cargo test -p ocentra-parent-agent-service lan_pairing -- --nocapture
cmd /c npm run test --workspace @ocentra-parent/parent-domain
cmd /c npm run test --workspace @ocentra-parent/agent-protocol-domain
cmd /c cargo build -p ocentra-parent-agent-service
cmd /c node scripts/test/v0-9-lan-discovery-challenge-mvp.mjs
```

Observed proof summary:

```text
v0-9-lan-discovery-challenge-mvp-ok:wrong-origin-websocket-rejected-before-upgrade,first-discovery-agent:anonymous-control-rejected,first-discovery-agent:wrong-origin-proof-rejected,first-discovery-agent:malformed-proof-rejected,first-discovery-agent:stale-proof-rejected,first-discovery-agent:challenge-preview-issued,first-discovery-agent:challenge-proof-accepted,first-discovery-agent:challenge-proof-replay-rejected,first-discovery-agent:route-selected-after-challenge,first-discovery-agent:rule-query-accepted-after-challenge,second-discovery-agent:anonymous-control-rejected,second-discovery-agent:wrong-origin-proof-rejected,second-discovery-agent:malformed-proof-rejected,second-discovery-agent:stale-proof-rejected,second-discovery-agent:challenge-preview-issued,second-discovery-agent:challenge-proof-accepted,second-discovery-agent:challenge-proof-replay-rejected,second-discovery-agent:route-selected-after-challenge,second-discovery-agent:rule-query-accepted-after-challenge,wrong-agent-port-challenge-rejected-as-wrong-device
```

Ignored local artifact:

- `test-results/v0-9-lan-discovery-challenge-mvp/proof.json`

The proof starts two real `ocentra-parent-agent-service` processes on
`0.0.0.0:4494` and `0.0.0.0:4495`, each with
`OCENTRA_PARENT_AGENT_LAN_CHILD_DEVICE_ID`,
`OCENTRA_PARENT_AGENT_LAN_PAIRING_REGISTRY_PATH`,
`OCENTRA_PARENT_AGENT_LOCAL_NETWORK_ENABLED=true`, and the loopback portal
origin allowlist.

## Proof Labels

| Area                                     | Evidence                                                                                                             | Label                                      |
| ---------------------------------------- | -------------------------------------------------------------------------------------------------------------------- | ------------------------------------------ |
| Typed challenge request and preview      | Parent-domain, agent-protocol-domain, and Rust protocol tests parse challenge request, challenge, and proof preview. | `implemented` for direct WebSocket MVP     |
| Discovery/challenge runtime status       | Real service status payload reports `websocket-direct` for discovery, challenge, and proof preview.                  | `implemented` for direct WebSocket MVP     |
| HTTP discovery/challenge/proof endpoints | Runtime support surface still lists HTTP endpoint paths as unsupported.                                              | `planned-unsupported`                      |
| Challenge-issued audit                   | Harness observes `pairing-challenge-issued` with child, parent, route, origin, challenge id, and digest reference.   | `implemented` for audit spine              |
| Accepted challenge proof                 | Harness uses the previewed challenge id and proof digest, then receives `pairing-proof-accepted`.                    | `implemented` for service path             |
| Wrong-origin proof                       | Harness submits a proof whose payload origin differs from the WebSocket origin and receives `wrong-origin`.          | `implemented` for service path             |
| Stale and malformed proof                | Harness submits expired and bad-digest proofs and receives `stale` and `malformed`.                                  | `implemented` for service path             |
| Replay rejection                         | Harness resubmits an accepted challenge proof and receives `replayed`.                                               | `implemented` for service path             |
| Wrong child service port                 | Harness sends the second child challenge request to the first service port and receives `wrong-device`.              | `implemented` for two-instance local proof |
| Route select and signed child-side query | Harness selects the route after accepted proof, then sends an accepted signed `rule-query`.                          | `implemented` for MVP service path         |
| Physical household two-device LAN        | This run used two local service processes on one Windows host, not two physical devices through router/firewall.     | `manual-required`                          |
| Portal discovery/selector UX             | This branch avoids C-owned portal files.                                                                             | `not-yet-proven`                           |
| Production pairing authentication        | Current proof digest is an MVP preview/reference, not production cryptographic device authentication.                | `not-yet-proven`                           |

## Remaining Owner-Ready Proof

Run this on two physical devices after the service branch lands and C-owned
portal selector work is ready:

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

- child and parent OS versions, IPs, subnet, firewall state, and router segment;
- parent-to-child reachability and failed wrong-origin WebSocket upgrade;
- challenge preview payload with no raw activity, raw token, or local path data;
- wrong-origin, stale, malformed, replayed, and wrong-device proof rejection;
- accepted proof, route selection, accepted signed query, revoke, and
  post-revoke rejection;
- restart behavior with the same registry path;
- portal-selected device display and offline/stale state once the C-owned portal
  surface exists.

Keep V0.9 labeled as direct WebSocket MVP proof until physical LAN,
router/firewall, cryptographic pairing, and portal-selector evidence exists.
