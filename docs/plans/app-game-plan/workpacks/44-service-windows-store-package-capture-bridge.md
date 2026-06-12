# WP44 - Service Windows Store Package Capture Bridge

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP44 - Service Windows Store Package Capture Bridge`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

## Scope

Wire the WP43 live Windows packaged-app manifest source into the existing
agent-service activity-capture journal/store path.

This workpack proves that bounded `AppxManifest.xml` inventory events can be
captured by the service and projected into the existing app/game read model as
Store/UWP inventory-only rows.

It does not add registry crawling, portal UI, source freshness rows, policy
consumption, adapter execution, broad blocking, or platform support claims.

## Implementation

- Export the core live Windows packaged-app journal-event helper for service
  use.
- Extend the service app/game activity-capture event list with live packaged-app
  manifest events.
- Add a deterministic service test that injects a temporary packaged-app
  manifest root and proves encrypted journal replay plus SQLite read-model
  projection.
- Keep environment-dependent default Windows package counts bounded instead of
  treating read-model row limits as event counts.

## Proof

- `cargo test -p ocentra-parent-agent-service activity_capture`
- `cargo test -p ocentra-parent-agent-core store_package`
- `cargo test -p ocentra-parent-agent-core app_game`
- `cargo test -p ocentra-parent-agent-protocol app_game`
- `cargo fmt --all --check`
- `cmd /c npm run lint:schema-boundaries`
- `git diff --check`
- `cmd /c npm run lanes:guard`
- `cmd /c npm run hub:guard`

Proof artifacts live in:

```text
output/app-game-plan-proof/44-service-windows-store-package-capture-bridge
```

## No-Claim Boundaries

- Service capture stores packaged-app manifests as inventory evidence only.
- Store package inventory does not prove runtime use, foreground use, content
  knowledge, registry crawling, portal UI, policy decisions, adapter execution,
  broad app blocking, or platform support.
- Raw manifest paths remain hashed before becoming source refs.

## Product Doc Decision

`docs/product-capability-checklist.md` is intentionally unchanged. WP44 moves
service capture proof forward, but product status should not move until portal
freshness, policy consumption, live platform proof, and adapter boundaries are
finished.
