# Network WP08 Control-Catalog Routing Validation

Status: validation only; this slice does not claim network runtime, policy,
enforcement, portal, platform, or production readiness.

## Required routing fields

| Field | Value |
| --- | --- |
| `source_file` | `docs/plans/network-plan/workpacks/network-control-capability-guide.md`, `network-control-schema-proposal.md`, and `network-control-settings-inventory.md` |
| `search_term_or_section` | WP08 ownership boundary, expected outcome, required tests/proof, and failure conditions |
| `selected_control_family` | `network.control-catalog` reference-routing controls |
| `owning_plan` | `network-plan` |
| `selected_workpack` | `WP08 Control Catalog Reference Routing` |
| `rejected_out_of_scope_controls` | Runtime network capture, policy enforcement, platform adapters, portal behavior, billing, custody, device trust, and notification delivery |
| `runtime_claim_state` | not claimed |
| `implementation_claim_state` | routing/test contract only; no runtime implementation claimed |
| `no_default_read_state` | asserted by the focused contract test and workpack failure conditions |
| `no_claim` | A catalog row, generated inventory, or reference document is not implementation proof or product completion. |

## Validation

Command:

```text
node --test tests/network-plan/contract/control-catalog-reference-routing.test.mjs
```

Result: 1 test passed, 0 failed.

Scoped architecture validation also passed for the focused test, WP08 workpack,
and workpack index. `npm run hub:guard` passed with no findings or conflicts.

Local reproducibility output is intentionally ignored and lives at:
`output/network-plan-proof/08-control-catalog-reference-routing/`.

## Remaining work

WP01-WP07 still own the network runtime/parser/classification/cascade,
intervention, analyzer, and rollout work. This proof only makes WP08's routing
boundary executable and moves it from `ready` to `validation` in the graph.
