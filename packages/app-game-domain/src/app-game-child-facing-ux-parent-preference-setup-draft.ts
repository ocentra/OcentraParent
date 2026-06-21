import {
  AppGameChildUxLocalOutboxParentSurfaceIntentReadModelSchema,
  type AppGameChildUxLocalOutboxParentSurfaceIntentReadModel,
  type AppGameChildUxLocalOutboxParentSurfaceIntentRow,
} from '@ocentra-parent/schema-domain/app-game-child-facing-ux-local-outbox-parent-surface-intent';
import {
  AppGameChildUxParentPreferenceSetupDraftReadModelSchema,
  AppGameChildUxParentPreferenceSetupDraftRowSchema,
  AppGameChildUxParentPreferenceSetupDraftStatus,
  RequiredAppGameChildUxParentPreferenceSetupDraftNonClaims,
  type AppGameChildUxParentPreferenceSetupDraftReadModel,
  type AppGameChildUxParentPreferenceSetupDraftRow,
  type AppGameChildUxParentPreferenceSetupDraftStatusValue,
} from '@ocentra-parent/schema-domain/app-game-child-facing-ux-parent-preference-setup-draft';
import { ParentContractSchemaVersion } from '@ocentra-parent/schema-domain/family-reference-primitives';

export type AppGameChildUxParentPreferenceSetupDraftOptions = {
  readonly generatedAt: string;
  readonly draftId: string;
  readonly sourceContractRefs: readonly string[];
};

export function buildAppGameChildUxParentPreferenceSetupDraftReadModel(
  options: AppGameChildUxParentPreferenceSetupDraftOptions,
  sourceReadModel: AppGameChildUxLocalOutboxParentSurfaceIntentReadModel
): AppGameChildUxParentPreferenceSetupDraftReadModel {
  const parsedSource = AppGameChildUxLocalOutboxParentSurfaceIntentReadModelSchema.parse(sourceReadModel);
  const rows = parsedSource.rows.map(parentPreferenceSetupDraftRow);

  return AppGameChildUxParentPreferenceSetupDraftReadModelSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    draftId: options.draftId,
    generatedAt: options.generatedAt,
    family: parsedSource.family,
    sourceParentSurfaceIntentId: parsedSource.intentId,
    sourceContractRefs: options.sourceContractRefs,
    rows,
    draftReadyCount: countDraftStatus(rows, AppGameChildUxParentPreferenceSetupDraftStatus.DraftReady),
    unavailableVisibleCount: countDraftStatus(rows, AppGameChildUxParentPreferenceSetupDraftStatus.UnavailableVisible),
    draftNonClaims: RequiredAppGameChildUxParentPreferenceSetupDraftNonClaims,
    parentPreferenceUiRendered: false,
    parentFrequencyControlUiRendered: false,
    parentPreferenceMutationClaimed: false,
    notificationRuleMutationClaimed: false,
    providerDeliveryRuntimeClaimed: false,
    providerReceiptIngestionClaimed: false,
    childDeliveryClaimed: false,
    productionDurableOutboxStorageClaimed: false,
    adapterDispatchClaimed: false,
    platformEnforcementClaimed: false,
    rawPrivateSourceRowsIncluded: false,
  });
}

function parentPreferenceSetupDraftRow(
  sourceRow: AppGameChildUxLocalOutboxParentSurfaceIntentRow
): AppGameChildUxParentPreferenceSetupDraftRow {
  return AppGameChildUxParentPreferenceSetupDraftRowSchema.parse({
    draftRowId: `app-game-child-ux-parent-preference-setup-draft-${sourceRow.surfaceRowId}`,
    sourceParentSurfaceRowId: sourceRow.surfaceRowId,
    sourceSchedulerEntryRef: sourceRow.sourceSchedulerEntryRef,
    sourceOutboxRecordRef: sourceRow.sourceOutboxRecordRef,
    providerChannel: sourceRow.providerChannel,
    parentPreferenceState: sourceRow.parentPreferenceState,
    quietHoursDecision: sourceRow.quietHoursDecision,
    draftStatus: draftStatusForSourceRow(sourceRow),
    preferenceRequirementRefs: preferenceRequirementRefs(sourceRow),
    quietHoursRequirementRefs: quietHoursRequirementRefs(sourceRow),
    manualProofRequirements: sourceRow.manualProofRequirements,
    parentSafeDrillInRefs: sourceRow.drillInRefs,
    parentPreferenceUiRendered: false,
    parentFrequencyControlUiRendered: false,
    parentPreferenceMutationClaimed: false,
    notificationRuleMutationClaimed: false,
    providerDeliveryClaimed: false,
    childDeliveryClaimed: false,
    adapterDispatchClaimed: false,
    platformEnforcementClaimed: false,
    rawPrivateSourceRowsIncluded: false,
  });
}

function draftStatusForSourceRow(
  sourceRow: AppGameChildUxLocalOutboxParentSurfaceIntentRow
): AppGameChildUxParentPreferenceSetupDraftStatusValue {
  if (sourceRow.preferenceVisibility === 'preference-setup-required') {
    return AppGameChildUxParentPreferenceSetupDraftStatus.DraftReady;
  }
  return AppGameChildUxParentPreferenceSetupDraftStatus.UnavailableVisible;
}

function preferenceRequirementRefs(sourceRow: AppGameChildUxLocalOutboxParentSurfaceIntentRow): readonly string[] {
  return sourceRow.manualProofRequirements.filter(
    (reference: AppGameChildUxLocalOutboxParentSurfaceIntentRow['manualProofRequirements'][number]) =>
      reference.includes('parent-preference')
  );
}

function quietHoursRequirementRefs(sourceRow: AppGameChildUxLocalOutboxParentSurfaceIntentRow): readonly string[] {
  return sourceRow.manualProofRequirements.filter(
    (reference: AppGameChildUxLocalOutboxParentSurfaceIntentRow['manualProofRequirements'][number]) =>
      reference.includes('quiet-hours')
  );
}

function countDraftStatus(
  rows: readonly AppGameChildUxParentPreferenceSetupDraftRow[],
  status: AppGameChildUxParentPreferenceSetupDraftStatusValue
): number {
  return rows.filter((row) => row.draftStatus === status).length;
}
