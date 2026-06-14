# Workpack 05: Pairing Readiness Recovery

Goal: define first-run pairing and readiness as product state, not scattered protocol notes.

Owns: pairing journey, setup status model, recovery UX, stale/revoked/offline states, and final readiness checklist.

Handoff: `lan-plan` owns local pairing protocol; `account-identity-family-plan` owns household/device authority; `portal-ux-household-surfaces-plan` owns UI rendering.

Expected shape:

- Pairing is a two-stage flow: parent portal creates pairing authority, child bootstrap redeems pairing authority, and parent portal confirms the detected child device.
- Pairing code/link/QR state has expiry, revocation, household binding, and replay rejection.
- Readiness separates account, parent app, child app, permissions, network reachability, custody sync, and policy baseline.
- Recovery handles lost parent device, child reinstall, revoked child, wrong account, offline device, and permission loss.

Expected proof:

- Success and negative pairing states.
- Wrong household, stale code, replay, revoked device, and offline child proof.
- Readiness checklist artifact.
- Logs/traces with redaction.

Failure: claiming first-run complete when only LAN discovery or UI rendering is proven.

## Execution Detail

Minimum context:

- `docs/plans/lan-plan/AGENTS.md`
- `docs/plans/account-identity-family-plan/AGENTS.md`
- `docs/expectations/lan-pairing.md`
- `docs/features/family-setup-device-roles.md`
- `docs/plans/setup-install-provisioning-plan/SETUP_STATE_MACHINE.md`
- `docs/plans/setup-install-provisioning-plan/PAIRING_READINESS_MODEL.md`

Agent decision tree:

- If the task is signed hello, local discovery, or protocol detail, route to `lan-plan`.
- If the task is household/device authority, route to `account-identity-family-plan`.
- If the task is readiness UX and recovery state, stay here.
- If remote pairing outside LAN is in scope, route to `remote-access-plan`.

Required output:

- Pairing lifecycle: generated, displayed, accepted, expired, revoked, replayed, wrong household, trusted, untrusted, recovered.
- Readiness model: account, parent app, child agent, permissions, pairing, policy baseline, data custody, network reachability.
- Recovery flows: lost parent, child reinstall, revoked child, stale code, offline device, permission loss.
- Audit and support diagnostics expectations.

Expected tests/proof names:

- `setup.pairing.lifecycle-state-machine`
- `setup.pairing.code-generated-state`
- `setup.pairing.code-expired-rejected`
- `setup.pairing.code-revoked-rejected`
- `setup.pairing.replay-rejected`
- `setup.pairing.wrong-household-rejected`
- `setup.pairing.wrong-device-rejected`
- `setup.pairing.anonymous-device-rejected`
- `setup.pairing.revoked-device-rejected`
- `setup.pairing.stale-signed-hello-rejected`
- `setup.pairing.parent-role-required`
- `setup.readiness.matrix`
- `setup.readiness.no-fake-ready-state`
- `setup.readiness.offline-child-degraded`
- `setup.readiness.permission-missing-degraded`
- `setup.readiness.policy-baseline-missing`
- `setup.readiness.data-custody-unavailable`
- `setup.recovery.lost-parent-device`
- `setup.recovery.child-reinstall`
- `setup.recovery.revoked-child`
- `setup.recovery.permission-loss`
- `setup.recovery.offline-device`
- `setup.observability.redacted-pairing-logs`
- `setup.guided.parent-portal-generates-child-pairing`
- `setup.guided.parent-sees-child-pending-confirmation`
- `setup.guided.child-not-trusted-until-parent-confirmed`
- `setup.guided.no-fake-ready-after-install`
- `setup.guided.no-child-data-public-site`
- `setup.guided.redacted-bootstrap-logs`

Proof artifact expectations:

- `05-pairing-state-machine-proof.md`
- `05-pairing-negative-proof.md`
- `05-readiness-matrix-proof.md`
- `05-no-fake-ready-state-proof.md`
- `05-recovery-flow-proof.md`
- `05-redacted-pairing-log-proof.md`
- `guided-parent-child-pairing-proof.md`
- `no-fake-ready-after-install-proof.md`
- `redacted-bootstrap-logs-proof.md`
