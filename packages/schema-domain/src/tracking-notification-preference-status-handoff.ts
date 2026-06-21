import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema,
  NonEmptyStringSchema
} from './effect';
import {
  ParentContractSchemaVersion,
  ParentContractSchemaVersionSchema,
  ParentTimestampSchema,
} from './family-reference-primitives';
import { FamilyReferenceSchema } from './family-references';
import {
  TrackingNotificationPreferencePreflightReadModelSchema,
  TrackingNotificationPreferencePreflightStatus,
  TrackingNotificationPreferencePreflightStatusSchema,
  type TrackingNotificationPreferencePreflightReadModel,
  type TrackingNotificationPreferencePreflightRow,
} from './tracking-notification-preference-preflight-proof';
import {
  V3NotificationProviderChannelSchema,
  V3NotificationRuleProviderRetryContractEntrySchema,
  V3NotificationRuleProviderRetryContractReadModel,
  V3NotificationRuleReasonCodeSchema,
  type V3NotificationParentPreferenceState,
  type V3NotificationProviderChannel,
  type V3NotificationQuietHoursDecision,
  type V3NotificationRuleReasonCode,
} from './notification-v3-provider-retry';

export const RequiredTrackingNotificationPreferenceStatusHandoffNonClaims = [
  'no-parent-notification-preference-ui',
  'no-parent-notification-history-ui',
  'no-parent-frequency-control-ui',
  'no-parent-notification-ui',
  'no-quiet-hours-timer-runtime',
  'no-provider-delivery-execution',
  'no-provider-receipt-ingestion-runtime',
  'no-provider-credentials',
  'no-cloud-routing',
  'no-child-device-delivery',
  'no-mobile-physical-device-proof',
  'no-retry-worker-runtime',
  'no-production-durable-outbox-storage',
  'no-adapter-dispatch',
] as const;

export const TrackingNotificationPreferenceStatusHandoffNonClaimSchema = withParser(
  Schema.Literal(...RequiredTrackingNotificationPreferenceStatusHandoffNonClaims)
);
export const TrackingNotificationPreferenceStatusHandoffIdSchema = brandedNonEmptyStringSchema('TrackingNotificationPreferenceStatusHandoffId');
export const TrackingNotificationPreferenceStatusHandoffReferenceSchema = brandedNonEmptyStringSchema('TrackingNotificationPreferenceStatusHandoffReference');

const TrackingNotificationPreferenceStatusHandoffRowBaseSchema = Schema.Struct({
  handoffRowId: TrackingNotificationPreferenceStatusHandoffReferenceSchema,
  sourcePreferenceRowId: TrackingNotificationPreferenceStatusHandoffReferenceSchema,
  sourcePreferenceStatus: TrackingNotificationPreferencePreflightStatusSchema,
  sourceProviderNotificationRowId: TrackingNotificationPreferenceStatusHandoffReferenceSchema,
  sourceAlertId: TrackingNotificationPreferenceStatusHandoffReferenceSchema,
  sourceProviderAttemptRef: Schema.Union(TrackingNotificationPreferenceStatusHandoffReferenceSchema, Schema.Null),
  sourcePolicyDecisionId: TrackingNotificationPreferenceStatusHandoffReferenceSchema,
  sourceReasonCodeRef: Schema.Union(TrackingNotificationPreferenceStatusHandoffReferenceSchema, Schema.Null),
  sourceParentPreferenceState: Schema.Union(NonEmptyStringSchema, Schema.Null),
  sourceQuietHoursDecision: Schema.Union(NonEmptyStringSchema, Schema.Null),
  evidenceRefs: Schema.Array(TrackingNotificationPreferenceStatusHandoffReferenceSchema),
  providerPreferenceRefs: Schema.Array(TrackingNotificationPreferenceStatusHandoffReferenceSchema),
  parentPreferenceRequirementRefs: Schema.Array(TrackingNotificationPreferenceStatusHandoffReferenceSchema),
  quietHoursRequirementRefs: Schema.Array(TrackingNotificationPreferenceStatusHandoffReferenceSchema),
  notificationPreferenceStatusEntry: V3NotificationRuleProviderRetryContractEntrySchema,
  manualProofRequirements: Schema.Array(TrackingNotificationPreferenceStatusHandoffReferenceSchema),
});

export const TrackingNotificationPreferenceStatusHandoffRowSchema = withParser(
  TrackingNotificationPreferenceStatusHandoffRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        preferenceStatusHandoffRowIsHonest(row) ||
        'Expected tracking notification preference status handoff rows to map preference preflight rows into V3 notification preference/quiet-hours status entries without claiming delivery'
    )
  )
);

const TrackingNotificationPreferenceStatusHandoffReadModelBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  handoffId: TrackingNotificationPreferenceStatusHandoffIdSchema,
  generatedAt: ParentTimestampSchema,
  family: FamilyReferenceSchema,
  sourcePreferencePreflightId: TrackingNotificationPreferenceStatusHandoffReferenceSchema,
  sourceContractRefs: Schema.Array(TrackingNotificationPreferenceStatusHandoffReferenceSchema),
  notificationRuleProviderRetryReadModelRef: TrackingNotificationPreferenceStatusHandoffReferenceSchema,
  notificationRuleProviderRetryCoverageRefs: Schema.Array(TrackingNotificationPreferenceStatusHandoffReferenceSchema),
  rows: Schema.Array(TrackingNotificationPreferenceStatusHandoffRowSchema),
  parentPreferenceManualSetupRequiredCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  quietHoursManualRequiredCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  preferenceStatusUnavailableCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  handoffNonClaims: Schema.Array(TrackingNotificationPreferenceStatusHandoffNonClaimSchema),
  parentNotificationPreferenceUiClaimed: Schema.Literal(false),
  parentNotificationHistoryUiClaimed: Schema.Literal(false),
  parentFrequencyControlUiClaimed: Schema.Literal(false),
  parentNotificationUiClaimed: Schema.Literal(false),
  quietHoursTimerRuntimeClaimed: Schema.Literal(false),
  providerDeliveryRuntimeClaimed: Schema.Literal(false),
  providerReceiptIngestionRuntimeClaimed: Schema.Literal(false),
  providerCredentialsClaimed: Schema.Literal(false),
  cloudRoutingClaimed: Schema.Literal(false),
  childDeviceDeliveryClaimed: Schema.Literal(false),
  mobilePhysicalDeviceProofClaimed: Schema.Literal(false),
  retryExecutionRuntimeClaimed: Schema.Literal(false),
  productionDurableOutboxStorageClaimed: Schema.Literal(false),
  adapterDispatchClaimed: Schema.Literal(false),
});

export const TrackingNotificationPreferenceStatusHandoffReadModelSchema = withParser(
  TrackingNotificationPreferenceStatusHandoffReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        preferenceStatusHandoffReadModelIsHonest(readModel) ||
        'Expected tracking notification preference status handoff counts and non-claims to match V3 notification preference status rows'
    )
  )
);

export type TrackingNotificationPreferenceStatusHandoffRow = Infer<
  typeof TrackingNotificationPreferenceStatusHandoffRowSchema
>;
export type TrackingNotificationPreferenceStatusHandoffReadModel = Infer<
  typeof TrackingNotificationPreferenceStatusHandoffReadModelSchema
>;

export type TrackingNotificationPreferenceStatusHandoffOptions = {
  readonly generatedAt: string;
  readonly handoffId: string;
  readonly sourceContractRefs: readonly string[];
};

type PreferenceStatusHandoffRowInput = Infer<typeof TrackingNotificationPreferenceStatusHandoffRowBaseSchema>;
type PreferenceStatusEntry = PreferenceStatusHandoffRowInput['notificationPreferenceStatusEntry'];
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

export function buildTrackingNotificationPreferenceStatusHandoffReadModel(
  options: TrackingNotificationPreferenceStatusHandoffOptions,
  sourceReadModel: TrackingNotificationPreferencePreflightReadModel
): TrackingNotificationPreferenceStatusHandoffReadModel {
  const parsedSource = TrackingNotificationPreferencePreflightReadModelSchema.parse(sourceReadModel);
  const rows = parsedSource.rows.map((row) => preferenceStatusHandoffRowForPreflightRow(options, row));

  return TrackingNotificationPreferenceStatusHandoffReadModelSchema.parse({
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
      TrackingNotificationPreferencePreflightStatus.SourceUnavailable
    ),
    handoffNonClaims: RequiredTrackingNotificationPreferenceStatusHandoffNonClaims,
    parentNotificationPreferenceUiClaimed: false,
    parentNotificationHistoryUiClaimed: false,
    parentFrequencyControlUiClaimed: false,
    parentNotificationUiClaimed: false,
    quietHoursTimerRuntimeClaimed: false,
    providerDeliveryRuntimeClaimed: false,
    providerReceiptIngestionRuntimeClaimed: false,
    providerCredentialsClaimed: false,
    cloudRoutingClaimed: false,
    childDeviceDeliveryClaimed: false,
    mobilePhysicalDeviceProofClaimed: false,
    retryExecutionRuntimeClaimed: false,
    productionDurableOutboxStorageClaimed: false,
    adapterDispatchClaimed: false,
  });
}

function preferenceStatusHandoffRowForPreflightRow(
  options: TrackingNotificationPreferenceStatusHandoffOptions,
  row: TrackingNotificationPreferencePreflightRow
): TrackingNotificationPreferenceStatusHandoffRow {
  return TrackingNotificationPreferenceStatusHandoffRowSchema.parse({
    handoffRowId: `tracking-notification-preference-status-handoff-${row.preferenceRowId}`,
    sourcePreferenceRowId: row.preferenceRowId,
    sourcePreferenceStatus: row.status,
    sourceProviderNotificationRowId: row.sourceProviderNotificationRowId,
    sourceAlertId: row.sourceAlertId,
    sourceProviderAttemptRef: row.providerAttemptRef,
    sourcePolicyDecisionId: row.sourcePolicyDecisionId,
    sourceReasonCodeRef: row.reasonCodeRefs[0] ?? null,
    sourceParentPreferenceState: row.parentPreferenceState,
    sourceQuietHoursDecision: row.quietHoursDecision,
    evidenceRefs: row.evidenceRefs,
    providerPreferenceRefs: row.providerPreferenceRefs,
    parentPreferenceRequirementRefs: row.parentPreferenceRequirementRefs,
    quietHoursRequirementRefs: row.quietHoursRequirementRefs,
    notificationPreferenceStatusEntry: preferenceStatusEntryForPreflightRow(options, row),
    manualProofRequirements: row.manualProofRequirements,
  });
}

function preferenceStatusEntryForPreflightRow(
  options: TrackingNotificationPreferenceStatusHandoffOptions,
  row: TrackingNotificationPreferencePreflightRow
): PreferenceStatusEntry {
  const expectation = preferenceStatusExpectationFor(row.status);

  return V3NotificationRuleProviderRetryContractEntrySchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    contractEntryId: `tracking-notification-preference-status-${row.preferenceRowId}`,
    reasonCode: reasonCodeForRow(row),
    providerChannel: providerChannelForRow(row),
    deliveryAttemptState: expectation.deliveryAttemptState,
    deliveryResultState: expectation.deliveryResultState,
    retryPolicyState: expectation.retryPolicyState,
    quietHoursDecision: expectation.quietHoursDecision,
    escalationDecision: expectation.escalationDecision,
    parentPreferenceState: expectation.parentPreferenceState,
    notificationRuleRef: `tracking-notification-preference-status-rule-${row.sourceAlertId}`,
    notificationIntentRef: `tracking-notification-preference-status-intent-${row.sourceAlertId}`,
    deliveryAttemptRef: deliveryAttemptRefFor(row),
    deliveryResultRef: `tracking-notification-preference-status-result-${row.preferenceRowId}`,
    retryPolicyRef: `tracking-notification-preference-status-retry-${row.preferenceRowId}`,
    quietHoursPolicyRef: policyRefOrFallback(row.quietHoursRequirementRefs, row.preferenceRowId, 'quiet-hours'),
    escalationPolicyRef: `tracking-notification-preference-status-escalation-${row.preferenceRowId}`,
    parentPreferenceRef: policyRefOrFallback(row.parentPreferenceRequirementRefs, row.preferenceRowId, 'preference'),
    auditRefs: [`tracking-notification-preference-status-audit-${row.preferenceRowId}`],
    evidenceRefs: row.evidenceRefs,
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
  status: TrackingNotificationPreferencePreflightStatus
): PreferenceStatusExpectation {
  return status === TrackingNotificationPreferencePreflightStatus.SourceUnavailable
    ? UnavailablePreferenceStatusExpectation
    : ManualPreferenceStatusExpectation;
}

function reasonCodeForRow(row: TrackingNotificationPreferencePreflightRow): V3NotificationRuleReasonCode {
  const parsedReason = row.reasonCodeRefs
    .map((reasonCodeRef) => V3NotificationRuleReasonCodeSchema.safeParse(reasonCodeRef))
    .find((result) => result.success);
  if (parsedReason?.success) {
    return parsedReason.data;
  }
  return V3NotificationRuleReasonCodeSchema.parse(
    row.status === TrackingNotificationPreferencePreflightStatus.SourceUnavailable
      ? 'provider-failure'
      : 'parent-request'
  );
}

function providerChannelForRow(row: TrackingNotificationPreferencePreflightRow): V3NotificationProviderChannel {
  return V3NotificationProviderChannelSchema.parse(row.providerPreferenceRefs.length > 0 ? 'push' : 'in-app');
}

function deliveryAttemptRefFor(row: TrackingNotificationPreferencePreflightRow): string {
  return (
    row.providerAttemptRef ?? `tracking-notification-preference-status-attempt-not-executed-${row.preferenceRowId}`
  );
}

function policyRefOrFallback(refs: readonly string[], rowId: string, kind: string): string {
  return refs[0] ?? `tracking-notification-preference-status-${kind}-${rowId}`;
}

function minimalProviderPayloadBoundaryFor(status: TrackingNotificationPreferencePreflightStatus): string {
  return status === TrackingNotificationPreferencePreflightStatus.SourceUnavailable
    ? 'Unavailable tracking notification preference row records a disabled status only; no provider payload is sent.'
    : 'Manual tracking notification preference row records parent preference and quiet-hours setup requirements before any provider payload can be sent.';
}

function preferenceStatusHandoffRowIsHonest(row: PreferenceStatusHandoffRowInput): boolean {
  const entry = row.notificationPreferenceStatusEntry;
  return (
    preferenceStatusEntryMatchesExpectation(entry, preferenceStatusExpectationFor(row.sourcePreferenceStatus)) &&
    preferenceStatusEntryKeepsDeliveryUnclaimed(entry) &&
    row.manualProofRequirements.length > 0 &&
    entry.manualProofRequirements.length > 0 &&
    entry.evidenceRefs.length > 0
  );
}

function preferenceStatusEntryMatchesExpectation(
  entry: PreferenceStatusEntry,
  expected: PreferenceStatusExpectation
): boolean {
  return (
    entry.deliveryAttemptState === expected.deliveryAttemptState &&
    entry.deliveryResultState === expected.deliveryResultState &&
    entry.retryPolicyState === expected.retryPolicyState &&
    entry.quietHoursDecision === expected.quietHoursDecision &&
    entry.escalationDecision === expected.escalationDecision &&
    entry.parentPreferenceState === expected.parentPreferenceState
  );
}

function preferenceStatusEntryKeepsDeliveryUnclaimed(entry: PreferenceStatusEntry): boolean {
  return (
    entry.providerReceiptRefs.length === 0 &&
    [
      entry.providerAdapterImplemented,
      entry.deliveryAttemptExecuted,
      entry.providerReceiptObserved,
      entry.rawEvidenceInProviderPayload,
      entry.providerStoresChildEvidenceClaimed,
    ].every((claim) => claim === false)
  );
}

function preferenceStatusHandoffReadModelIsHonest(
  readModel: Infer<typeof TrackingNotificationPreferenceStatusHandoffReadModelBaseSchema>
): boolean {
  return (
    readModel.parentPreferenceManualSetupRequiredCount ===
      countParentPreferenceState(readModel.rows, 'manual-setup-required') &&
    readModel.quietHoursManualRequiredCount === countQuietHoursDecision(readModel.rows, 'manual-required') &&
    readModel.preferenceStatusUnavailableCount ===
      countSourceStatus(readModel.rows, TrackingNotificationPreferencePreflightStatus.SourceUnavailable) &&
    RequiredTrackingNotificationPreferenceStatusHandoffNonClaims.every((claim) =>
      readModel.handoffNonClaims.includes(claim)
    ) &&
    readModel.notificationRuleProviderRetryCoverageRefs.length ===
      V3NotificationRuleProviderRetryContractReadModel.entries.length
  );
}

function countParentPreferenceState(
  rows: ReadonlyArray<{
    readonly notificationPreferenceStatusEntry: { readonly parentPreferenceState: V3NotificationParentPreferenceState };
  }>,
  state: V3NotificationParentPreferenceState
): number {
  return rows.filter((row) => row.notificationPreferenceStatusEntry.parentPreferenceState === state).length;
}

function countQuietHoursDecision(
  rows: ReadonlyArray<{
    readonly notificationPreferenceStatusEntry: { readonly quietHoursDecision: V3NotificationQuietHoursDecision };
  }>,
  decision: V3NotificationQuietHoursDecision
): number {
  return rows.filter((row) => row.notificationPreferenceStatusEntry.quietHoursDecision === decision).length;
}

function countSourceStatus(
  rows: ReadonlyArray<{ readonly sourcePreferenceStatus: TrackingNotificationPreferencePreflightStatus }>,
  status: TrackingNotificationPreferencePreflightStatus
): number {
  return rows.filter((row) => row.sourcePreferenceStatus === status).length;
}


