# App-game foreground content boundary gate source snapshot

- Branch: codex/app-game-inventory-display-gate-proof
- Commit: 7e4961f19bde741f49919e8c30d9b799c75c313c
- Git status: M docs/features/app-game-control.md
 M docs/plans/app-game-plan/implementation-checklist.md
?? scripts/test/app-game-foreground-content-boundary-gate-proof.mjs

Evidence:
- Activity-domain foreground evidence restricts contentKnowledgeState to notClaimed.
- Activity-domain foreground tests reject content-knowledge promotion and keep title data behind refs.
- Portal app/game dashboard evidence rows render refs, counts, capability state, and timestamps.
- Core SVG app/game dashboard rows do not render window title refs, raw window titles, or executable paths.
