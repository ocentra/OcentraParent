/* generated from crates/browser-core/src/social_alert_report_preference_status_handoff.rs */

import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema,
  NonEmptyStringSchema,
} from '@ocentra-parent/schema-domain/effect';
import {
  ParentContractSchemaVersion,
  ParentContractSchemaVersionSchema,
  ParentTimestampSchema,
} from '@ocentra-parent/schema-domain/family-reference-primitives';
import { FamilyReferenceSchema } from '@ocentra-parent/schema-domain/family-references';
import {
  SocialAlertReportPreferencePreflightReadModelSchema,
  SocialAlertReportPreferencePreflightStatus,
  SocialAlertReportPreferencePreflightStatusSchema,
  type SocialAlertReportPreferencePreflightReadModel,
  type SocialAlertReportPreferencePreflightRow,
} from './social-alert-report-preference-preflight';
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

export const RequiredSocialAlertReportPreferenceStatusHandoffNonClaims = [
  'no-parent-notification-preference-ui',
  'no-parent-notification-history-ui',
  'no-parent-frequency-control-ui',
  'no-parent-notification-ui',
  'no-quiet-hours-timer-runtime',
  'no-provider-delivery-execution',
  'no-provider-receipt-ingestion',
  'no-provider-credentials',
  'no-cloud-routing',
  'no-child-delivery',
  'no-retry-worker-runtime',
  'no-production-durable-outbox-storage',
  'no-adapter-dispatch',
  'no-report-delivery-execution',
  'no-final-policy-execution',
  'no-enforcement',
] as const;

export const SocialAlertReportPreferenceStatusHandoffNonClaimSchema = withParser(
  Schema.Literal(...RequiredSocialAlertReportPreferenceStatusHandoffNonClaims)
);
export const SocialAlertReportPreferenceStatusHandoffIdSchema = brandedNonEmptyStringSchema(
  'SocialAlertReportPreferenceStatusHandoffId'
);
export const SocialAlertReportPreferenceStatusHandoffReferenceSchema = brandedNonEmptyStringSchema(
  'SocialAlertReportPreferenceStatusHandoffReference'
);

const SocialAlertReportPreferenceStatusHandoffRowBaseSchema = Schema.Struct({
  handoffRowId: SocialAlertReportPreferenceStatusHandoffReferenceSchema,
  sourcePreferenceRowId: SocialAlertReportPreferenceStatusHandoffReferenceSchema,
  sourcePreferenceStatus: SocialAlertReportPreferencePreflightStatusSchema,
  sourceSchedulerEntryRef: Schema.Union(SocialAlertReportPreferenceStatusHandoffReferenceSchema, Schema.Null),
  sourceOutboxRecordRef: Schema.Union(SocialAlertReportPreferenceStatusHandoffReferenceSchema, Schema.Null),
  sourceProviderChannelRef: Schema.Union(SocialAlertReportPreferenceStatusHandoffReferenceSchema, Schema.Null),
  sourceReasonCodeRef: Schema.Union(SocialAlertReportPreferenceStatusHandoffReferenceSchema, Schema.Null),
  sourceSchedulerDecisionRef: Schema.Union(SocialAlertReportPreferenceStatusHandoffReferenceSchema, Schema.Null),
  sourceParentPreferenceState: Schema.Union(NonEmptyStringSchema, Schema.Null),
  sourceQuietHoursDecision: Schema.Union(NonEmptyStringSchema, Schema.Null),
  sourceParentPreferenceRequirementRefs: Schema.Array(SocialAlertReportPreferenceStatusHandoffReferenceSchema),
  sourceQuietHoursRequirementRefs: Schema.Array(SocialAlertReportPreferenceStatusHandoffReferenceSchema),
  notificationPreferenceStatusEntry: V3NotificationRuleProviderRetryContractEntrySchema,
  manualProofRequirements: Schema.Array(SocialAlertReportPreferenceStatusHandoffReferenceSchema),
});

export const SocialAlertReportPreferenceStatusHandoffRowSchema = withParser(
  SocialAlertReportPreferenceStatusHandoffRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        preferenceStatusHandoffRowIsHonest(row) ||
        'Expected social alert/report preference status handoff rows to map preference preflight rows into V3 notification preference/quiet-hours status entries without claiming delivery'
    )
  )
);

const SocialAlertReportPreferenceStatusHandoffReadModelBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  handoffId: SocialAlertReportPreferenceStatusHandoffIdSchema,
  generatedAt: ParentTimestampSchema,
  family: FamilyReferenceSchema,
  sourcePreferencePreflightId: SocialAlertReportPreferenceStatusHandoffReferenceSchema,
  sourceContractRefs: Schema.Array(SocialAlertReportPreferenceStatusHandoffReferenceSchema),
  notificationRuleProviderRetryReadModelRef: SocialAlertReportPreferenceStatusHandoffReferenceSchema,
  notificationRuleProviderRetryCoverageRefs: Schema.Array(SocialAlertReportPreferenceStatusHandoffReferenceSchema),
  rows: Schema.Array(SocialAlertReportPreferenceStatusHandoffRowSchema),
  parentPreferenceManualSetupRequiredCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  quietHoursManualRequiredCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  preferenceStatusUnavailableCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  handoffNonClaims: Schema.Array(SocialAlertReportPreferenceStatusHandoffNonClaimSchema),
  parentNotificationPreferenceUiClaimed: Schema.Literal(false),
  parentNotificationHistoryUiClaimed: Schema.Literal(false),
  parentFrequencyControlUiClaimed: Schema.Literal(false),
  parentNotificationUiClaimed: Schema.Literal(false),
  quietHoursTimerRuntimeClaimed: Schema.Literal(false),
  providerDeliveryRuntimeClaimed: Schema.Literal(false),
  providerReceiptIngestionClaimed: Schema.Literal(false),
  providerCredentialsClaimed: Schema.Literal(false),
  cloudRoutingClaimed: Schema.Literal(false),
  childDeliveryClaimed: Schema.Literal(false),
  retryExecutionRuntimeClaimed: Schema.Literal(false),
  productionDurableOutboxStorageClaimed: Schema.Literal(false),
  adapterDispatchClaimed: Schema.Literal(false),
  reportDeliveryExecutionClaimed: Schema.Literal(false),
  finalPolicyExecutionClaimed: Schema.Literal(false),
  enforcementClaimed: Schema.Literal(false),
});

export const SocialAlertReportPreferenceStatusHandoffReadModelSchema = withParser(
  SocialAlertReportPreferenceStatusHandoffReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        preferenceStatusHandoffReadModelIsHonest(readModel) ||
        'Expected social alert/report preference status handoff counts and non-claims to match V3 notification preference status rows'
    )
  )
);

export type SocialAlertReportPreferenceStatusHandoffRow = Infer<
  typeof SocialAlertReportPreferenceStatusHandoffRowSchema
>;
export type SocialAlertReportPreferenceStatusHandoffReadModel = Infer<
  typeof SocialAlertReportPreferenceStatusHandoffReadModelSchema
>;
export type SocialAlertReportPreferenceStatusHandoffOptions = {
  readonly generatedAt: string;
  readonly handoffId: string;
  readonly sourceContractRefs: readonly string[];
};

type HandoffRowInput = Infer<typeof SocialAlertReportPreferenceStatusHandoffRowBaseSchema>;
type StatusEntry = HandoffRowInput['notificationPreferenceStatusEntry'];
type StatusExpectation = Pick<
  StatusEntry,
  | 'deliveryAttemptState'
  | 'deliveryResultState'
  | 'retryPolicyState'
  | 'quietHoursDecision'
  | 'escalationDecision'
  | 'parentPreferenceState'
>;

const ManualStatusExpectation: StatusExpectation = {
  deliveryAttemptState: 'eligible',
  deliveryResultState: 'manual-required',
  retryPolicyState: 'manual-review',
  quietHoursDecision: 'manual-required',
  escalationDecision: 'manual-review',
  parentPreferenceState: 'manual-setup-required',
};

const UnavailableStatusExpectation: StatusExpectation = {
  deliveryAttemptState: 'provider-disabled',
  deliveryResultState: 'not-sent',
  retryPolicyState: 'provider-disabled',
  quietHoursDecision: 'allow',
  escalationDecision: 'none',
  parentPreferenceState: 'channel-disabled',
};

export function buildSocialAlertReportPreferenceStatusHandoffReadModel(
  options: SocialAlertReportPreferenceStatusHandoffOptions,
  sourceReadModel: SocialAlertReportPreferencePreflightReadModel
): SocialAlertReportPreferenceStatusHandoffReadModel {
  const parsedSource = SocialAlertReportPreferencePreflightReadModelSchema.parse(sourceReadModel);
  const rows = parsedSource.rows.map((row) => preferenceStatusHandoffRowForPreflightRow(options, row));

  return SocialAlertReportPreferenceStatusHandoffReadModelSchema.parse({
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
    preferenceStatusUnavailableCount: countSourceStatus(rows, SocialAlertReportPreferencePreflightStatus.Unavailable),
    handoffNonClaims: RequiredSocialAlertReportPreferenceStatusHandoffNonClaims,
    parentNotificationPreferenceUiClaimed: false,
    parentNotificationHistoryUiClaimed: false,
    parentFrequencyControlUiClaimed: false,
    parentNotificationUiClaimed: false,
    quietHoursTimerRuntimeClaimed: false,
    providerDeliveryRuntimeClaimed: false,
    providerReceiptIngestionClaimed: false,
    providerCredentialsClaimed: false,
    cloudRoutingClaimed: false,
    childDeliveryClaimed: false,
    retryExecutionRuntimeClaimed: false,
    productionDurableOutboxStorageClaimed: false,
    adapterDispatchClaimed: false,
    reportDeliveryExecutionClaimed: false,
    finalPolicyExecutionClaimed: false,
    enforcementClaimed: false,
  });
}

function preferenceStatusHandoffRowForPreflightRow(
  options: SocialAlertReportPreferenceStatusHandoffOptions,
  row: SocialAlertReportPreferencePreflightRow
): SocialAlertReportPreferenceStatusHandoffRow {
  return SocialAlertReportPreferenceStatusHandoffRowSchema.parse({
    handoffRowId: `social-alert-report-preference-status-handoff-${row.preferenceRowId}`,
    sourcePreferenceRowId: row.preferenceRowId,
    sourcePreferenceStatus: row.status,
    sourceSchedulerEntryRef: row.sourceSchedulerEntryRef,
    sourceOutboxRecordRef: row.sourceOutboxRecordRef,
    sourceProviderChannelRef: row.providerChannelRef,
    sourceReasonCodeRef: row.reasonCodeRef,
    sourceSchedulerDecisionRef: row.schedulerDecisionRef,
    sourceParentPreferenceState: row.parentPreferenceState,
    sourceQuietHoursDecision: row.quietHoursDecision,
    sourceParentPreferenceRequirementRefs: row.parentPreferenceRequirementRefs,
    sourceQuietHoursRequirementRefs: row.quietHoursRequirementRefs,
    notificationPreferenceStatusEntry: preferenceStatusEntryForPreflightRow(options, row),
    manualProofRequirements: row.manualProofRequirements,
  });
}

function preferenceStatusEntryForPreflightRow(
  options: SocialAlertReportPreferenceStatusHandoffOptions,
  row: SocialAlertReportPreferencePreflightRow
): StatusEntry {
  const expectation = preferenceStatusExpectationFor(row.status);

  return V3NotificationRuleProviderRetryContractEntrySchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    contractEntryId: `social-alert-report-preference-status-${row.preferenceRowId}`,
    reasonCode: reasonCodeForRow(row),
    providerChannel: providerChannelForRow(row),
    deliveryAttemptState: expectation.deliveryAttemptState,
    deliveryResultState: expectation.deliveryResultState,
    retryPolicyState: expectation.retryPolicyState,
    quietHoursDecision: expectation.quietHoursDecision,
    escalationDecision: expectation.escalationDecision,
    parentPreferenceState: expectation.parentPreferenceState,
    notificationRuleRef: ruleRefForRow(row),
    notificationIntentRef: `social-alert-report-preference-status-intent-${row.sourceSchedulerBridgeRecordId}`,
    deliveryAttemptRef: `social-alert-report-preference-status-attempt-not-executed-${row.preferenceRowId}`,
    deliveryResultRef: `social-alert-report-preference-status-result-${row.preferenceRowId}`,
    retryPolicyRef: `social-alert-report-preference-status-retry-${row.preferenceRowId}`,
    quietHoursPolicyRef: policyRefOrFallback(row.quietHoursRequirementRefs, row.preferenceRowId, 'quiet-hours'),
    escalationPolicyRef: `social-alert-report-preference-status-escalation-${row.preferenceRowId}`,
    parentPreferenceRef: policyRefOrFallback(row.parentPreferenceRequirementRefs, row.preferenceRowId, 'preference'),
    auditRefs: [`social-alert-report-preference-status-audit-${row.preferenceRowId}`],
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

function preferenceStatusExpectationFor(status: SocialAlertReportPreferencePreflightStatus): StatusExpectation {
  return status === SocialAlertReportPreferencePreflightStatus.Unavailable
    ? UnavailableStatusExpectation
    : ManualStatusExpectation;
}

function reasonCodeForRow(row: SocialAlertReportPreferencePreflightRow): V3NotificationRuleReasonCode {
  if (row.reasonCodeRef !== null) {
    return V3NotificationRuleReasonCodeSchema.parse(row.reasonCodeRef);
  }
  return V3NotificationRuleReasonCodeSchema.parse(
    row.status === SocialAlertReportPreferencePreflightStatus.Unavailable ? 'provider-failure' : 'parent-request'
  );
}

function providerChannelForRow(row: SocialAlertReportPreferencePreflightRow): V3NotificationProviderChannel {
  return V3NotificationProviderChannelSchema.parse(row.providerChannelRef ?? 'in-app');
}

function ruleRefForRow(row: SocialAlertReportPreferencePreflightRow): string {
  return row.reasonCodeRef === null
    ? `social-alert-report-preference-status-rule-${row.preferenceRowId}`
    : `social-alert-report-preference-status-rule-${row.reasonCodeRef}`;
}

function policyRefOrFallback(refs: readonly string[], rowId: string, kind: string): string {
  return refs[0] ?? `social-alert-report-preference-status-${kind}-${rowId}`;
}

function evidenceRefsForRow(row: SocialAlertReportPreferencePreflightRow): readonly string[] {
  const sourceRefs = [row.sourceSchedulerEntryRef, row.sourceOutboxRecordRef, row.schedulerDecisionRef].flatMap(
    (ref) => (ref === null ? [] : [ref])
  );
  return sourceRefs.length === 0 ? row.manualProofRequirements : sourceRefs;
}

function minimalProviderPayloadBoundaryFor(status: SocialAlertReportPreferencePreflightStatus): string {
  return status === SocialAlertReportPreferencePreflightStatus.Unavailable
    ? 'Unavailable social alert/report preference row records a disabled status only; no provider payload is sent.'
    : 'Manual social alert/report preference row records parent preference and quiet-hours setup requirements before any provider payload can be sent.';
}

function preferenceStatusHandoffRowIsHonest(row: HandoffRowInput): boolean {
  const entry = row.notificationPreferenceStatusEntry;
  return (
    preferenceStatusEntryMatchesPreflight(row) &&
    preferenceStatusEntryKeepsDeliveryUnclaimed(entry) &&
    row.manualProofRequirements.length > 0 &&
    entry.manualProofRequirements.length > 0 &&
    entry.evidenceRefs.length > 0
  );
}

function preferenceStatusEntryMatchesPreflight(row: HandoffRowInput): boolean {
  const expected = preferenceStatusExpectationFor(row.sourcePreferenceStatus);
  return preferenceStatusEntryMatchesExpectation(row.notificationPreferenceStatusEntry, expected);
}

function preferenceStatusEntryMatchesExpectation(entry: StatusEntry, expected: StatusExpectation): boolean {
  return (
    entry.deliveryAttemptState === expected.deliveryAttemptState &&
    entry.deliveryResultState === expected.deliveryResultState &&
    entry.retryPolicyState === expected.retryPolicyState &&
    entry.quietHoursDecision === expected.quietHoursDecision &&
    entry.escalationDecision === expected.escalationDecision &&
    entry.parentPreferenceState === expected.parentPreferenceState
  );
}

function preferenceStatusEntryKeepsDeliveryUnclaimed(entry: StatusEntry): boolean {
  const deliveryClaims = [
    entry.providerAdapterImplemented,
    entry.deliveryAttemptExecuted,
    entry.providerReceiptObserved,
    entry.rawEvidenceInProviderPayload,
    entry.providerStoresChildEvidenceClaimed,
  ];

  return entry.providerReceiptRefs.length === 0 && deliveryClaims.every((claim) => claim === false);
}

function preferenceStatusHandoffReadModelIsHonest(
  readModel: Infer<typeof SocialAlertReportPreferenceStatusHandoffReadModelBaseSchema>
): boolean {
  return (
    readModel.parentPreferenceManualSetupRequiredCount ===
      countParentPreferenceState(readModel.rows, 'manual-setup-required') &&
    readModel.quietHoursManualRequiredCount === countQuietHoursDecision(readModel.rows, 'manual-required') &&
    readModel.preferenceStatusUnavailableCount ===
      countSourceStatus(readModel.rows, SocialAlertReportPreferencePreflightStatus.Unavailable) &&
    RequiredSocialAlertReportPreferenceStatusHandoffNonClaims.every((claim) =>
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
  rows: ReadonlyArray<{ readonly sourcePreferenceStatus: SocialAlertReportPreferencePreflightStatus }>,
  status: SocialAlertReportPreferencePreflightStatus
): number {
  return rows.filter((row) => row.sourcePreferenceStatus === status).length;
}
