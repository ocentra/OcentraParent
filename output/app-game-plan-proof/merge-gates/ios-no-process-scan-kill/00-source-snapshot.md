# App-game iOS no process scan/kill gate source snapshot

- Branch: codex/app-game-inventory-display-gate-proof
- Commit: dd478ad2cf233f066d05812dc5a899b23aeee04c
- Git status: M docs/features/app-game-control.md
 M docs/plans/app-game-plan/implementation-checklist.md
?? scripts/test/app-game-ios-no-process-scan-kill-gate-proof.mjs

Evidence:
- iOS process termination is represented as a not-claimed gate.
- The iOS gate is not-dispatched, cannot call adapters, has no supported modes, and does not claim broad blocking.
- Required proof kinds stay tied to FamilyControls, ManagedSettings, or supervised MDM paths before any stronger iOS control claim.
- Tests assert the iOS terminate-process gate remains not-claimed and cannot call adapters.
- App control guide catalog keeps iOS Screen Time/entitlement behavior manual-required until real device/platform proof exists.
