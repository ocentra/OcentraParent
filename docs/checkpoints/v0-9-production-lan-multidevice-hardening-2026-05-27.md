<!-- agent-capsule -->

> Agent Capsule
> Doc: V0.9 Production LAN Multidevice Hardening Checkpoint - 2026-05-27
> Kind: historical checkpoint/proof documentation; read only when CHECKPOINT_INDEX or PROOF_INDEX names it.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# V0.9 Production LAN Multidevice Hardening Checkpoint - 2026-05-27

Branch: `codex/v09-production-lan-multidevice-hardening`

## Scope Proved Locally

- Explicit LAN discovery state values: `discovered`, `pending`, `paired`,
  `revoked`, `stale`, `offline`, and `unavailable`.
- Trusted-device registry persistence now stores selected route recovery state
  and restores the selected route after restart.
- Controller lease/write-authority proof covers active writer, observer
  read-only behavior, expired/missing lease rejection, denied takeover while an
  active controller is current, accepted takeover after release, wrong-device,
  stale, malformed, replay, and revocation rejection.
- LAN AI provider routing proof covers opted-in provider, authorized result,
  unsupported capability rejection, busy provider degraded state, unavailable
  provider degraded state, observer rejection, and custody labels.

## Evidence Commands

- `npm run build:contracts`
- `cargo test -p ocentra-parent-agent-protocol lan_pairing --no-fail-fast`
- `cargo test -p ocentra-parent-agent-core trusted_device_registry --no-fail-fast`
- `cargo test -p ocentra-parent-agent-service lan_pairing --no-fail-fast`
- `cargo build -p ocentra-parent-agent-service`
- `node scripts/test/v0-9-production-lan-multidevice-hardening.mjs`

Generated local evidence:

- `test-results/v0-9-production-lan-multidevice-hardening/proof.json`
- `test-results/v0-9-lan-discovery-challenge-mvp/proof.json`
- `test-results/v0-9-lan-pairing-control-mvp/proof.json`
- `test-results/platform-roles-lan-ai-provider-pool/proof.json`

## Manual-Required Gap

The local proof uses multiple real Rust service processes on one machine. It
does not prove household router discovery, firewall prompts, OS mobile
background behavior, or two physical devices. Physical LAN readiness still
requires a manual run with:

- child service bound to `0.0.0.0:4477`;
- parent origin allowlisted for the parent LAN IP and portal port;
- two distinct host names or LAN IPs recorded;
- service logs proving no secret-bearing payloads;
- generated proof JSON from the V0.9 hardening script;
- router/firewall note showing the child service port was reachable from the
  parent host.
