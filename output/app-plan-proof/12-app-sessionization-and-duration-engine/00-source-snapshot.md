# Source Snapshot

Workpack: app-plan WP12 app sessionization and duration engine

Branch: `codex/app-game-sessionization-duration`

Starting head while preparing proof: `8fda8c3`

This app-plan proof mirrors the shared app/game WP13 proof. The app slice did
not create a parallel app-only evidence spine.

Before-state gap:

- App session duration and daily app rollups needed deterministic stored-row
  replay proof.
- The old summary path grouped stored app/game rows without deriving running,
  foreground, background, stale-gap, exit, or daily rollup duration values.

Primary shared proof root:

```text
output/app-game-plan-proof/13-sessionization-and-duration-engine/
```
