# WP03 Adapter Capability Matrix

- checkedAt: `2026-06-17T02:20:37Z`
- branch: `codex/tracking-plan-full-continuation-a`
- commit: `1f192e52b931d3b2b8080f3e9479d37a94172958`
- result: `pass`

## Commands

- `node scripts/test/v0-8-supported-adapter-runtime-proof.mjs`
- `node scripts/test/v0-8-cross-platform-enforcement-capability-proof.mjs`
- `node scripts/test/v0-8-broad-os-adapter-runtime-proof.mjs`
- `npm run lint:architecture -- --files scripts/test/v0-8-supported-adapter-runtime-proof.mjs scripts/test/v0-8-cross-platform-enforcement-capability-proof.mjs scripts/test/v0-8-broad-os-adapter-runtime-proof.mjs docs/plans/v0-8-enforcement-control-plan/workpacks/03-adapter-capability-matrix.md docs/plans/v0-8-enforcement-control-plan/WORKPACK_INDEX.md docs/plans/v0-8-enforcement-control-plan/PLAN_STATE.md docs/plans/v0-8-enforcement-control-plan/NEXT_ACTIONS.md docs/plans/v0-8-enforcement-control-plan/PROOF_INDEX.md docs/proof/v0-8-enforcement-control-plan/slice-05-adapter-capability-matrix.md`

## Owning surfaces

- `packages/enforcement-domain/src/v0-8-supported-adapter-runtime-proof.ts`
- `packages/enforcement-domain/src/v0-8-cross-platform-enforcement-capability-proof.ts`
- `packages/enforcement-domain/src/v0-8-broad-os-adapter-runtime-proof.ts`
- `packages/agent-protocol-domain/src/enforcement-supported-adapter-runtime-proof-adapter.ts`
- `crates/agent-protocol/src/enforcement_supported_adapter_runtime_proof.rs`
- `crates/agent-protocol/src/enforcement_cross_platform_capability_proof.rs`
- `crates/agent-service/src/enforcement_api/enforcement_supported_adapter_runtime_proof_read_model.rs`
- `crates/agent-service/src/enforcement_cross_platform_capability_proof_read_model.rs`
- `packages/browser-domain/src/v0-8-browser-domain-adapter-proof.ts`

## Covered proof

- supported-adapter runtime proof tracks 13 entries across platform, adapter capability, boundary state, permission/dependency degradation, rollback references, and audit references
- cross-platform capability proof tracks 15 surfaces across Windows, Linux, macOS, Android, and iOS with implemented-boundary, manual-required, scaffold, and planned separation
- broad OS runtime proof tracks 10 runtime-facing entries and keeps broad app/domain/browser/mobile claims manual-required, unavailable, or not-claimed until target artifacts exist
- TypeScript contract tests, the agent-protocol-domain adapter test, Rust protocol tests, and Rust service read-model tests prove consumer parity rather than leaving the matrix as domain-only data
- named feature docs already reflected the manual-required, scaffold, unavailable, degraded, and not-claimed boundaries proved here, so no feature-doc or product-checklist text delta was required for this stale-doc closure

## Remaining gaps

- global installed-app blocking, host network/domain blocking, and managed exact URL enforcement remain unproved/manual-required
- unmanaged browser exact evidence, notification delivery, tamper hardening, and uninstall resistance remain not-claimed or manual-required
- Linux, macOS, Android, and iOS target-specific enforcement support still requires platform-native artifacts before any claim upgrade
- later service and portal consumption workpacks remain open even though this capability source of truth is now proved
