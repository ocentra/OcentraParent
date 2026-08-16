# Proof Trace Pipeline

## Purpose

The logging system must support proof collection, not only debugging.

A test should be able to enable proof logging, execute a UI/API/runtime path, then query structured logs to prove the expected route, action, command, event, and read-model chain happened.

Example:

```text
Playwright clicks button
  -> portal logs action clicked
  -> portal sends typed command
  -> local service logs command received
  -> runtime/domain logic logs decision branch
  -> event/read-model update logs completion
  -> portal logs rendered result
  -> proof query validates ordered trace
```

This lets logs become proof artifacts instead of loose terminal evidence.

## Reference idea

Games already used structured logs, run IDs, bridge run-start, NDJSON, DB ingest, and query commands as evidence. Parent should formalize this into a proof-trace mode that can support Playwright, local service tests, Rust tests, and future Cloudflare tests.

## Required proof mode

Add a scoped proof mode that tests can enable and disable.

Suggested controls:

```text
OCENTRA_PARENT_PROOF_TRACE=true|false
OCENTRA_PARENT_PROOF_TRACE_ID=<proof-id>
OCENTRA_PARENT_PROOF_TRACE_SCOPE=parent-portal|parent-agent|parent-test|parent-codex
OCENTRA_PARENT_PROOF_TRACE_SOURCES=portal,agent-service,tracking-core
OCENTRA_PARENT_PROOF_TRACE_LEVEL=info|debug
```

Proof mode must not globally turn on noisy logs forever. It is run-scoped.

Recommended lifecycle:

```text
start proof trace
  -> wipe selected proof scope if fresh=true
  -> set proof_id/run_id/correlation_id
  -> enable selected sources/levels
  -> execute test
  -> flush/ingest
  -> query/assert expected ordered trace
  -> write proof artifact
  -> turn proof mode off
```

## Required trace fields

Each proof-relevant log row should be able to carry:

```text
proof_id
run_id
test_id
correlation_id
causation_id
trace_step
source
context
message
event_type
route
action
command
status
expected_next
artifact_ref
```

Do not put raw private payloads in proof rows. Store redacted fields and artifact refs.

## Proof trace query service

Add query helpers that can validate an ordered chain.

Minimum operations:

```text
getProofTrace(proofId)
assertProofTraceContains(proofId, expectedSteps)
assertProofTraceOrder(proofId, orderedSteps)
getProofTraceGaps(proofId, expectedSteps)
```

Expected step shape:

```text
source
context?
event_type?
action?
command?
status?
contains?
```

The query must return compact proof output:

```text
matched steps
missing steps
out-of-order steps
unexpected error/warn rows
artifact refs
```

## Playwright proof use case

A Playwright test should be able to:

```text
1. Start proof trace with proof_id.
2. Navigate to route.
3. Click target UI control.
4. Wait for visible UI state.
5. Flush/ingest logs.
6. Query proof trace.
7. Assert expected path.
8. Save proof artifact.
```

Example expected trace:

```text
portal.route.opened
portal.action.clicked
portal.command.sent
local-api.command.received
agent-service.command.validated
tracking-core.decision.completed
local-api.response.sent
portal.read-model.received
portal.ui.rendered
```

## MCP relation

MCP should expose proof trace tools once the query service exists:

```text
get_proof_trace
get_proof_trace_gaps
query_proof_trace
```

These tools should return proof rows, not raw logs.

## Test relation

Proof trace should be usable by:

```text
Playwright E2E tests
Vitest portal tests
Rust service tests
agent-run validation wrappers
future Cloudflare/miniflare tests
```

## Acceptance criteria

```text
proof mode can be enabled per run/test
proof mode can be disabled after run/test
proof trace rows carry proof_id and correlation_id
Playwright or equivalent smoke test proves a click-to-result path through logs
query service can validate ordered expected steps
proof artifact records matched/missing/out-of-order steps
MCP/CLI can query proof traces compactly
retention/wipe prevents proof traces from polluting normal local logs forever
```
