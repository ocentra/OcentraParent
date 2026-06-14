# Workpack 08: Parent Storage Settings Apply Flow

Goal: define the parent-facing storage settings flow and the apply-back decision surface.

Context to read:

- `docs/plans/data-custody-storage-plan/UI_EXPECTATIONS.md`
- `docs/plans/data-custody-storage-plan/RESEARCH_AND_UI_GUIDANCE.md`
- `docs/plans/data-custody-storage-plan/PARENT_STORAGE_PROVIDER_MATRIX.md`
- `docs/plans/data-custody-storage-plan/PARENT_SAVE_RETRIEVE_APPLY_FLOW.md`
- `docs/plans/data-custody-storage-plan/PLAN_STATE.md`
- `docs/features/reports-notifications-sync.md`
- `docs/features/remote-lan-mobile-platforms.md`

In scope:

- Data and Backup or Data Custody / Storage screen.
- Storage destination cards and mode chooser.
- Current mode, encryption, key, sync, retention, restore, and provider health cards.
- Restore preview and apply confirmation.
- Delete and disconnect flow.
- Manual-required and claim-safe copy.

Out of scope:

- UI styling or polish.
- OAuth or provider SDK implementation.
- Route contract changes without a selected adjacent plan.

Acceptance:

- Parent can choose storage mode without a vague cloud toggle.
- Parent can see honest current mode and failure state.
- Restore is previewed before apply.
- Disconnect and delete are separate actions.
- Manual-required states are visible instead of hidden.

Expected artifacts:

- UI expectations doc.
- Provider matrix.
- Platform custody matrix.
- Save, retrieve, apply flow doc.
- Research and UI guidance doc.
- Claim-safe copy inventory.

Expected proof names:

- `data-custody.ui.state-proof`
- `data-custody.ui.restore-preview-flow`
- `data-custody.ui.delete-disconnect-flow`
- `data-custody.ui.no-claim-language`

Adjacent handoff:

- `portal-ux-household-surfaces-plan` owns the final portal surface implementation.
- `parent-client-runtime-distribution-plan` owns the desktop host and packaging wiring.
- `account-identity-family-plan` owns identity and household authority when storage access depends on it.

Failure conditions:

- Generic "cloud sync" labels instead of explicit custody modes.
- Auto-apply of provider data.
- Disconnect quietly performing provider delete or local delete.
- Manual-required states hidden behind a success-looking UI.
