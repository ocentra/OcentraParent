import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema,
  NonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import {
  AppGameChildUxLocalOutboxPreferencePreflightReadModelSchema,
  AppGameChildUxLocalOutboxPreferencePreflightStatus,
  AppGameChildUxLocalOutboxPreferencePreflightStatusSchema,
  type AppGameChildUxLocalOutboxPreferencePreflightReadModel,
  type AppGameChildUxLocalOutboxPreferencePreflightRow,
} from './app-game-child-facing-ux-local-outbox-preference-preflight';
import {
  ParentContractSchemaVersion,
  ParentContractSchemaVersionSchema,
  ParentTimestampSchema,
} from '@ocentra-parent/schema-domain/family-reference-primitives';
import { FamilyReferenceSchema } from '@ocentra-parent/family-domain/references';
import {
  V3NotificationProviderChannelSchema,
  V3NotificationRuleProviderRetryContractEntrySchema,
  V3NotificationRuleProviderRetryContractReadModel,
  V3NotificationRuleReasonCodeSchema,
  type V3NotificationParentPreferenceState,
  type V3NotificationProviderChannel,
  type V3NotificationQuietHoursDecision,
  type V3NotificationRuleReasonCode,
} from '@ocentra-parent/notification-domain/v3-notification-rule-provider-retry-contract';

export const RequiredAppGameChildUxLocalOutboxPreferenceStatusHandoffNonClaims = [
  'no-parent-preference-ui',
  'no-parent-frequency-control-ui',
  'no-parent-notification-ui',
  'no-parent-preference-mutation',
  'no-quiet-hours-timer-runtime',
  'no-provider-delivery-execution',
  'no-provider-receipt-ingestion',
  'no-provider-credentials',
  'no-cloud-routing',
  'no-child-delivery',
  'no-retry-worker-runtime',
  'no-production-durable-outbox-storage',
  'no-adapter-dispatch',
  'no-platform-enforcement',
  'no-raw-private-source-rows',
] as const;

export const AppGameChildUxLocalOutboxPreferenceStatusHandoffNonClaimSchema = withParser(
  Schema.Literal(...RequiredAppGameChildUxLocalOutboxPreferenceStatusHandoffNonClaims)
);
export const AppGameChildUxLocalOutboxPreferenceStatusHandoffIdSchema = brandedNonEmptyStringSchema('AppGameChildUxLocalOutboxPreferenceStatusHandoffId');
export const AppGameChildUxLocalOutboxPreferenceStatusHandoffReferenceSchema = brandedNonEmptyStringSchema('AppGameChildUxLocalOutboxPreferenceStatusHandoffReference');

const AppGameChildUxLocalOutboxPreferenceStatusHandoffRowBaseSchema = Schema.Struct({
  handoffRowId: AppGameChildUxLocalOutboxPreferenceStatusHandoffReferenceSchema,
  sourcePreferenceRowId: AppGameChildUxLocalOutboxPreferenceStatusHandoffReferenceSchema,
  sourcePreferenceStatus: AppGameChildUxLocalOutboxPreferencePreflightStatusSchema,
  sourceSchedulerEntryRef: Schema.Union(AppGameChildUxLocalOutboxPreferenceStatusHandoffReferenceSchema, Schema.Null),
  sourceOutboxRecordRef: Schema.Union(AppGameChildUxLocalOutboxPreferenceStatusHandoffReferenceSchema, Schema.Null),
  sourceProviderChannelRef: Schema.Union(AppGameChildUxLocalOutboxPreferenceStatusHandoffReferenceSchema, Schema.Null),
  sourceReasonCodeRef: Schema.Union(AppGameChildUxLocalOutboxPreferenceStatusHandoffReferenceSchema, Schema.Null),
  sourceParentPreferenceState: Schema.Union(NonEmptyStringSchema, Schema.Null),
  sourceQuietHoursDecision: Schema.Union(NonEmptyStringSchema, Schema.Null),
  sourceParentPreferenceRequirementRefs: Schema.Array(AppGameChildUxLocalOutboxPreferenceStatusHandoffReferenceSchema),
  sourceQuietHoursRequirementRefs: Schema.Array(AppGameChildUxLocalOutboxPreferenceStatusHandoffReferenceSchema),
  notificationPreferenceStatusEntry: V3NotificationRuleProviderRetryContractEntrySchema,
  manualProofRequirements: Schema.Array(AppGameChildUxLocalOutboxPreferenceStatusHandoffReferenceSchema),
});

export const AppGameChildUxLocalOutboxPreferenceStatusHandoffRowSchema = withParser(
  AppGameChildUxLocalOutboxPreferenceStatusHandoffRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        childUxPreferenceStatusHandoffRowIsHonest(row) ||
        'Expected child UX preference status handoff rows to map preference preflight rows into V3 preference and quiet-hours status entries without claiming delivery or parent UI mutation'
    )
  )
);

const AppGameChildUxLocalOutboxPreferenceStatusHandoffReadModelBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  handoffId: AppGameChildUxLocalOutboxPreferenceStatusHandoffIdSchema,
  generatedAt: ParentTimestampSchema,
  family: FamilyReferenceSchema,
  sourcePreferencePreflightId: AppGameChildUxLocalOutboxPreferenceStatusHandoffReferenceSchema,
  sourceContractRefs: Schema.Array(AppGameChildUxLocalOutboxPreferenceStatusHandoffReferenceSchema),
  notificationRuleProviderRetryReadModelRef: AppGameChildUxLocalOutboxPreferenceStatusHandoffReferenceSchema,
  notificationRuleProviderRetryCoverageRefs: Schema.Array(
    AppGameChildUxLocalOutboxPreferenceStatusHandoffReferenceSchema
  ),
  rows: Schema.Array(AppGameChildUxLocalOutboxPreferenceStatusHandoffRowSchema),
  parentPreferenceManualSetupRequiredCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  quietHoursManualRequiredCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  preferenceStatusUnavailableCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  handoffNonClaims: Schema.Array(AppGameChildUxLocalOutboxPreferenceStatusHandoffNonClaimSchema),
  parentPreferenceUiClaimed: Schema.Literal(false),
  parentFrequencyControlUiClaimed: Schema.Literal(false),
  parentNotificationUiClaimed: Schema.Literal(false),
  parentPreferenceMutationClaimed: Schema.Literal(false),
  quietHoursTimerRuntimeClaimed: Schema.Literal(false),
  providerDeliveryRuntimeClaimed: Schema.Literal(false),
  providerReceiptIngestionClaimed: Schema.Literal(false),
  providerCredentialsClaimed: Schema.Literal(false),
  cloudRoutingClaimed: Schema.Literal(false),
  childDeliveryClaimed: Schema.Literal(false),
  retryExecutionRuntimeClaimed: Schema.Literal(false),
  productionDurableOutboxStorageClaimed: Schema.Literal(false),
  adapterDispatchClaimed: Schema.Literal(false),
  platformEnforcementClaimed: Schema.Literal(false),
  rawPrivateSourceRowsIncluded: Schema.Literal(false),
});

export const AppGameChildUxLocalOutboxPreferenceStatusHandoffReadModelSchema = withParser(
  AppGameChildUxLocalOutboxPreferenceStatusHandoffReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        childUxPreferenceStatusHandoffReadModelIsHonest(readModel) ||
        'Expected child UX preference status handoff counts and non-claims to match V3 notification preference status rows'
    )
  )
);

export type AppGameChildUxLocalOutboxPreferenceStatusHandoffRow = Infer<
  typeof AppGameChildUxLocalOutboxPreferenceStatusHandoffRowSchema
>;
export type AppGameChildUxLocalOutboxPreferenceStatusHandoffReadModel = Infer<
  typeof AppGameChildUxLocalOutboxPreferenceStatusHandoffReadModelSchema
>;

type PreferenceStatusHandoffRowInput = Infer<typeof AppGameChildUxLocalOutboxPreferenceStatusHandoffRowBaseSchema>;
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
  status: AppGameChildUxLocalOutboxPreferencePreflightStatus
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

function minimalProviderPayloadBoundaryFor(status: AppGameChildUxLocalOutboxPreferencePreflightStatus): string {
  return status === AppGameChildUxLocalOutboxPreferencePreflightStatus.Unavailable
    ? 'Unavailable child UX preference row records a disabled status only; no provider payload is sent.'
    : 'Manual child UX preference row records parent preference and quiet-hours setup requirements before any provider payload can be sent.';
}

function childUxPreferenceStatusHandoffRowIsHonest(row: PreferenceStatusHandoffRowInput): boolean {
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
  const entry = row.notificationPreferenceStatusEntry;

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

function childUxPreferenceStatusHandoffReadModelIsHonest(
  readModel: Infer<typeof AppGameChildUxLocalOutboxPreferenceStatusHandoffReadModelBaseSchema>
): boolean {
  return (
    readModel.parentPreferenceManualSetupRequiredCount ===
      countParentPreferenceState(readModel.rows, 'manual-setup-required') &&
    readModel.quietHoursManualRequiredCount === countQuietHoursDecision(readModel.rows, 'manual-required') &&
    readModel.preferenceStatusUnavailableCount ===
      countSourceStatus(readModel.rows, AppGameChildUxLocalOutboxPreferencePreflightStatus.Unavailable) &&
    RequiredAppGameChildUxLocalOutboxPreferenceStatusHandoffNonClaims.every((claim) =>
      readModel.handoffNonClaims.includes(claim)
    ) &&
    readModel.notificationRuleProviderRetryCoverageRefs.length ===
      V3NotificationRuleProviderRetryContractReadModel.entries.length &&
    !readModel.parentPreferenceUiClaimed &&
    !readModel.parentFrequencyControlUiClaimed &&
    !readModel.parentNotificationUiClaimed &&
    !readModel.parentPreferenceMutationClaimed &&
    !readModel.quietHoursTimerRuntimeClaimed &&
    !readModel.providerDeliveryRuntimeClaimed &&
    !readModel.providerReceiptIngestionClaimed &&
    !readModel.providerCredentialsClaimed &&
    !readModel.cloudRoutingClaimed &&
    !readModel.childDeliveryClaimed &&
    !readModel.retryExecutionRuntimeClaimed &&
    !readModel.productionDurableOutboxStorageClaimed &&
    !readModel.adapterDispatchClaimed &&
    !readModel.platformEnforcementClaimed &&
    !readModel.rawPrivateSourceRowsIncluded
  );
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
  rows: ReadonlyArray<{ readonly sourcePreferenceStatus: AppGameChildUxLocalOutboxPreferencePreflightStatus }>,
  status: AppGameChildUxLocalOutboxPreferencePreflightStatus
): number => rows.filter((row) => row.sourcePreferenceStatus === status).length;


