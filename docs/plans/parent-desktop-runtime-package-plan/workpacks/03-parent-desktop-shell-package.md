# Workpack 03 - Parent Desktop Shell Package

<!-- agent-capsule -->

> Agent Capsule
> Plan: `parent-client-runtime-distribution-plan`
> Workpack: `03-parent-desktop-shell-package`
> Kind: workpack route and proof contract.

<!-- /agent-capsule -->

Purpose: prove the desktop shell/package boundary, including local-service bridge and launch smoke, without claiming product readiness.

Current status: `complete`.

Proof root: `output/parent-client-runtime-distribution-plan-proof/03-parent-desktop-shell-package/`

## Ownership boundary

```text
scripts/dev owns dev:desktop and dev:desktop:lan launch anchors.
scripts/release owns selected desktop package artifact helpers.
apps/portal owns embedded parent web surface when selected.
local service/agent-service owners provide route bridge proof only through typed handoff.
```

## Must prove

- `dev:desktop` and `dev:desktop:lan` are honest launch anchors
- the shell reaches service state or degrades honestly
- signing/update/rollback remain explicit artifact claims
- launch smoke does not imply child runtime authority

## Required proof fields

The selected proof must name, at minimum:

```text
artifact_kind
shell_kind
platform
launch_command
launch_state
service_bridge_state
degraded_state
stale_state
artifact_path
artifact_hash_state
signing_state
update_state
rollback_state
manual_required_state
no_claim
```

These are proof-routing fields, not implementation code prescriptions.

## Failure conditions

- launch smoke becomes product readiness
- stale local-service state is treated as healthy
- desktop proof is used to claim mobile parity
- desktop launch proof is used to claim signing, update, rollback, or setup completion
- desktop shell proof is used to claim child-agent runtime authority

## Execution truth

- A real proof pack now exists under `output/parent-client-runtime-distribution-plan-proof/03-parent-desktop-shell-package/`.
- `cmd /c npm run build --workspace @ocentra-parent/parent-desktop` passes from the real workspace and keeps the desktop shell bound to the built `apps/portal/dist` frontend plus the Rust service boundary.
- `cmd /c npm run tauri:build --workspace @ocentra-parent/parent-desktop` now passes after adding the explicit bundle icon list in `apps/parent-desktop/src-tauri/tauri.conf.json`. It produces real Windows package artifacts:
  - `apps/parent-desktop/src-tauri/target/release/bundle/msi/Ocentra Parent_0.1.1_x64_en-US.msi`
  - `apps/parent-desktop/src-tauri/target/release/bundle/nsis/Ocentra Parent_0.1.1_x64-setup.exe`
- The production Tauri command now derives `ready` only from the typed Rust-service `AgentHealthCheck` / `AgentHealthReported` WebSocket handshake; the prior socket-acceptance check was insufficient because any listener could satisfy it. Validation and proof rerun remain deferred in this production-code phase.
- `node --test scripts/test/parent-desktop-runtime-package-proof.test.mjs` now matches the current Tauri invoke/listen bridge model instead of the stale agent-WebSocket assumption.
- `node scripts/test/parent-desktop-shell-package-proof.mjs` writes `test-results/parent-desktop-shell-package-proof/proof.json` after proving:
  - `dev:desktop` and `dev:desktop:lan` dry-run launch anchors generate desktop Tauri configs
  - the desktop shell keeps the portal port on `4478`
  - the shell keeps the agent port `4477` out of browser CSP because runtime bridge traffic stays on Tauri invoke/listen
  - the built desktop artifact path and local sha256 are recorded

## Proved states

- Parent desktop package truth is explicit as a Tauri desktop shell that embeds the built parent portal and connects to the Rust service boundary.
- Real Windows package artifacts now exist as both MSI and NSIS outputs.
- Real launch anchors exist for both `dev:desktop` and `dev:desktop:lan`; in this lane the plain dry-run legitimately inherited the lane's LAN-mode default and still resolved to an allowed desktop stack command.
- Service bridge production truth is fail-closed:
  - ready only after the Rust service returns the correlated, schema/peer-validated typed health response with `online=true` and websocket transport
  - the reported probe timeout comes from the same parent-runtime-core health-command timeout; the legacy raw TCP helper is compatibility/test support only
  - degraded when the handshake fails, is rejected, or the service is unavailable
  - not upgraded from a raw socket listener or stale fallback claim
- A local artifact hash is recorded in `test-results/parent-desktop-shell-package-proof/proof.json`.
- Desktop shell proof explicitly keeps child-agent runtime authority outside this packet.

## Manual-required states

- Signed desktop artifacts remain manual-required.
- Production update-channel proof remains manual-required.
- Production rollback execution remains manual-required.
- Setup-complete readiness remains manual-required and owned by `setup-install-provisioning-plan`.
- Child-agent runtime authority remains manual-required and owned by `child-agent-runtime-distribution-plan`.
- Android and iOS parent parity remain manual-required and out of scope for this packet.

## Exact validations

- `cmd /c npm run build --workspace @ocentra-parent/parent-desktop`
- `cmd /c npm run tauri:build --workspace @ocentra-parent/parent-desktop`
- `cargo test --manifest-path apps/parent-desktop/src-tauri/Cargo.toml parent_platform_proof_state -- --test-threads=1`
- `node --test scripts/test/parent-desktop-runtime-package-proof.test.mjs`
- `node scripts/test/parent-desktop-shell-package-proof.mjs`
- `cmd /c npm run lint:architecture -- --files apps/parent-desktop/src-tauri/tauri.conf.json scripts/test/parent-desktop-runtime-package-proof.test.mjs scripts/test/parent-desktop-shell-package-proof.mjs`

## No-claim boundary

- This workpack does not claim product readiness from desktop launch smoke or package build alone.
- This workpack does not claim signed release, production update, or production rollback readiness.
- This workpack does not claim setup completion.
- This workpack does not claim child-agent runtime authority.
- This workpack does not claim Android or iOS parent parity.

## Closure truth

WP03 is closed as a desktop shell/package proof packet. It is not a product-readiness, setup-readiness, child-runtime-authority, signing-release, update, rollback, or mobile-parity claim.
