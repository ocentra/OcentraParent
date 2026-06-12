# Workpack 02: Registration Login Entry

Goal: define how the family site starts account creation/login without owning identity internals.

Owns: user journey entry, account handoff labels, unauthenticated/authenticated route boundaries, error states, and recovery links.

Handoff: `account-identity-family-plan` owns provider choice, token/session lifecycle, household membership, invites, and recovery authority.

Expected shape:

- Register, login, resume setup, invite accept, password/passkey/email-link recovery, and logout routes.
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

- `registration.entry-route-state`
- `registration.expired-invite-rejected`
- `registration.revoked-invite-rejected`
- `registration.cross-family-rejected`
- `registration.no-child-data-before-household`

Proof artifact expectations:

- Route matrix.
- Screenshot proof for success/error/recovery states when UI exists.
- Auth provider decision reference.
- Logs with redacted email/user identifiers.
