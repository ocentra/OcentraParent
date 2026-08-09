# Engineering graph dogfood

This is the first live query of the graph against the existing repository. It
does not claim a workpack is complete; it demonstrates that the graph can
answer the orchestration questions without chat history.

```text
Imported plans: 23
Imported workpacks: 654
Graph valid: 678 nodes, 678 edges
Review items: 24
READY: 2
BLOCKED: 5
VALIDATION: 441
```

The policy-control slice is a useful dependency example:

```text
graph:inspect WP-policy-control-plane-plan-05-ask-parent-overrides
  state: blocked
  depends on: WP-policy-control-plane-plan-04-delivery-ack-audit

graph:why WP-policy-control-plane-plan-05-ask-parent-overrides
  WP-policy-control-plane-plan-04-delivery-ack-audit is blocked
```

The graph currently exposes two independent READY workpacks, so those can be
assigned in parallel after the normal Enforcer claim/guard step. Remote WP01
was moved to `validation` after its focused contract/test slice was replayed;
its runtime lifecycle remains open:

- `WP-device-trust-bootstrap-plan-08-open-source-dependency-adoption`
- `WP-network-plan-08-control-catalog-reference-routing`

The queries used were:

```powershell
npm run graph:bootstrap -- --write
npm run graph:validate
npm run graph:status
npm run graph:ready
npm run graph:inspect WP-policy-control-plane-plan-05-ask-parent-overrides
npm run graph:why WP-policy-control-plane-plan-05-ask-parent-overrides
```
