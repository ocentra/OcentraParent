# Workpack 08: Parent Storage Settings Apply Flow

Goal: define the parent-facing storage settings flow and the apply-back decision surface.

Current exact status:

- `done`
- Canonical Rust contract owner: `crates/schema/src/parent_storage_settings_apply_flow.rs`
- Rust-owned TS contract generator: `crates/schema/src/parent_storage_settings_apply_flow_ts.rs`
- Canonical Rust runtime owner: `crates/storage-custody-core/src/parent_storage_settings_apply_flow.rs`
- Thin/generated schema edge: `packages/schema-domain/src/generated/parent-storage-settings-apply-flow-contracts.ts` and `packages/schema-domain/src/parent-storage-settings-apply-flow.ts`
- Real runtime proof: `crates/storage-custody-core/tests/unit/parent_storage_settings_apply_flow.rs`
- Real schema contract proof: `crates/schema/tests/contract/parent_storage_settings_apply_flow.rs` and `packages/schema-domain/tests/contract/parent-storage-settings-apply-flow.test.ts`
- Focused proof runner: `scripts/test/parent-storage-settings-apply-flow-proof.mjs`
- Proof root: `output/data-custody-storage-plan-proof/08-parent-storage-settings-apply-flow/`
- Focused pass: `cargo test -q -p ocentra-schema --test contract parent_storage_settings_apply_flow`
- Focused pass: `cargo test -q -p ocentra-storage-custody-core parent_storage_settings_apply_flow`
- Focused pass: `cmd /c npm run build --workspace @ocentra-parent/schema-domain`
- Focused pass: `cmd /c npm run test --workspace @ocentra-parent/schema-domain -- tests/contract/parent-storage-settings-apply-flow.test.ts`
- Focused pass: `node scripts/test/parent-storage-settings-apply-flow-proof.mjs`
- Focused pass: `cargo lint-architecture crates/schema/src/lib.rs crates/schema/src/parent_storage_settings_apply_flow.rs crates/schema/src/parent_storage_settings_apply_flow_ts.rs crates/schema/src/bin/export_parent_storage_settings_apply_flow_contract_types.rs crates/schema/tests/contract.rs crates/schema/tests/contract/parent_storage_settings_apply_flow.rs crates/storage-custody-core/src/lib.rs crates/storage-custody-core/src/parent_storage_settings_apply_flow.rs crates/storage-custody-core/tests/unit.rs crates/storage-custody-core/tests/unit/parent_storage_settings_apply_flow.rs`
- Focused pass: `cmd /c npm run lint:architecture -- --files packages/schema-domain/src/generated/parent-storage-settings-apply-flow-contracts.ts packages/schema-domain/src/parent-storage-settings-apply-flow.ts packages/schema-domain/src/parent-storage-settings-apply-flow-rules.ts packages/schema-domain/tests/contract/parent-storage-settings-apply-flow.test.ts scripts/test/parent-storage-settings-apply-flow-proof.mjs`
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

Proved states:

- Explicit parent storage mode labels stay typed at the Rust contract boundary and remain visible through the thin/generated schema-domain edge.
- Restore remains retrieve -> preview -> confirm apply, with explicit `importPreviewPassed`, `partialRestore`, `wrongHousehold`, `wrongKey`, `schemaUnsupported`, `bundleCorrupt`, `tombstoneConflict`, and `manualRequired` preview states.
- Apply remains confirmation-gated with explicit `applyRequiresConfirmation`, `applyPending`, `applied`, `partial`, `rollbackManualRequired`, and `blockedManualRequired` state labels.
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
