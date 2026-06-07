# App-game manual-required no-adapter gate source snapshot

- Branch: codex/app-game-manual-required-no-adapter-gate-proof-split
- Commit: branch-head-validated-by-harness
- Git status: validated-by-explicit-handoff-status-check

Evidence:
- Broad-blocking manual-required rows are blocked-before-adapter, cannot call adapters, have no supported modes, and do not claim broad blocking.
- Broad-blocking tests reject manual-required and unavailable rows that try to become dispatch-eligible.
- Policy preview tests keep manual-required native-game block-launch rows not-dispatched with adapter and platform enforcement claims false.
- Policy compiler tests reject unproved block-launch upgrades out of manual-required state.
- Category/risk manual-review routes map to manual-required and remain not-dispatched.
