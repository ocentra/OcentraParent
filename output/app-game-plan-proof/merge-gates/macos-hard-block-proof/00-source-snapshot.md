# App-game macOS hard-block proof gate source snapshot

- Branch: codex/app-game-macos-hard-block-gate-proof-split
- Commit: branch-head-validated-by-harness
- Git status: validated-by-explicit-handoff-status-check

Evidence:
- macOS hard block launch is represented as manual-required and system-extension-required.
- The macOS gate is blocked-before-adapter, cannot call adapters, has no supported modes, and does not claim broad blocking.
- Rules require setup, authority, rollback, audit, and macOS MDM/Endpoint/System Extension proof before broad block upgrades.
- Tests assert the macOS gate requires Endpoint Security, rollback, and audit proof.
- App control guide catalog names privileged macOS control paths and rejects Windows-to-macOS process-control assumptions.
