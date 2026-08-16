<!-- agent-capsule -->

> Agent Capsule
> Doc: V0.8 Cross-Platform Enforcement Capability Proof Checkpoint
> Kind: historical checkpoint/proof documentation; read only when CHECKPOINT_INDEX or PROOF_INDEX names it.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# V0.8 Cross-Platform Enforcement Capability Proof Checkpoint

- Branch: `codex/v0-8-cross-platform-enforcement-capability-proof`
- Worker: `codex-b`
- Date: 2026-05-30
- Proof command: `node scripts/test/v0-8-cross-platform-enforcement-capability-proof.mjs`
- Proof artifact: `test-results/v0-8-cross-platform-enforcement-capability-proof/proof.json`

## Scope

This checkpoint adds a non-visual V0.8 read model that keeps cross-platform enforcement capability claims honest across Windows, Linux, macOS, Android, and iOS.

Implemented-boundary states are limited to Windows owned-process terminate, Windows app time-limit lifecycle, Windows managed-browser boundary, and Windows unmanaged-browser process boundary. These entries point to existing proof commands and artifacts and do not upgrade broad app blocking, exact URL, privileged mobile, or production distribution claims.

## Manual And Scaffold States

- Windows broad installed-app blocking remains `manual-required`.
- Windows network/domain blocking remains `manual-required`.
- Linux and macOS enforcement adapters remain `scaffold`.
- Android device-owner policy and package lifecycle remain `manual-required`.
- Android store distribution remains `planned`.
- iOS Family Controls, signing/entitlements, and TestFlight remain `manual-required`.
- iOS store distribution remains `planned`.

## Non-Claims

This checkpoint does not prove global installed-app blocking, host network/domain blocking, managed browser exact URL enforcement, unmanaged browser URL certainty, Linux or macOS child enforcement support, Android device-owner enforcement, iOS Family Controls support, signing, TestFlight, Google Play, App Store readiness, or privileged mobile behavior.

## Validation

Expected focused validation:

- `cmd /c npm run test --workspace @ocentra-parent/parent-domain -- v0-8-cross-platform-enforcement-capability-proof`
- `cargo test -p ocentra-parent-agent-protocol enforcement_cross_platform_capability_proof`
- `cargo test -p ocentra-parent-agent-service enforcement_cross_platform_capability_proof_read_model`
- `cmd /c node --check scripts/test/v0-8-cross-platform-enforcement-capability-proof.mjs`
- `cmd /c node scripts/test/v0-8-cross-platform-enforcement-capability-proof.mjs`
- `cmd /c npm run test:pre-ai-proof`
- `cmd /c npm run lint:schema-boundaries`
- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cmd /c npm run lanes:guard`
- `cmd /c npm run hub:guard`
- `cmd /c npm run validate`
