<!-- agent-capsule -->

> Agent Capsule
> Plan: `eventing-plan`
> Doc: `Eventing Plan Proof Index`
> Kind: proof artifact router.
> Read when: selected workpack needs proof paths or PR_READY/DONE proof validation.
> Stop rule: use only the proof root for the selected workpack.
> Proves: proof location routing only.
> Does not prove: implementation completion by itself.
> Proof rule: proof artifacts are valid only after focused commands run or precise blockers are recorded.

<!-- /agent-capsule -->

# Eventing Plan Proof Index

## Proof root

```text
output/eventing-plan-proof/<workpack-file-stem>/
```

`docs/proof/eventing-plan/` is accepted for the current WP12 route-proof bundle
and the hand-authored WP06 durable manifest. Historical references still do not
close runtime work by themselves.

## WP06 prerequisite durable manifest

```text
docs/proof/eventing-plan/wp06-00-enforcement-wp11-handoff.md
docs/proof/eventing-plan/wp06-01-journal-replay-proof.md
docs/proof/eventing-plan/wp06-02-topology-lineage-proof.md
docs/proof/eventing-plan/wp06-16-validation-commands.md
```

These tracked, hand-authored artifacts retain the WP06 proof pass. The first
records the typed generic mechanics handoff and the remaining entries retain
the focused journal/topology evidence. Raw/generated output remains ignored.
This releases only the Eventing generic prerequisite; enforcement still owns
its adapter, authority, audit, rollback, and side-effect proof. WP10 remains
open and is not satisfied by this bundle.

## Expected route-closure proof bundle

```text
docs/proof/eventing-plan/slice-01-envelope-version.md
docs/proof/eventing-plan/slice-02-ordering-replay.md
docs/proof/eventing-plan/slice-03-consumer-boundary.md
output/eventing-plan-proof/rollout-proof/proof-summary.json
test-results/eventing-rollout-proof/proof.json
output/eventing-plan-proof/rollout-proof/pr-done-report.md
output/eventing-plan-proof/rollout-proof/command-logs/
```

If any path above is missing, keep WP12 open and record the blocker in `PLAN_STATE.md` and `NEXT_ACTIONS.md`. Historical doc references do not prove route closure by themselves.

## Current WP11 local proof roots

```text
output/eventing-plan-proof/63-type-safety-source-gate/proof-summary.json
test-results/eventing-type-safety-source-gate-proof/proof.json
output/eventing-plan-proof/66-76-source-safety/proof-summary.json
output/eventing-plan-proof/67-lock-await/proof-summary.json
output/eventing-plan-proof/68-fixture-parity/proof-summary.json
```

## Command log format

```text
command: <exact command>
exit: <code>
result: pass | fail | blocked
artifact: <path or n/a>
notes: <short note>
```

If blocked:

```text
blocker:
required environment:
why this does not prove completion:
next command:
```

## Structured proof metadata

For new proof artifacts and new command-log entries, include structured metadata when available:

```text
plan: eventing-plan
workpack: <workpack id and name>
owner: ocentra-eventing | schema-domain | event-domain | agent-protocol | agent-service | agent-protocol-domain | lan-handoff | remote-handoff | network-handoff | ai-handoff | policy-handoff | enforcement-handoff | portal-handoff | data-custody-handoff | docs-only
event_namespace: <namespace or n/a>
event_type: <event type or n/a>
schema_version: <schema version or n/a>
aggregate_key: <aggregate key or n/a>
event_id_state: generated | validated | rejected | not-tested | n/a
idempotency_state: accepted | duplicate-rejected | missing-rejected | not-tested | n/a
correlation_id_state: present | missing | rejected | not-tested | n/a
causation_id_state: present | missing | rejected | not-tested | n/a
request_response_state: requested | completed | timed-out | cancelled | duplicate-completion-rejected | not-tested | n/a
queue_state: enqueued | drained | no-subscriber | overflowed | ttl-expired | not-tested | n/a
retry_dead_letter_state: retried | dead-lettered | not-tested | n/a
journal_replay_state: appended | replayed | hash-checked | version-skew-checked | corrupted-rejected | not-tested | n/a
delivery_route_state: local-only | transport-required | blocked | manual-required | not-applicable
consumer_handoff_state: not-tested | validated | rejected | local-republish-only | blocked | manual-required | n/a
transport_boundary: local-bus-only | lan-handoff | remote-handoff | service-handoff | not-applicable
redaction_state: redacted | blocked | not-tested | n/a
run_id: <wrapper run id or n/a>
command_id: <wrapper command id or n/a>
command: <exact command>
exit: <code>
result: pass | fail | blocked
artifact: <stdout/stderr artifact pointer, proof file, test result path, or n/a>
diagnostics_summary: <short unique failure or proof summary>
manual_required_note: <explicit manual-required gap or n/a>
no_claim: <what this result does not prove>
```

The command log is a compact index, not a raw terminal transcript. Store command output, test reports, proof JSON, route-sync reports, or long failure dumps under artifact paths and reference them by pointer. If no wrapper exists, write `run_id: n/a` and `command_id: n/a`; do not omit the proof row.

## Runtime and local harness split

Runtime/product-safe proof must show event identity, schema, idempotency, queue, journal/replay, request/response, delivery route, consumer handoff, redaction, and no-claim boundaries. Local harness proof may include richer diagnostics, but it still stores logs by pointer and keeps plan docs compact.

```text
runtime-safe: no private payload bodies, child activity payloads, provider secrets, account tokens, raw policy/enforcement payloads, or consumer-private data unless a selected expectation explicitly allows the field.
local harness: enough file/line/command/artifact/event/queue/journal/handoff context for Codex/MCP/humans to debug without reading terminal walls.
```

## No-claim language

Do not claim:

```text
cross-device transport ready
LAN mesh ready
remote relay ready
service delivery ready
policy/enforcement behavior ready
AI behavior ready
portal behavior ready
production durability/retention ready
consumer product behavior ready
WP10 ready
PR_READY
```

unless the selected proof root proves the exact claim and WP12/WP10 aggregation rules allow it.
