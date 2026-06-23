import {
  AppGameChildUxLocalOutboxProviderPreflightReadModelSchema,
  AppGameChildUxLocalOutboxProviderPreflightStatus,
  RequiredAppGameChildUxLocalOutboxProviderPreflightNonClaims,
  type AppGameChildUxLocalOutboxProviderPreflightReadModel,
} from '@ocentra-parent/schema-domain/app-game-child-facing-ux-local-outbox-provider-preflight';
import {
  AppGameChildUxLocalOutboxPreferencePreflightReadModelSchema,
  AppGameChildUxLocalOutboxPreferencePreflightStatus,
  RequiredAppGameChildUxLocalOutboxPreferencePreflightNonClaims,
  type AppGameChildUxLocalOutboxPreferencePreflightReadModel,
} from '@ocentra-parent/schema-domain/app-game-child-facing-ux-local-outbox-preference-preflight';
import {
  AppGameChildUxLocalOutboxProviderStatusHandoffReadModelSchema,
  RequiredAppGameChildUxLocalOutboxProviderStatusHandoffNonClaims,
  type AppGameChildUxLocalOutboxProviderStatusHandoffReadModel,
} from '@ocentra-parent/schema-domain/app-game-child-facing-ux-local-outbox-provider-status-handoff';
import {
  AppGameChildUxLocalOutboxPreferenceStatusHandoffReadModelSchema,
  RequiredAppGameChildUxLocalOutboxPreferenceStatusHandoffNonClaims,
  type AppGameChildUxLocalOutboxPreferenceStatusHandoffReadModel,
} from '@ocentra-parent/schema-domain/app-game-child-facing-ux-local-outbox-preference-status-handoff';
import {
  AppGameChildUxLocalOutboxParentSurfaceIntentReadModelSchema,
  RequiredAppGameChildUxLocalOutboxParentSurfaceIntentNonClaims,
  historyVisibilityFor,
  type AppGameChildUxLocalOutboxParentSurfaceIntentReadModel,
} from '@ocentra-parent/schema-domain/app-game-child-facing-ux-local-outbox-parent-surface-intent';
import {
  AppGameChildUxParentPreferenceSetupDraftReadModelSchema,
  AppGameChildUxParentPreferenceSetupDraftStatus,
  RequiredAppGameChildUxParentPreferenceSetupDraftNonClaims,
  type AppGameChildUxParentPreferenceSetupDraftReadModel,
} from '@ocentra-parent/schema-domain/app-game-child-facing-ux-parent-preference-setup-draft';
import {
  AppGameChildUxLocalOutboxSchedulerBridgeReadModelSchema,
  AppGameChildUxLocalOutboxSchedulerBridgeStatus,
} from '@ocentra-parent/schema-domain/app-game-child-facing-ux-local-outbox-scheduler-bridge';
import type { AppGameChildUxLocalOutboxSchedulerBridgeReadModel } from '@ocentra-parent/schema-domain/app-game-child-facing-ux-local-outbox-scheduler-bridge';
import { ParentContractSchemaVersion } from '@ocentra-parent/schema-domain/family-reference-primitives';
import {
  V08NotificationProviderStatusBoundaryReadModel,
  type V08NotificationProviderStatus,
} from '@ocentra-parent/schema-domain/v0-8-notification-provider-status-boundary';
import {
  V3NotificationRuleProviderRetryContractReadModel,
  type V3NotificationProviderChannel,
} from '@ocentra-parent/schema-domain/notification-v3-provider-retry';

type ProviderPreflightOptions = {
  readonly generatedAt: string;
  readonly providerPreflightId: string;
  readonly sourceContractRefs: readonly string[];
};

type ProviderStatusOptions = {
  readonly generatedAt: string;
  readonly handoffId: string;
  readonly sourceContractRefs: readonly string[];
};

type PreferencePreflightOptions = {
  readonly generatedAt: string;
  readonly preferencePreflightId: string;
  readonly sourceContractRefs: readonly string[];
};

type PreferenceStatusOptions = {
  readonly generatedAt: string;
  readonly handoffId: string;
  readonly sourceContractRefs: readonly string[];
};

type ParentSurfaceOptions = {
  readonly generatedAt: string;
  readonly intentId: string;
  readonly sourceContractRefs: readonly string[];
};

type ParentPreferenceSetupDraftOptions = {
  readonly generatedAt: string;
  readonly draftId: string;
  readonly sourceContractRefs: readonly string[];
};

type SchedulerBridgeRow = AppGameChildUxLocalOutboxSchedulerBridgeReadModel['rows'][number];
type SchedulerPreflightMode = 'scheduled' | 'unavailable' | 'manual';

const ProviderStatusBoundaryCoverageRefs = V08NotificationProviderStatusBoundaryReadModel.entries.map(
  (entry) => entry.statusEntryId
);
const PreferenceStatusCoverageRefs = V3NotificationRuleProviderRetryContractReadModel.entries.map(
  (entry) => entry.contractEntryId
);

export function buildAppGameChildUxLocalOutboxProviderPreflightReadModel(
  options: ProviderPreflightOptions,
  sourceReadModel: AppGameChildUxLocalOutboxSchedulerBridgeReadModel
): AppGameChildUxLocalOutboxProviderPreflightReadModel {
  const source = AppGameChildUxLocalOutboxSchedulerBridgeReadModelSchema.parse(sourceReadModel);
  const rows = source.rows.map(providerPreflightRow);

  return AppGameChildUxLocalOutboxProviderPreflightReadModelSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    providerPreflightId: options.providerPreflightId,
    generatedAt: options.generatedAt,
    family: source.family,
    sourceSchedulerBridgeId: source.schedulerBridgeId,
    sourceContractRefs: [...options.sourceContractRefs],
    rows,
    providerAdapterRequiredCount: countRows(
      rows,
      AppGameChildUxLocalOutboxProviderPreflightStatus.ProviderAdapterRequired
    ),
    manualRequiredCount: countRows(rows, AppGameChildUxLocalOutboxProviderPreflightStatus.ManualRequired),
    unavailableCount: countRows(rows, AppGameChildUxLocalOutboxProviderPreflightStatus.Unavailable),
    preflightNonClaims: [...RequiredAppGameChildUxLocalOutboxProviderPreflightNonClaims],
    childDeliveryRuntimeClaimed: false,
    providerDeliveryRuntimeClaimed: false,
    providerReceiptIngestionClaimed: false,
    providerCredentialsClaimed: false,
    cloudRoutingClaimed: false,
    parentNotificationUiClaimed: false,
    retryExecutionRuntimeClaimed: false,
    quietHoursTimerRuntimeClaimed: false,
    productionDurableOutboxStorageClaimed: false,
    adapterDispatchClaimed: false,
    platformEnforcementClaimed: false,
    rawPrivateSourceRowsIncluded: false,
  });
}

export function buildAppGameChildUxLocalOutboxProviderStatusHandoffReadModel(
  options: ProviderStatusOptions,
  sourceReadModel: AppGameChildUxLocalOutboxProviderPreflightReadModel
): AppGameChildUxLocalOutboxProviderStatusHandoffReadModel {
  const source = AppGameChildUxLocalOutboxProviderPreflightReadModelSchema.parse(sourceReadModel);
  const rows = source.rows.map((row) => {
    const unavailable = row.status === AppGameChildUxLocalOutboxProviderPreflightStatus.Unavailable;
    const providerStatus: V08NotificationProviderStatus = unavailable ? 'unavailable' : 'manual-required';
    const baseId = row.preflightRowId;

    return {
      handoffRowId: `app-game-child-ux-provider-status-handoff-${baseId}`,
      sourcePreflightRowId: row.preflightRowId,
      sourcePreflightStatus: row.status,
      sourceSchedulerEntryRef: row.sourceSchedulerEntryRef,
      sourceOutboxRecordRef: row.sourceOutboxRecordRef,
      sourceProviderChannelRef: row.providerChannelRef,
      providerStatusBoundaryEntry: {
        schemaVersion: ParentContractSchemaVersion.V0_6,
        statusEntryId: `app-game-child-ux-provider-status-entry-${baseId}`,
        providerStatus,
        statusProofState: unavailable ? 'provider-unavailable-contract' : 'manual-action-required',
        quietHoursReadiness: unavailable ? 'unavailable' : 'manual-required',
        escalationReadiness: unavailable ? 'unavailable' : 'manual-required',
        deliveryClaimState: 'not-observed',
        notificationIntentRef: `app-game-child-ux-provider-intent-${baseId}`,
        notificationStatusRef: `app-game-child-ux-provider-status-ref-${baseId}`,
        providerAttemptRef: `app-game-child-ux-provider-attempt-${baseId}`,
        auditRefs: auditRefs(baseId),
        preferenceRefs: preferenceRefs(baseId),
        readinessRefs: readinessRefs(baseId),
        providerReceiptRefs: [],
        manualProofRequirements: [...row.manualProofRequirements],
        minimalPayloadBoundary:
          'Child UX provider status remains contract-only: no adapter send, no receipt ingestion, and no provider payload custody.',
        providerDeliveryImplemented: false,
        providerDeliveryObserved: false,
        deliveredNotificationClaimed: false,
        sensitiveProviderPayloadClaimed: false,
        providerStoresChildEvidenceClaimed: false,
        lastCheckedAt: options.generatedAt,
      },
      manualProofRequirements: [...row.manualProofRequirements],
    };
  });

  return AppGameChildUxLocalOutboxProviderStatusHandoffReadModelSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    handoffId: options.handoffId,
    generatedAt: options.generatedAt,
    family: source.family,
    sourceProviderPreflightId: source.providerPreflightId,
    sourceContractRefs: [...options.sourceContractRefs],
    providerStatusBoundaryReadModelRef: V08NotificationProviderStatusBoundaryReadModel.readModelId,
    providerStatusBoundaryCoverageRefs: [...ProviderStatusBoundaryCoverageRefs],
    rows,
    providerStatusManualRequiredCount: rows.filter(
      (row) => row.providerStatusBoundaryEntry.providerStatus === 'manual-required'
    ).length,
    providerStatusUnavailableCount: rows.filter(
      (row) => row.providerStatusBoundaryEntry.providerStatus === 'unavailable'
    ).length,
    handoffNonClaims: [...RequiredAppGameChildUxLocalOutboxProviderStatusHandoffNonClaims],
    childDeliveryRuntimeClaimed: false,
    providerDeliveryRuntimeClaimed: false,
    providerReceiptIngestionClaimed: false,
    providerCredentialsClaimed: false,
    cloudRoutingClaimed: false,
    parentNotificationUiClaimed: false,
    retryExecutionRuntimeClaimed: false,
    quietHoursTimerRuntimeClaimed: false,
    productionDurableOutboxStorageClaimed: false,
    adapterDispatchClaimed: false,
    platformEnforcementClaimed: false,
    rawPrivateSourceRowsIncluded: false,
  });
}

export function buildAppGameChildUxLocalOutboxPreferencePreflightReadModel(
  options: PreferencePreflightOptions,
  sourceReadModel: AppGameChildUxLocalOutboxSchedulerBridgeReadModel
): AppGameChildUxLocalOutboxPreferencePreflightReadModel {
  const source = AppGameChildUxLocalOutboxSchedulerBridgeReadModelSchema.parse(sourceReadModel);
  const rows = source.rows.map(preferencePreflightRow);

  return AppGameChildUxLocalOutboxPreferencePreflightReadModelSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    preferencePreflightId: options.preferencePreflightId,
    generatedAt: options.generatedAt,
    family: source.family,
    sourceSchedulerBridgeId: source.schedulerBridgeId,
    sourceContractRefs: [...options.sourceContractRefs],
    rows,
    parentPreferenceRequiredCount: countRows(
      rows,
      AppGameChildUxLocalOutboxPreferencePreflightStatus.ParentPreferenceRequired
    ),
    manualRequiredCount: countRows(rows, AppGameChildUxLocalOutboxPreferencePreflightStatus.ManualRequired),
    unavailableCount: countRows(rows, AppGameChildUxLocalOutboxPreferencePreflightStatus.Unavailable),
    preflightNonClaims: [...RequiredAppGameChildUxLocalOutboxPreferencePreflightNonClaims],
    parentPreferenceUiClaimed: false,
    parentFrequencyControlUiClaimed: false,
    parentNotificationUiClaimed: false,
    quietHoursTimerRuntimeClaimed: false,
    childDeliveryRuntimeClaimed: false,
    providerDeliveryRuntimeClaimed: false,
    providerReceiptIngestionClaimed: false,
    providerCredentialsClaimed: false,
    cloudRoutingClaimed: false,
    retryExecutionRuntimeClaimed: false,
    productionDurableOutboxStorageClaimed: false,
    adapterDispatchClaimed: false,
    platformEnforcementClaimed: false,
    rawPrivateSourceRowsIncluded: false,
  });
}

export function buildAppGameChildUxLocalOutboxPreferenceStatusHandoffReadModel(
  options: PreferenceStatusOptions,
  sourceReadModel: AppGameChildUxLocalOutboxPreferencePreflightReadModel
): AppGameChildUxLocalOutboxPreferenceStatusHandoffReadModel {
  const source = AppGameChildUxLocalOutboxPreferencePreflightReadModelSchema.parse(sourceReadModel);
  const rows = source.rows.map((row) => preferenceStatusHandoffRow(options.generatedAt, row));

  return AppGameChildUxLocalOutboxPreferenceStatusHandoffReadModelSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    handoffId: options.handoffId,
    generatedAt: options.generatedAt,
    family: source.family,
    sourcePreferencePreflightId: source.preferencePreflightId,
    sourceContractRefs: [...options.sourceContractRefs],
    notificationRuleProviderRetryReadModelRef: V3NotificationRuleProviderRetryContractReadModel.readModelId,
    notificationRuleProviderRetryCoverageRefs: [...PreferenceStatusCoverageRefs],
    rows,
    parentPreferenceManualSetupRequiredCount: rows.filter(
      (row) => row.notificationPreferenceStatusEntry.parentPreferenceState === 'manual-setup-required'
    ).length,
    quietHoursManualRequiredCount: rows.filter(
      (row) => row.notificationPreferenceStatusEntry.quietHoursDecision === 'manual-required'
    ).length,
    preferenceStatusUnavailableCount: rows.filter(
      (row) => row.sourcePreferenceStatus === AppGameChildUxLocalOutboxPreferencePreflightStatus.Unavailable
    ).length,
    handoffNonClaims: [...RequiredAppGameChildUxLocalOutboxPreferenceStatusHandoffNonClaims],
    parentPreferenceUiClaimed: false,
    parentFrequencyControlUiClaimed: false,
    parentNotificationUiClaimed: false,
    parentPreferenceMutationClaimed: false,
    quietHoursTimerRuntimeClaimed: false,
    providerDeliveryRuntimeClaimed: false,
    providerReceiptIngestionClaimed: false,
    providerCredentialsClaimed: false,
    cloudRoutingClaimed: false,
    childDeliveryClaimed: false,
    retryExecutionRuntimeClaimed: false,
    productionDurableOutboxStorageClaimed: false,
    adapterDispatchClaimed: false,
    platformEnforcementClaimed: false,
    rawPrivateSourceRowsIncluded: false,
  });
}

export function buildAppGameChildUxLocalOutboxParentSurfaceIntentReadModel(
  options: ParentSurfaceOptions,
  providerStatusReadModel: AppGameChildUxLocalOutboxProviderStatusHandoffReadModel,
  preferenceStatusReadModel: AppGameChildUxLocalOutboxPreferenceStatusHandoffReadModel
): AppGameChildUxLocalOutboxParentSurfaceIntentReadModel {
  const provider = AppGameChildUxLocalOutboxProviderStatusHandoffReadModelSchema.parse(providerStatusReadModel);
  const preference = AppGameChildUxLocalOutboxPreferenceStatusHandoffReadModelSchema.parse(preferenceStatusReadModel);

  if (provider.rows.length !== preference.rows.length) {
    throw new Error('Expected child UX parent-surface inputs to have matching row counts');
  }

  const rows = provider.rows.map((providerRow, index) => parentSurfaceIntentRow(providerRow, preference.rows[index]));

  return AppGameChildUxLocalOutboxParentSurfaceIntentReadModelSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    intentId: options.intentId,
    generatedAt: options.generatedAt,
    family: provider.family,
    sourceProviderStatusHandoffId: provider.handoffId,
    sourcePreferenceStatusHandoffId: preference.handoffId,
    sourceContractRefs: [...options.sourceContractRefs],
    rows,
    manualActionRequiredCount: rows.filter((row) => row.parentSurfaceStatus === 'manual-action-required').length,
    unavailableVisibleCount: rows.filter((row) => row.parentSurfaceStatus === 'unavailable-visible').length,
    historyVisibleCount: rows.length,
    preferenceSetupRequiredCount: rows.filter((row) => row.preferenceVisibility === 'preference-setup-required').length,
    parentSurfaceNonClaims: [...RequiredAppGameChildUxLocalOutboxParentSurfaceIntentNonClaims],
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

function preferenceStatusHandoffRow(
  generatedAt: string,
  row: AppGameChildUxLocalOutboxPreferencePreflightReadModel['rows'][number]
) {
  const baseId = row.preferenceRowId;

  return {
    handoffRowId: `app-game-child-ux-preference-status-handoff-${baseId}`,
    sourcePreferenceRowId: row.preferenceRowId,
    sourcePreferenceStatus: row.status,
    sourceSchedulerEntryRef: row.sourceSchedulerEntryRef,
    sourceOutboxRecordRef: row.sourceOutboxRecordRef,
    sourceProviderChannelRef: row.providerChannelRef,
    sourceReasonCodeRef: row.reasonCodeRef,
    sourceParentPreferenceState: row.parentPreferenceState,
    sourceQuietHoursDecision: row.quietHoursDecision,
    sourceParentPreferenceRequirementRefs: [...row.parentPreferenceRequirementRefs],
    sourceQuietHoursRequirementRefs: [...row.quietHoursRequirementRefs],
    notificationPreferenceStatusEntry: preferenceStatusEntry(generatedAt, row),
    manualProofRequirements: [...row.manualProofRequirements],
  };
}

function providerPreflightRow(row: SchedulerBridgeRow) {
  const baseId = row.schedulerBridgeRecordId;
  const mode = schedulerPreflightMode(row.status);

  return {
    preflightRowId: `app-game-child-ux-provider-preflight-${baseId}`,
    sourceSchedulerBridgeRecordId: baseId,
    status: providerPreflightStatusForMode(mode),
    sourceSchedulerEntryRef: row.schedulerRecord?.schedulerEntryId ?? null,
    sourceOutboxRecordRef: row.sourceOutboxRecordRef,
    schedulerDecisionRef: row.schedulerRecord?.schedulerDecisionRef ?? null,
    providerChannelRef: schedulerProviderChannel(mode),
    reasonCodeRef: schedulerReasonCodeRef(mode, `app-game-child-ux-provider-reason-${baseId}`),
    adapterRequirementRefs: providerAdapterRequirementRefs(baseId, mode),
    manualProofRequirements: providerManualProofRefs(baseId, mode),
  };
}

function preferencePreflightRow(row: SchedulerBridgeRow) {
  const baseId = row.schedulerBridgeRecordId;
  const mode = schedulerPreflightMode(row.status);

  return {
    preferenceRowId: `app-game-child-ux-preference-preflight-${baseId}`,
    sourceSchedulerBridgeRecordId: baseId,
    status: preferencePreflightStatusForMode(mode),
    sourceSchedulerEntryRef: row.schedulerRecord?.schedulerEntryId ?? null,
    sourceOutboxRecordRef: row.sourceOutboxRecordRef,
    schedulerDecisionRef: row.schedulerRecord?.schedulerDecisionRef ?? null,
    providerChannelRef: schedulerProviderChannel(mode),
    reasonCodeRef: schedulerReasonCodeRef(mode, `app-game-child-ux-preference-reason-${baseId}`),
    parentPreferenceState: mode === 'scheduled' ? 'manual-setup-required' : null,
    quietHoursDecision: mode === 'scheduled' ? 'manual-required' : null,
    parentPreferenceRequirementRefs: parentPreferenceRequirementRefs(baseId, mode),
    quietHoursRequirementRefs: quietHoursRequirementRefs(baseId, mode),
    manualProofRequirements: preferenceManualProofRefs(baseId, mode),
  };
}

function schedulerPreflightMode(status: AppGameChildUxLocalOutboxSchedulerBridgeStatus): SchedulerPreflightMode {
  switch (status) {
    case AppGameChildUxLocalOutboxSchedulerBridgeStatus.ScheduledLocal:
      return 'scheduled';
    case AppGameChildUxLocalOutboxSchedulerBridgeStatus.Unavailable:
      return 'unavailable';
    default:
      return 'manual';
  }
}

function providerPreflightStatusForMode(mode: SchedulerPreflightMode) {
  switch (mode) {
    case 'scheduled':
      return AppGameChildUxLocalOutboxProviderPreflightStatus.ProviderAdapterRequired;
    case 'unavailable':
      return AppGameChildUxLocalOutboxProviderPreflightStatus.Unavailable;
    default:
      return AppGameChildUxLocalOutboxProviderPreflightStatus.ManualRequired;
  }
}

function preferencePreflightStatusForMode(mode: SchedulerPreflightMode) {
  switch (mode) {
    case 'scheduled':
      return AppGameChildUxLocalOutboxPreferencePreflightStatus.ParentPreferenceRequired;
    case 'unavailable':
      return AppGameChildUxLocalOutboxPreferencePreflightStatus.Unavailable;
    default:
      return AppGameChildUxLocalOutboxPreferencePreflightStatus.ManualRequired;
  }
}

function schedulerProviderChannel(mode: SchedulerPreflightMode) {
  return mode === 'scheduled' ? 'in-app' : null;
}

function schedulerReasonCodeRef(mode: SchedulerPreflightMode, reasonCodeRef: string) {
  return mode === 'scheduled' ? reasonCodeRef : null;
}

function providerAdapterRequirementRefs(baseId: string, mode: SchedulerPreflightMode) {
  if (mode === 'scheduled') {
    return requirementRefs(baseId, 'provider-adapter-required', 3);
  }

  return requirementRefs(baseId, mode === 'unavailable' ? 'provider-unavailable' : 'provider-manual-required', 1);
}

function providerManualProofRefs(baseId: string, mode: SchedulerPreflightMode) {
  if (mode === 'scheduled') {
    return requirementRefs(baseId, 'provider-manual-proof', 3);
  }

  return requirementRefs(baseId, mode === 'unavailable' ? 'provider-unavailable-proof' : 'provider-manual-proof', 1);
}

function parentPreferenceRequirementRefs(baseId: string, mode: SchedulerPreflightMode) {
  if (mode === 'scheduled') {
    return requirementRefs(baseId, 'parent-preference-required', 2);
  }

  return requirementRefs(baseId, mode === 'unavailable' ? 'parent-preference-disabled' : 'parent-preference-manual', 1);
}

function quietHoursRequirementRefs(baseId: string, mode: SchedulerPreflightMode) {
  if (mode === 'scheduled') {
    return requirementRefs(baseId, 'quiet-hours-required', 1);
  }

  return requirementRefs(baseId, mode === 'unavailable' ? 'quiet-hours-disabled' : 'quiet-hours-manual', 1);
}

function preferenceManualProofRefs(baseId: string, mode: SchedulerPreflightMode) {
  if (mode === 'scheduled') {
    return requirementRefs(baseId, 'preference-manual-proof', 3);
  }

  return requirementRefs(
    baseId,
    mode === 'unavailable' ? 'preference-unavailable-proof' : 'preference-manual-proof',
    1
  );
}

function preferenceStatusEntry(
  generatedAt: string,
  row: AppGameChildUxLocalOutboxPreferencePreflightReadModel['rows'][number]
) {
  const unavailable = row.status === AppGameChildUxLocalOutboxPreferencePreflightStatus.Unavailable;
  const baseId = row.preferenceRowId;

  return {
    schemaVersion: ParentContractSchemaVersion.V0_6,
    contractEntryId: `app-game-child-ux-preference-status-entry-${baseId}`,
    reasonCode: 'policy-violation' as const,
    providerChannel: 'in-app' as const,
    deliveryAttemptState: unavailable ? 'provider-disabled' : 'eligible',
    deliveryResultState: unavailable ? 'not-sent' : 'manual-required',
    retryPolicyState: unavailable ? 'provider-disabled' : 'manual-review',
    quietHoursDecision: unavailable ? 'allow' : 'manual-required',
    escalationDecision: unavailable ? 'none' : 'manual-review',
    parentPreferenceState: unavailable ? 'channel-disabled' : 'manual-setup-required',
    notificationRuleRef: `app-game-child-ux-notification-rule-${baseId}`,
    notificationIntentRef: `app-game-child-ux-notification-intent-${baseId}`,
    deliveryAttemptRef: `app-game-child-ux-delivery-attempt-${baseId}`,
    deliveryResultRef: `app-game-child-ux-delivery-result-${baseId}`,
    retryPolicyRef: `app-game-child-ux-retry-policy-${baseId}`,
    quietHoursPolicyRef: `app-game-child-ux-quiet-hours-policy-${baseId}`,
    escalationPolicyRef: `app-game-child-ux-escalation-policy-${baseId}`,
    parentPreferenceRef: `app-game-child-ux-parent-preference-${baseId}`,
    auditRefs: auditRefs(baseId),
    evidenceRefs: evidenceRefs(baseId),
    providerReceiptRefs: [],
    manualProofRequirements: [...row.manualProofRequirements],
    minimalProviderPayloadBoundary:
      'Child UX preference status is parent-safe contract state only; no provider send, no receipt observation, and no raw evidence payload exposure.',
    providerAdapterImplemented: false,
    deliveryAttemptExecuted: false,
    providerReceiptObserved: false,
    rawEvidenceInProviderPayload: false,
    providerStoresChildEvidenceClaimed: false,
    lastCheckedAt: generatedAt,
  };
}

function parentSurfaceIntentRow(
  providerRow: AppGameChildUxLocalOutboxProviderStatusHandoffReadModel['rows'][number],
  preferenceRow: AppGameChildUxLocalOutboxPreferenceStatusHandoffReadModel['rows'][number]
) {
  const baseId = providerRow.handoffRowId;
  const providerStatus = providerRow.providerStatusBoundaryEntry.providerStatus;
  const parentPreferenceState = preferenceRow.notificationPreferenceStatusEntry.parentPreferenceState;
  const quietHoursDecision = preferenceRow.notificationPreferenceStatusEntry.quietHoursDecision;
  const deliveryResultState = preferenceRow.notificationPreferenceStatusEntry.deliveryResultState;

  return {
    surfaceRowId: `app-game-child-ux-parent-surface-${baseId}`,
    sourceProviderHandoffRowId: providerRow.handoffRowId,
    sourcePreferenceHandoffRowId: preferenceRow.handoffRowId,
    sourceSchedulerEntryRef: providerRow.sourceSchedulerEntryRef,
    sourceOutboxRecordRef: providerRow.sourceOutboxRecordRef,
    providerStatus,
    deliveryResultState,
    parentPreferenceState,
    quietHoursDecision,
    providerChannel: 'in-app' as V3NotificationProviderChannel,
    parentSurfaceStatus: providerStatus === 'unavailable' ? 'unavailable-visible' : 'manual-action-required',
    historyVisibility: historyVisibilityFor(providerStatus),
    preferenceVisibility:
      parentPreferenceState === 'channel-disabled' ? 'preference-disabled-visible' : 'preference-setup-required',
    drillInRefs: drillInRefs(baseId),
    auditRefs: auditRefs(baseId),
    manualProofRequirements: [...providerRow.manualProofRequirements, ...preferenceRow.manualProofRequirements],
    minimalSurfacePayloadBoundary:
      'Parent surface shows only redacted child UX alert status, parent-safe preference state, and contract refs without delivery or raw child evidence claims.',
    childUxLocalOutboxSurfaceClaim: `app-game-child-ux-parent-surface-claim-${baseId}`,
    sensitiveDetailIncluded: false,
    providerDeliveryClaimed: false,
    providerReceiptClaimed: false,
    parentPreferenceMutationClaimed: false,
    childDeliveryClaimed: false,
    adapterDispatchClaimed: false,
    platformEnforcementClaimed: false,
  };
}

export function buildAppGameChildUxParentPreferenceSetupDraftReadModel(
  options: ParentPreferenceSetupDraftOptions,
  sourceReadModel: AppGameChildUxLocalOutboxParentSurfaceIntentReadModel
): AppGameChildUxParentPreferenceSetupDraftReadModel {
  const source = AppGameChildUxLocalOutboxParentSurfaceIntentReadModelSchema.parse(sourceReadModel);
  const rows = source.rows.map((row) => {
    const draftReady = row.preferenceVisibility === 'preference-setup-required';
    const baseId = row.surfaceRowId;

    return {
      draftRowId: `app-game-child-ux-parent-preference-draft-${baseId}`,
      sourceParentSurfaceRowId: row.surfaceRowId,
      sourceSchedulerEntryRef: row.sourceSchedulerEntryRef,
      sourceOutboxRecordRef: row.sourceOutboxRecordRef,
      providerChannel: row.providerChannel,
      parentPreferenceState: row.parentPreferenceState,
      quietHoursDecision: row.quietHoursDecision,
      draftStatus: draftReady
        ? AppGameChildUxParentPreferenceSetupDraftStatus.DraftReady
        : AppGameChildUxParentPreferenceSetupDraftStatus.UnavailableVisible,
      preferenceRequirementRefs: draftReady ? requirementRefs(baseId, 'parent-preference-draft', 2) : [],
      quietHoursRequirementRefs: draftReady ? requirementRefs(baseId, 'parent-preference-quiet-hours', 1) : [],
      manualProofRequirements: [...row.manualProofRequirements],
      parentSafeDrillInRefs: [...row.drillInRefs],
      parentPreferenceUiRendered: false,
      parentFrequencyControlUiRendered: false,
      parentPreferenceMutationClaimed: false,
      notificationRuleMutationClaimed: false,
      providerDeliveryClaimed: false,
      childDeliveryClaimed: false,
      adapterDispatchClaimed: false,
      platformEnforcementClaimed: false,
      rawPrivateSourceRowsIncluded: false,
    };
  });

  return AppGameChildUxParentPreferenceSetupDraftReadModelSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    draftId: options.draftId,
    generatedAt: options.generatedAt,
    family: source.family,
    sourceParentSurfaceIntentId: source.intentId,
    sourceContractRefs: [...options.sourceContractRefs],
    rows,
    draftReadyCount: rows.filter((row) => row.draftStatus === AppGameChildUxParentPreferenceSetupDraftStatus.DraftReady)
      .length,
    unavailableVisibleCount: rows.filter(
      (row) => row.draftStatus === AppGameChildUxParentPreferenceSetupDraftStatus.UnavailableVisible
    ).length,
    draftNonClaims: [...RequiredAppGameChildUxParentPreferenceSetupDraftNonClaims],
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

function countRows<TStatus>(rows: ReadonlyArray<{ readonly status: TStatus }>, status: TStatus): number {
  return rows.filter((row) => row.status === status).length;
}

function requirementRefs(baseId: string, kind: string, count: number): string[] {
  return Array.from({ length: count }, (_, index) => `app-game-child-ux-${kind}-${baseId}-${index + 1}`);
}

function auditRefs(baseId: string): string[] {
  return [`app-game-child-ux-audit-${baseId}-1`, `app-game-child-ux-audit-${baseId}-2`];
}

function drillInRefs(baseId: string): string[] {
  return [`app-game-child-ux-drill-in-${baseId}-1`, `app-game-child-ux-drill-in-${baseId}-2`];
}

function evidenceRefs(baseId: string): string[] {
  return [`app-game-child-ux-evidence-${baseId}`];
}

function preferenceRefs(baseId: string): string[] {
  return [`app-game-child-ux-preference-${baseId}`];
}

function readinessRefs(baseId: string): string[] {
  return [`app-game-child-ux-readiness-${baseId}`];
}
