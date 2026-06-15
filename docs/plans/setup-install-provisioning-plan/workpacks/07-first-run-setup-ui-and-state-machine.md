# Workpack 07: First-Run Setup UI And State Machine

Goal: define the exact first-run parent-visible setup sequence and state machine from public site to household-ready status.

Owns: first-run UI screens, state labels, readiness cards, degraded/manual-required states, and parent-visible setup flow.

Does not own: package build mechanics, auth provider implementation, LAN protocol internals, or portal component internals.

Expected shape:

- Public site / invite entry
- Create account / sign in
- Create or join household
- Parent install link / QR / code
- Parent bootstrap tutorial / agreement
- Parent bootstrap code entry
- Parent package download / install progress
- Parent portal guided setup start
- Create child profile
- Generate child pairing link / QR / code
- Child install instructions
- Waiting for child device
- Child detected / confirm trust
- Permission readiness checklist
- Policy baseline setup
- Data custody status
- Setup complete / setup blocked / manual required

Expected proof:

- First-run UI state machine.
- Empty/error/degraded UI proof.
- Manual-required visible proof.
- No-fake-ready proof.
- Adjacent handoff proof to account, package, LAN, and data custody owners.

Failure: UI that implies install or pairing alone means trust, readiness, or policy baseline completion.

## Execution Detail

Minimum context:

- `docs/expectations/family-setup.md`
- `docs/expectations/release-installer.md`
- `docs/expectations/platform-deliverables.md`
- `docs/expectations/data-custody.md`
- `docs/plans/setup-install-provisioning-plan/SETUP_STATE_MACHINE.md`
- `docs/plans/setup-install-provisioning-plan/PAIRING_READINESS_MODEL.md`

Rules:

- Keep parent account, parent bootstrap, child bootstrap, pairing, readiness, and recovery separate.
- Show manual-required states explicitly.
- Never claim setup complete until the readiness matrix is visible.

Required screens:

- Welcome
- Sign in / create account
- Create or join household
- Parent install link / QR / code
- Parent bootstrap tutorial / agreement
- Parent bootstrap code entry
- Parent package download / install progress
- Parent portal guided setup start
- Create child profile
- Generate child pairing link / QR / code
- Child install instructions
- Waiting for child device
- Child detected / confirm trust
- Permission readiness checklist
- Policy baseline setup
- Data custody status
- Setup complete / setup blocked / manual required

Expected tests/proof names:

- `setup.first-run.state-machine`
- `setup.first-run.welcome-screen`
- `setup.first-run.sign-in-screen`
- `setup.first-run.create-household-screen`
- `setup.first-run.join-household-screen`
- `setup.first-run.child-profile-screen`
- `setup.first-run.parent-install-screen`
- `setup.first-run.child-install-screen`
- `setup.first-run.pair-device-screen`
- `setup.first-run.permission-checklist-screen`
- `setup.first-run.readiness-checklist-screen`
- `setup.first-run.recovery-screen`
- `setup.first-run.setup-complete-screen`
- `setup.first-run.empty-error-degraded-ui`
- `setup.first-run.manual-required-visible`
- `setup.first-run.adjacent-handoff-visible`
- `setup.first-run.no-fake-ready-state`

Proof artifact expectations:

- `07-first-run-state-machine-proof.md`
- `07-first-run-ui-screen-map.md`
- `07-empty-error-degraded-ui-proof.md`
- `07-manual-required-visible-proof.md`
- `07-no-fake-ready-state-proof.md`
