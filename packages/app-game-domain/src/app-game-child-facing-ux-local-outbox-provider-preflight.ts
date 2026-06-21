import {
  AppGameChildUxLocalOutboxProviderPreflightReadModelSchema,
  AppGameChildUxLocalOutboxProviderPreflightRowSchema,
  AppGameChildUxLocalOutboxProviderPreflightStatus,
  RequiredAppGameChildUxLocalOutboxProviderPreflightNonClaims,
} from '@ocentra-parent/schema-domain/app-game-child-facing-ux-local-outbox-provider-preflight';
import type {
  AppGameChildUxLocalOutboxProviderPreflightReadModel,
  AppGameChildUxLocalOutboxProviderPreflightRow,
} from '@ocentra-parent/schema-domain/app-game-child-facing-ux-local-outbox-provider-preflight';
import {
  AppGameChildUxLocalOutboxSchedulerBridgeReadModelSchema,
  AppGameChildUxLocalOutboxSchedulerBridgeStatus,
} from '@ocentra-parent/schema-domain/app-game-child-facing-ux-local-outbox-scheduler-bridge';
import type {
  AppGameChildUxLocalOutboxSchedulerBridgeReadModel,
  AppGameChildUxLocalOutboxSchedulerBridgeRow,
} from '@ocentra-parent/schema-domain/app-game-child-facing-ux-local-outbox-scheduler-bridge';
import { ParentContractSchemaVersion } from '@ocentra-parent/schema-domain/family-reference-primitives';

type AppGameChildUxLocalOutboxProviderPreflightStatusValue =
  import('@ocentra-parent/schema-domain/app-game-child-facing-ux-local-outbox-provider-preflight').AppGameChildUxLocalOutboxProviderPreflightStatus;

export type AppGameChildUxLocalOutboxProviderPreflightOptions = {
  readonly generatedAt: string;
  readonly providerPreflightId: string;
  readonly sourceContractRefs: readonly string[];
};

export function buildAppGameChildUxLocalOutboxProviderPreflightReadModel(
  options: AppGameChildUxLocalOutboxProviderPreflightOptions,
  sourceReadModel: AppGameChildUxLocalOutboxSchedulerBridgeReadModel
): AppGameChildUxLocalOutboxProviderPreflightReadModel {
  const parsedSource = AppGameChildUxLocalOutboxSchedulerBridgeReadModelSchema.parse(sourceReadModel);
  const rows = parsedSource.rows.map(providerPreflightRowForChildUxSchedulerRow);

  return AppGameChildUxLocalOutboxProviderPreflightReadModelSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    providerPreflightId: options.providerPreflightId,
    generatedAt: options.generatedAt,
    family: parsedSource.family,
    sourceSchedulerBridgeId: parsedSource.schedulerBridgeId,
    sourceContractRefs: options.sourceContractRefs,
    rows,
    providerAdapterRequiredCount: countRows(
      rows,
      AppGameChildUxLocalOutboxProviderPreflightStatus.ProviderAdapterRequired
    ),
    manualRequiredCount: countRows(rows, AppGameChildUxLocalOutboxProviderPreflightStatus.ManualRequired),
    unavailableCount: countRows(rows, AppGameChildUxLocalOutboxProviderPreflightStatus.Unavailable),
    preflightNonClaims: RequiredAppGameChildUxLocalOutboxProviderPreflightNonClaims,
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

function providerPreflightRowForChildUxSchedulerRow(
  row: AppGameChildUxLocalOutboxSchedulerBridgeRow
): AppGameChildUxLocalOutboxProviderPreflightRow {
  if (row.status === AppGameChildUxLocalOutboxSchedulerBridgeStatus.ScheduledLocal && row.schedulerRecord !== null) {
    return scheduledProviderPreflightRow(row);
  }
  return blockedProviderPreflightRow(row);
}

function scheduledProviderPreflightRow(
  row: AppGameChildUxLocalOutboxSchedulerBridgeRow
): AppGameChildUxLocalOutboxProviderPreflightRow {
  const record = row.schedulerRecord;
  if (record === null) {
    throw new Error(`Missing scheduler record for child UX provider preflight: ${row.schedulerBridgeRecordId}`);
  }

  const requirementRefs = [
    `child-ux-provider-adapter-required-${record.schedulerEntryId}`,
    `child-ux-provider-credentials-required-${record.schedulerEntryId}`,
    `child-ux-provider-smoke-proof-required-${record.schedulerEntryId}`,
  ];

  return AppGameChildUxLocalOutboxProviderPreflightRowSchema.parse({
    preflightRowId: `app-game-child-ux-local-outbox-provider-preflight-${row.schedulerBridgeRecordId}`,
    sourceSchedulerBridgeRecordId: row.schedulerBridgeRecordId,
    status: AppGameChildUxLocalOutboxProviderPreflightStatus.ProviderAdapterRequired,
    sourceSchedulerEntryRef: record.schedulerEntryId,
    sourceOutboxRecordRef: row.sourceOutboxRecordRef,
    schedulerDecisionRef: record.schedulerDecisionRef,
    providerChannelRef: record.providerChannel,
    reasonCodeRef: record.reasonCode,
    adapterRequirementRefs: requirementRefs,
    manualProofRequirements: requirementRefs,
  });
}

function blockedProviderPreflightRow(
  row: AppGameChildUxLocalOutboxSchedulerBridgeRow
): AppGameChildUxLocalOutboxProviderPreflightRow {
  return AppGameChildUxLocalOutboxProviderPreflightRowSchema.parse({
    preflightRowId: `app-game-child-ux-local-outbox-provider-preflight-${row.schedulerBridgeRecordId}`,
    sourceSchedulerBridgeRecordId: row.schedulerBridgeRecordId,
    status:
      row.status === AppGameChildUxLocalOutboxSchedulerBridgeStatus.Unavailable
        ? AppGameChildUxLocalOutboxProviderPreflightStatus.Unavailable
        : AppGameChildUxLocalOutboxProviderPreflightStatus.ManualRequired,
    sourceSchedulerEntryRef: null,
    sourceOutboxRecordRef: null,
    schedulerDecisionRef: null,
    providerChannelRef: null,
    reasonCodeRef: null,
    adapterRequirementRefs: row.blockedReasonRefs,
    manualProofRequirements: row.blockedReasonRefs,
  });
}

function countRows(
  rows: ReadonlyArray<{ readonly status: AppGameChildUxLocalOutboxProviderPreflightStatusValue }>,
  status: AppGameChildUxLocalOutboxProviderPreflightStatusValue
): number {
  return rows.filter((row) => row.status === status).length;
}
