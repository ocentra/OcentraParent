import { type Infer, NonEmptyStringSchema, Schema, withParser, brandedNonEmptyStringSchema } from './effect';
import { FamilyReferenceSchema } from './family-references';
import {
  ParentContractSchemaVersionSchema,
  ParentTimestampSchema,
} from './family-reference-primitives';
import {
  AppGameChildUxLocalOutboxPreferencePreflightStatus,
  AppGameChildUxLocalOutboxPreferencePreflightStatusSchema,
} from './app-game-child-facing-ux-local-outbox-preference-preflight';
import {
  V3NotificationRuleProviderRetryContractEntrySchema,
  V3NotificationRuleProviderRetryContractReadModel,
  type V3NotificationParentPreferenceState,
  type V3NotificationQuietHoursDecision,
} from './notification-v3-provider-retry';

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
export const AppGameChildUxLocalOutboxPreferenceStatusHandoffIdSchema = brandedNonEmptyStringSchema(
  'AppGameChildUxLocalOutboxPreferenceStatusHandoffId'
);
export const AppGameChildUxLocalOutboxPreferenceStatusHandoffReferenceSchema = brandedNonEmptyStringSchema(
  'AppGameChildUxLocalOutboxPreferenceStatusHandoffReference'
);

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

function preferenceStatusExpectationFor(
  status: AppGameChildUxLocalOutboxPreferencePreflightStatus
): PreferenceStatusExpectation {
  if (status === AppGameChildUxLocalOutboxPreferencePreflightStatus.Unavailable) {
    return UnavailablePreferenceStatusExpectation;
  }
  return ManualPreferenceStatusExpectation;
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

export const decodeAppGameChildUxLocalOutboxPreferenceStatusHandoffReadModel = Schema.decodeUnknownSync(
  AppGameChildUxLocalOutboxPreferenceStatusHandoffReadModelSchema
);
