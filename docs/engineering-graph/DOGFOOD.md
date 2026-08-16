# Engineering graph dogfood

This is a checked-in live snapshot of the graph against the existing
repository. It does not claim all workpacks are complete; it demonstrates that
the graph can answer the orchestration questions without chat history.

```text
Imported plans: 23
Imported workpacks: 679
Graph valid: 703 nodes, 705 edges
Review items: 34
Unindexed workpack files requiring review: 40
PLANNED: 376
READY: 0
ACTIVE: 1
BLOCKED: 9
VALIDATION: 292
DONE: 1
Implementation files: 2764
Test files: 1133
Reviewed workpack maps: 679
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

It reports all 23 plans and 679 workpack rows, with graph-derived workpack
state alongside live implementation/test topology under reviewed plan roots.
All 679 workpacks now have reviewed code/test ownership maps. Rows expose exact
reviewed roots (or an explicit reviewed no-code requirement); that topology is
not an acceptance, test-run, proof, CI, or merge certificate.

The policy-control slice is a useful dependency example:

```text
graph:inspect WP-policy-control-plane-plan-05-ask-parent-overrides
  state: blocked
  depends on: WP-policy-control-plane-plan-04-delivery-ack-audit

graph:why WP-policy-control-plane-plan-05-ask-parent-overrides
  WP-policy-control-plane-plan-04-delivery-ack-audit is blocked
```

The graph currently exposes no READY workpack: 376 remain planned pending
readiness/dependency review, 292 are in validation, and 9 are blocked. Remote
WP01, device-trust WP08, and network WP08 are in `validation` after focused
slices were replayed; all retain explicit runtime/no-claim boundaries. The
graph therefore refuses to authorize unreviewed `Open` rows while keeping
validation and blocked work visible. `DONE` is one: Eventing WP06 is promoted
only because its reviewed code/test map, durable proof bundle, checklist, and
explicit durable-proof override all exist. The graph still refuses every row
whose completion contract is incomplete; the remaining Eventing validation rows
retain their missing evidence instead of inheriting WP06's proof.

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
npm run graph:inspect WP-policy-control-plane-plan-05-ask-parent-overrides
npm run graph:why WP-policy-control-plane-plan-05-ask-parent-overrides
npm run graph:inspect WP-network-plan-08-control-catalog-reference-routing
```
