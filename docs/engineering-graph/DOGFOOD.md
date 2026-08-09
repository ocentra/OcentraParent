# Engineering graph dogfood

This is the first live query of the graph against the existing repository. It
does not claim a workpack is complete; it demonstrates that the graph can
answer the orchestration questions without chat history.

```text
Imported plans: 23
Imported workpacks: 654
Graph valid: 678 nodes, 678 edges
Review items: 24
READY: 0
BLOCKED: 5
VALIDATION: 446
DONE: 0
```

The policy-control slice is a useful dependency example:

```text
graph:inspect WP-policy-control-plane-plan-05-ask-parent-overrides
  state: blocked
  depends on: WP-policy-control-plane-plan-04-delivery-ack-audit

graph:why WP-policy-control-plane-plan-05-ask-parent-overrides
  WP-policy-control-plane-plan-04-delivery-ack-audit is blocked
```

The graph currently exposes no READY workpack. Remote WP01, device-trust WP08,
and network WP08 are in `validation` after focused slices were replayed; all
retain explicit runtime/no-claim boundaries. The graph therefore tells the next
worker to inspect validation/blocked work rather than inventing a new READY
task. Three imported eventing rows that were previously labelled `done` are
also now `validation`: their durable plan manifest exists, but the generated
proof roots declared by the workpack contract are absent in this checkout.
That is intentional evidence-first demotion, not lost work.

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
