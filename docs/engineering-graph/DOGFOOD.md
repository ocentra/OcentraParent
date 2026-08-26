# Engineering graph dogfood

This is a checked-in live snapshot of the graph against the existing
repository. It does not claim all workpacks are complete; it demonstrates that
the graph can answer the orchestration questions without chat history.

```text
Imported plans: all plan directories parsed from current source
Imported workpacks: all indexed workpack rows parsed from current source
Graph valid: 705 nodes, 742 edges
Review items: 34
Unindexed workpack files requiring review: 40
PLANNED: 368
READY: 2
ACTIVE: 2
BLOCKED: 21
VALIDATION: 287
DONE: 1
Implementation files: 2938
Test files: 1145
Reviewed workpack maps: 680
```

The joined report is the canonical operator view:

```powershell
npm run graph:report
npm run --silent graph:report -- --json
npm run graph:report -- PLAN-policy-control-plane-plan
```

For the complete plan/workpack handoff matrix use:

```powershell
npm run graph:matrix
npm run graph:matrix -- --state validation
npm run --silent graph:matrix -- --json
```

When the graph has no READY work, `graph:next` distinguishes the legal READY
set from the unblocked validation/review queue. The queue is a repair or
evidence handoff, not authorization to bypass the READY gate.

For the code-first pass, use the explicit phase query:

```powershell
npm run graph:next -- --phase implementation
```

Its rows authorize implementation-source edits only. Normal READY, tests,
proof, PR readiness, service activation, and DONE remain unchanged. An existing
dependency edge still requires DONE unless that one reviewed edge opts into
`implementationGate: "reviewed-implementation"` and its predecessor has exact
reviewed implementation evidence. No dependency edge is opted in merely because
source files exist.

It reports all imported plans and indexed workpack rows, with graph-derived
workpack state alongside live implementation/test topology under reviewed plan
roots.
680 workpacks have reviewed code/test ownership maps. The unmapped tracking
runtime-ingress workpack remains explicit unknown ownership rather than
inheriting a plan-wide count. Mapped rows expose exact reviewed roots (or an
explicit reviewed no-code requirement); that topology is not an acceptance,
test-run, proof, CI, or merge certificate.

Graph queries are source-authoritative: report, matrix, and implementation-phase
queries regenerate the graph and reject a hand-edited `graph.json` before
returning authorization. Override files are strict schema inputs; unknown keys,
duplicate records, malformed arrays, unknown IDs, unsupported edge gates, and
invalid evidence fail bootstrap. Implementation evidence must be a
repository-contained regular source file with an allowed executable extension;
directories, traversal paths, documentation, and test paths are rejected.

The policy-control slice is a useful dependency example:

```text
graph:inspect WP-policy-control-plane-plan-05-ask-parent-overrides
  state: blocked
  depends on: WP-policy-control-plane-plan-04-delivery-ack-audit

graph:why WP-policy-control-plane-plan-05-ask-parent-overrides
  WP-policy-control-plane-plan-04-delivery-ack-audit is blocked
```

The strict graph currently exposes exactly two READY workpacks: Device Trust
WP01 and Eventing WP11. Another 368 remain planned, 287 are in validation, 21
are blocked, and 2 are active. The graph therefore refuses to authorize
unreviewed `Open` rows while keeping validation and blocked work visible.
`DONE` is one: Eventing WP06 is promoted only because its reviewed code/test
map, durable proof bundle, checklist, and explicit durable-proof override all
exist. The graph still refuses every row whose completion contract is
incomplete; the remaining Eventing validation rows retain their missing
evidence instead of inheriting WP06's proof.

The policy WP02 preview slice is also now correctly shown as `validation`: PR
#615 is merged, but the authoring write boundary, remaining proof, and complete
acceptance contract are still open.

The migration audit also finds 40 Markdown files under `workpacks/` that are
not linked by an index row. They remain review items rather than silently
becoming graph workpacks; most are README, legacy, proposal, or support files.

The queries used were:

```powershell
npm run graph:bootstrap -- --write
npm run graph:validate
npm run graph:status
npm run graph:ready
npm run graph:parallel
npm run graph:next -- --phase implementation
npm run graph:inspect WP-policy-control-plane-plan-05-ask-parent-overrides
npm run graph:why WP-policy-control-plane-plan-05-ask-parent-overrides
npm run graph:why WP-device-trust-bootstrap-plan-03-parent-step-up-auth -- --phase implementation
npm run graph:inspect WP-network-plan-08-control-catalog-reference-routing
```
