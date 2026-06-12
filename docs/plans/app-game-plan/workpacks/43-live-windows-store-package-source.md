# WP43 - Live Windows Store Package Source

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP43 - Live Windows Store Package Source`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

## Scope

Add a bounded core source for Windows packaged-app manifest evidence.

This workpack proves that local `AppxManifest.xml` files can be parsed through a
structured XML parser, mapped into app/game store-package inventory rows, and
converted into journal events without exposing raw package-root paths or
claiming app use.

It does not add registry crawling, service capture, portal UI, source freshness
rows, policy consumption, adapter execution, broad blocking, or platform support
claims.

## Implementation

- Add a bounded `WindowsApps` manifest source module in `agent-core`.
- Parse package identity, publisher, display label, and application id from
  `AppxManifest.xml` with `roxmltree`.
- Reuse the existing staged Store/UWP inventory row mapper so package inventory
  remains inventory-only.
- Hash manifest paths into source refs before rows enter the journal boundary.
- Keep default live source optional on unsupported platforms and
  permission-limited hosts.

## Proof

- `cargo test -p ocentra-parent-agent-core store_package`
- `cargo fmt --all --check`
- `cmd /c npm run hub:guard`

Proof artifacts live in:

```text
output/app-game-plan-proof/43-live-windows-store-package-source
```

## No-Claim Boundaries

- Store package manifest evidence is inventory evidence only.
- The source does not prove runtime use, foreground use, content knowledge,
  registry crawling, live service capture, portal UI, policy decisions, adapter
  execution, broad app blocking, or platform support.
- Raw manifest paths remain hashed before becoming source refs.

## Product Doc Decision

`docs/product-capability-checklist.md` is intentionally unchanged. WP43 moves
core Windows packaged-app evidence forward, but product status should not move
until service capture, portal freshness, policy consumption, and adapter
boundaries are finished.
