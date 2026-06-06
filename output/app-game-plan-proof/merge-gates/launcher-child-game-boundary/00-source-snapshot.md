# App-game launcher child-game boundary gate source snapshot

- Branch: codex/app-game-inventory-display-gate-proof
- Commit: 6abe653b1855faafc9031f8416fe63258fa748f8
- Git status: M docs/features/app-game-control.md
 M docs/plans/app-game-plan/implementation-checklist.md
?? scripts/test/app-game-launcher-child-game-boundary-gate-proof.mjs

Evidence:
- Launcher evidence contract requires child-game proof before known-game classification.
- Generic app/game evidence claims require childGameProof before launcher evidence can become knownGame.
- Identity tests reject launcher-as-game identity without childGameEvidenceClaimId.
- Portal app/game dashboard renders launcher rows as launcher-only metrics/rows.
