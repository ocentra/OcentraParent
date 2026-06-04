# WP36 - Live Foreground Window Source

## Scope

Cross-record the shared app/game WP36 proof for the native app plan. The work
adds Rust core active-window foreground evidence plumbing for native app/game
rows without claiming product-complete native app foreground capture.

## Implementation

- Reuse the shared app/game foreground evidence row and journal event path.
- Map active-window metadata into foreground rows with opaque window/title refs.
- Prove encrypted-journal replay and SQLite foreground-now projection in core.
- Leave service capture, portal UI, policy consumption, adapter execution, and
  platform support outside this workpack.

## Proof

Proof artifacts live in:

```text
output/app-plan-proof/36-live-foreground-window-source
```

The authoritative implementation proof is the shared app/game workpack:

```text
docs/plans/app-game-plan/workpacks/36-live-foreground-window-source.md
output/app-game-plan-proof/36-live-foreground-window-source
```

## No-Claim Boundaries

- Native app foreground evidence remains separate from runtime evidence and raw
  content.
- Core active-window source proof does not imply service capture freshness.
- Opaque window/title refs do not expose raw window titles.
- Product status remains unchanged until service, portal, policy, and platform
  authority proof exist.

## Product Doc Decision

`docs/product-capability-checklist.md` is intentionally unchanged. This is a
source/projection proof only.
