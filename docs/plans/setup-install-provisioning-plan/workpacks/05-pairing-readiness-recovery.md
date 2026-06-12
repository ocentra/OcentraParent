# Workpack 05: Pairing Readiness Recovery

Goal: define first-run pairing and readiness as product state, not scattered protocol notes.

Owns: pairing journey, setup status model, recovery UX, stale/revoked/offline states, and final readiness checklist.

Handoff: `lan-plan` owns local pairing protocol; `account-identity-family-plan` owns household/device authority; `portal-ux-household-surfaces-plan` owns UI rendering.

Expected shape:

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

- `pairing.stale-code-rejected`
- `pairing.replay-rejected`
- `pairing.wrong-household-rejected`
- `pairing.revoked-device-rejected`
- `readiness.no-fake-ready-state`
- `readiness.offline-child-degraded`

Proof artifact expectations:

- Pairing state table.
- Logs/traces with redacted codes.
- UI proof for success, expired, revoked, offline, permission missing.
