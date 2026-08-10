# Engineering graph dogfood

This is the first live query of the graph against the existing repository. It
does not claim all workpacks are complete; it demonstrates that the graph can
answer the orchestration questions without chat history.

```text
Imported plans: 23
Imported workpacks: 679
Graph valid: 703 nodes, 705 edges
Review items: 24
PLANNED: 454
READY: 0
ACTIVE: 2
BLOCKED: 9
VALIDATION: 213
DONE: 1
Implementation files: 2801
Test files: 1175
```

The joined report is the canonical operator view:

```powershell
npm run graph:report
npm run graph:report -- --json
npm run graph:report -- PLAN-policy-control-plane-plan
```

It reports all 23 plans and 679 workpack rows, with graph-derived workpack
state alongside live implementation/test topology under reviewed plan roots.
Nine focused workpacks now also have explicit reviewed code/test maps; those
rows expose exact paths, while every unmapped row remains
`unknown-workpack-ownership`. Neither topology mode is an acceptance/CI/merge
certificate.

The policy-control slice is a useful dependency example:

```text
graph:inspect WP-policy-control-plane-plan-05-ask-parent-overrides
  state: blocked
  depends on: WP-policy-control-plane-plan-04-delivery-ack-audit

graph:why WP-policy-control-plane-plan-05-ask-parent-overrides
  WP-policy-control-plane-plan-04-delivery-ack-audit is blocked
```

The graph currently exposes no READY workpack: 454 remain planned pending
readiness/dependency review, 213 are in validation, and 9 are blocked. Remote
WP01, device-trust WP08, and network WP08 are in `validation` after focused
slices were replayed; all retain explicit runtime/no-claim boundaries. The
graph therefore refuses to authorize unreviewed `Open` rows while keeping
validation and blocked work visible. `DONE` is one: Eventing WP06 is promoted
only because its reviewed code/test map, durable proof bundle, checklist, and
explicit durable-proof override all exist. The graph still refuses every row
whose completion contract is incomplete; the remaining Eventing validation rows
retain their missing evidence instead of inheriting WP06's proof.

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
