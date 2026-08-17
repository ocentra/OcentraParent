<!-- agent-capsule -->

> Agent Capsule
> Plan: `eventing-plan`
> Doc: `Eventing Plan Test Proof Expectations`
> Kind: command/test selector.
> Read when: selected workpack asks which commands or proof artifacts are expected.
> Stop rule: run focused commands first; do not jump to full validation unless required by the workpack or PR_READY.
> Proves: command expectations only.
> Does not prove: implementation completion without matching artifacts.

<!-- /agent-capsule -->

# Eventing Plan Test Proof Expectations

## Proof root

```text
output/eventing-plan-proof/<workpack-file-stem>/
```

`docs/proof/eventing-plan/` holds the current WP12 route-proof manifest bundle
and the hand-authored WP06 durable proof manifest. New raw/generated
implementation output remains under `output/eventing-plan-proof/<workpack-file-stem>/`
and stays ignored unless a route explicitly selects a hand-authored durable
manifest.

## Common commands

Use the subset relevant to the selected workpack:

```bash
cargo test -p ocentra-eventing --test unit
cargo test -p ocentra-eventing --test contract
cargo test -p ocentra-eventing --test journal_replay
cargo test -p ocentra-eventing --test integration
cargo test -p ocentra-eventing --test version_skew
cargo test -p ocentra-eventing --tests
cargo lint-architecture crates/ocentra-eventing/src crates/ocentra-eventing/tests
npm run test --workspace @ocentra-parent/event-domain
npm run type-check --workspace @ocentra-parent/event-domain
cmd /c npm run test --workspace @ocentra-parent/agent-protocol-domain -- network-runtime-events.test.ts contracts.test.ts
cargo test -p ocentra-parent-agent-protocol child_domain_runtime_events --quiet
npm run type-check --workspace @ocentra-parent/agent-protocol-domain
cmd /c npm run test --workspace @ocentra-parent/agent-protocol-domain -- policy-control-audit-redaction.test.ts policy-control-delivery-read-model.test.ts contracts.test.ts
npm run lint:architecture -- --files packages/agent-protocol-domain/src/contracts.ts packages/agent-protocol-domain/src/policy-control-audit-redaction.ts packages/agent-protocol-domain/src/policy-control-delivery-read-model.ts
node scripts/test/eventing-rollout-proof.mjs
git diff --check -- docs/proof/eventing-plan docs/plans/eventing-plan
```

Run through `npm run agent:run --` when collecting proof if the logging/evidence wrapper is available.

## Command ownership notes

- `crates/ocentra-eventing` owns local event bus semantics: typed envelopes, event ids, idempotency, ordering, queue/dead-letter, request/response, journal/replay, topology, contract registry, local dispatch lifecycle, and testkit helpers.
- `schema-domain` owns neutral shared event/contract shapes when they cross package or plan boundaries.
- `packages/event-domain` is package-boundary metadata only unless a selected workpack names an explicit public surface.
- `crates/agent-protocol`, `crates/agent-service`, and `packages/agent-protocol-domain` prove protocol/service/TS mirror behavior only when selected.
- LAN, remote, network, AI, policy, enforcement, portal, data-custody, setup, payment, account, browser, app-game, screen, and tracking scopes run only when the selected workpack explicitly touches their handoff.

## Eventing E2E meaning

Do not use one proof family to claim the whole eventing path. For this plan, E2E has separate meanings:

```text
crate contract E2E: typed ids + event contract + envelope + registry/topology shape -> crate tests.
dispatch lifecycle E2E: publisher/subscriber/registrar -> local dispatch -> lifecycle/shutdown report.
queue/dead-letter E2E: duplicate/no-subscriber/overflow/TTL/retry -> queue/dead-letter metrics/report.
request-response E2E: request id + response contract -> completion/timeout/cancel/duplicate behavior.
journal/replay E2E: stored envelope -> append/hash chain -> replay/filter/version-skew proof.
protocol-shape E2E: Rust/TS protocol event shape -> serde/parser tests -> no service-delivery claim.
consumer-handoff E2E: consumer-owned event shape -> local publish/read-model bridge -> owning consumer proof path.
LAN mesh E2E: selected export/import -> LAN authority/transport validation -> local republish only after validation.
rollout gate E2E: accepted proof roots + carried blockers -> route proof -> no-claim and open workpack state.
```

A workpack can be complete for one tier while other tiers remain open. Record the non-claim instead of broad DONE.

## Structured harness logging expectations

Every eventing proof slice must preserve product-safe logging and local harness logging.

Product/runtime-safe logging:

```text
redact private payload bodies, child activity payloads, provider secrets, account tokens, raw policy/enforcement payloads, and consumer-private data unless a selected expectation explicitly allows the field
log event namespace/type, schema version, aggregate key, event id, idempotency key, correlation id, causation id, request id, queue state, retry/dead-letter state, journal/replay state, delivery route, consumer handoff state, blocker note, and no-claim boundary when safe
separate local bus, transport, protocol, consumer read-model, storage/custody, policy/enforcement, and portal/UI states
never treat crate logs, protocol logs, route docs, or consumer read-model logs as proof of another owner without a selected proof root
```

Local Codex/MCP/debug harness logging:

```text
prefer npm run agent:run -- <command> when available
store raw stdout/stderr by artifact pointer instead of pasting terminal walls into plan docs
write compact command summaries into 16-validation-commands.log
include run id, command id, workpack id, owner module, event family, exit code, result, artifact pointer, diagnostics summary, blocker note, and no-claim note when available
if the wrapper is unavailable, write wrapper: unavailable and keep the same compact command-log shape
```

## Current local proof roots

```text
docs/proof/eventing-plan/PLAN_PROOF_MANIFEST.md
docs/proof/eventing-plan/slice-01-envelope-version.md
docs/proof/eventing-plan/slice-02-ordering-replay.md
docs/proof/eventing-plan/slice-03-consumer-boundary.md
output/eventing-plan-proof/rollout-proof/
test-results/eventing-rollout-proof/
output/eventing-plan-proof/13-test-folder-layout-regression-audit/
test-results/eventing-test-folder-layout-regression-audit/
output/eventing-plan-proof/63-type-safety-source-gate/
test-results/eventing-type-safety-source-gate-proof/
output/eventing-plan-proof/66-76-source-safety/
output/eventing-plan-proof/67-lock-await/
output/eventing-plan-proof/68-fixture-parity/
```

## WP06 journal and enforcement handoff durable manifest

WP06's durable hand-authored manifest is retained as
`docs/proof/eventing-plan/wp06-00-enforcement-wp11-handoff.md`,
`wp06-01-journal-replay-proof.md`, `wp06-02-topology-lineage-proof.md`, and
`wp06-16-validation-commands.md`. The first artifact records the typed generic
journal/replay/idempotency handoff consumed by enforcement WP11; the next two
prove the journal and topology/lineage slice. Raw/generated output remains
ignored. This releases only the generic Eventing prerequisite: WP10 remains
open and WP11 still owns enforcement-specific durable journal proof.

## WP09 network consumer local evidence

WP09 uses one workpack-scoped, ignored/regenerable evidence root rather than
four stale one-off proof scripts:

```text
output/eventing-plan-proof/09-network-consumer-event-chain/
```

`proof-summary.json`, `00-source-snapshot.md`, and
`10-validation-commands.log` point to the raw `npm run agent:run --` artifacts
for the focused journal, protocol, core, ActivityStore, service, and
parent-runtime commands. Rows 57-61 use the real current test names recorded in
the implementation checklist. Portal command-boundary validation additionally
runs `apps/portal/tests/unit/portal-command-boundary.test.ts`, the portal
workspace tests/type-check, and the focused parent-assistant service-router
test.

This evidence proves durable network-owned observation publication/replay and
the direct-command authority negatives only. It does not require a fake shipped
queue/request-response caller, does not synthesize an AI-policy-enforcement
chain, and does not claim downstream consumer execution, broker/relay delivery,
CI, review, merge, or Network WP04 readiness.

## Required states

```text
envelope schema
idempotency
ordering/replay
retry/dead-letter
request-response
consumer contract
consumer handoff
LAN mesh handoff
redaction
manual-required blockers
proof-root presence
WP12 rollout-proof route restored without PR_READY claims
WP13 source-side test scaffold cleanup locally proved
WP11 scoped proof roots restored locally, package-wide agent-protocol-domain type-check passes again, and focused policy-control plus contracts validation is green
WP06 generic journal/topology proof and `00-enforcement-wp11-handoff.md` are retained in the tracked durable manifest; WP10 and enforcement-specific WP11 proof remain open
WP10 remains open until its proof roots and blocking validation exist
```

## Required negative states

```text
crate-local proof cannot claim cross-device transport
journal/replay proof cannot claim production retention/deletion/export
protocol shape proof cannot claim service delivery
consumer read-model proof cannot claim reusable crate readiness
provider or peer device cannot direct-publish policy/enforcement events
WP12/WP13 proof cannot close WP10
```
