<!-- agent-capsule -->

> Agent Capsule
> Plan: `device-trust-bootstrap-plan`
> Doc: `PLAN_STATE.md`
> Kind: plan state and current gap summary.
> Read when: After this plan is selected and before opening workpacks.
> Stop rule: Use this to choose one next action; do not scan historical docs.
> Proves: current planning state only.
> Does not prove: product completion or implementation readiness.

<!-- /agent-capsule -->

# Device Trust Bootstrap Plan State

Status: blocked / audit-truth-synced / not complete.

## Current Truth

This plan owns the one-time trust bootstrap layer for parent and child devices. The product model is still: pair once, trust once, seal locally, and keep that trust until a parent revokes, removes, or resets the device.

Current direction from research and the pasted plan set:

- WebAuthn and passkeys are the right parent-presence proof foundation, including QR and hybrid cross-device flows.
- Biometric verification stays inside the authenticator or OS prompt; the relying party does not get biometric data.
- Local trust material must be sealed with platform-backed stores, not custom app-managed plaintext keys.
- Platform-backed local sealing is documented in `LOCAL_KEY_SEALING_MODEL.md` and `PLATFORM_KEY_CUSTODY_MATRIX.md`.
- Plan-local test folders now live under `test/device-trust-bootstrap-plan/<major-category>/`.
- Current top-level categories are `unit`, `contract`, `integration`, `e2e`, and `security`.
- Recovery must use an encrypted bundle or equivalent sealed backup artifact; account recovery is not the same thing as data or device recovery.
- Device trust is separate from account login, subscription entitlement, policy delivery, and remote-access grant state.
- RustDesk is useful as architecture reference material for remote-desktop patterns, but not as embedded trust-root product code by default.
- Android Play Integrity is a supporting signal only; it is not the trust root.
- No proof roots currently exist on disk under `output/device-trust-bootstrap-plan-proof/` or `docs/proof/device-trust-bootstrap-plan/`.
- The current plan-local tests are mostly doc-shape and route-alignment checks, not runtime trust-bootstrap proof.

## Proof Coverage

- Proof roots are planned under `output/device-trust-bootstrap-plan-proof/<workpack-file-stem>/`, but they are absent on disk today.
- Device-trust tests now live under `test/device-trust-bootstrap-plan/<major-category>/`.
- Current top-level categories are `unit`, `contract`, `integration`, `e2e`, and `security`.
- The legacy `docs/proof/device-trust-bootstrap-plan/*` path is also absent on disk.

## Verified implementation boundary

- `packages/family-domain` contains typed trust-adjacent authority and recovery contracts, including `DeviceTrustState`, privileged device actions, setup invite rules, and recovery authorization boundaries.
- `packages/lan-domain` plus the Rust LAN pairing runtime contain trusted-route and selected-device registry contracts, restart behavior, and explicit manual proof gaps for LAN pairing.
- `packages/parent-domain` is mostly frontage for this slice and currently fails the repo re-export architecture gate on the named LAN/tamper bridge files.
- No execution-grade device-trust state machine exists yet in repo code.
- No execution-grade local key sealing implementation exists yet in repo code.
- No execution-grade parent step-up, phone QR approval bridge, encrypted recovery bundle handling, entitlement-binding runtime, or child uninstall authorization runtime exists yet in repo code.
- Login alone does not create trust, child devices do not own the trust root, and revocation must win over stale state.

## Execution Gate

- Route and implementation continue from [PLAN_EXECUTION_BLUEPRINT.md](PLAN_EXECUTION_BLUEPRINT.md).
- Update this plan only through the blueprint and the selected workpack.
- Do not mark this plan complete from checklist deltas alone.
- Proof must be collected in the designated local artifact path or crate-local proof folder, not inside this plan folder.
- True completion remains blocked until the runtime ownership split is resolved across the actual source owners and real proof artifacts exist for the missing trust-bootstrap slices.
