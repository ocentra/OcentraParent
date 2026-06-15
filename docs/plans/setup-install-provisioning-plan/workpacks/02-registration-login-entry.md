# Workpack 02: Registration Login Entry

Goal: define how the family site starts account creation/login without owning identity internals.

Owns: user journey entry, account handoff labels, unauthenticated/authenticated route boundaries, error states, and recovery links.

Handoff: `account-identity-family-plan` owns provider choice, token/session lifecycle, household membership, invites, and recovery authority.

Expected shape:

- Register, login, resume setup, invite accept, password/passkey/email-link recovery, and logout routes.
- Invite-to-account flow, account-to-household flow, and household-to-parent-install-code flow are explicit.
- Parent-friendly wording for account versus child profile.
- Explicit cross-family rejection and expired/revoked invite states.
- No child device enrollment before account/household authority is established.

Expected proof:

- Auth route state matrix.
- Replay/expired invite negative proof.
- Cross-family and wrong-role rejection proof.
- UI screenshot proof for empty/error/recovery states.

Failure: implementing account logic ad hoc in website docs or collecting child data before household authority exists.

## Execution Detail

Minimum context:

- `docs/plans/account-identity-family-plan/AGENTS.md`
- `docs/expectations/family-setup.md`
- `docs/expectations/portal.md`
- `docs/features/family-setup-device-roles.md`

Agent decision tree:

- If provider choice is undecided, stop in this workpack and route to `account-identity-family-plan/workpacks/01-auth-provider-decision.md`.
- If the task is route copy or setup state only, stay here and define the handoff.
- If the task touches user/session/token logic, the owning plan is `account-identity-family-plan`.
- If the task touches child profile/device creation after login, confirm setup state and account authority first.

Required output:

- Auth entry route map: register, login, logout, invite accept, resume setup, recovery.
- State matrix: unauthenticated, authenticated/no household, household/no child, household/child/no device, paired, degraded.
- Handoff contract: what data the website sends to identity, and what it must not collect.
- Recovery copy and failure states.

Expected tests/proof names:

- `setup.registration.entry-route-state`
- `setup.registration.expired-invite-rejected`
- `setup.registration.revoked-invite-rejected`
- `setup.registration.cross-family-rejected`
- `setup.registration.no-child-data-before-household`
- `setup.registration.wrong-role-rejected`
- `setup.registration.authenticated-no-household-state`
- `setup.registration.household-create-required-state`
- `setup.registration.household-child-no-device-state`
- `setup.registration.invite-accept-state`
- `setup.registration.session-expired-state`
- `setup.registration.provider-unavailable-state`
- `setup.registration.redacted-log-proof`

Proof artifact expectations:

- `02-registration-route-state-proof.md`
- `02-auth-handoff-contract-proof.md`
- `02-invite-negative-proof.md`
- `02-no-child-data-before-household-proof.md`
- `02-registration-ui-state-proof.md`
