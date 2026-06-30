<!-- agent-capsule -->

> Agent Capsule
> Plan: `lan-plan`
> Doc: `LAN Plan Test Proof Expectations`
> Kind: command/test selector.
> Read when: selected workpack asks which commands or proof artifacts are expected.
> Stop rule: run focused commands first; do not jump to full validation unless required by the workpack or PR_READY.
> Proves: command expectations only.
> Does not prove: implementation completion without matching artifacts.

<!-- /agent-capsule -->

# LAN Plan Test Proof Expectations

## Proof root

```text
output/lan-plan-proof/<workpack-file-stem>/
```

## Current LAN test truth

- As of 2026-06-17, `packages/lan-domain/tests/unit` was the only populated TS-side LAN test category on this branch/worktree. That is historical migration residue only, and it must not be reactivated as current LAN ownership.
- All current product logic, contracts, runtime truth, read models, and proof truth are Rust-owned. TS tests only prove presentation/UI behavior or generated edge consumption.
- Empty or placeholder directories under any LAN test surface, including historical `packages/lan-domain/tests/**`, do not count as integration, contract, e2e, property, security, observability, release, or load coverage.
- LAN closure requires real organized test files under the owning Rust crate `tests/` surfaces.
- UI presentation tests may supplement selected presentation slices only; they never replace Rust-owned LAN proof.
- Inline source-owned tests do not count as plan closure. Follow-on cleanup should move LAN logic coverage into organized crate test groups.

## Historical B2 residue

The old TS-side B2 residue stays documented as historical proof context only.
Do not use legacy `packages/lan-domain` test commands as current LAN validation
or as a fallback execution path.

## Common commands

Use the subset relevant to the selected workpack:

```bash
cargo test -p ocentra-lan-core lan
cargo test -p ocentra-parent-agent-protocol lan
cargo test -p ocentra-parent-agent-service lan
npm run test --workspace @ocentra-parent/portal -- lan
npm run lint:architecture -- --files crates/lan-core crates/agent-protocol crates/agent-service apps/portal docs/plans/lan-plan
cargo lint-architecture crates/lan-core crates/agent-protocol crates/agent-service
```

Manual/operator cross-checks when the selected proof scope explicitly requires
them:

```bash
nmap -sn -PR <subnet>
nmap -sS -sV -O --osscan-limit --top-ports 100 <subnet>
avahi-browse -art
dns-sd -B _services._dns-sd._udp local
```

If a selected workpack explicitly audits historical TS migration residue, keep
that audit fenced and do not treat it as active LAN ownership, completion
proof, or a default validation surface.

Run through `npm run agent:run --` when collecting proof if the logging/evidence wrapper is available.

## Command ownership notes

- The Rust schema crate owns canonical LAN contract/read-model/proof shapes when shapes cross crate/app/plan boundaries.
- Rust-owned crates also own LAN product logic, runtime truth, read models, and proof truth.
- Historical `packages/lan-domain` residue may still exist on disk as migration
  scaffolding or proof-consumer helpers when explicitly selected. It does not
  own current LAN business logic, read models, runtime truth, or proof truth.
- `crates/agent-protocol` and `crates/agent-service` prove protocol/service/read-model behavior only when selected.
- `apps/portal` or another presentation surface may prove UI rendering only when selected; that rendering remains supporting evidence and is not LAN truth proof.
- `eventing-plan` proves local event bus semantics only; eventing proof is not LAN transport or topology proof.
- Account, device-trust, remote, setup, package distribution, Cloudflare, policy, enforcement, and child runtime scopes run only when the selected workpack names the handoff.

## LAN E2E meaning

Do not use one proof family to claim the whole LAN path. For this plan, E2E has separate meanings:

```text
contract/schema E2E: canonical LAN shape -> parser/contract tests -> no packet/runtime claim.
evidence/device-record E2E: LAN source row -> evidence/device record -> weak/strong/manual classification.
interface/neighbor E2E: platform interface/neighbor source -> normalized LAN evidence -> platform-specific proof state.
active/passive discovery E2E: ARP/sweep/listener packet source -> bounded parser/runtime proof -> stale/offline/manual states.
passive collector E2E: ARP/DHCP/mDNS/SSDP/WS-Discovery/LLMNR/NetBIOS/SNMP response -> bounded evidence row -> no identity escalation without corroboration.
service-probe/vendor E2E: curated safe-port probe -> sanitized banner/title/header/redirect/certificate evidence -> no trust/assignment claim and no full-scan/page-crawl behavior.
classifier/installability E2E: weighted evidence set -> explicit classification reasons + installability state -> unknown/manual-required when evidence is weak or contradictory.
merge/classification E2E: multiple sources -> dedupe/confidence/explanation -> no child identity claim without signed/trusted proof.
household-store E2E: canonical device state -> assignment/revocation/read-model persistence -> prior-scan continuity snapshot -> wrong-household/device negatives.
read-model/event E2E: service-backed state -> read-model/event stream -> replay/duplicate/stale handling -> portal projection boundary.
advertisement E2E: parent/child advertisement -> source proof -> no signed child-agent claim unless signed hello/heartbeat proof exists.
signed hello/heartbeat E2E: signed artifact -> family/route binding -> expiry/replay/revocation rejection -> physical/manual state.
portal projection E2E: service-backed LAN state -> portal render -> no portal-owned LAN truth.
physical household E2E: at least two real devices + router/firewall/local permission proof -> manual artifact set.
rollout gate E2E: accepted proof roots + carried blockers -> manual-required gap register -> no-claim boundaries.
```

A workpack can be complete for one tier while other tiers remain open. Record the non-claim instead of broad DONE.

## Structured harness logging expectations

Every LAN proof slice must preserve product-safe logging and local harness logging.

Product/runtime-safe logging:

```text
redact private addresses when not needed, raw packet payloads unless selected proof requires fixtures, child private activity, credentials, pairing secrets, signed private material, and support-private diagnostics
log workpack, source kind, platform, device ref, household ref, route id, evidence ref, discovery state, trust state, reachability state, signed hello state, heartbeat state, event-stream state, portal projection state, physical topology state, manual-required note, and no-claim boundary when safe
log interface, default gateway, DNS server, DHCP server, subnet, broadcast address, IPv6 prefix, classification confidence, and install-eligibility state when those fields are part of the selected proof
separate schema, packet, service, portal, eventing, trust, remote, package, and physical/manual proof states
never treat unit logs, source-matrix logs, portal logs, or schema logs as proof of another owner without a selected proof root
```

Local Codex/MCP/debug harness logging:

```text
prefer npm run agent:run -- <command> when available
store raw stdout/stderr by artifact pointer instead of pasting terminal walls into plan docs
write compact command summaries into 16-validation-commands.log
include run id, command id, workpack id, owner module, platform, source kind, exit code, result, artifact pointer, diagnostics summary, manual-required note, and no-claim note when available
if the wrapper is unavailable, write wrapper: unavailable and keep the same compact command-log shape
```

## Required negative states

```text
unsupported route visible
stale state visible
offline state visible
wrong household/device state visible
manual-required state visible
ICMP-only reachability not used as device truth
MAC vendor alone not used as platform claim
open port/banner/title/certificate not used as child confirmation
installability not claimed without an allowed path or explicit manual-required state
previous-scan JSON not used as permanent identity override
single-machine proof not used for multi-device claim
unit tests not used as integration/e2e/security coverage
source matrix not used as physical discovery proof
portal projection not used as LAN truth proof
B1/B2 proof not used as signed hello/heartbeat/service/physical/relay proof
active workpacks 21-25 require their own current completion proof and test truth
```
