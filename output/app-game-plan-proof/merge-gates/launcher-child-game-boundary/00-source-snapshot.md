# App-game launcher child-game boundary gate source snapshot

- Branch: codex/app-game-launcher-child-boundary-gate-proof-split
- Commit: branch-head-validated-by-harness
- Git status: validated-by-explicit-handoff-status-check

Evidence:
- Launcher evidence contract requires child-game proof before known-game classification.
- Generic app/game evidence claims require childGameProof before launcher evidence can become knownGame.
- Identity tests reject launcher-as-game identity without childGameEvidenceClaimId.
- Portal app/game dashboard renders launcher rows as launcher-only metrics/rows.
