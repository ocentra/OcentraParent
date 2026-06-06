# App-game manual-required no-adapter gate source snapshot

- Branch: codex/app-game-inventory-display-gate-proof
- Commit: e279351566f4bc80fc96257495eb9c0038d6ac28
- Git status: M docs/features/app-game-control.md
 M docs/plans/app-game-plan/implementation-checklist.md
?? scripts/test/app-game-manual-required-no-adapter-gate-proof.mjs

Evidence:
- Broad-blocking manual-required rows are blocked-before-adapter, cannot call adapters, have no supported modes, and do not claim broad blocking.
- Broad-blocking tests reject manual-required and unavailable rows that try to become dispatch-eligible.
- Policy preview tests keep manual-required native-game block-launch rows not-dispatched with adapter and platform enforcement claims false.
- Policy compiler tests reject unproved block-launch upgrades out of manual-required state.
- Category/risk manual-review routes map to manual-required and remain not-dispatched.
