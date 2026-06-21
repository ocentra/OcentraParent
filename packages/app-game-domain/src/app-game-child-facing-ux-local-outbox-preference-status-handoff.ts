import { ParentContractSchemaVersion } from '@ocentra-parent/schema-domain/family-reference-primitives';
import {
  AppGameChildUxLocalOutboxPreferenceStatusHandoffReadModelSchema,
  AppGameChildUxLocalOutboxPreferenceStatusHandoffRowSchema,
  RequiredAppGameChildUxLocalOutboxPreferenceStatusHandoffNonClaims,
  type AppGameChildUxLocalOutboxPreferenceStatusHandoffReadModel,
  type AppGameChildUxLocalOutboxPreferenceStatusHandoffRow,
} from '@ocentra-parent/schema-domain/app-game-child-facing-ux-local-outbox-preference-status-handoff';
import {
  V3NotificationProviderChannelSchema,
  V3NotificationRuleProviderRetryContractEntrySchema,
  V3NotificationRuleProviderRetryContractReadModel,
  V3NotificationRuleReasonCodeSchema,
  type V3NotificationParentPreferenceState,
  type V3NotificationProviderChannel,
  type V3NotificationQuietHoursDecision,
  type V3NotificationRuleReasonCode,
} from '@ocentra-parent/schema-domain/notification-v3-provider-retry';
import {
  AppGameChildUxLocalOutboxPreferencePreflightReadModelSchema,
  AppGameChildUxLocalOutboxPreferencePreflightStatus,
} from '@ocentra-parent/schema-domain/app-game-child-facing-ux-local-outbox-preference-preflight';
import type {
  AppGameChildUxLocalOutboxPreferencePreflightReadModel,
  AppGameChildUxLocalOutboxPreferencePreflightRow,
} from '@ocentra-parent/schema-domain/app-game-child-facing-ux-local-outbox-preference-preflight';

type AppGameChildUxLocalOutboxPreferencePreflightStatusValue = AppGameChildUxLocalOutboxPreferencePreflightRow['status'];

type PreferenceStatusEntry = AppGameChildUxLocalOutboxPreferenceStatusHandoffRow['notificationPreferenceStatusEntry'];
type PreferenceStatusExpectation = Pick<
  PreferenceStatusEntry,
  | 'deliveryAttemptState'
  | 'deliveryResultState'
  | 'retryPolicyState'
  | 'quietHoursDecision'
  | 'escalationDecision'
  | 'parentPreferenceState'
>;

const ManualPreferenceStatusExpectation: PreferenceStatusExpectation = {
  deliveryAttemptState: 'eligible',
  deliveryResultState: 'manual-required',
  retryPolicyState: 'manual-review',
  quietHoursDecision: 'manual-required',
  escalationDecision: 'manual-review',
  parentPreferenceState: 'manual-setup-required',
};

const UnavailablePreferenceStatusExpectation: PreferenceStatusExpectation = {
  deliveryAttemptState: 'provider-disabled',
  deliveryResultState: 'not-sent',
  retryPolicyState: 'provider-disabled',
  quietHoursDecision: 'allow',
  escalationDecision: 'none',
  parentPreferenceState: 'channel-disabled',
};

export type AppGameChildUxLocalOutboxPreferenceStatusHandoffOptions = {
  readonly generatedAt: string;
  readonly handoffId: string;
  readonly sourceContractRefs: readonly string[];
};

export function buildAppGameChildUxLocalOutboxPreferenceStatusHandoffReadModel(
  options: AppGameChildUxLocalOutboxPreferenceStatusHandoffOptions,
  sourceReadModel: AppGameChildUxLocalOutboxPreferencePreflightReadModel
): AppGameChildUxLocalOutboxPreferenceStatusHandoffReadModel {
  const parsedSource = AppGameChildUxLocalOutboxPreferencePreflightReadModelSchema.parse(sourceReadModel);
  const rows = parsedSource.rows.map((row) => preferenceStatusHandoffRowForPreflightRow(options, row));

  return AppGameChildUxLocalOutboxPreferenceStatusHandoffReadModelSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    handoffId: options.handoffId,
    generatedAt: options.generatedAt,
    family: parsedSource.family,
    sourcePreferencePreflightId: parsedSource.preferencePreflightId,
    sourceContractRefs: options.sourceContractRefs,
    notificationRuleProviderRetryReadModelRef: V3NotificationRuleProviderRetryContractReadModel.readModelId,
    notificationRuleProviderRetryCoverageRefs: V3NotificationRuleProviderRetryContractReadModel.entries.map(
      (entry) => entry.contractEntryId
    ),
    rows,
    parentPreferenceManualSetupRequiredCount: countParentPreferenceState(rows, 'manual-setup-required'),
    quietHoursManualRequiredCount: countQuietHoursDecision(rows, 'manual-required'),
    preferenceStatusUnavailableCount: countSourceStatus(
      rows,
      AppGameChildUxLocalOutboxPreferencePreflightStatus.Unavailable
    ),
    handoffNonClaims: RequiredAppGameChildUxLocalOutboxPreferenceStatusHandoffNonClaims,
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

function preferenceStatusHandoffRowForPreflightRow(
  options: AppGameChildUxLocalOutboxPreferenceStatusHandoffOptions,
  row: AppGameChildUxLocalOutboxPreferencePreflightRow
): AppGameChildUxLocalOutboxPreferenceStatusHandoffRow {
  return AppGameChildUxLocalOutboxPreferenceStatusHandoffRowSchema.parse({
    handoffRowId: `app-game-child-ux-preference-status-handoff-${row.preferenceRowId}`,
    sourcePreferenceRowId: row.preferenceRowId,
    sourcePreferenceStatus: row.status,
    sourceSchedulerEntryRef: row.sourceSchedulerEntryRef,
    sourceOutboxRecordRef: row.sourceOutboxRecordRef,
    sourceProviderChannelRef: row.providerChannelRef,
    sourceReasonCodeRef: row.reasonCodeRef,
    sourceParentPreferenceState: row.parentPreferenceState,
    sourceQuietHoursDecision: row.quietHoursDecision,
    sourceParentPreferenceRequirementRefs: row.parentPreferenceRequirementRefs,
    sourceQuietHoursRequirementRefs: row.quietHoursRequirementRefs,
    notificationPreferenceStatusEntry: preferenceStatusEntryForPreflightRow(options, row),
    manualProofRequirements: row.manualProofRequirements,
  });
}

function preferenceStatusEntryForPreflightRow(
  options: AppGameChildUxLocalOutboxPreferenceStatusHandoffOptions,
  row: AppGameChildUxLocalOutboxPreferencePreflightRow
): PreferenceStatusEntry {
  const expectation = preferenceStatusExpectationFor(row.status);

  return V3NotificationRuleProviderRetryContractEntrySchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    contractEntryId: `app-game-child-ux-preference-status-${row.preferenceRowId}`,
    reasonCode: reasonCodeForRow(row),
    providerChannel: providerChannelForRow(row),
    deliveryAttemptState: expectation.deliveryAttemptState,
    deliveryResultState: expectation.deliveryResultState,
    retryPolicyState: expectation.retryPolicyState,
    quietHoursDecision: expectation.quietHoursDecision,
    escalationDecision: expectation.escalationDecision,
    parentPreferenceState: expectation.parentPreferenceState,
    notificationRuleRef: ruleRefForRow(row),
    notificationIntentRef: `app-game-child-ux-preference-status-intent-${row.sourceSchedulerBridgeRecordId}`,
    deliveryAttemptRef: `app-game-child-ux-preference-status-attempt-not-executed-${row.preferenceRowId}`,
    deliveryResultRef: `app-game-child-ux-preference-status-result-${row.preferenceRowId}`,
    retryPolicyRef: `app-game-child-ux-preference-status-retry-${row.preferenceRowId}`,
    quietHoursPolicyRef: policyRefOrFallback(row.quietHoursRequirementRefs, row.preferenceRowId, 'quiet-hours'),
    escalationPolicyRef: `app-game-child-ux-preference-status-escalation-${row.preferenceRowId}`,
    parentPreferenceRef: policyRefOrFallback(row.parentPreferenceRequirementRefs, row.preferenceRowId, 'preference'),
    auditRefs: [`app-game-child-ux-preference-status-audit-${row.preferenceRowId}`],
    evidenceRefs: evidenceRefsForRow(row),
    providerReceiptRefs: [],
    manualProofRequirements: row.manualProofRequirements,
    minimalProviderPayloadBoundary: minimalProviderPayloadBoundaryFor(row.status),
    providerAdapterImplemented: false,
    deliveryAttemptExecuted: false,
    providerReceiptObserved: false,
    rawEvidenceInProviderPayload: false,
    providerStoresChildEvidenceClaimed: false,
    lastCheckedAt: options.generatedAt,
  });
}

function preferenceStatusExpectationFor(
  status: AppGameChildUxLocalOutboxPreferencePreflightStatusValue
): PreferenceStatusExpectation {
  if (status === AppGameChildUxLocalOutboxPreferencePreflightStatus.Unavailable) {
    return UnavailablePreferenceStatusExpectation;
  }
  return ManualPreferenceStatusExpectation;
}

function reasonCodeForRow(row: AppGameChildUxLocalOutboxPreferencePreflightRow): V3NotificationRuleReasonCode {
  if (row.reasonCodeRef !== null) {
    return V3NotificationRuleReasonCodeSchema.parse(row.reasonCodeRef);
  }

  return V3NotificationRuleReasonCodeSchema.parse(
    row.status === AppGameChildUxLocalOutboxPreferencePreflightStatus.Unavailable
      ? 'provider-failure'
      : 'parent-request'
  );
}

function providerChannelForRow(row: AppGameChildUxLocalOutboxPreferencePreflightRow): V3NotificationProviderChannel {
  return V3NotificationProviderChannelSchema.parse(row.providerChannelRef ?? 'in-app');
}

function ruleRefForRow(row: AppGameChildUxLocalOutboxPreferencePreflightRow): string {
  return row.reasonCodeRef === null
    ? `app-game-child-ux-preference-status-rule-${row.preferenceRowId}`
    : `app-game-child-ux-preference-status-rule-${row.reasonCodeRef}`;
}

function policyRefOrFallback(refs: readonly string[], rowId: string, kind: string): string {
  return refs[0] ?? `app-game-child-ux-preference-status-${kind}-${rowId}`;
}

function evidenceRefsForRow(row: AppGameChildUxLocalOutboxPreferencePreflightRow): readonly string[] {
  const sourceRefs = [row.sourceSchedulerEntryRef, row.sourceOutboxRecordRef].flatMap((ref) =>
    ref === null ? [] : [ref]
  );
  return sourceRefs.length === 0 ? row.manualProofRequirements : sourceRefs;
}

function minimalProviderPayloadBoundaryFor(status: AppGameChildUxLocalOutboxPreferencePreflightStatusValue): string {
  return status === AppGameChildUxLocalOutboxPreferencePreflightStatus.Unavailable
    ? 'Unavailable child UX preference row records a disabled status only; no provider payload is sent.'
    : 'Manual child UX preference row records parent preference and quiet-hours setup requirements before any provider payload can be sent.';
}

const countParentPreferenceState = (
  rows: ReadonlyArray<{
    readonly notificationPreferenceStatusEntry: {
      readonly parentPreferenceState: V3NotificationParentPreferenceState;
    };
  }>,
  state: V3NotificationParentPreferenceState
): number => rows.filter((row) => row.notificationPreferenceStatusEntry.parentPreferenceState === state).length;

const countQuietHoursDecision = (
  rows: ReadonlyArray<{
    readonly notificationPreferenceStatusEntry: {
      readonly quietHoursDecision: V3NotificationQuietHoursDecision;
    };
  }>,
  decision: V3NotificationQuietHoursDecision
): number => rows.filter((row) => row.notificationPreferenceStatusEntry.quietHoursDecision === decision).length;

const countSourceStatus = (
  rows: ReadonlyArray<{ readonly sourcePreferenceStatus: AppGameChildUxLocalOutboxPreferencePreflightStatusValue }>,
  status: AppGameChildUxLocalOutboxPreferencePreflightStatusValue
): number => rows.filter((row) => row.sourcePreferenceStatus === status).length;
