import { ParentContractSchemaVersion } from '@ocentra-parent/schema-domain/family-reference-primitives';
import {
  AppGameChildUxLocalOutboxProviderStatusHandoffReadModelSchema,
  AppGameChildUxLocalOutboxProviderStatusHandoffRowSchema,
  RequiredAppGameChildUxLocalOutboxProviderStatusHandoffNonClaims,
  type AppGameChildUxLocalOutboxProviderStatusHandoffReadModel,
  type AppGameChildUxLocalOutboxProviderStatusHandoffRow,
} from '@ocentra-parent/schema-domain/app-game-child-facing-ux-local-outbox-provider-status-handoff';
import {
  V08NotificationProviderStatusBoundaryEntrySchema,
  V08NotificationProviderStatusBoundaryReadModel,
  type V08NotificationProviderStatus,
} from '@ocentra-parent/schema-domain/v0-8-notification-provider-status-boundary';
import {
  AppGameChildUxLocalOutboxProviderPreflightReadModelSchema,
  AppGameChildUxLocalOutboxProviderPreflightStatus,
} from '@ocentra-parent/schema-domain/app-game-child-facing-ux-local-outbox-provider-preflight';
import type {
  AppGameChildUxLocalOutboxProviderPreflightReadModel,
  AppGameChildUxLocalOutboxProviderPreflightRow,
} from '@ocentra-parent/schema-domain/app-game-child-facing-ux-local-outbox-provider-preflight';

export type AppGameChildUxLocalOutboxProviderStatusHandoffOptions = {
  readonly generatedAt: string;
  readonly handoffId: string;
  readonly sourceContractRefs: readonly string[];
};

export function buildAppGameChildUxLocalOutboxProviderStatusHandoffReadModel(
  options: AppGameChildUxLocalOutboxProviderStatusHandoffOptions,
  sourceReadModel: AppGameChildUxLocalOutboxProviderPreflightReadModel
): AppGameChildUxLocalOutboxProviderStatusHandoffReadModel {
  const parsedSource = AppGameChildUxLocalOutboxProviderPreflightReadModelSchema.parse(sourceReadModel);
  const rows = parsedSource.rows.map((row) => providerStatusHandoffRowForPreflightRow(options, row));

  return AppGameChildUxLocalOutboxProviderStatusHandoffReadModelSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    handoffId: options.handoffId,
    generatedAt: options.generatedAt,
    family: parsedSource.family,
    sourceProviderPreflightId: parsedSource.providerPreflightId,
    sourceContractRefs: options.sourceContractRefs,
    providerStatusBoundaryReadModelRef: V08NotificationProviderStatusBoundaryReadModel.readModelId,
    providerStatusBoundaryCoverageRefs: V08NotificationProviderStatusBoundaryReadModel.entries.map(
      (entry) => entry.statusEntryId
    ),
    rows,
    providerStatusManualRequiredCount: countProviderStatus(rows, 'manual-required'),
    providerStatusUnavailableCount: countProviderStatus(rows, 'unavailable'),
    handoffNonClaims: RequiredAppGameChildUxLocalOutboxProviderStatusHandoffNonClaims,
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

function providerStatusHandoffRowForPreflightRow(
  options: AppGameChildUxLocalOutboxProviderStatusHandoffOptions,
  row: AppGameChildUxLocalOutboxProviderPreflightRow
): AppGameChildUxLocalOutboxProviderStatusHandoffRow {
  return AppGameChildUxLocalOutboxProviderStatusHandoffRowSchema.parse({
    handoffRowId: `app-game-child-ux-provider-status-handoff-${row.preflightRowId}`,
    sourcePreflightRowId: row.preflightRowId,
    sourcePreflightStatus: row.status,
    sourceSchedulerEntryRef: row.sourceSchedulerEntryRef,
    sourceOutboxRecordRef: row.sourceOutboxRecordRef,
    sourceProviderChannelRef: row.providerChannelRef,
    providerStatusBoundaryEntry: providerStatusBoundaryEntryForPreflightRow(options, row),
    manualProofRequirements: row.manualProofRequirements,
  });
}

function providerStatusBoundaryEntryForPreflightRow(
  options: AppGameChildUxLocalOutboxProviderStatusHandoffOptions,
  row: AppGameChildUxLocalOutboxProviderPreflightRow
) {
  const unavailable = row.status === AppGameChildUxLocalOutboxProviderPreflightStatus.Unavailable;

  return V08NotificationProviderStatusBoundaryEntrySchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    statusEntryId: `app-game-child-ux-provider-status-${row.preflightRowId}`,
    providerStatus: unavailable ? 'unavailable' : 'manual-required',
    statusProofState: unavailable ? 'provider-unavailable-contract' : 'manual-action-required',
    quietHoursReadiness: unavailable ? 'unavailable' : 'manual-required',
    escalationReadiness: unavailable ? 'unavailable' : 'manual-required',
    deliveryClaimState: unavailable ? 'not-implemented' : 'not-observed',
    notificationIntentRef: `app-game-child-ux-provider-status-intent-${row.sourceSchedulerBridgeRecordId}`,
    notificationStatusRef: `app-game-child-ux-provider-status-ref-${row.preflightRowId}`,
    providerAttemptRef: `app-game-child-ux-provider-attempt-not-started-${row.preflightRowId}`,
    auditRefs: [`app-game-child-ux-provider-status-audit-${row.preflightRowId}`],
    preferenceRefs: providerPreferenceRefsForRow(row),
    readinessRefs: providerReadinessRefsForRow(row),
    providerReceiptRefs: [],
    manualProofRequirements: row.manualProofRequirements,
    minimalPayloadBoundary: unavailable
      ? 'Provider unavailable keeps child UX local outbox delivery unclaimed and visible for manual review.'
      : 'Provider manual-required keeps child UX local outbox delivery blocked until adapter, credentials, preferences, and smoke proof exist.',
    providerDeliveryImplemented: false,
    providerDeliveryObserved: false,
    deliveredNotificationClaimed: false,
    sensitiveProviderPayloadClaimed: false,
    providerStoresChildEvidenceClaimed: false,
    lastCheckedAt: options.generatedAt,
  });
}

function providerPreferenceRefsForRow(row: AppGameChildUxLocalOutboxProviderPreflightRow): readonly string[] {
  return row.providerChannelRef === null
    ? ['app-game-child-ux-provider-preference-manual-review']
    : [`app-game-child-ux-provider-preference-${row.providerChannelRef}`];
}

function providerReadinessRefsForRow(row: AppGameChildUxLocalOutboxProviderPreflightRow): readonly string[] {
  if (row.status === AppGameChildUxLocalOutboxProviderPreflightStatus.Unavailable) {
    return ['app-game-child-ux-provider-readiness-unavailable'];
  }

  return row.adapterRequirementRefs.length === 0
    ? ['app-game-child-ux-provider-readiness-manual-required']
    : row.adapterRequirementRefs;
}

const countProviderStatus = (
  rows: ReadonlyArray<{
    readonly providerStatusBoundaryEntry: { readonly providerStatus: V08NotificationProviderStatus };
  }>,
  providerStatus: V08NotificationProviderStatus
): number => rows.filter((row) => row.providerStatusBoundaryEntry.providerStatus === providerStatus).length;
