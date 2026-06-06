# App-game AI output direct-enforcement gate source snapshot

- Branch: codex/app-game-inventory-display-gate-proof
- Commit: ebe1c6ab289bfac280e5911525b7fb7c015da712
- Git status: M docs/features/app-game-control.md
 M docs/plans/app-game-plan/implementation-checklist.md
?? scripts/test/app-game-ai-output-direct-enforcement-gate-proof.mjs

Evidence:
- Activity-domain AI classification digests expose action hints and evidence/session refs, not adapter commands.
- Activity-domain local-AI category candidates require aiDigestRef and stay notEnforcement.
- Activity-domain tests reject local-AI hard action candidates such as shieldApp.
- Parent-domain category/risk policy routes constrain adapterDispatchState to not-dispatched.
- Parent-domain tests reject hard risk-candidate actions and require digest refs for local-AI routes.
