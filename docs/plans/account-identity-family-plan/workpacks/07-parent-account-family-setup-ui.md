# Workpack 07: Parent Account And Family Setup UI

Goal: define the first-run parent account and family setup UI.

Expected shape:

- Welcome / sign in.
- Create or join household.
- Create child profile.
- Add child device.
- Pair child device.
- Devices and roles.
- Invite co-parent.
- Invite observer.
- Recovery and revoke.
- Account / security settings.
- Support access.

Expected proof:

- First-run UI state machine.
- Household setup UI proof.
- Device role UI proof.
- Observer read-only UI proof.
- Recovery UI proof.
- Mobile parent/child claim split proof.

Failure: UI that implies login equals trust, hides custody state, or lets support act as owner.

## Execution Detail

Minimum context:

- `docs/features/family-setup-device-roles.md`
- `docs/expectations/family-setup.md`
- `docs/expectations/portal.md`
- `docs/expectations/platforms.md`
- `docs/plans/setup-install-provisioning-plan/AGENTS.md`

Rules:

- Add or reuse contracts first.
- Then add service-backed state.
- Then portal rendering and tests.
- Keep parent-account membership separate from child-device authority.

UI state labels:

- live local
- LAN
- parent cache
- parent-owned storage
- stale
- degraded
- unavailable
- manual-required

Expected tests/proof names:

- `account-identity.ui.sign-in-states`
- `account-identity.ui.no-household-state`
- `account-identity.ui.create-household-flow`
- `account-identity.ui.join-household-flow`
- `account-identity.ui.add-child-profile-flow`
- `account-identity.ui.pair-child-device-flow`
- `account-identity.ui.invite-co-parent-flow`
- `account-identity.ui.invite-observer-flow`
- `account-identity.ui.role-visibility`
- `account-identity.ui.device-trust-status`
- `account-identity.ui.revoked-device-status`
- `account-identity.ui.expired-session-status`
- `account-identity.ui.recovery-status`
- `account-identity.ui.support-access-status`
- `account-identity.ui.manual-required-status`
- `account-identity.ui.mobile-parent-state`
- `account-identity.ui.child-mobile-no-claim`

Proof artifact expectations:

- `07-first-run-ui-state-machine.md`
- `07-household-setup-ui-proof.md`
- `07-device-role-ui-proof.md`
- `07-observer-read-only-ui-proof.md`
- `07-recovery-ui-proof.md`
- `07-mobile-parent-child-claim-split-proof.md`
