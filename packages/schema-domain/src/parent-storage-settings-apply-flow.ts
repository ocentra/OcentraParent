/* thin adapter over Rust-generated parent storage settings apply flow contracts */

import { type Infer, Schema, brandedNonEmptyStringSchema, withParser } from './effect';
import { countProductionProofValues } from './proof-shape';
import { ParentContractSchemaVersionSchema, ParentTimestampSchema } from './family-reference-primitives';
import {
  ParentOwnedSyncExportDataClassSchema,
  ParentOwnedSyncExportDeleteVisibilityStateSchema,
  ParentOwnedSyncExportDisconnectVisibilityStateSchema,
  ParentOwnedSyncExportProviderModeSchema,
  ParentOwnedSyncExportProviderStatusSchema,
  ParentOwnedSyncExportSyncStateSchema,
} from './parent-owned-sync-export';
import {
  GeneratedParentStorageApplyStates,
  type GeneratedParentStorageDeleteActionRow,
  GeneratedParentStorageCopyKeys,
  GeneratedParentStorageDeleteActionKinds,
  GeneratedParentStorageEncryptionStatuses,
  GeneratedParentStorageKeyStatuses,
  GeneratedParentStorageKnownGaps,
  GeneratedParentStorageModeLabels,
  GeneratedParentStorageNoClaims,
  GeneratedParentStoragePreviewStates,
  GeneratedParentStorageSettingsApplyFlowContractProof,
  GeneratedParentStorageUiStates,
  ParentStorageSettingsApplyFlowContractRuntime,
  type GeneratedParentStorageClaimSafeCopyRow,
  type GeneratedParentStorageSettingsApplyFlowContractProof as GeneratedParentStorageSettingsApplyFlowContractProofShape,
} from './generated-parent-storage-settings-apply-flow-contracts';
import {
  parentStorageApplyDecisionIsHonestGenerated,
  parentStorageClaimSafeCopyRowIsHonestGenerated,
  parentStorageDeleteActionRowIsHonestGenerated,
  parentStorageDisconnectRowIsHonestGenerated,
  parentStorageModeCardIsHonestGenerated,
  parentStorageRestorePreviewIsHonestGenerated,
  parentStorageSettingsApplyFlowProofIsHonestGenerated,
} from './generated-parent-storage-settings-apply-flow-contract-rules';

export const ParentStorageSettingsApplyFlowSchemaVersionSchema = withParser(
  Schema.Literal(ParentStorageSettingsApplyFlowContractRuntime.SchemaVersion)
);

export const RequiredParentStorageModeLabels = [...GeneratedParentStorageModeLabels] as const;
export const RequiredParentStorageDeleteActionKinds = [...GeneratedParentStorageDeleteActionKinds] as const;
export const RequiredParentStorageNoClaims = [...GeneratedParentStorageNoClaims] as const;
export const ParentStorageSettingsApplyFlowKnownGaps = [...GeneratedParentStorageKnownGaps] as const;

const ParentStorageModeLabelSchema = withParser(Schema.Literal(...GeneratedParentStorageModeLabels));
const ParentStorageUiStateSchema = withParser(Schema.Literal(...GeneratedParentStorageUiStates));
const ParentStorageEncryptionStatusSchema = withParser(Schema.Literal(...GeneratedParentStorageEncryptionStatuses));
const ParentStorageKeyStatusSchema = withParser(Schema.Literal(...GeneratedParentStorageKeyStatuses));
const ParentStoragePreviewStateSchema = withParser(Schema.Literal(...GeneratedParentStoragePreviewStates));
const ParentStorageApplyStateSchema = withParser(Schema.Literal(...GeneratedParentStorageApplyStates));
const ParentStorageDeleteActionKindSchema = withParser(Schema.Literal(...GeneratedParentStorageDeleteActionKinds));
const ParentStorageCopyKeySchema = withParser(Schema.Literal(...GeneratedParentStorageCopyKeys));
const ParentStorageNoClaimSchema = withParser(Schema.Literal(...GeneratedParentStorageNoClaims));

const ParentStorageSettingsRowIdSchema = brandedNonEmptyStringSchema('ParentStorageSettingsRowId');
const ParentStoragePreviewIdSchema = brandedNonEmptyStringSchema('ParentStoragePreviewId');
const ParentStorageApplyIdSchema = brandedNonEmptyStringSchema('ParentStorageApplyId');
const ParentStorageActionIdSchema = brandedNonEmptyStringSchema('ParentStorageActionId');
const ParentStorageTextSchema = brandedNonEmptyStringSchema('ParentStorageText');

export type ParentStorageSettingsRowId = typeof ParentStorageSettingsRowIdSchema.Type;
export type ParentStoragePreviewId = typeof ParentStoragePreviewIdSchema.Type;
export type ParentStorageApplyId = typeof ParentStorageApplyIdSchema.Type;
export type ParentStorageActionId = typeof ParentStorageActionIdSchema.Type;
export type ParentStorageTimestamp = typeof ParentTimestampSchema.Type;
export type ParentStorageModeLabel = Infer<typeof ParentStorageModeLabelSchema>;
export type ParentStorageApplyState = Infer<typeof ParentStorageApplyStateSchema>;
export type ParentStorageDeleteActionKind = Infer<typeof ParentStorageDeleteActionKindSchema>;
export type ParentStorageNoClaim = Infer<typeof ParentStorageNoClaimSchema>;

export const ParentStorageModeCardSchema = withParser(
  Schema.Struct({
    rowId: ParentStorageSettingsRowIdSchema,
    currentModeLabel: ParentStorageModeLabelSchema,
    uiState: ParentStorageUiStateSchema,
    providerMode: ParentOwnedSyncExportProviderModeSchema,
    providerStatus: ParentOwnedSyncExportProviderStatusSchema,
    syncState: ParentOwnedSyncExportSyncStateSchema,
    encryptionStatus: ParentStorageEncryptionStatusSchema,
    keyStatus: ParentStorageKeyStatusSchema,
    manualRequiredVisible: Schema.Boolean,
    disconnectVisible: Schema.Boolean,
    deleteVisible: Schema.Boolean,
    restorePreviewAvailable: Schema.Boolean,
    applyBackAvailable: Schema.Boolean,
    lastSuccessAt: Schema.Union(ParentTimestampSchema, Schema.Null),
    lastFailureAt: Schema.Union(ParentTimestampSchema, Schema.Null),
    summary: ParentStorageTextSchema,
  }).pipe(
    Schema.filter(
      (card) =>
        parentStorageModeCardIsHonestGenerated(
          card as GeneratedParentStorageSettingsApplyFlowContractProofShape['modeCard']
        ) ||
        'Expected the parent storage mode card to keep explicit mode, failure, and manual-required states honest'
    )
  )
);

export const ParentStorageRestorePreviewSchema = withParser(
  Schema.Struct({
    previewId: ParentStoragePreviewIdSchema,
    previewState: ParentStoragePreviewStateSchema,
    createdAt: ParentTimestampSchema,
    productVersion: ParentStorageTextSchema,
    schemaVersion: ParentStorageTextSchema,
    householdMatch: Schema.Boolean,
    deviceMatch: Schema.Boolean,
    dataClasses: Schema.Array(ParentOwnedSyncExportDataClassSchema),
    conflicts: Schema.Array(ParentStorageTextSchema),
    rejectedSections: Schema.Array(ParentOwnedSyncExportDataClassSchema),
    partialRestore: Schema.Boolean,
    confirmationRequired: Schema.Boolean,
    localTruthAuthoritative: Schema.Boolean,
    tombstonesPreserved: Schema.Boolean,
    manualRequiredNote: Schema.Union(ParentStorageTextSchema, Schema.Null),
  }).pipe(
    Schema.filter(
      (preview) =>
        parentStorageRestorePreviewIsHonestGenerated(
          preview as GeneratedParentStorageSettingsApplyFlowContractProofShape['restorePreview']
        ) || 'Expected restore preview to stay non-mutating, confirmation-gated, and tombstone-aware'
    )
  )
);

export const ParentStorageApplyDecisionSchema = withParser(
  Schema.Struct({
    applyId: ParentStorageApplyIdSchema,
    applyState: ParentStorageApplyStateSchema,
    confirmationRequired: Schema.Boolean,
    willChange: Schema.Array(ParentOwnedSyncExportDataClassSchema),
    willNotChange: Schema.Array(ParentOwnedSyncExportDataClassSchema),
    preservedTombstones: Schema.Array(ParentOwnedSyncExportDataClassSchema),
    manualReviewRequired: Schema.Array(ParentStorageTextSchema),
    rollbackAvailable: Schema.Boolean,
    manualRequiredNote: Schema.Union(ParentStorageTextSchema, Schema.Null),
  }).pipe(
    Schema.filter(
      (decision) =>
        parentStorageApplyDecisionIsHonestGenerated(
          decision as GeneratedParentStorageSettingsApplyFlowContractProofShape['applyDecision']
        ) || 'Expected apply state to stay confirmation-gated and explicit about manual-required outcomes'
    )
  )
);

export const ParentStorageDeleteActionRowSchema = withParser(
  Schema.Struct({
    actionId: ParentStorageActionIdSchema,
    actionKind: ParentStorageDeleteActionKindSchema,
    state: ParentOwnedSyncExportDeleteVisibilityStateSchema,
    separateFromDisconnect: Schema.Boolean,
    proofRequired: Schema.Boolean,
    notes: ParentStorageTextSchema,
  }).pipe(
    Schema.filter(
      (row) =>
        parentStorageDeleteActionRowIsHonestGenerated(row as GeneratedParentStorageDeleteActionRow) ||
        'Expected delete actions to stay separate from disconnect and explicit about proof'
    )
  )
);

export const ParentStorageDisconnectRowSchema = withParser(
  Schema.Struct({
    actionId: ParentStorageActionIdSchema,
    state: ParentOwnedSyncExportDisconnectVisibilityStateSchema,
    existingFilesMayRemain: Schema.Boolean,
    providerDeleteRequestedSeparately: Schema.Boolean,
    notes: ParentStorageTextSchema,
  }).pipe(
    Schema.filter(
      (row) =>
        parentStorageDisconnectRowIsHonestGenerated(
          row as GeneratedParentStorageSettingsApplyFlowContractProofShape['disconnectAction']
        ) ||
        'Expected disconnect to stay separate from provider delete and preserve existing-files-may-remain copy'
    )
  )
);

export const ParentStorageClaimSafeCopyRowSchema = withParser(
  Schema.Struct({
    copyKey: ParentStorageCopyKeySchema,
    statement: ParentStorageTextSchema,
    forbiddenWithoutState: Schema.Boolean,
    notes: ParentStorageTextSchema,
  }).pipe(
    Schema.filter(
      (row) =>
        parentStorageClaimSafeCopyRowIsHonestGenerated(row as GeneratedParentStorageClaimSafeCopyRow) ||
        'Expected claim-safe copy rows to stay explicit and state-gated'
    )
  )
);

export const ParentStorageSettingsApplyFlowContractProofSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentStorageSettingsApplyFlowSchemaVersionSchema,
    contractVersion: ParentContractSchemaVersionSchema,
    modeCard: ParentStorageModeCardSchema,
    restorePreview: ParentStorageRestorePreviewSchema,
    applyDecision: ParentStorageApplyDecisionSchema,
    deleteActions: Schema.Array(ParentStorageDeleteActionRowSchema),
    disconnectAction: ParentStorageDisconnectRowSchema,
    claimSafeCopy: Schema.Array(ParentStorageClaimSafeCopyRowSchema),
    noClaims: Schema.Array(ParentStorageNoClaimSchema),
    updatedAt: ParentTimestampSchema,
  }).pipe(
    Schema.filter(
      (proof) =>
        parentStorageSettingsApplyFlowProofIsHonestGenerated(
          proof as GeneratedParentStorageSettingsApplyFlowContractProofShape
        ) || 'Expected parent storage settings apply flow proof to keep preview, disconnect, delete, and no-claim boundaries honest'
    )
  )
);

export type ParentStorageSettingsApplyFlowContractProof = Infer<typeof ParentStorageSettingsApplyFlowContractProofSchema>;

export const ParentStorageSettingsApplyFlowContractProofReadModel =
  ParentStorageSettingsApplyFlowContractProofSchema.parse(GeneratedParentStorageSettingsApplyFlowContractProof);

export function summarizeParentStorageModeLabels(
  labels: ReadonlyArray<ParentStorageModeLabel>
): Record<ParentStorageModeLabel, number> {
  return countProductionProofValues(labels, RequiredParentStorageModeLabels);
}

export function summarizeParentStorageDeleteActionKinds(
  rows: ReadonlyArray<Infer<typeof ParentStorageDeleteActionRowSchema>>
): Record<ParentStorageDeleteActionKind, number> {
  return countProductionProofValues(
    rows.map((row) => row.actionKind),
    RequiredParentStorageDeleteActionKinds
  );
}
