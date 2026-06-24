# LAN Plan Proof Index

<!-- agent-capsule -->

> Agent Capsule
> Plan: `lan-plan`
> Doc: `LAN Plan Proof Index`
> Kind: canonical proof routing.
> Read when: a LAN slice needs exact proof paths.
> Stop rule: claim only files that exist on disk.
> Proves: proof-root routing and current artifact status only.
> Does not prove: implementation completion by itself.
> Proof rule: if a file is absent, mark the claim open/manual-required/not yet regenerated.

<!-- /agent-capsule -->

## Current Authoritative Proof Roots

`Slice A` proof root:

```text
output/lan-plan-proof/00-plan-model-reconciliation/
```

Files present for `Slice A`:

- `00-source-snapshot.md`
- `01-lan-domain-validation.log`
- `02-plan-truth-sync.md`
- `03-missing-proof-inventory.md`

`B1` proof root:

```text
output/lan-plan-proof/01-lan-b1-proof-regeneration/
```

Files present for `B1`:

- `01-lan-source-matrix-plan-completion-proof.json`
- `02-lan-signed-discovery-relay-spine-proof.json`
- `03-production-discovery-household-proof.json`
- `04-household-lan-proof-readiness.json`

`B1` proves local mechanical LAN-domain proof only. It does not claim portal screenshots, service/runtime-backed proof, physical household readiness, real signed child hello/heartbeat artifacts, or cloud relay implementation.

`B2` proof root:

```text
output/lan-plan-proof/02-lan-b2-test-truth-repair/
```

Files present for `B2`:

- `00-b2-test-truth-note.md`

`B2` proves LAN test-category truth only. It does not claim new LAN runtime behavior, new integration/e2e/security coverage, or closure of protocol, service, portal, or physical proof gaps.

`16 Read Models And LAN Events` proof root:

```text
output/lan-plan-proof/16-read-models-and-lan-events/
```

Files present for the current Rust bridge slice:

- `01-rust-lan-read-model-validation.log`
- `02-rust-lan-read-model-note.md`
- `03-product-route-overlay-removal-note.md`
- `04-tauri-devices-auto-scan-proof.md`
- `05-tauri-host-subscription-bridge.md`

This proof root currently proves only the parent Rust bridge LAN-sample removal,
inventory-backed read-model serialization path, and the typed Tauri
host-subscription bridge that delivers deduplicated route snapshots into the
product shell. It does not yet prove full `agent-service` parity,
replay/event-stream behavior, portal/browser screenshots, dev-web parity,
signed relay/cache rows, or physical household/manual network artifacts.

`06 Bounded ARP Sweep` proof root:

```text
output/lan-plan-proof/06-bounded-arp-sweep/
```

Files present for the current smart-scan suppression safety fix:

- `01-ip-reuse-suppression-fix.md`

This proof root currently proves only that bounded active refresh no longer
suppresses a reused IP on stale child truth alone and still suppresses the live
gateway/default-route path. It does not yet prove packet-IO abstraction,
duplicate-reply handling, service/runtime parity outside the focused
`network_inventory` crate proof, or physical household/manual network
artifacts.

If a proof script emits `test-results/.../proof.json`, the workpack proof pack must reference that file and must not imply the artifact exists until it has been regenerated on this branch/worktree.

## Proof Paths Explicitly Not Claimed By Current Slices

These previously cited paths are absent on disk as of 2026-06-17 and are not current proof:

- `test-results/v0-9-lan-source-matrix-plan-completion/proof.json`
- `output/playwright/lan-source-matrix-plan-completion/devices-lan-source-matrix.png`
- `output/playwright/lan-source-matrix-plan-completion/activity-network-source-matrix.png`
- `output/playwright/lan-source-matrix-plan-completion/policy-network-target-binding.png`
- `output/playwright/lan-source-matrix-plan-completion/browser-proof.json`
- `output/lan-plan-proof/15-household-device-store/devices-identity-routing-proof.md`
- `output/lan-plan-proof/15-household-device-store/06-ui-snapshots/devices-identity-persisted.png`
- `output/lan-plan-proof/15-household-device-store/06-ui-snapshots/devices-update-gated.png`
- `docs/proof/lan-plan/PLAN_PROOF_MANIFEST.md`

## Structured proof metadata

For new proof artifacts and command-log entries, include structured metadata when available:

```text
plan: lan-plan
workpack: <workpack id and name>
owner: schema-domain | lan-domain | agent-protocol | agent-service | portal | eventing-handoff | account-handoff | device-trust-handoff | remote-handoff | distribution-handoff | cloudflare-handoff | docs-only
platform: windows | macos | linux | android | ios | cross-platform | physical-mixed | n/a
source_kind: schema | unit-test | interface | neighbor-table | arp | passive-listener | mdns | dns-sd | ssdp | upnp | netbios | llmnr | reverse-dns | service-probe | oui | signed-child-hello | heartbeat | portal | physical-manual | n/a
device_ref: <device ref or n/a>
household_ref: <household/family ref or n/a>
route_id: <route id or n/a>
evidence_ref: <evidence ref or n/a>
discovery_state: not-tested | discovered | weak-source | unsupported | stale | offline | blocked | manual-required | n/a
trust_state: not-tested | untrusted | paired | trusted | revoked | wrong-household | wrong-device | manual-required | n/a
reachability_state: not-tested | online | offline | degraded | router-blocked | firewall-blocked | local-permission-blocked | manual-required | n/a
signed_hello_state: not-tested | signed | invalid-signature | expired | replayed | wrong-family | missing | manual-required | n/a
heartbeat_state: not-tested | received | missed | stale | expired | replayed | manual-required | n/a
event_stream_state: not-tested | emitted | replayed | duplicate-rejected | stale-rejected | blocked | manual-required | n/a
portal_projection_state: not-tested | rendered-from-service | stale | blocked | manual-required | n/a
physical_topology_state: not-tested | single-machine | two-device-proved | router-proved | firewall-proved | manual-required | n/a
manual_required_note: <explicit manual-required gap or n/a>
run_id: <wrapper run id or n/a>
command_id: <wrapper command id or n/a>
command: <exact command>
exit: <code>
result: pass | fail | blocked
artifact: <stdout/stderr artifact pointer, proof file, test result path, screenshot path, or n/a>
diagnostics_summary: <short unique failure or proof summary>
no_claim: <what this result does not prove>
```

## Proof Routing Rules

- Do not cite absent files as proof.
- Do not use `docs/proof/lan-plan/` as the active proof root for current LAN work.
- Use manual-required or open status whenever a physical/device/network artifact has not been regenerated yet.
- Do not treat unit tests, source-matrix output, B1/B2 proof, or portal output as proof for another owner.
- Do not claim frozen `21-25` follow-on rows as current completion proof.
