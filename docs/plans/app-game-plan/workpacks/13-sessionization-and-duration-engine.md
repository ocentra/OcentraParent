# 13 Sessionization And Duration Engine

## Target State

Running and foreground durations are derived from stored evidence and replayable
for apps, games, launchers, and candidates.

## Scope

- Session start, continuation, gap, close, stale, restart/replay, foreground
  interval, background duration, launcher-only duration, and game-candidate
  duration.
- App and game daily rollups.

## Tests And Proof

- Session starts on first runtime observation.
- Session continues within gap window.
- Session closes on exit/stale timeout.
- Foreground duration never exceeds running duration.
- Replay reconstructs the same summary.
- Launcher-only session does not become game session.

## Done Signal

Session summaries are deterministic read models, not portal refresh counters.

Use the standard checklist in [workpacks README](README.md).
