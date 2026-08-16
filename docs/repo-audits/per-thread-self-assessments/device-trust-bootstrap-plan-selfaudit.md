# device-trust-bootstrap-plan

## Normalized Header

- plan/thread name: `device-trust-bootstrap-plan`
- source thread label: `codex-a lane manager`
- source thread id: `019ecea1-0fde-7992-9607-d73ef97bfbbd`
- plan thread id: `019ed328-6299-75b3-9369-13fe3e4f325e`
- worktree: `C:\Users\sujan\.codex\worktrees\ocentra-parent-codex-a\OcentraParent`
- branch: `codex/tracking-plan-full-continuation-a`
- claimed status: `WP01 trust-state-core-and-architecture-cleanup at coherent atomic checkpoint; plan not done`
- claimed source files/crates/packages: `packages/family-domain/src/household-authority.ts`, `packages/family-domain/src/setup-lifecycle.ts`, `packages/family-domain/tests/unit/household-authority.test.ts`, `packages/family-domain/tests/unit/setup-lifecycle.test.ts`, `packages/lan-domain/src/household-device-spine.ts`, `packages/lan-domain/src/lan-pairing.ts`, `packages/lan-domain/tests/unit/household-device-spine.test.ts`, `packages/parent-domain/src/lan-pairing.ts`, `packages/parent-domain/package.json`, `crates/agent-protocol/src/lan_pairing.rs`, deleted `packages/parent-domain/src/device-roles.ts`, `packages/parent-domain/src/household-device-spine.ts`, `packages/parent-domain/src/lan-pairing-device.ts`, `packages/parent-domain/src/lan-pairing-support.ts`, `packages/parent-domain/src/tamper-uninstall-artifact-status.ts`
- claimed tests: `packages/family-domain` unit tests for authority and setup lifecycle; `packages/lan-domain` unit tests for household device spine; Rust `lan_pairing` targeted tests
- claimed proof commands/artifacts: `npm run test --workspace @ocentra-parent/family-domain -- tests/unit/household-authority.test.ts tests/unit/setup-lifecycle.test.ts`; `npm run test --workspace @ocentra-parent/lan-domain -- tests/unit/household-device-spine.test.ts`; `npm run build --workspace @ocentra-parent/parent-domain`; `npm run type-check --workspace @ocentra-parent/parent-domain`; `npm run lint:architecture -- --files packages/family-domain/src/household-authority.ts packages/family-domain/src/setup-lifecycle.ts packages/lan-domain/src/household-device-spine.ts packages/lan-domain/src/lan-pairing.ts packages/parent-domain/src/lan-pairing.ts`; `cargo lint-architecture crates/agent-protocol/src/lan_pairing.rs`; `cargo test -p ocentra-parent-agent-protocol lan_pairing`
- claimed blockers: `no unresolved ownership edge for WP01`; remaining plan closure depends on later setup contract work, recovery persistence substrate, and external Apple-host proof where relevant
- claimed next actions: `step-up/QR approval` next; do not start `key sealing` before the recovery persistence/encryption substrate exists
- obvious missing evidence fields: no canonical proof-root artifacts landed from this self-report; no real Android/Linux/Apple proof captured in this archival pass; no end-to-end or security proof for later slices in this file
- archival note: self-report only; unverified input for structural review

## Raw Self-Report

**WP01 Checkpoint**

`trust-state-core-and-architecture-cleanup` is at a coherent atomic checkpoint with no unresolved ownership edge.

The trust-state core now lives explicitly in [household-authority.ts](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/packages/family-domain/src/household-authority.ts:50) and [setup-lifecycle.ts](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/packages/family-domain/src/setup-lifecycle.ts:593). I added `reset-required`, centralized trusted-state evaluation in `isTrustedDeviceState`, and mapped recovery outcomes back into explicit trust rebuild states instead of silently restoring trust. The new behavior is covered in [household-authority.test.ts](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/packages/family-domain/tests/unit/household-authority.test.ts:532) and [setup-lifecycle.test.ts](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/packages/family-domain/tests/unit/setup-lifecycle.test.ts:453).

On the LAN side, [household-device-spine.ts](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/packages/lan-domain/src/household-device-spine.ts:162) now maps LAN pairing trust into family trust semantics and rejects child-agent rows whose top-level and inventory trust states diverge. That helper is exposed through the LAN public surface in [lan-pairing.ts](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/packages/lan-domain/src/lan-pairing.ts:90), and the real tests landed in [household-device-spine.test.ts](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/packages/lan-domain/tests/unit/household-device-spine.test.ts:55). In `parent-domain`, [lan-pairing.ts](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/packages/parent-domain/src/lan-pairing.ts:1) is now a narrow façade instead of a silent barrel, `package.json` no longer exports the removed device-trust shims, and these deleted shim files are gone: `packages/parent-domain/src/device-roles.ts`, `packages/parent-domain/src/household-device-spine.ts`, `packages/parent-domain/src/lan-pairing-device.ts`, `packages/parent-domain/src/lan-pairing-support.ts`, `packages/parent-domain/src/tamper-uninstall-artifact-status.ts`. Rust `lan_pairing` cleanup also landed in [lan_pairing.rs](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/crates/agent-protocol/src/lan_pairing.rs:7) by replacing `pub use` re-exports with explicit public aliases.

**Scoped Validation**

Passed:
- `npm run test --workspace @ocentra-parent/family-domain -- tests/unit/household-authority.test.ts tests/unit/setup-lifecycle.test.ts`
- `npm run test --workspace @ocentra-parent/lan-domain -- tests/unit/household-device-spine.test.ts`
- `npm run build --workspace @ocentra-parent/parent-domain`
- `npm run type-check --workspace @ocentra-parent/parent-domain`
- `npm run lint:architecture -- --files packages/family-domain/src/household-authority.ts packages/family-domain/src/setup-lifecycle.ts packages/lan-domain/src/household-device-spine.ts packages/lan-domain/src/lan-pairing.ts packages/parent-domain/src/lan-pairing.ts`
- `cargo lint-architecture crates/agent-protocol/src/lan_pairing.rs`
- `cargo test -p ocentra-parent-agent-protocol lan_pairing`

No repo-wide validation was run.

**Next Slice**

The next slice should be `step-up/QR approval`, not `key sealing`.

Reason: `step-up/QR approval` can proceed now inside this lane’s trust semantics plus the setup-domain producer contract boundary, while `key sealing` still depends on the later recovery persistence/encryption substrate owned by `data-custody-storage-plan`.

## Optional Addendum

- Earlier audit/completion passes established that the plan is not closeable from WP01 alone. Remaining plan-owned work still includes the bootstrap/add-device setup handoff contract consumption, step-up and QR approval semantics, recovery-state validation and trust transitions beyond the core vocabulary, genuine proof collection for Windows/Android/Linux where relevant, and explicit Apple-host rows marked external/manual-required rather than silently ignored.
- Earlier audit/completion passes also called out missing proof and coverage categories for later slices: no real integration, contract, e2e, abuse/security, concurrency/idempotency, logging/metrics/tracing, or monitoring/alerting proof was claimed for the overall plan yet. This archival file should not be read as evidence that those categories are complete.
- Dependency split from earlier passes: `packages/setup-domain` owns producer-side bootstrap/add-device handoff shape; this lane defines and consumes the contract. `data-custody-storage-plan` owns recovery bundle persistence, storage, and encryption substrate; this lane owns recovery semantics and trust-state transitions on top. `payment-subscription-plan` is a downstream consumer of trusted-device subject semantics and does not define them.
