# LAN Plan Test and Proof Expectations

<!-- agent-capsule -->

> Agent Capsule
> Plan: `lan-plan`
> Doc: `LAN Plan Test and Proof Expectations`
> Kind: plan-local test and proof decision tree.
> Read when: After the assigned workpack/checklist row is known; use to choose required tests/proof.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: This file defines required local tests/proof; missing tests keep rows open.

<!-- /agent-capsule -->

Use this after the assigned LAN workpack is known. LAN proof must separate discovery, pairing, trust, signed hello/heartbeat, parent action, and physical-household proof.

## Where tests should live

When the LAN implementation crate/package exists, tests belong under its test tree and proof output under its proof folder. Until then, colocate with the owning protocol/runtime package and record paths in the workpack and `PROOF_INDEX.md`.

## Decision Tree

| If the assigned work is...                        | Read next         | Expected tests or proof                                                                                   |
| ------------------------------------------------- | ----------------- | --------------------------------------------------------------------------------------------------------- |
| Discovery source matrix or signed hello/heartbeat | assigned workpack | source authority invariants, signature validation, replay/stale heartbeat, duplicate device negatives.    |
| Pairing challenge/add-device state                | assigned workpack | authZ, token lifecycle, replay, expiry, brute force/rate limit, double-submit, cross-household isolation. |
| LAN route/controller state                        | assigned workpack | route state transitions, offline/degraded states, idempotency, ordering, retry/partial outage.            |
| mDNS/ARP/probing/listeners                        | assigned workpack | bounded scan proof, origin/host constraints, DoS/resource limits, manual network artifact proof.          |
| Parent device actions                             | assigned workpack | authority matrix, audit trail, rollback, stale target, manual-required proof.                             |
| Physical household proof                          | `PROOF_INDEX.md`  | device/OS/router/firewall details, screenshots/logs, generated artifact path, limitation notes.           |

## Expected test/proof inventory

Use these names as proof intent labels in the assigned workpack/proof note. Implementers choose the actual crate/package test names after the owning implementation boundary exists.

- `lan.peer.authn-authz-matrix`: household peer identity, role, and authorization boundaries reject spoofing/cross-family cases.
- `lan.discovery.partial-outage`: discovery handles offline, duplicate, stale, and slow peer states.
- `lan.lease.token-lifecycle`: claim/lease/token lifecycle rejects replay, expiry, double-claim, and clock skew.
- `lan.transport.retry-storm-boundary`: retry storms, rate limits, and connection exhaustion remain bounded.
- `lan.mesh.no-raw-sensitive-transfer`: bridge messages transfer only approved validated payloads.
- `lan.topology.two-device-proof`: physical or documented two-device proof records device/OS/network state.
- `lan.audit.trace-completeness`: pairing/claim/result paths emit safe logs, metrics, and trace refs.

## Required proof contents

- Household/device identity and trust boundary proof.
- Negative replay/expiry/cross-household cases.
- Network resource limits and cleanup evidence.
- Manual physical proof when real LAN/device behavior is claimed.

## Failure conditions

Do not claim DONE or PR_READY if any apply:

- The expected test/proof row for the touched work type is missing.
- The implementation crate/package test folder does not exist and the missing expected location is not recorded.
- Only happy-path tests pass for a trust, policy, persistence, protocol, UI, AI, platform, security, performance, or observability boundary.
- A product/checklist row moved without command logs and proof artifact path.
- A manual-required/platform limitation was converted into a runtime capability claim.
- A proof artifact lacks negative cases, logs/traces where relevant, or exact workpack/checklist linkage.
- A sibling plan or broad source tree was read without a route reason recorded in the workpack/proof note.
