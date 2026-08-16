# WP41 - Live Windows Inventory Source

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP41 - Live Windows Inventory Source`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

## Scope

Add a bounded core live Windows shortcut inventory source that feeds existing
app/game inventory evidence rows and journal events.

This workpack reads Start Menu `.lnk` entries from provided roots in tests and
from platform-discovered Windows Start Menu roots in the default helper. It
hashes path-derived source and desktop-entry refs before rows leave core.

It does not add registry crawling, Store package enumeration, service capture,
portal UI, source freshness rows, policy consumption, adapter execution, broad
blocking, or platform support claims.

## Implementation

- Add protocol-owned constants for inventory source refs, desktop-entry refs,
  shortcut extension, and Windows Start Menu path segments.
- Add `app_game_windows_inventory_source` in `agent-core`.
- Convert bounded shortcut paths into existing
  `WindowsInstalledAppInventoryRecord` values.
- Reuse `windows_installed_inventory_rows_from_records` and
  `app_game_inventory_journal_event` so the no-use inventory boundary remains
  shared.
- Add focused tests for no-use rows, bounded limit behavior, SQLite journal
  replay, and optional default live source behavior.

## Proof

- `cargo test -p ocentra-parent-agent-core live_inventory`
- `cargo test -p ocentra-parent-agent-core app_game`
- `cargo test -p ocentra-parent-agent-protocol app_game`
- `cmd /c npm run lint:schema-boundaries`
- `git diff --check`
- `cmd /c npm run lanes:guard`
- `cmd /c npm run hub:guard`

Proof artifacts live in:

```text
output/app-game-plan-proof/41-live-windows-inventory-source
```

## No-Claim Boundaries

- Shortcut inventory is inventory evidence only.
- Raw shortcut paths are not exposed as source ids or desktop-entry ids.
- Shortcut inventory does not prove runtime use, foreground use, content
  knowledge, registry crawling, Store package enumeration, portal UI, policy
  decisions, adapter execution, broad app blocking, or platform support.

## Product Doc Decision

`docs/product-capability-checklist.md` is intentionally unchanged. WP41 moves
core live inventory evidence forward, but product status should not move until
service capture, portal freshness, policy consumption, live platform proof, and
adapter boundaries are finished.
