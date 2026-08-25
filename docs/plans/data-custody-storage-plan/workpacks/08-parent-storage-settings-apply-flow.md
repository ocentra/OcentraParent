# Workpack 08: Parent Storage Settings Apply Flow

Goal: define the parent-facing storage settings flow and the apply-back decision surface.

Current exact status:

- `source incomplete / expected tests open`
- Canonical Rust contract owner: `crates/schema/src/parent_storage_settings_apply_flow.rs`
- Rust-owned TS contract generator: `crates/schema/src/parent_storage_settings_apply_flow_ts.rs`
- Canonical Rust runtime owner: `crates/storage-custody-core/src/parent_storage_settings_apply_flow.rs`
- Thin/generated schema edge: `packages/schema-domain/src/generated-parent-storage-settings-apply-flow-contracts.ts` and `packages/schema-domain/src/generated-parent-storage-settings-apply-flow-contract-rules.ts`.
- Existing Rust tests: `crates/storage-custody-core/tests/unit/parent_storage_settings_apply_flow.rs` and `crates/schema/tests/contract/parent_storage_settings_apply_flow.rs`; they were not run in the current source/status reconciliation.
- Exact source defect: `ParentStorageApplyDecisionInput` carries no trusted confirmation receipt or confirmed state. Every preview sets `confirmation_required = true`, so `Applied` and `Partial` are unreachable and always rejected by the apply derivation.
- Required source packet: add an authority-bound, replay-safe confirmation input and a legal confirmed `Applied`/`Partial` decision path without accepting caller-supplied authority or auto-applying provider data.
- Dependency gate: confirmation staging/consume must receive a typed opaque Account WP05 effect handoff backed by the durable Account-owned CAS/recovery owner. The existing WP05 manual-required fence, current-authority CAS, and unrelated mutation-effect table cannot authorize this path; do not bypass the dependency with a caller-made receipt.
- The source attempt ending at `7c232efbfb1c4c4c5f227332e3a66734432276fe` is held and rejected for current integration. Its only `HouseholdAuthorityRuntimeCasFence` implementation returns `RuntimeFenceUnavailable`, and no production caller reaches either confirmation entrypoint. It also consumes the Account effect before staging confirmation in a separate transaction, then marks confirmation consumed before the Data executor receives the handoff. Those crash windows have no durable recoverable outcome. Retain the branch only for narrow redesign after Account WP05 supplies the real owner/coordinator; do not merge or cherry-pick it wholesale.
- Required expected-test packet: successful confirmed `Applied` and `Partial`, missing/expired/replayed/wrong-household confirmation, rollback/manual-required, and unchanged disconnect-versus-delete negatives.
- The old handwritten TypeScript adapter/test and focused proof runner are absent after Rust-first convergence; do not restore them unless a real consumer requires that edge.
- Historical proof root: `output/data-custody-storage-plan-proof/08-parent-storage-settings-apply-flow/` is ignored/absent in a clean checkout and is not current acceptance.
- No-claim boundary: this packet does not claim final portal rendering, desktop host wiring, provider SDK runtime, or automatic provider delete/apply execution.

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

Declared states and current limitation:

- Explicit parent storage mode labels stay typed at the Rust contract boundary and remain visible through the thin/generated schema-domain edge.
- Restore remains retrieve -> preview -> confirm apply, with explicit `importPreviewPassed`, `partialRestore`, `wrongHousehold`, `wrongKey`, `schemaUnsupported`, `bundleCorrupt`, `tombstoneConflict`, and `manualRequired` preview states.
- Apply declares `applyRequiresConfirmation`, `applyPending`, `applied`, `partial`, `rollbackManualRequired`, and `blockedManualRequired` state labels, but the current input cannot prove confirmation, so `applied` and `partial` are not reachable production outcomes yet.
- Disconnect and delete stay separate, and provider delete remains a distinct action from disconnect.
- Claim-safe copy and canonical no-claim rows are checked in from the Rust owner and consumed through the thin schema adapter.

Manual-required states kept explicit:

- Provider backup delete remains `manual-required` until provider runtime proof exists.
- Final portal rendering remains owned by `portal-ux-household-surfaces-plan`.
- Desktop host wiring remains owned by `parent-client-runtime-distribution-plan`.
- Provider SDK runtime and automatic provider delete/apply execution remain unclaimed in this packet.

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
