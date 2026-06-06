# App-game iOS no process scan/kill gate source snapshot

- Branch: codex/app-game-ios-process-authority-gate-proof-split
- Commit: branch-head-validated-by-harness
- Git status: validated-by-explicit-handoff-status-check

Evidence:
- iOS process termination is represented as a not-claimed gate.
- The iOS gate is not-dispatched, cannot call adapters, has no supported modes, and does not claim broad blocking.
- Required proof kinds stay tied to FamilyControls, ManagedSettings, or supervised MDM paths before any stronger iOS control claim.
- Tests assert the iOS terminate-process gate remains not-claimed and cannot call adapters.
- App control guide catalog keeps iOS Screen Time/entitlement behavior manual-required until real device/platform proof exists.
