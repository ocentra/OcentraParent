<!-- agent-capsule -->

> Agent Capsule
> Plan: `network-plan`
> Doc: `Network Plan Test Proof Expectations`
> Kind: command/test selector.
> Read when: selected workpack asks which commands or proof artifacts are expected.
> Stop rule: run focused commands first; do not jump to full validation unless required by the workpack or PR_READY.
> Proves: command expectations only.
> Does not prove: implementation completion without matching artifacts.

<!-- /agent-capsule -->

# Network Plan Test Proof Expectations

## Proof root

```text
output/network-plan-proof/<workpack-file-stem>/
```

## Common commands

Use the subset relevant to the selected workpack:

```bash
npm run build --workspace @ocentra-parent/network-domain
npm run test --workspace @ocentra-parent/network-domain
cargo test -p ocentra-network-evidence
cargo test -p ocentra-parent-agent-protocol network
cargo test -p ocentra-parent-agent-core network
cargo test -p ocentra-parent-agent-service network
npm run test --workspace @ocentra-parent/portal -- network
npm run lint:architecture -- --files packages/network-domain packages/agent-protocol-domain crates/ocentra-network-evidence crates/agent-protocol crates/agent-core crates/agent-service apps/portal docs/plans/network-plan
```

Run through `npm run agent:run --` when collecting proof if the wrapper is available.

## Command ownership notes

- `schema-domain` owns canonical shared TypeScript network contracts where shapes cross package, app, crate, or plan boundaries.
- `packages/network-domain` is package metadata/proof-consumer surface unless an explicit public export exists.
- `crates/ocentra-network-evidence` owns Rust evidence/proof helper behavior for packet, DNS, domain, flow, classifier, cascade, policy handoff, platform gates, and risk/audit helpers.
- `crates/agent-protocol`, `crates/agent-core`, and `crates/agent-service` prove protocol/runtime/service behavior only when the selected workpack names those surfaces.
- Portal, eventing, browser, screen, AI, policy, enforcement, LAN, data-custody, device-trust, and notification scopes run only when the selected workpack names the handoff.

## Network E2E meaning

Do not use one proof family to claim the whole network path. For this plan, E2E has separate meanings:

```text
contract/schema E2E: canonical network shape -> parser/contract/fuzz tests -> no runtime claim.
Rust evidence/proof E2E: network evidence helper -> Rust unit/fixture proof -> no service/portal/platform claim.
passive capture/fixture E2E: fixture packet or PCAP -> parser/flow output -> redaction/no-content boundary.
live capture E2E: platform capture source -> permission/capability state -> captured artifact -> manual-required notes.
classification/correlation E2E: evidence refs -> classifier/correlation output -> ambiguity and no exact-content claim.
cascade/parent surface E2E: service-backed evidence bundle -> parent read model/projection -> no portal-owned truth claim.
intervention adapter E2E: adapter capability -> authority check -> reversible lab action or manual-required state -> rollback/audit proof.
AI audit/risk E2E: fixture/risk input -> bounded evaluation -> AI runtime/provider no-claim unless proven by AI plan.
platform proof E2E: Windows/Android/Linux local proof or macOS/iOS external-platform constraint -> no production rollout claim.
rollout gate E2E: accepted proof roots + carried blockers -> remaining manual-required states -> no broad READY from checklist count.
```

A workpack can be complete for one tier while other tiers remain open. Record the non-claim instead of broad DONE.

## Structured harness logging expectations

Every network proof slice must preserve product-safe logging and local harness logging.

Product/runtime-safe logging:

```text
redact raw packet payloads, decrypted payloads, private content, private messages, full URLs when exact URL proof is not selected, credentials, tokens, and support-private diagnostics
log workpack, owner, platform, evidence kind, source kind, adapter kind, domain attribution state, process attribution state, browser exact-url state, screen fallback state, AI runtime state, policy handoff state, enforcement authority state, intervention state, rollback state, manual-required note, and no-claim boundary when safe
separate schema, Rust evidence, service/runtime, portal, eventing, browser, screen, AI, policy, enforcement, platform, and rollout proof states
never treat schema logs, fixture replay logs, catalog docs, or checklist rows as another owner proof without a selected proof root
```

Local Codex/MCP/debug harness logging:

```text
prefer npm run agent:run -- <command> when available
store raw stdout/stderr by artifact pointer instead of pasting terminal walls into plan docs
write compact command summaries into 16-validation-commands.log
include run id, command id, workpack id, owner module, platform, evidence kind, exit code, result, artifact pointer, diagnostics summary, manual-required note, and no-claim note when available
if the wrapper is unavailable, write wrapper: unavailable and keep the same compact command-log shape
```

## Required negative states

```text
unknown domain attribution visible
unknown process attribution visible
adapter unsupported visible
permission missing visible
stale evidence visible
mock evidence not product proof
private network-content claim blocked without explicit proof
schema proof not used as live capture proof
fixture replay not used as live capture proof
policy handoff not used as enforcement authority
adapter contract not used as production action readiness
control catalog not used as implementation proof
checklist count not used as completion proof
```
