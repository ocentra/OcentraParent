# App-game running foreground display gate source snapshot

- Branch: codex/app-game-running-foreground-gate-proof-split
- Commit: f70cc73934a4822b9474deed0b271ee7f6e47623
- Git status: clean before proof generation

Evidence:
- Portal app/game dashboard intent test includes separate Running and Foreground metric totals.
- The test includes running rows where foregroundRowCount remains 0.
- Core dashboard intent maps runningRowCount and foregroundRowCount into separate dashboard fields.
- Core SVG route renders Running and Foreground counts separately.
- Portal route scaffold E2E assertion expects both RUNNING and FOREGROUND text on App/Game Sessions.
