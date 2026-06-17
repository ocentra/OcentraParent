# Test Placement and Proof Layers

## Rule

A test belongs at the layer that owns the behavior being proved.

## Layer map

| Layer | Owns | Should test |
| --- | --- | --- |
| schema/domain package | schemas, parsers, pure decisions, read-model builders | unit and contract tests |
| protocol package/crate | wire shapes, constants, serialization, compatibility | contract and version-skew tests |
| runtime crate/service | event handling, persistence, queues, journals, dispatch, adapters | unit, integration, security, load where applicable |
| app/orchestrator | cross-domain chain and coordination | integration/e2e/proof runner |
| portal UI | rendering, route state, dev command panels, visible copy | unit/integration/Playwright |
| proof script | artifact generation and claim audit | proof JSON, logs, screenshots, manifests |

## Cross-domain proof rule

A domain should not prove another domain's behavior by importing it directly.

Example: tracking should not directly prove tracking -> AI -> action inside `tracking-domain`. That chain belongs in an app/service/orchestrator proof where tracking emits evidence, AI consumes a typed request, action/policy emits a result, and the proof checks logs/events/read models.

## Rust placement rule

Public behavior should be represented in crate-level `tests/` categories where feasible. Inline `src` tests may remain only for private seams or small implementation helpers, and must not be the only closure proof for a public chain.

## TypeScript placement rule

Package `src/*proof*.ts` files are not tests. They may be proof builders or contract helpers, but closure still needs executable tests or proof runners.

## Proof layer rule

Proof is valid when it includes:

- executable command/test;
- trace logs or event journal;
- read-model or output artifact;
- explicit no-claim boundary;
- scoped validation command.
