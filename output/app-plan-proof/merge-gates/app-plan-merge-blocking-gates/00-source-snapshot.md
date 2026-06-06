# App-plan merge-blocking gates source snapshot

- Branch: codex/app-game-inventory-display-gate-proof
- Commit: 566a9d9882538f5f95897213475b90555e98e3f8
- Git status: M docs/features/app-game-control.md
 M docs/plans/app-plan/implementation-checklist.md
?? scripts/test/app-plan-merge-blocking-gates-proof.mjs

Evidence:
- The app-plan merge-blocking gates are cross-recorded from the current shared app/game proof spine.
- Each source proof has `sharedEvidenceSpine=true`, `nativeAppMeaningProven=true`, and `browserGameWorkDuplicated=false`.
- This proof changes no product status, package exports, adapter dispatch, policy execution, or browser-game path.

Gate proof map:
- Inventory evidence is displayed as app usage. -> output/app-game-plan-proof/merge-gates/inventory-display/proof.json
- Running evidence is displayed as foreground usage. -> output/app-game-plan-proof/merge-gates/running-foreground-display/proof.json
- Foreground evidence is displayed as content knowledge. -> output/app-game-plan-proof/merge-gates/foreground-content-boundary/proof.json
- AI output can directly enforce. -> output/app-game-plan-proof/merge-gates/ai-output-direct-enforcement/proof.json
- Dry-run terminates or blocks app. -> output/app-game-plan-proof/merge-gates/dry-run-no-action/proof.json
- Manual-required action calls an adapter. -> output/app-game-plan-proof/merge-gates/manual-required-no-adapter/proof.json
- Android normal mode claims package suspend/hide. -> output/app-game-plan-proof/merge-gates/android-normal-mode-no-suspend-hide/proof.json
- iOS claims process scanning/killing. -> output/app-game-plan-proof/merge-gates/ios-no-process-scan-kill/proof.json
- macOS hard block is claimed without entitlement/profile proof. -> output/app-game-plan-proof/merge-gates/macos-hard-block-proof/proof.json
- Linux universal block is claimed without mechanism/distro proof. -> output/app-game-plan-proof/merge-gates/linux-universal-block-proof/proof.json
- Session duration changes after journal replay. -> output/app-game-plan-proof/merge-gates/session-duration-replay/proof.json
- Portal hides stale, permission-limited, manual-required, or not-claimed states. -> output/app-game-plan-proof/merge-gates/portal-state-visibility/proof.json
- Raw private executable paths leak into parent UI. -> output/app-game-plan-proof/merge-gates/raw-executable-path-ui-leak/proof.json
- Malicious app metadata causes XSS or layout breakage. -> output/app-game-plan-proof/merge-gates/malicious-metadata-ui-safety/proof.json
