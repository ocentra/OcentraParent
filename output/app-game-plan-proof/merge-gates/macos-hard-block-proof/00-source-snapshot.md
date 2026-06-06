# App-game macOS hard-block proof gate source snapshot

- Branch: codex/app-game-inventory-display-gate-proof
- Commit: 284b22239e703c01649180b15c67f339382f39ce
- Git status: M docs/features/app-game-control.md
 M docs/plans/app-game-plan/implementation-checklist.md
?? scripts/test/app-game-macos-hard-block-proof-gate.mjs

Evidence:
- macOS hard block launch is represented as manual-required and system-extension-required.
- The macOS gate is blocked-before-adapter, cannot call adapters, has no supported modes, and does not claim broad blocking.
- Rules require setup, authority, rollback, audit, and macOS MDM/Endpoint/System Extension proof before broad block upgrades.
- Tests assert the macOS gate requires Endpoint Security, rollback, and audit proof.
- App control guide catalog names privileged macOS control paths and rejects Windows-to-macOS process-control assumptions.
