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

## Current Checkout Truth - 2026-08-15

No generated `output/lan-plan-proof/`,
`output/playwright/lan-source-matrix-plan-completion/`, or
`test-results/v0-9-lan-source-matrix-plan-completion/` artifact is present in
this clean checkout. The only retained LAN proof source currently tracked is
`docs/proof/lan-plan/25-rollout-checklist-and-pr-gate/01-rollout-gate-truth.md`.
Every later `Files present` list in this document is therefore a historical or
expected artifact manifest, not current proof. Regenerate and re-check files
before moving any Phase 3 proof/checklist row.

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

`B1` proves local mechanical LAN slice proof only. Artifact names inside this root still carry historical `lan-domain` labels. It does not claim portal screenshots, service/runtime-backed proof, physical household readiness, real signed child hello/heartbeat artifacts, or cloud relay implementation.

`B2` proof root:

```text
output/lan-plan-proof/02-lan-b2-test-truth-repair/
```

Files present for `B2`:

- `00-b2-test-truth-note.md`

`B2` proves LAN test-category truth only. It does not claim new LAN runtime behavior, new integration/e2e/security coverage, or closure of protocol, service, portal, or physical proof gaps.

`01 Contract Boundary And Bridge Validation` proof root:

```text
output/lan-plan-proof/01-contract-boundary-and-effect-schemas/
```

Files present for the current local Rust contract slice:

- `01-local-validation.md`

This proof root currently proves local Rust LAN contract closure for the
selected protocol boundary: fail-closed schema-version deserialization,
explicit signed-child and mDNS advertisement contract families, discovery
evidence and browser read-model drift coverage, and focused `agent-protocol`
contract plus architecture validation. It does not claim downstream portal UI
proof, service-runtime parity outside the selected contract family, or physical
household/manual network artifacts.

`02 Evidence Model And Device Record` proof root:

```text
output/lan-plan-proof/02-evidence-model-and-device-record/
```

Files present for the current local Rust evidence slice:

- `01-local-validation.md`

This proof root currently proves local Rust evidence-model durability and
projection behavior: distinct source-backed evidence rows persist in the
trusted-device registry, paired-child/router scan-truth enriches the existing
device row instead of creating a duplicate suppression candidate, and the
service add-device read model remains aligned with the durable registry shape.
It does not claim broader all-source merge closure, portal UI proof, or
physical household/manual network artifacts.

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
- `07-dev-web-bridge-and-devices-runtime-parity.md`
- `08-lan-source-matrix-visual-proof.md`
- `06-portal-source-matrix-proof.md`
- `11-backend-lan-runtime-stream.md`

The historical proof root described below covered only the parent Rust bridge
LAN-sample removal, inventory-backed read-model serialization path, and the typed Tauri
host-subscription bridge that delivers deduplicated route snapshots into the
product shell. It now also references refreshed Rust-backed Playwright artifacts
under `output/playwright/lan-source-matrix-plan-completion/` for `/devices`,
policy-target persistence, and Activity or Network evidence rendering, plus the
dev-web bridge parity note for the `4777/4778/4779` desktop lane. The
supporting regenerated browser proof artifact was historically written under
`test-results/v0-9-lan-source-matrix-plan-completion/proof.json`. Generated
output is not retained verifier source, and the aggregate source-matrix runner
named by WP20 is absent from the current repository. The tracked code also has
the focused backend `agent-service` LAN runtime event-chain stream in focused
tests. It does not yet prove full
backend-to-real-Tauri-`AppHandle`-to-portal-listener delivery, the missing
aggregate verifier programs, broader network-flow evidence breadth, or
physical household/manual network artifacts.

`15 Household Device Store` proof root:

```text
output/lan-plan-proof/15-household-device-store/
```

Files present for the current local Rust store slice:

- `01-local-validation.md`

This proof root currently proves local Rust known-household device durability:
trusted-registry persistence, migration/fail-closed handling, stale and offline
restart restoration, previous-scan continuity as weak evidence only, router
visibility without enrollability, and scan-suppression truth reuse. It does not
claim physical multi-device topology, router/firewall reachability, signed
child hello/heartbeat proof, or replayable event-stream closure outside the
durable store/read-model path.

`06 Bounded ARP Sweep` proof root:

```text
output/lan-plan-proof/06-bounded-arp-sweep/
```

Files present for the current smart-scan suppression safety fix:

- `01-ip-reuse-suppression-fix.md`

This proof root currently proves only that bounded active refresh no longer
suppresses a reused IP on stale child truth alone and still suppresses the live
gateway/default-route path. It now also proves packet-IO abstraction,
duplicate-reply handling, persisted scan-plan suppression metadata, and the
focused service/runtime passive-after-probe path. It does not claim physical
household/manual network artifacts or broader rollout readiness by itself.

`03 Interface Detection` proof root:

```text
output/lan-plan-proof/03-interface-detection/
```

Files present for the current local Rust interface slice:

- `00-interface-map-proof.md`

This proof root currently proves local Rust interface selection and propagation:
gateway, DNS, DHCP, broadcast, and IPv6-prefix capture, explicit manual
interface selection, ignored-interface reason-code proof, persisted scan-plan
metadata, discovery-evidence selected-interface attribution, and scan-history
runtime serialization. It does not claim macOS live parity, physical household
proof, or broader rollout readiness by itself.

`05 Targeted ARP Checks` proof root:

```text
output/lan-plan-proof/05-targeted-arp-checks/
```

Files present for the current targeted-host refresh slice:

- `00-validation-note.md`

This proof root currently proves only the bounded targeted-ARP slice:
selected-interface/local-subnet gating, response/no-response/malformed
evidence handling, repeat throttling, packet-IO abstraction coverage, and
scan-plan metadata persistence. It does not claim child ownership, control
authority, portal parity, or broader discovery closure.

`07 Passive Discovery Listeners` proof root:

```text
output/lan-plan-proof/07-passive-discovery-listeners/
```

Files present for the current local passive-discovery slice:

- `01-local-validation.md`

This proof root currently proves local Rust passive-discovery ingestion for
ARP weak hints, mDNS, SSDP, WS-Discovery, LLMNR, NetBIOS, Ocentra beacon
updates, and allowed SNMP response history bridging, plus explicit bounded
listener lifecycle and malformed or oversized payload rejection. It does not
claim a real long-running DHCP listener, physical packet capture proof, or
broader platform/manual network artifacts.

`11 Light Service Probing` proof root:

```text
output/lan-plan-proof/11-light-service-probing/
```

Files present for the current bounded service-probe slice:

- `00-validation-note.md`
- `01-allowed-snmp-passive-history-bridge.md`

This proof root currently proves only bounded weak service identity enrichment:
sanitized HTTP/HTTPS/TLS hints, no-crawl behavior, trusted/router suppression,
reused-IP handling, scan-history persistence, weak route-snapshot projection,
explicit selected-interface scope and match gating, bounded WSD and SNMP
identity-query delivery, and passive-history bridging for allowed SNMP replies.
It does not claim OS fingerprinting, child ownership, controllable-device
authority, or live household/manual WSD or SNMP proof.

`12 OUI And Vendor Lookup` proof root:

```text
output/lan-plan-proof/12-oui-vendor-lookup/
```

Files present for the current local Rust vendor-evidence slice:

- `01-local-validation.md`

This proof root currently proves local Rust MAC parsing and weak-only
vendor/randomized-MAC enrichment: normalized MAC parsing, OUI lookup,
locally-administered confidence downgrade, multicast/malformed rejection, and
read-model surfacing of the randomized/private warning without owner, OS, or
child-identity escalation. It does not claim richer cross-source classifier
closure, physical household proof, or installability truth by itself.

`13 Merge And De-Duplication Engine` proof root:

```text
output/lan-plan-proof/13-merge-deduplication-engine/
```

Files present for the current local Rust merge slice:

- `01-local-validation.md`

This proof root currently proves local Rust household merge behavior for the
install-id/pairing-id strong-key path across protocol, service, network
inventory, and canonical read-model merging, plus the rerun randomized-MAC
non-merge regression. It does not claim broader weak-evidence dedupe closure,
portal UI proof, or physical household/manual network artifacts.

`14 Explainable Classification` proof root:

```text
output/lan-plan-proof/14-explainable-classification/
```

Files present for the current local Rust classification slice:

- `01-local-validation.md`

This proof root currently proves local Rust weighted LAN classification only:
router or unsupported or unknown visibility, explicit reasons and confidence,
scanner-only non-child boundaries, and focused `lan-core` plus
`agent-service` validation for the canonical household-device projection. It
now also has focused portal label-rendering proof and refreshed `/devices`
Rust-snapshot browser proof. It does not claim broader installability honesty
across the full product path or physical household/manual network artifacts.

`08 mDNS And DNS-SD Discovery` proof root:

```text
output/lan-plan-proof/08-mdns-dns-sd-discovery/
```

Files present for the current local Rust discovery slice:

- `01-local-validation.md`

This proof root currently proves local Rust mDNS/DNS-SD discovery validation:
selected service enumeration/types, PTR/SRV/TXT/A/AAAA parsing, hostile-name
sanitization, and hint-only agent handling. It does not claim packet capture,
physical multi-device discovery, or signed-child confirmation.

`09 SSDP And UPnP Discovery` proof root:

```text
output/lan-plan-proof/09-ssdp-upnp-discovery/
```

Files present for the current local Rust discovery slice:

- `01-local-validation.md`

This proof root currently proves local Rust SSDP/UPnP validation: bounded
`M-SEARCH`, descriptor safety rules, router-visible/non-enrollable handling,
and malformed/timeout/oversize rejection. It does not claim packet capture,
physical LAN topology proof, or controllable-device authority.

`10 NetBIOS, LLMNR, And Reverse DNS` proof root:

```text
output/lan-plan-proof/10-netbios-llmnr-reverse-dns/
```

Files present for the current local Rust name-evidence slice:

- `01-local-validation.md`

This proof root currently proves local Rust hostname-evidence behavior for
NetBIOS, LLMNR, and reverse DNS: names stay weak-only, malformed or unsafe
values are rejected, duplicate-name fixtures remain below auto-merge
thresholds, and hostname signals do not confirm child identity or assignment.
It does not claim portal UI proof, packet capture, or physical household/manual
network artifacts.

`17 Parent Child mDNS Advertisements` proof root:

```text
output/lan-plan-proof/17-parent-child-mdns-advertisements/
```

Files present for the current local Rust advertisement slice:

- `01-local-validation.md`

This proof root currently proves local Rust advertisement contract/packet
encoding/lifecycle validation only. It does not claim signed-child discovery
confirmation or broader physical/manual platform proof.

`18 Signed Child Hello And Heartbeat` proof root:

```text
output/lan-plan-proof/18-signed-child-hello-heartbeat/
```

Files present for the current local Rust/core slice:

- `01-local-validation.md`

This proof root currently proves local Rust/core signed hello and heartbeat
validation only: verifier rejection states, fail-closed schema or message
drift, future-safe capability passthrough, unpaired runtime rejection, and
stale/offline/manual-required read-model projection. It now also has focused
portal/manual-required label proof for the selected-device diagnostics path. It
does not claim physical Android/iOS artifacts or real second-device household
proof.

`19 Assignment, Revocation, And Audit` proof root:

```text
output/lan-plan-proof/19-assignment-revocation-audit/
```

Files present for the current local Rust command slice:

- `01-local-validation.md`

This proof root currently proves local Rust assignment/revoke/audit behavior:
route trust and selected-route recovery, restart readback of the live child
canonical id, rename evidence, ignore/restore decisions, multi-device route
selection, revoke audit evidence, and portal LAN-target command routing. It now
also includes the selected-route local-network child-target dispatch fix and
supporting browser proof for Trust/Ignore/Restore/Revoke, Browser Settings,
AI Runtime, Entitlements, Activity, `/devices`, and Policy Network target
persistence. It does not claim physical two-device route topology,
signed-child/manual route artifacts, or broader manual replay/event-stream
closure.

`21 Contract Boundary And Domain Schemas` proof root:

```text
output/lan-plan-proof/21-contract-boundary-and-domain-schemas/
```

Files present for the current local Rust contract slice:

- `01-local-validation.md`

This proof root currently proves the Rust-owned household/setup contract family
and organized contract tests in `crates/family-identity-core`. It does not
claim downstream runtime, portal, or physical LAN readiness by itself.

`22 Current State And Gap Map` proof root:

```text
output/lan-plan-proof/22-current-state-and-gap-map/
```

Files present for the current truth-sync slice:

- `01-local-validation.md`

This proof root currently proves the LAN gap-map/status row is synchronized to
the current Rust/runtime/UI validation truth: rows `21` and `24` are locally
complete with their own proof, rows `23` and `25` remain partial/manual, and
physical/router/firewall/mobile/manual gaps stay explicit. It does not claim
new DB/runtime implementation, physical two-device topology, or broad PR-ready
state by itself.

`23 Pairing And Route Proof` proof root:

```text
output/lan-plan-proof/23-pairing-and-route-proof/
```

Files present for the current local Rust route-proof slice:

- `01-local-validation.md`

This proof root currently proves local Rust route-custody, stale/offline,
rejection, revoke, and read-model projection validation. It does not claim
physical two-device household topology proof or router/firewall manual proof.

`24 Portal UX And First-Run Handoff` proof root:

```text
output/lan-plan-proof/24-portal-ux-and-first-run-handoff/
```

Files present for the current local portal slice:

- `01-local-validation.md`

This proof root currently proves focused portal/runtime/start-route validation,
including portal unit tests, portal build, and the exact Windows
`setup-first-run-ui-proof` Playwright command. It does not claim physical
multi-device LAN readiness, router/firewall conditions, or non-Windows/manual
platform artifacts.

`25 Rollout Checklist And PR Gate` proof root:

```text
docs/proof/lan-plan/25-rollout-checklist-and-pr-gate/
```

Files present for the current rollout truth-sync slice:

- `01-rollout-gate-truth.md`
- `16-validation-commands.log`

These two WP25 artifacts are tracked source-of-record proof documents. Generated
or rerun output remains local or CI-owned and must not be committed under
`output/`.

This proof root now proves the active rollout-gate truth through 2026-07-19,
including Rust-owned replay validation and status-history state/latest binding,
safe host-owned rejection warnings, recent/stale persisted-cache parity, the
Tauri host decision for unseen event IDs with a stable snapshot, and isolated
portal-state replay binding plus stable newest-128 buffering. These are separate
automated seams. The root does not prove a real backend-to-Tauri-`AppHandle`-to-
portal-listener chain or manual runtime behavior and does not claim PR-ready or
broad DONE while physical multi-device, router/firewall, signed-artifact,
restart, and manual topology artifacts remain open.

If a proof script emits `test-results/.../proof.json`, the workpack proof pack must reference that file and must not imply the artifact exists until it has been regenerated on this branch/worktree.

## Proof Paths Explicitly Not Claimed By Current Slices

These previously cited paths are absent on disk as of 2026-06-28 and are not current proof:

- `output/lan-plan-proof/15-household-device-store/devices-identity-routing-proof.md`
- `output/lan-plan-proof/15-household-device-store/06-ui-snapshots/devices-identity-persisted.png`
- `output/lan-plan-proof/15-household-device-store/06-ui-snapshots/devices-update-gated.png`
- `docs/proof/lan-plan/PLAN_PROOF_MANIFEST.md`

## Structured proof metadata

For new proof artifacts and command-log entries, include structured metadata when available:

```text
plan: lan-plan
workpack: <workpack id and name>
owner: rust-schema | rust-lan-core | rust-agent-protocol | rust-agent-service | presentation-support | eventing-handoff | account-handoff | device-trust-handoff | remote-handoff | distribution-handoff | cloudflare-handoff | docs-only
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

`presentation-support` is for non-authoritative UI screenshots, browser checks,
or display snapshots that hang off an already-selected Rust proof owner. It is
never a substitute for Rust-owned LAN proof.

## Proof Routing Rules

- Do not cite absent files as proof.
- Do not use `docs/proof/lan-plan/` as the active proof root for current LAN work.
- Use manual-required or open status whenever a physical/device/network artifact has not been regenerated yet.
- Do not treat unit tests, source-matrix output, B1/B2 proof, or portal output as proof for another owner.
- Do not treat `presentation-support` artifacts as authoritative LAN proof.
- Do not claim follow-on workpacks complete from adjacent proof roots; each
  active row needs its own current proof or explicit open/manual-required
  status.
