import { ParentContractSchemaVersion } from '@ocentra-parent/schema-domain/family-reference-primitives';
import {
  AppGameChildUxLocalOutboxParentSurfaceIntentReadModelSchema,
  AppGameChildUxLocalOutboxParentSurfaceIntentRowSchema,
  RequiredAppGameChildUxLocalOutboxParentSurfaceIntentNonClaims,
  historyVisibilityFor,
  type AppGameChildUxLocalOutboxParentSurfaceIntentReadModel,
  type AppGameChildUxLocalOutboxParentSurfaceIntentRow,
} from '@ocentra-parent/schema-domain/app-game-child-facing-ux-local-outbox-parent-surface-intent';
import {
  AppGameChildUxLocalOutboxPreferenceStatusHandoffReadModelSchema,
  type AppGameChildUxLocalOutboxPreferenceStatusHandoffReadModel,
  type AppGameChildUxLocalOutboxPreferenceStatusHandoffRow,
} from '@ocentra-parent/schema-domain/app-game-child-facing-ux-local-outbox-preference-status-handoff';
import {
  type AppGameChildUxLocalOutboxProviderStatusHandoffReadModel,
  type AppGameChildUxLocalOutboxProviderStatusHandoffRow,
  AppGameChildUxLocalOutboxProviderStatusHandoffReadModelSchema,
} from '@ocentra-parent/schema-domain/app-game-child-facing-ux-local-outbox-provider-status-handoff';

export type AppGameChildUxLocalOutboxParentSurfaceIntentOptions = {
  readonly generatedAt: string;
  readonly intentId: string;
  readonly sourceContractRefs: readonly string[];
};

export function buildAppGameChildUxLocalOutboxParentSurfaceIntentReadModel(
  options: AppGameChildUxLocalOutboxParentSurfaceIntentOptions,
  providerReadModel: AppGameChildUxLocalOutboxProviderStatusHandoffReadModel,
  preferenceReadModel: AppGameChildUxLocalOutboxPreferenceStatusHandoffReadModel
): AppGameChildUxLocalOutboxParentSurfaceIntentReadModel {
  const parsedProvider = AppGameChildUxLocalOutboxProviderStatusHandoffReadModelSchema.parse(providerReadModel);
  const parsedPreference = AppGameChildUxLocalOutboxPreferenceStatusHandoffReadModelSchema.parse(preferenceReadModel);
  assertCompatibleInputs(parsedProvider, parsedPreference);

  const rows = parsedProvider.rows.map((providerRow, index) =>
    parentSurfaceIntentRowForStatusRows(providerRow, preferenceRowAt(parsedPreference, index))
  );

  return AppGameChildUxLocalOutboxParentSurfaceIntentReadModelSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    intentId: options.intentId,
    generatedAt: options.generatedAt,
    family: parsedProvider.family,
    sourceProviderStatusHandoffId: parsedProvider.handoffId,
    sourcePreferenceStatusHandoffId: parsedPreference.handoffId,
    sourceContractRefs: options.sourceContractRefs,
    rows,
    manualActionRequiredCount: countSurfaceStatus(rows, 'manual-action-required'),
    unavailableVisibleCount: countSurfaceStatus(rows, 'unavailable-visible'),
    historyVisibleCount: rows.length,
    preferenceSetupRequiredCount: countPreferenceVisibility(rows, 'preference-setup-required'),
    parentSurfaceNonClaims: RequiredAppGameChildUxLocalOutboxParentSurfaceIntentNonClaims,
    parentNotificationUiRendered: false,
    parentPreferenceUiRendered: false,
    parentFrequencyControlUiRendered: false,
    parentPreferenceMutationClaimed: false,
    providerDeliveryRuntimeClaimed: false,
    providerReceiptIngestionClaimed: false,
    providerCredentialsClaimed: false,
    cloudRoutingClaimed: false,
    childDeliveryClaimed: false,
    productionRuntimeClaimed: false,
    productionDurableOutboxStorageClaimed: false,
    adapterDispatchClaimed: false,
    platformEnforcementClaimed: false,
    rawPrivateSourceRowsIncluded: false,
  });
}

function assertCompatibleInputs(
  providerReadModel: AppGameChildUxLocalOutboxProviderStatusHandoffReadModel,
  preferenceReadModel: AppGameChildUxLocalOutboxPreferenceStatusHandoffReadModel
): void {
  if (providerReadModel.family.familyId !== preferenceReadModel.family.familyId) {
    throw new Error('Expected child UX parent-surface inputs to use the same family ref');
  }
  if (providerReadModel.rows.length !== preferenceReadModel.rows.length) {
    throw new Error('Expected child UX parent-surface inputs to have matching row counts');
  }
}

function preferenceRowAt(
  preferenceReadModel: AppGameChildUxLocalOutboxPreferenceStatusHandoffReadModel,
  index: number
): AppGameChildUxLocalOutboxPreferenceStatusHandoffRow {
  const preferenceRow = preferenceReadModel.rows[index];
  if (preferenceRow === undefined) {
    throw new Error('Expected child UX parent-surface preference row to exist');
  }
  return preferenceRow;
}

function parentSurfaceIntentRowForStatusRows(
  providerRow: AppGameChildUxLocalOutboxProviderStatusHandoffRow,
  preferenceRow: AppGameChildUxLocalOutboxPreferenceStatusHandoffRow
): AppGameChildUxLocalOutboxParentSurfaceIntentRow {
  const providerEntry = providerRow.providerStatusBoundaryEntry;
  const preferenceEntry = preferenceRow.notificationPreferenceStatusEntry;

  return AppGameChildUxLocalOutboxParentSurfaceIntentRowSchema.parse({
    surfaceRowId: `app-game-child-ux-parent-surface-${providerRow.handoffRowId}`,
    sourceProviderHandoffRowId: providerRow.handoffRowId,
    sourcePreferenceHandoffRowId: preferenceRow.handoffRowId,
    sourceSchedulerEntryRef: providerRow.sourceSchedulerEntryRef ?? preferenceRow.sourceSchedulerEntryRef,
    sourceOutboxRecordRef: providerRow.sourceOutboxRecordRef ?? preferenceRow.sourceOutboxRecordRef,
    providerStatus: providerEntry.providerStatus,
    deliveryResultState: preferenceEntry.deliveryResultState,
    parentPreferenceState: preferenceEntry.parentPreferenceState,
    quietHoursDecision: preferenceEntry.quietHoursDecision,
    providerChannel: preferenceEntry.providerChannel,
    parentSurfaceStatus:
      providerEntry.providerStatus === 'unavailable' ? 'unavailable-visible' : 'manual-action-required',
    historyVisibility: historyVisibilityFor(providerEntry.providerStatus),
    preferenceVisibility:
      preferenceEntry.parentPreferenceState === 'channel-disabled'
        ? 'preference-disabled-visible'
        : 'preference-setup-required',
    drillInRefs: [providerEntry.notificationStatusRef, preferenceEntry.deliveryResultRef],
    auditRefs: [...providerEntry.auditRefs, ...preferenceEntry.auditRefs],
    manualProofRequirements: [...providerEntry.manualProofRequirements, ...preferenceEntry.manualProofRequirements],
    minimalSurfacePayloadBoundary:
      'Child UX parent surface intent contains setup status refs only; sensitive app/game evidence stays behind authenticated drill-in.',
    childUxLocalOutboxSurfaceClaim:
      'Parent-visible child UX local outbox setup row only; no rendered UI, delivery, preference mutation, adapter dispatch, or platform enforcement is claimed.',
    sensitiveDetailIncluded: false,
    providerDeliveryClaimed: false,
    providerReceiptClaimed: false,
    parentPreferenceMutationClaimed: false,
    childDeliveryClaimed: false,
    adapterDispatchClaimed: false,
    platformEnforcementClaimed: false,
  });
}

const countSurfaceStatus = (
  rows: readonly AppGameChildUxLocalOutboxParentSurfaceIntentRow[],
  status: AppGameChildUxLocalOutboxParentSurfaceIntentRow['parentSurfaceStatus']
): number => rows.filter((row) => row.parentSurfaceStatus === status).length;

const countPreferenceVisibility = (
  rows: readonly AppGameChildUxLocalOutboxParentSurfaceIntentRow[],
  visibility: AppGameChildUxLocalOutboxParentSurfaceIntentRow['preferenceVisibility']
): number => rows.filter((row) => row.preferenceVisibility === visibility).length;
