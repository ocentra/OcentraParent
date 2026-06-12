<!-- agent-capsule -->

> Agent Capsule
> Doc: Platform Roles And LAN AI Provider Pool Checkpoint - 2026-05-27
> Kind: historical checkpoint/proof documentation; read only when CHECKPOINT_INDEX or PROOF_INDEX names it.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# Platform Roles And LAN AI Provider Pool Checkpoint - 2026-05-27

Scope:

- Device role runtime contract/read model for `parent-controller`, `parent-observer`, `child-agent`, and `ai-provider` roles on one physical device.
- Rust service role state for dual-role devices without duplicate local AI runtime claims.
- Parent desktop Tauri package proof command that connects to the Rust service and exposes controller lease, route, device-role, and LAN AI provider state.
- Parent mobile proof-first scaffold/unavailable state for observer/controller-takeover and LAN AI provider behavior.
- LAN AI provider pool routing with provider opt-in, capability advertisement, authorized completed result, unsupported-capability rejection, observer rejection, degraded provider-unavailable result, and audit events.
- Platform proof matrix states for parent desktop, parent mobile, child desktop, child Android, and child iOS.

Focused validation:

- `cmd /c npm run test --workspace @ocentra-parent/parent-domain -- tests/device-roles.test.ts`
- `cargo test -p ocentra-parent-agent-protocol lan_pairing_tests::device_role_runtime_read_model_serializes_dual_parent_child_ai_provider_state`
- `cargo test -p ocentra-parent-agent-service device_role_read_model_reports_dual_role_without_duplicate_ai_runtime_claims`
- `cargo test -p ocentra-parent-agent-service lan_ai_job`
- `cmd /c npm run tauri:check --workspace @ocentra-parent/parent-desktop`
- `cargo test --manifest-path apps/parent-desktop/src-tauri/Cargo.toml`
- `node --test scripts/test/platform-packaging.test.mjs`
- `cmd /c npm run build:contracts`
- `cargo build -p ocentra-parent-agent-service`
- `node scripts/test/platform-roles-lan-ai-provider-pool.mjs`
- `cmd /c npm run test:pre-ai-proof`

Evidence:

- LAN proof harness output:
  `platform-roles-lan-ai-provider-pool-ok:parent-desktop-controller-ai-provider:route-selected,parent-desktop-controller-ai-provider:provider-advertised-available,parent-desktop-controller-ai-provider:controller-job-completed-observer-job-rejected,parent-desktop-controller-ai-provider:unsupported-capability-rejected,parent-mobile-observer-scaffold:route-selected,parent-mobile-observer-scaffold:provider-unavailable,parent-mobile-observer-scaffold:controller-job-degraded-with-provider-unavailable,parent-mobile-observer-scaffold:observer-job-rejected`
- Harness evidence JSON: `test-results/platform-roles-lan-ai-provider-pool/proof.json`

Honest gaps:

- Parent mobile is still scaffold/proof-first, not a complete mobile controller UI.
- Android child remains manual-required for device-owner, UsageStats, accessibility, VPN/DNS, and managed-profile proof.
- iOS child remains unavailable for parity claims until Family Controls, DeviceActivity, Screen Time, Network Extension, signing, and device/TestFlight proof exist.
- LAN proof uses local real Rust service processes and direct WebSocket routing; production discovery and household-device proof remain future work.
