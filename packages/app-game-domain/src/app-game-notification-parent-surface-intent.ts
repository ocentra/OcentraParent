import {
  AppGameNotificationPreferenceStatusHandoffReadModelSchema,
} from '@ocentra-parent/schema-domain/app-game-notification-preference-status-handoff';
import type {
  AppGameNotificationPreferenceStatusHandoffReadModel,
  AppGameNotificationPreferenceStatusHandoffRow,
} from '@ocentra-parent/schema-domain/app-game-notification-preference-status-handoff';
import {
  AppGameNotificationProviderStatusHandoffReadModelSchema,
} from '@ocentra-parent/schema-domain/app-game-notification-provider-status-handoff';
import type {
  AppGameNotificationProviderStatusHandoffReadModel,
  AppGameNotificationProviderStatusHandoffRow,
} from '@ocentra-parent/schema-domain/app-game-notification-provider-status-handoff';
import {
  AppGameNotificationParentSurfaceIntentReadModelSchema,
  AppGameNotificationParentSurfaceIntentRowSchema,
  RequiredAppGameNotificationParentSurfaceIntentNonClaims,
  appGameNotificationParentSurfaceHistoryVisibilityFor,
} from '@ocentra-parent/schema-domain/app-game-notification-parent-surface-intent';
import type {
  AppGameNotificationParentSurfaceIntentReadModel,
  AppGameNotificationParentSurfaceIntentRow,
} from '@ocentra-parent/schema-domain/app-game-notification-parent-surface-intent';
import { ParentContractSchemaVersion } from '@ocentra-parent/schema-domain/family-reference-primitives';

export type AppGameNotificationParentSurfaceIntentOptions = {
  readonly generatedAt: string;
  readonly intentId: string;
  readonly sourceContractRefs: readonly string[];
};

export function buildAppGameNotificationParentSurfaceIntentReadModel(
  options: AppGameNotificationParentSurfaceIntentOptions,
  providerReadModel: AppGameNotificationProviderStatusHandoffReadModel,
  preferenceReadModel: AppGameNotificationPreferenceStatusHandoffReadModel
): AppGameNotificationParentSurfaceIntentReadModel {
  const parsedProvider = AppGameNotificationProviderStatusHandoffReadModelSchema.parse(providerReadModel);
  const parsedPreference = AppGameNotificationPreferenceStatusHandoffReadModelSchema.parse(preferenceReadModel);
  assertCompatibleInputs(parsedProvider, parsedPreference);

  const rows = parsedProvider.rows.map((providerRow, index) =>
    parentSurfaceIntentRowForStatusRows(providerRow, preferenceRowAt(parsedPreference, index))
  );

  return AppGameNotificationParentSurfaceIntentReadModelSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    intentId: options.intentId,
    generatedAt: options.generatedAt,
    family: parsedProvider.family,
    sourceProviderStatusHandoffId: parsedProvider.handoffId,
    sourcePreferenceStatusHandoffId: parsedPreference.handoffId,
    sourceContractRefs: options.sourceContractRefs,
    rows,
    manualActionRequiredCount: rows.filter((row) => row.parentSurfaceStatus === 'manual-action-required').length,
    unavailableVisibleCount: rows.filter((row) => row.parentSurfaceStatus === 'unavailable-visible').length,
    historyVisibleCount: rows.length,
    preferenceSetupRequiredCount: rows.filter((row) => row.preferenceVisibility === 'preference-setup-required').length,
    parentSurfaceNonClaims: RequiredAppGameNotificationParentSurfaceIntentNonClaims,
    parentNotificationUiRendered: false,
    parentPreferenceUiRendered: false,
    parentFrequencyControlUiRendered: false,
    providerDeliveryRuntimeClaimed: false,
    providerReceiptIngestionClaimed: false,
    providerCredentialsClaimed: false,
    cloudRoutingClaimed: false,
    childDeliveryClaimed: false,
    productionRuntimeClaimed: false,
    productionDurableOutboxStorageClaimed: false,
    adapterDispatchClaimed: false,
  });
}

function assertCompatibleInputs(
  providerReadModel: AppGameNotificationProviderStatusHandoffReadModel,
  preferenceReadModel: AppGameNotificationPreferenceStatusHandoffReadModel
): void {
  if (providerReadModel.family.familyId !== preferenceReadModel.family.familyId) {
    throw new Error('Expected app/game notification parent-surface inputs to use the same family ref');
  }
  if (providerReadModel.rows.length !== preferenceReadModel.rows.length) {
    throw new Error('Expected app/game notification parent-surface inputs to have matching row counts');
  }
}

function preferenceRowAt(
  preferenceReadModel: AppGameNotificationPreferenceStatusHandoffReadModel,
  index: number
): AppGameNotificationPreferenceStatusHandoffRow {
  const preferenceRow = preferenceReadModel.rows[index];
  if (preferenceRow === undefined) {
    throw new Error('Expected app/game notification parent-surface preference row to exist');
  }
  return preferenceRow;
}

function parentSurfaceIntentRowForStatusRows(
  providerRow: AppGameNotificationProviderStatusHandoffRow,
  preferenceRow: AppGameNotificationPreferenceStatusHandoffRow
): AppGameNotificationParentSurfaceIntentRow {
  const providerEntry = providerRow.providerStatusBoundaryEntry;
  const preferenceEntry = preferenceRow.notificationPreferenceStatusEntry;

  return AppGameNotificationParentSurfaceIntentRowSchema.parse({
    surfaceRowId: `app-game-notification-parent-surface-${providerRow.handoffRowId}`,
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
    historyVisibility: appGameNotificationParentSurfaceHistoryVisibilityFor(providerEntry.providerStatus),
    preferenceVisibility:
      preferenceEntry.parentPreferenceState === 'channel-disabled'
        ? 'preference-disabled-visible'
        : 'preference-setup-required',
    drillInRefs: [providerEntry.notificationStatusRef, preferenceEntry.deliveryResultRef],
    auditRefs: [...providerEntry.auditRefs, ...preferenceEntry.auditRefs],
    manualProofRequirements: [...providerEntry.manualProofRequirements, ...preferenceEntry.manualProofRequirements],
    minimalSurfacePayloadBoundary:
      'Parent surface intent contains status refs and setup requirements only; sensitive app/game evidence stays behind authenticated drill-in.',
    sensitiveDetailIncluded: false,
    providerDeliveryClaimed: false,
    providerReceiptClaimed: false,
    parentPreferenceMutationClaimed: false,
    childDeliveryClaimed: false,
  });
}
