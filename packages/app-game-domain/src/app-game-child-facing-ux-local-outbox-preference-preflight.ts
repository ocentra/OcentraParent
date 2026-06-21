import {
  AppGameChildUxLocalOutboxPreferencePreflightReadModelSchema,
  AppGameChildUxLocalOutboxPreferencePreflightRowSchema,
  AppGameChildUxLocalOutboxPreferencePreflightStatus,
  RequiredAppGameChildUxLocalOutboxPreferencePreflightNonClaims,
} from '@ocentra-parent/schema-domain/app-game-child-facing-ux-local-outbox-preference-preflight';
import type {
  AppGameChildUxLocalOutboxPreferencePreflightReadModel,
  AppGameChildUxLocalOutboxPreferencePreflightRow,
} from '@ocentra-parent/schema-domain/app-game-child-facing-ux-local-outbox-preference-preflight';
import {
  AppGameChildUxLocalOutboxSchedulerBridgeReadModelSchema,
  AppGameChildUxLocalOutboxSchedulerBridgeStatus,
} from '@ocentra-parent/schema-domain/app-game-child-facing-ux-local-outbox-scheduler-bridge';
import type {
  AppGameChildUxLocalOutboxSchedulerBridgeReadModel,
  AppGameChildUxLocalOutboxSchedulerBridgeRow,
} from '@ocentra-parent/schema-domain/app-game-child-facing-ux-local-outbox-scheduler-bridge';
import { ParentContractSchemaVersion } from '@ocentra-parent/schema-domain/family-reference-primitives';

type AppGameChildUxLocalOutboxPreferencePreflightStatusValue =
  import('@ocentra-parent/schema-domain/app-game-child-facing-ux-local-outbox-preference-preflight').AppGameChildUxLocalOutboxPreferencePreflightStatus;

export type AppGameChildUxLocalOutboxPreferencePreflightOptions = {
  readonly generatedAt: string;
  readonly preferencePreflightId: string;
  readonly sourceContractRefs: readonly string[];
};

export function buildAppGameChildUxLocalOutboxPreferencePreflightReadModel(
  options: AppGameChildUxLocalOutboxPreferencePreflightOptions,
  sourceReadModel: AppGameChildUxLocalOutboxSchedulerBridgeReadModel
): AppGameChildUxLocalOutboxPreferencePreflightReadModel {
  const parsedSource = AppGameChildUxLocalOutboxSchedulerBridgeReadModelSchema.parse(sourceReadModel);
  const rows = parsedSource.rows.map(preferencePreflightRowForChildUxSchedulerRow);

  return AppGameChildUxLocalOutboxPreferencePreflightReadModelSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    preferencePreflightId: options.preferencePreflightId,
    generatedAt: options.generatedAt,
    family: parsedSource.family,
    sourceSchedulerBridgeId: parsedSource.schedulerBridgeId,
    sourceContractRefs: options.sourceContractRefs,
    rows,
    parentPreferenceRequiredCount: countRows(
      rows,
      AppGameChildUxLocalOutboxPreferencePreflightStatus.ParentPreferenceRequired
    ),
    manualRequiredCount: countRows(rows, AppGameChildUxLocalOutboxPreferencePreflightStatus.ManualRequired),
    unavailableCount: countRows(rows, AppGameChildUxLocalOutboxPreferencePreflightStatus.Unavailable),
    preflightNonClaims: RequiredAppGameChildUxLocalOutboxPreferencePreflightNonClaims,
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

function preferencePreflightRowForChildUxSchedulerRow(
  row: AppGameChildUxLocalOutboxSchedulerBridgeRow
): AppGameChildUxLocalOutboxPreferencePreflightRow {
  if (row.status === AppGameChildUxLocalOutboxSchedulerBridgeStatus.ScheduledLocal && row.schedulerRecord !== null) {
    return scheduledPreferencePreflightRow(row);
  }
  return blockedPreferencePreflightRow(row);
}

function scheduledPreferencePreflightRow(
  row: AppGameChildUxLocalOutboxSchedulerBridgeRow
): AppGameChildUxLocalOutboxPreferencePreflightRow {
  const record = row.schedulerRecord;
  if (record === null) {
    throw new Error(`Missing scheduler record for child UX preference preflight: ${row.schedulerBridgeRecordId}`);
  }

  const parentPreferenceRefs = [
    `child-ux-parent-preference-required-${record.providerChannel}-${record.schedulerEntryId}`,
    `child-ux-notification-frequency-control-required-${record.schedulerEntryId}`,
  ];
  const quietHoursRefs = [`child-ux-quiet-hours-policy-required-${record.schedulerEntryId}`];

  return AppGameChildUxLocalOutboxPreferencePreflightRowSchema.parse({
    preferenceRowId: `app-game-child-ux-local-outbox-preference-preflight-${row.schedulerBridgeRecordId}`,
    sourceSchedulerBridgeRecordId: row.schedulerBridgeRecordId,
    status: AppGameChildUxLocalOutboxPreferencePreflightStatus.ParentPreferenceRequired,
    sourceSchedulerEntryRef: record.schedulerEntryId,
    sourceOutboxRecordRef: row.sourceOutboxRecordRef,
    schedulerDecisionRef: record.schedulerDecisionRef,
    providerChannelRef: record.providerChannel,
    reasonCodeRef: record.reasonCode,
    parentPreferenceState: 'manual-setup-required',
    quietHoursDecision: 'manual-required',
    parentPreferenceRequirementRefs: parentPreferenceRefs,
    quietHoursRequirementRefs: quietHoursRefs,
    manualProofRequirements: [...parentPreferenceRefs, ...quietHoursRefs],
  });
}

function blockedPreferencePreflightRow(
  row: AppGameChildUxLocalOutboxSchedulerBridgeRow
): AppGameChildUxLocalOutboxPreferencePreflightRow {
  return AppGameChildUxLocalOutboxPreferencePreflightRowSchema.parse({
    preferenceRowId: `app-game-child-ux-local-outbox-preference-preflight-${row.schedulerBridgeRecordId}`,
    sourceSchedulerBridgeRecordId: row.schedulerBridgeRecordId,
    status:
      row.status === AppGameChildUxLocalOutboxSchedulerBridgeStatus.Unavailable
        ? AppGameChildUxLocalOutboxPreferencePreflightStatus.Unavailable
        : AppGameChildUxLocalOutboxPreferencePreflightStatus.ManualRequired,
    sourceSchedulerEntryRef: null,
    sourceOutboxRecordRef: null,
    schedulerDecisionRef: null,
    providerChannelRef: null,
    reasonCodeRef: null,
    parentPreferenceState: null,
    quietHoursDecision: null,
    parentPreferenceRequirementRefs: row.blockedReasonRefs,
    quietHoursRequirementRefs: row.blockedReasonRefs,
    manualProofRequirements: row.blockedReasonRefs,
  });
}

function countRows(
  rows: ReadonlyArray<{ readonly status: AppGameChildUxLocalOutboxPreferencePreflightStatusValue }>,
  status: AppGameChildUxLocalOutboxPreferencePreflightStatusValue
): number {
  return rows.filter((row) => row.status === status).length;
}
