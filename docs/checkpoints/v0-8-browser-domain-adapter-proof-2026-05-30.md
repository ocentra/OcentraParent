<!-- agent-capsule -->

> Agent Capsule
> Doc: V0.8 Browser/Domain Adapter Proof Checkpoint
> Kind: historical checkpoint/proof documentation; read only when CHECKPOINT_INDEX or PROOF_INDEX names it.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# V0.8 Browser/Domain Adapter Proof Checkpoint

- Branch: `codex/v0-8-browser-domain-adapter-proof`
- Worker: `codex-b`
- Date: 2026-05-30
- Proof command: `node scripts/test/v0-8-browser-domain-adapter-proof.mjs`
- Proof artifact: `test-results/v0-8-browser-domain-adapter-proof/proof.json`

## Scope

This checkpoint adds a non-visual V0.8 read model for browser/domain adapter boundaries after the cross-platform capability proof. It records what the current service and proof seams can honestly support without upgrading broad app, domain, exact URL, unsupported OS, Android, or iOS claims.

Implemented-boundary states are limited to Windows managed-browser intervention state, Windows unmanaged browser terminate guardrails, audit visibility, restart recovery visibility, and browser policy rollback visibility. Those entries point to existing proof commands or Rust service tests and remain limited to their proved seams.

## Manual, Degraded, And Unavailable States

- Managed browser exact URL enforcement remains `manual-required`.
- Unmanaged browser warning remains `degraded-boundary` until notification delivery and browser integration exist.
- Unmanaged browser exact URL, active tab, title, page, download source, HTTPS content, and intent evidence remain `not-claimed`.
- Windows network/domain filtering remains `manual-required`.
- Windows network/domain adapter execution remains `unavailable`.
- Linux and macOS browser/domain adapters remain `unavailable`.
- Android and iOS browser/domain control remain `manual-required`.

## Non-Claims

This checkpoint does not prove managed browser exact active-tab URL enforcement, unmanaged browser URL certainty, host network/domain blocking, broad browser control, Linux or macOS browser/domain support, Android VPN/DNS or device-owner support, iOS Network Extension or Family Controls support, signing, TestFlight, stores, or device behavior.

## Validation

Expected focused validation:

- `cmd /c npm run test --workspace @ocentra-parent/parent-domain -- v0-8-browser-domain-adapter-proof`
- `cargo test -p ocentra-parent-agent-protocol enforcement_browser_domain_adapter_proof`
- `cargo test -p ocentra-parent-agent-service enforcement_browser_domain_adapter_proof_read_model`
- `cargo test -p ocentra-parent-agent-service browser_policy_rollback_restores_earlier_persisted_revision`
- `cmd /c node --check scripts/test/v0-8-browser-domain-adapter-proof.mjs`
- `cmd /c node scripts/test/v0-8-browser-domain-adapter-proof.mjs`
- `cmd /c npm run test:pre-ai-proof`
- `cmd /c npm run lint:schema-boundaries`
- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cmd /c npm run lanes:guard`
- `cmd /c npm run hub:guard`
- `cmd /c npm run validate`
