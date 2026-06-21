import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema,
  NonEmptyStringSchema
} from './effect';
import {
  AppGameNotificationPreferencePreflightReadModelSchema,
  AppGameNotificationPreferencePreflightStatus,
  AppGameNotificationPreferencePreflightStatusSchema,
  type AppGameNotificationPreferencePreflightReadModel,
  type AppGameNotificationPreferencePreflightRow,
} from './app-game-notification-preference-preflight';
import {
  ParentContractSchemaVersion,
  ParentContractSchemaVersionSchema,
  ParentTimestampSchema,
} from './family-reference-primitives';
import { FamilyReferenceSchema } from './family-references';
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

export const RequiredAppGameNotificationPreferenceStatusHandoffNonClaims = [
  'no-parent-preference-ui',
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
] as const;

export const AppGameNotificationPreferenceStatusHandoffNonClaimSchema = withParser(
  Schema.Literal(...RequiredAppGameNotificationPreferenceStatusHandoffNonClaims)
);

export const AppGameNotificationPreferenceStatusHandoffIdSchema = brandedNonEmptyStringSchema(
  'AppGameNotificationPreferenceStatusHandoffId'
);
export const AppGameNotificationPreferenceStatusHandoffReferenceSchema = brandedNonEmptyStringSchema(
  'AppGameNotificationPreferenceStatusHandoffReference'
);

const AppGameNotificationPreferenceStatusHandoffRowBaseSchema = Schema.Struct({
  handoffRowId: AppGameNotificationPreferenceStatusHandoffReferenceSchema,
  sourcePreferenceRowId: AppGameNotificationPreferenceStatusHandoffReferenceSchema,
  sourcePreferenceStatus: AppGameNotificationPreferencePreflightStatusSchema,
  sourceSchedulerEntryRef: Schema.Union(AppGameNotificationPreferenceStatusHandoffReferenceSchema, Schema.Null),
  sourceOutboxRecordRef: Schema.Union(AppGameNotificationPreferenceStatusHandoffReferenceSchema, Schema.Null),
  sourceProviderChannelRef: Schema.Union(AppGameNotificationPreferenceStatusHandoffReferenceSchema, Schema.Null),
  sourceReasonCodeRef: Schema.Union(AppGameNotificationPreferenceStatusHandoffReferenceSchema, Schema.Null),
  sourceParentPreferenceState: Schema.Union(NonEmptyStringSchema, Schema.Null),
  sourceQuietHoursDecision: Schema.Union(NonEmptyStringSchema, Schema.Null),
  sourceParentPreferenceRequirementRefs: Schema.Array(AppGameNotificationPreferenceStatusHandoffReferenceSchema),
  sourceQuietHoursRequirementRefs: Schema.Array(AppGameNotificationPreferenceStatusHandoffReferenceSchema),
  notificationPreferenceStatusEntry: V3NotificationRuleProviderRetryContractEntrySchema,
  manualProofRequirements: Schema.Array(AppGameNotificationPreferenceStatusHandoffReferenceSchema),
});

export const AppGameNotificationPreferenceStatusHandoffRowSchema = withParser(
  AppGameNotificationPreferenceStatusHandoffRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        preferenceStatusHandoffRowIsHonest(row) ||
        'Expected app/game notification preference status handoff rows to map preference preflight rows into V3 notification preference/quiet-hours status entries without claiming delivery'
    )
  )
);

const AppGameNotificationPreferenceStatusHandoffReadModelBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  handoffId: AppGameNotificationPreferenceStatusHandoffIdSchema,
  generatedAt: ParentTimestampSchema,
  family: FamilyReferenceSchema,
  sourcePreferencePreflightId: AppGameNotificationPreferenceStatusHandoffReferenceSchema,
  sourceContractRefs: Schema.Array(AppGameNotificationPreferenceStatusHandoffReferenceSchema),
  notificationRuleProviderRetryReadModelRef: AppGameNotificationPreferenceStatusHandoffReferenceSchema,
  notificationRuleProviderRetryCoverageRefs: Schema.Array(AppGameNotificationPreferenceStatusHandoffReferenceSchema),
  rows: Schema.Array(AppGameNotificationPreferenceStatusHandoffRowSchema),
  parentPreferenceManualSetupRequiredCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  quietHoursManualRequiredCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  preferenceStatusUnavailableCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  handoffNonClaims: Schema.Array(AppGameNotificationPreferenceStatusHandoffNonClaimSchema),
  parentPreferenceUiClaimed: Schema.Literal(false),
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
});

export const AppGameNotificationPreferenceStatusHandoffReadModelSchema = withParser(
  AppGameNotificationPreferenceStatusHandoffReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        preferenceStatusHandoffReadModelIsHonest(readModel) ||
        'Expected app/game notification preference status handoff counts and non-claims to match V3 notification preference status rows'
    )
  )
);

export type AppGameNotificationPreferenceStatusHandoffRow = Infer<
  typeof AppGameNotificationPreferenceStatusHandoffRowSchema
>;
export type AppGameNotificationPreferenceStatusHandoffReadModel = Infer<
  typeof AppGameNotificationPreferenceStatusHandoffReadModelSchema
>;

type PreferenceStatusHandoffRowInput = Infer<typeof AppGameNotificationPreferenceStatusHandoffRowBaseSchema>;
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

export type AppGameNotificationPreferenceStatusHandoffOptions = {
  readonly generatedAt: string;
  readonly handoffId: string;
  readonly sourceContractRefs: readonly string[];
};

export function buildAppGameNotificationPreferenceStatusHandoffReadModel(
  options: AppGameNotificationPreferenceStatusHandoffOptions,
  sourceReadModel: AppGameNotificationPreferencePreflightReadModel
): AppGameNotificationPreferenceStatusHandoffReadModel {
  const parsedSource = AppGameNotificationPreferencePreflightReadModelSchema.parse(sourceReadModel);
  const rows = parsedSource.rows.map((row) => preferenceStatusHandoffRowForPreflightRow(options, row));

  return AppGameNotificationPreferenceStatusHandoffReadModelSchema.parse({
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
    preferenceStatusUnavailableCount: countSourceStatus(rows, AppGameNotificationPreferencePreflightStatus.Unavailable),
    handoffNonClaims: RequiredAppGameNotificationPreferenceStatusHandoffNonClaims,
    parentPreferenceUiClaimed: false,
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
  });
}

function preferenceStatusHandoffRowForPreflightRow(
  options: AppGameNotificationPreferenceStatusHandoffOptions,
  row: AppGameNotificationPreferencePreflightRow
): AppGameNotificationPreferenceStatusHandoffRow {
  return AppGameNotificationPreferenceStatusHandoffRowSchema.parse({
    handoffRowId: `app-game-notification-preference-status-handoff-${row.preferenceRowId}`,
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
  options: AppGameNotificationPreferenceStatusHandoffOptions,
  row: AppGameNotificationPreferencePreflightRow
): PreferenceStatusEntry {
  const expectation = preferenceStatusExpectationFor(row.status);

  return V3NotificationRuleProviderRetryContractEntrySchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    contractEntryId: `app-game-notification-preference-status-${row.preferenceRowId}`,
    reasonCode: reasonCodeForRow(row),
    providerChannel: providerChannelForRow(row),
    deliveryAttemptState: expectation.deliveryAttemptState,
    deliveryResultState: expectation.deliveryResultState,
    retryPolicyState: expectation.retryPolicyState,
    quietHoursDecision: expectation.quietHoursDecision,
    escalationDecision: expectation.escalationDecision,
    parentPreferenceState: expectation.parentPreferenceState,
    notificationRuleRef: ruleRefForRow(row),
    notificationIntentRef: `app-game-notification-preference-status-intent-${row.sourceSchedulerBridgeRecordId}`,
    deliveryAttemptRef: `app-game-notification-preference-status-attempt-not-executed-${row.preferenceRowId}`,
    deliveryResultRef: `app-game-notification-preference-status-result-${row.preferenceRowId}`,
    retryPolicyRef: `app-game-notification-preference-status-retry-${row.preferenceRowId}`,
    quietHoursPolicyRef: policyRefOrFallback(row.quietHoursRequirementRefs, row.preferenceRowId, 'quiet-hours'),
    escalationPolicyRef: `app-game-notification-preference-status-escalation-${row.preferenceRowId}`,
    parentPreferenceRef: policyRefOrFallback(row.parentPreferenceRequirementRefs, row.preferenceRowId, 'preference'),
    auditRefs: [`app-game-notification-preference-status-audit-${row.preferenceRowId}`],
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
  status: AppGameNotificationPreferencePreflightStatus
): PreferenceStatusExpectation {
  if (status === AppGameNotificationPreferencePreflightStatus.Unavailable) {
    return UnavailablePreferenceStatusExpectation;
  }
  return ManualPreferenceStatusExpectation;
}

function reasonCodeForRow(row: AppGameNotificationPreferencePreflightRow): V3NotificationRuleReasonCode {
  if (row.reasonCodeRef !== null) {
    return V3NotificationRuleReasonCodeSchema.parse(row.reasonCodeRef);
  }
  return V3NotificationRuleReasonCodeSchema.parse(
    row.status === AppGameNotificationPreferencePreflightStatus.Unavailable ? 'provider-failure' : 'parent-request'
  );
}

function providerChannelForRow(row: AppGameNotificationPreferencePreflightRow): V3NotificationProviderChannel {
  return V3NotificationProviderChannelSchema.parse(row.providerChannelRef ?? 'in-app');
}

function ruleRefForRow(row: AppGameNotificationPreferencePreflightRow): string {
  return row.reasonCodeRef === null
    ? `app-game-notification-preference-status-rule-${row.preferenceRowId}`
    : `app-game-notification-preference-status-rule-${row.reasonCodeRef}`;
}

function policyRefOrFallback(refs: readonly string[], rowId: string, kind: string): string {
  return refs[0] ?? `app-game-notification-preference-status-${kind}-${rowId}`;
}

function evidenceRefsForRow(row: AppGameNotificationPreferencePreflightRow): readonly string[] {
  const sourceRefs = [row.sourceSchedulerEntryRef, row.sourceOutboxRecordRef].flatMap((ref) =>
    ref === null ? [] : [ref]
  );
  return sourceRefs.length === 0 ? row.manualProofRequirements : sourceRefs;
}

function minimalProviderPayloadBoundaryFor(status: AppGameNotificationPreferencePreflightStatus): string {
  return status === AppGameNotificationPreferencePreflightStatus.Unavailable
    ? 'Unavailable app/game notification preference row records a disabled status only; no provider payload is sent.'
    : 'Manual app/game notification preference row records parent preference and quiet-hours setup requirements before any provider payload can be sent.';
}

function preferenceStatusHandoffRowIsHonest(row: PreferenceStatusHandoffRowInput): boolean {
  const entry = row.notificationPreferenceStatusEntry;

  return (
    preferenceStatusEntryMatchesPreflight(row) &&
    preferenceStatusEntryKeepsDeliveryUnclaimed(entry) &&
    row.manualProofRequirements.length > 0 &&
    entry.manualProofRequirements.length > 0 &&
    entry.evidenceRefs.length > 0
  );
}

function preferenceStatusEntryMatchesPreflight(row: PreferenceStatusHandoffRowInput): boolean {
  const expected = preferenceStatusExpectationFor(row.sourcePreferenceStatus);
  return preferenceStatusEntryMatchesExpectation(row.notificationPreferenceStatusEntry, expected);
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
  readModel: Infer<typeof AppGameNotificationPreferenceStatusHandoffReadModelBaseSchema>
): boolean {
  return (
    readModel.parentPreferenceManualSetupRequiredCount ===
      countParentPreferenceState(readModel.rows, 'manual-setup-required') &&
    readModel.quietHoursManualRequiredCount === countQuietHoursDecision(readModel.rows, 'manual-required') &&
    readModel.preferenceStatusUnavailableCount ===
      countSourceStatus(readModel.rows, AppGameNotificationPreferencePreflightStatus.Unavailable) &&
    RequiredAppGameNotificationPreferenceStatusHandoffNonClaims.every((claim) =>
      readModel.handoffNonClaims.includes(claim)
    ) &&
    readModel.notificationRuleProviderRetryCoverageRefs.length ===
      V3NotificationRuleProviderRetryContractReadModel.entries.length
  );
}

const countParentPreferenceState = (
  rows: ReadonlyArray<{
    readonly notificationPreferenceStatusEntry: { readonly parentPreferenceState: V3NotificationParentPreferenceState };
  }>,
  state: V3NotificationParentPreferenceState
): number => rows.filter((row) => row.notificationPreferenceStatusEntry.parentPreferenceState === state).length;

const countQuietHoursDecision = (
  rows: ReadonlyArray<{
    readonly notificationPreferenceStatusEntry: { readonly quietHoursDecision: V3NotificationQuietHoursDecision };
  }>,
  decision: V3NotificationQuietHoursDecision
): number => rows.filter((row) => row.notificationPreferenceStatusEntry.quietHoursDecision === decision).length;

const countSourceStatus = (
  rows: ReadonlyArray<{ readonly sourcePreferenceStatus: AppGameNotificationPreferencePreflightStatus }>,
  status: AppGameNotificationPreferencePreflightStatus
): number => rows.filter((row) => row.sourcePreferenceStatus === status).length;


