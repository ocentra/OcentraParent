# WP36 - Live Foreground Window Source

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP36 - Live Foreground Window Source`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

## Scope

Add a bounded Rust core source that can read live active-window metadata and map
it into the existing app/game foreground evidence row shape.

This workpack is core evidence plumbing only. It does not wire service capture,
portal UI, policy consumption, adapter enforcement, broad app blocking, or
cross-platform product support.

## Implementation

- Add `agent-core` foreground source helpers that can use the Windows
  active-window adapter when available.
- Map a foreground snapshot into `WindowsForegroundWindowRecord`.
- Store window identity and title evidence as opaque SHA-256 refs using protocol
  prefixes instead of raw window titles, paths, or content strings.
- Bridge the source into app/game journal events and replay those events through
  the existing encrypted-journal and SQLite read-model path.
- Keep unavailable or unsupported platform capture as `None`, not as a product
  failure or support claim.

## Proof

- `cargo test -p ocentra-parent-agent-core foreground`
- `cargo test -p ocentra-parent-agent-protocol app_game`
- `git diff --check`

Proof artifacts live in:

```text
output/app-game-plan-proof/36-live-foreground-window-source
```

## No-Claim Boundaries

- Foreground is not content.
- Running is not foreground.
- A window title ref is not raw title capture.
- Core source proof is not service polling or subscription proof.
- Core journal/SQLite projection is not portal freshness, policy execution, or
  adapter enforcement proof.
- Unsupported platform adapter state remains not-claimed until platform proof
  exists.

## Product Doc Decision

`docs/product-capability-checklist.md` is intentionally unchanged. WP36 adds a
core source proof only; product status should not move until service capture,
portal rendering, policy/runtime consumers, and platform authority proof exist.
