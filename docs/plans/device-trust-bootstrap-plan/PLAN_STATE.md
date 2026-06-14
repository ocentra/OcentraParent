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

Status: research-created / architecture-open.

## Current Truth

This plan owns the one-time trust bootstrap layer for parent and child devices. The product model is still: pair once, trust once, seal locally, and keep that trust until a parent revokes, removes, or resets the device.

Current direction from research and the pasted plan set:

- WebAuthn and passkeys are the right parent-presence proof foundation, including QR and hybrid cross-device flows.
- Biometric verification stays inside the authenticator or OS prompt; the relying party does not get biometric data.
- Local trust material must be sealed with platform-backed stores, not custom app-managed plaintext keys.
- Recovery must use an encrypted bundle or equivalent sealed backup artifact; account recovery is not the same thing as data or device recovery.
- Device trust is separate from account login, subscription entitlement, policy delivery, and remote-access grant state.
- RustDesk is useful as architecture reference material for remote-desktop patterns, but not as embedded trust-root product code by default.
- Android Play Integrity is a supporting signal only; it is not the trust root.

## Open Gaps

- No execution-grade device-trust state machine exists yet in repo code.
- No final platform coverage matrix exists for Windows, macOS, iOS, Android, and Linux trust sealing.
- No finalized parent step-up policy exists for high-risk actions.
- No QR approval bridge contract exists for desktop-to-phone action approval.
- No signed entitlement snapshot contract exists yet for device-bound unlock.
- No recovery/reset/re-pair proof pack exists yet.
- No child uninstall or anti-tamper proof pack exists yet.
- No dependency adoption decision record exists yet.
- No proof artifacts under `docs/proof/device-trust-bootstrap-plan/` yet.

## Execution Gate

- Route and implementation continue from [PLAN_EXECUTION_BLUEPRINT.md](PLAN_EXECUTION_BLUEPRINT.md).
- Update this plan only through the blueprint and the selected workpack.
- Do not mark this plan complete from checklist deltas alone.
- Proof must be collected in the designated local artifact path or crate-local proof folder, not inside this plan folder.