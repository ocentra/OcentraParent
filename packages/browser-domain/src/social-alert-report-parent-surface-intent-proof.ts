import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema,
  NonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import { SocialAlertReportProviderPreflightStatusSchema } from './social-alert-report-provider-preflight-proof';
import {
  SocialAlertReportPreferenceStatusHandoffReadModelSchema,
  type SocialAlertReportPreferenceStatusHandoffReadModel,
  type SocialAlertReportPreferenceStatusHandoffRow,
} from './social-alert-report-preference-status-handoff';
import {
  SocialAlertReportProviderStatusHandoffReadModelSchema,
  type SocialAlertReportProviderStatusHandoffReadModel,
  type SocialAlertReportProviderStatusHandoffRow,
} from './social-alert-report-provider-status-handoff-proof';
import {
  ParentContractSchemaVersion,
  ParentContractSchemaVersionSchema,
  ParentTimestampSchema,
} from '@ocentra-parent/schema-domain/family-reference-primitives';
import {
  V08NotificationProviderStatusSchema,
  type V08NotificationProviderStatus,
} from '@ocentra-parent/notification-domain/v0-8-notification-provider-status-boundary';
import {
  V3NotificationDeliveryResultStateSchema,
  V3NotificationParentPreferenceStateSchema,
  V3NotificationProviderChannelSchema,
  V3NotificationQuietHoursDecisionSchema,
} from '@ocentra-parent/notification-domain/v3-notification-rule-provider-retry-contract';

export const RequiredSocialAlertReportParentSurfaceIntentNonClaims = [
  'no-parent-notification-ui-rendered',
  'no-parent-notification-preference-ui-rendered',
  'no-parent-frequency-control-ui-rendered',
  'no-parent-notification-history-ui-rendered',
  'no-provider-delivery-execution',
  'no-provider-receipt-ingestion',
  'no-provider-credentials',
  'no-cloud-routing',
  'no-child-delivery',
  'no-quiet-hours-timer-runtime',
  'no-retry-worker-runtime',
  'no-production-durable-outbox-storage',
  'no-adapter-dispatch',
  'no-report-delivery-execution',
  'no-final-policy-execution',
  'no-connector-native-runtime',
  'no-enforcement',
] as const;

export const SocialAlertReportParentSurfaceIntentNonClaimSchema = withParser(
  Schema.Literal(...RequiredSocialAlertReportParentSurfaceIntentNonClaims)
);
export const SocialAlertReportParentSurfaceStatusSchema = withParser(
  Schema.Literal('manual-action-required', 'unavailable-visible')
);
export const SocialAlertReportParentSurfaceHistoryVisibilitySchema = withParser(
  Schema.Literal('history-row-visible', 'manual-review-only', 'unavailable-row-visible')
);
export const SocialAlertReportParentSurfacePreferenceVisibilitySchema = withParser(
  Schema.Literal('preference-setup-required', 'preference-disabled-visible')
);
export const SocialAlertReportParentSurfaceIntentIdSchema = brandedNonEmptyStringSchema('SocialAlertReportParentSurfaceIntentId');
export const SocialAlertReportParentSurfaceIntentReferenceSchema = brandedNonEmptyStringSchema('SocialAlertReportParentSurfaceIntentReference');

const SocialAlertReportParentSurfaceIntentRowBaseSchema = Schema.Struct({
  surfaceRowId: SocialAlertReportParentSurfaceIntentReferenceSchema,
  sourceProviderHandoffRowId: SocialAlertReportParentSurfaceIntentReferenceSchema,
  sourcePreferenceHandoffRowId: SocialAlertReportParentSurfaceIntentReferenceSchema,
  sourceIntentRef: SocialAlertReportParentSurfaceIntentReferenceSchema,
  sourceLocalOutboxRecordRef: Schema.Union(SocialAlertReportParentSurfaceIntentReferenceSchema, Schema.Null),
  sourceProviderChannelRef: Schema.Union(SocialAlertReportParentSurfaceIntentReferenceSchema, Schema.Null),
  sourceSchedulerEntryRef: Schema.Union(SocialAlertReportParentSurfaceIntentReferenceSchema, Schema.Null),
  sourcePreferenceStatusRef: SocialAlertReportParentSurfaceIntentReferenceSchema,
  sourcePreflightStatus: SocialAlertReportProviderPreflightStatusSchema,
  providerStatus: V08NotificationProviderStatusSchema,
  deliveryResultState: V3NotificationDeliveryResultStateSchema,
  parentPreferenceState: V3NotificationParentPreferenceStateSchema,
  quietHoursDecision: V3NotificationQuietHoursDecisionSchema,
  providerChannel: V3NotificationProviderChannelSchema,
  notificationStatusRef: SocialAlertReportParentSurfaceIntentReferenceSchema,
  parentSurfaceStatus: SocialAlertReportParentSurfaceStatusSchema,
  historyVisibility: SocialAlertReportParentSurfaceHistoryVisibilitySchema,
  preferenceVisibility: SocialAlertReportParentSurfacePreferenceVisibilitySchema,
  drillInRefs: Schema.Array(SocialAlertReportParentSurfaceIntentReferenceSchema),
  auditRefs: Schema.Array(SocialAlertReportParentSurfaceIntentReferenceSchema),
  manualProofRequirements: Schema.Array(SocialAlertReportParentSurfaceIntentReferenceSchema),
  minimalSurfacePayloadBoundary: NonEmptyStringSchema,
  sensitiveDetailIncluded: Schema.Literal(false),
  parentNotificationUiRendered: Schema.Literal(false),
  parentNotificationPreferenceUiRendered: Schema.Literal(false),
  parentFrequencyControlUiRendered: Schema.Literal(false),
  parentNotificationHistoryUiRendered: Schema.Literal(false),
  providerDeliveryClaimed: Schema.Literal(false),
  providerReceiptClaimed: Schema.Literal(false),
  parentPreferenceMutationClaimed: Schema.Literal(false),
  childDeliveryClaimed: Schema.Literal(false),
  quietHoursTimerRuntimeClaimed: Schema.Literal(false),
  reportDeliveryExecutionClaimed: Schema.Literal(false),
  finalPolicyExecutionClaimed: Schema.Literal(false),
  adapterDispatchClaimed: Schema.Literal(false),
  enforcementClaimed: Schema.Literal(false),
});

export const SocialAlertReportParentSurfaceIntentRowSchema = withParser(
  SocialAlertReportParentSurfaceIntentRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        socialParentSurfaceRowIsHonest(row) ||
        'Expected social alert/report parent-surface rows to expose manual/unavailable refs without UI, provider delivery, report delivery, policy, or enforcement claims'
    )
  )
);

const SocialAlertReportParentSurfaceIntentReadModelBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  intentId: SocialAlertReportParentSurfaceIntentIdSchema,
  generatedAt: ParentTimestampSchema,
  sourceProviderStatusHandoffId: SocialAlertReportParentSurfaceIntentReferenceSchema,
  sourcePreferenceStatusHandoffId: SocialAlertReportParentSurfaceIntentReferenceSchema,
  sourceContractRefs: Schema.Array(SocialAlertReportParentSurfaceIntentReferenceSchema),
  rows: Schema.Array(SocialAlertReportParentSurfaceIntentRowSchema),
  manualActionRequiredCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  unavailableVisibleCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  historyVisibleCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  preferenceSetupRequiredCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  parentSurfaceNonClaims: Schema.Array(SocialAlertReportParentSurfaceIntentNonClaimSchema),
  parentNotificationUiRendered: Schema.Literal(false),
  parentNotificationPreferenceUiRendered: Schema.Literal(false),
  parentFrequencyControlUiRendered: Schema.Literal(false),
  parentNotificationHistoryUiRendered: Schema.Literal(false),
  providerDeliveryRuntimeClaimed: Schema.Literal(false),
  providerReceiptIngestionClaimed: Schema.Literal(false),
  providerCredentialsClaimed: Schema.Literal(false),
  cloudRoutingClaimed: Schema.Literal(false),
  childDeliveryClaimed: Schema.Literal(false),
  quietHoursTimerRuntimeClaimed: Schema.Literal(false),
  retryExecutionRuntimeClaimed: Schema.Literal(false),
  productionDurableOutboxStorageClaimed: Schema.Literal(false),
  adapterDispatchClaimed: Schema.Literal(false),
  reportDeliveryExecutionClaimed: Schema.Literal(false),
  finalPolicyExecutionClaimed: Schema.Literal(false),
  connectorNativeRuntimeClaimed: Schema.Literal(false),
  enforcementClaimed: Schema.Literal(false),
});

export const SocialAlertReportParentSurfaceIntentReadModelSchema = withParser(
  SocialAlertReportParentSurfaceIntentReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        socialParentSurfaceReadModelIsHonest(readModel) ||
        'Expected social alert/report parent-surface counts and non-claims to match row state'
    )
  )
);

export type SocialAlertReportParentSurfaceIntentRow = Infer<typeof SocialAlertReportParentSurfaceIntentRowSchema>;
export type SocialAlertReportParentSurfaceIntentReadModel = Infer<
  typeof SocialAlertReportParentSurfaceIntentReadModelSchema
>;

type ParentSurfaceRowInput = Infer<typeof SocialAlertReportParentSurfaceIntentRowBaseSchema>;
type ParentSurfaceReadModelInput = Infer<typeof SocialAlertReportParentSurfaceIntentReadModelBaseSchema>;

export type SocialAlertReportParentSurfaceIntentOptions = {
  readonly generatedAt: string;
  readonly intentId: string;
  readonly sourceContractRefs: readonly string[];
};

export function buildSocialAlertReportParentSurfaceIntentReadModel(
  options: SocialAlertReportParentSurfaceIntentOptions,
  providerReadModel: SocialAlertReportProviderStatusHandoffReadModel,
  preferenceReadModel: SocialAlertReportPreferenceStatusHandoffReadModel
): SocialAlertReportParentSurfaceIntentReadModel {
  const parsedProvider = SocialAlertReportProviderStatusHandoffReadModelSchema.parse(providerReadModel);
  const parsedPreference = SocialAlertReportPreferenceStatusHandoffReadModelSchema.parse(preferenceReadModel);
  assertCompatibleInputs(parsedProvider, parsedPreference);
  const rows = parsedProvider.rows.map((providerRow, index) =>
    socialParentSurfaceIntentRowForStatusRows(providerRow, preferenceRowAt(parsedPreference, index))
  );

  return SocialAlertReportParentSurfaceIntentReadModelSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    intentId: options.intentId,
    generatedAt: options.generatedAt,
    sourceProviderStatusHandoffId: parsedProvider.handoffId,
    sourcePreferenceStatusHandoffId: parsedPreference.handoffId,
    sourceContractRefs: options.sourceContractRefs,
    rows,
    manualActionRequiredCount: countSurfaceStatus(rows, 'manual-action-required'),
    unavailableVisibleCount: countSurfaceStatus(rows, 'unavailable-visible'),
    historyVisibleCount: rows.length,
    preferenceSetupRequiredCount: countPreferenceVisibility(rows, 'preference-setup-required'),
    parentSurfaceNonClaims: RequiredSocialAlertReportParentSurfaceIntentNonClaims,
    parentNotificationUiRendered: false,
    parentNotificationPreferenceUiRendered: false,
    parentFrequencyControlUiRendered: false,
    parentNotificationHistoryUiRendered: false,
    providerDeliveryRuntimeClaimed: false,
    providerReceiptIngestionClaimed: false,
    providerCredentialsClaimed: false,
    cloudRoutingClaimed: false,
    childDeliveryClaimed: false,
    quietHoursTimerRuntimeClaimed: false,
    retryExecutionRuntimeClaimed: false,
    productionDurableOutboxStorageClaimed: false,
    adapterDispatchClaimed: false,
    reportDeliveryExecutionClaimed: false,
    finalPolicyExecutionClaimed: false,
    connectorNativeRuntimeClaimed: false,
    enforcementClaimed: false,
  });
}

function assertCompatibleInputs(
  providerReadModel: SocialAlertReportProviderStatusHandoffReadModel,
  preferenceReadModel: SocialAlertReportPreferenceStatusHandoffReadModel
): void {
  if (providerReadModel.rows.length !== preferenceReadModel.rows.length) {
    throw new Error('Expected social alert/report parent-surface inputs to have matching row counts');
  }
}

function preferenceRowAt(
  preferenceReadModel: SocialAlertReportPreferenceStatusHandoffReadModel,
  index: number
): SocialAlertReportPreferenceStatusHandoffRow {
  const preferenceRow = preferenceReadModel.rows[index];
  if (preferenceRow === undefined) {
    throw new Error('Expected social alert/report parent-surface preference row to exist');
  }
  return preferenceRow;
}

function socialParentSurfaceIntentRowForStatusRows(
  providerRow: SocialAlertReportProviderStatusHandoffRow,
  preferenceRow: SocialAlertReportPreferenceStatusHandoffRow
): SocialAlertReportParentSurfaceIntentRow {
  const providerEntry = providerRow.providerStatusBoundaryEntry;
  const preferenceEntry = preferenceRow.notificationPreferenceStatusEntry;

  return SocialAlertReportParentSurfaceIntentRowSchema.parse({
    surfaceRowId: `social-alert-report-parent-surface-${providerRow.handoffRowId}`,
    sourceProviderHandoffRowId: providerRow.handoffRowId,
    sourcePreferenceHandoffRowId: preferenceRow.handoffRowId,
    sourceIntentRef: providerRow.sourceIntentRef,
    sourceLocalOutboxRecordRef: providerRow.sourceLocalOutboxRecordRef ?? preferenceRow.sourceOutboxRecordRef,
    sourceProviderChannelRef: providerRow.sourceProviderChannelRef ?? preferenceRow.sourceProviderChannelRef,
    sourceSchedulerEntryRef: preferenceRow.sourceSchedulerEntryRef,
    sourcePreferenceStatusRef: preferenceEntry.deliveryResultRef,
    sourcePreflightStatus: providerRow.sourcePreflightStatus,
    providerStatus: providerEntry.providerStatus,
    deliveryResultState: preferenceEntry.deliveryResultState,
    parentPreferenceState: preferenceEntry.parentPreferenceState,
    quietHoursDecision: preferenceEntry.quietHoursDecision,
    providerChannel: preferenceEntry.providerChannel,
    notificationStatusRef: providerEntry.notificationStatusRef,
    parentSurfaceStatus:
      providerEntry.providerStatus === 'unavailable' ? 'unavailable-visible' : 'manual-action-required',
    historyVisibility: historyVisibilityFor(providerEntry.providerStatus),
    preferenceVisibility:
      preferenceEntry.parentPreferenceState === 'channel-disabled'
        ? 'preference-disabled-visible'
        : 'preference-setup-required',
    drillInRefs: [providerEntry.notificationStatusRef, preferenceEntry.deliveryResultRef],
    auditRefs: [...providerEntry.auditRefs, ...preferenceEntry.auditRefs],
    manualProofRequirements: [
      ...providerRow.manualProofRequirements,
      ...providerEntry.manualProofRequirements,
      ...preferenceRow.manualProofRequirements,
      ...preferenceEntry.manualProofRequirements,
    ],
    minimalSurfacePayloadBoundary:
      'Parent surface intent contains social alert/report provider status, preference status, quiet-hours status, and manual requirements only; sensitive social evidence stays behind authenticated drill-in.',
    sensitiveDetailIncluded: false,
    parentNotificationUiRendered: false,
    parentNotificationPreferenceUiRendered: false,
    parentFrequencyControlUiRendered: false,
    parentNotificationHistoryUiRendered: false,
    providerDeliveryClaimed: false,
    providerReceiptClaimed: false,
    parentPreferenceMutationClaimed: false,
    childDeliveryClaimed: false,
    quietHoursTimerRuntimeClaimed: false,
    reportDeliveryExecutionClaimed: false,
    finalPolicyExecutionClaimed: false,
    adapterDispatchClaimed: false,
    enforcementClaimed: false,
  });
}

function historyVisibilityFor(
  providerStatus: V08NotificationProviderStatus
): ParentSurfaceRowInput['historyVisibility'] {
  return providerStatus === 'unavailable' ? 'unavailable-row-visible' : 'manual-review-only';
}

function socialParentSurfaceRowIsHonest(row: ParentSurfaceRowInput): boolean {
  return socialParentSurfaceRowHasRequiredRefs(row) && socialParentSurfaceRowKeepsClaimsFalse(row);
}

function socialParentSurfaceRowHasRequiredRefs(row: ParentSurfaceRowInput): boolean {
  return row.drillInRefs.length > 0 && row.auditRefs.length > 0 && row.manualProofRequirements.length > 0;
}

function socialParentSurfaceRowKeepsClaimsFalse(row: ParentSurfaceRowInput): boolean {
  return (
    socialParentSurfaceRowKeepsUiClaimsFalse(row) &&
    socialParentSurfaceRowKeepsDeliveryClaimsFalse(row) &&
    row.sensitiveDetailIncluded === false &&
    row.finalPolicyExecutionClaimed === false &&
    row.adapterDispatchClaimed === false &&
    row.enforcementClaimed === false
  );
}

function socialParentSurfaceRowKeepsUiClaimsFalse(row: ParentSurfaceRowInput): boolean {
  return (
    row.parentNotificationUiRendered === false &&
    row.parentNotificationPreferenceUiRendered === false &&
    row.parentFrequencyControlUiRendered === false &&
    row.parentNotificationHistoryUiRendered === false
  );
}

function socialParentSurfaceRowKeepsDeliveryClaimsFalse(row: ParentSurfaceRowInput): boolean {
  return (
    row.providerDeliveryClaimed === false &&
    row.providerReceiptClaimed === false &&
    row.parentPreferenceMutationClaimed === false &&
    row.childDeliveryClaimed === false &&
    row.quietHoursTimerRuntimeClaimed === false &&
    row.reportDeliveryExecutionClaimed === false
  );
}

function socialParentSurfaceReadModelIsHonest(readModel: ParentSurfaceReadModelInput): boolean {
  return (
    socialParentSurfaceReadModelCountsMatch(readModel) &&
    socialParentSurfaceReadModelKeepsUiClaimsFalse(readModel) &&
    socialParentSurfaceReadModelKeepsRuntimeClaimsFalse(readModel)
  );
}

function socialParentSurfaceReadModelCountsMatch(readModel: ParentSurfaceReadModelInput): boolean {
  return (
    readModel.manualActionRequiredCount === countSurfaceStatus(readModel.rows, 'manual-action-required') &&
    readModel.unavailableVisibleCount === countSurfaceStatus(readModel.rows, 'unavailable-visible') &&
    readModel.historyVisibleCount === readModel.rows.length &&
    readModel.preferenceSetupRequiredCount === countPreferenceVisibility(readModel.rows, 'preference-setup-required') &&
    readModel.parentSurfaceNonClaims.length === RequiredSocialAlertReportParentSurfaceIntentNonClaims.length
  );
}

function socialParentSurfaceReadModelKeepsUiClaimsFalse(readModel: ParentSurfaceReadModelInput): boolean {
  return (
    readModel.parentNotificationUiRendered === false &&
    readModel.parentNotificationPreferenceUiRendered === false &&
    readModel.parentFrequencyControlUiRendered === false &&
    readModel.parentNotificationHistoryUiRendered === false
  );
}

function socialParentSurfaceReadModelKeepsRuntimeClaimsFalse(readModel: ParentSurfaceReadModelInput): boolean {
  return (
    socialParentSurfaceReadModelKeepsDeliveryClaimsFalse(readModel) &&
    readModel.finalPolicyExecutionClaimed === false &&
    readModel.connectorNativeRuntimeClaimed === false &&
    readModel.adapterDispatchClaimed === false &&
    readModel.enforcementClaimed === false
  );
}

function socialParentSurfaceReadModelKeepsDeliveryClaimsFalse(readModel: ParentSurfaceReadModelInput): boolean {
  return (
    readModel.providerDeliveryRuntimeClaimed === false &&
    readModel.providerReceiptIngestionClaimed === false &&
    readModel.childDeliveryClaimed === false &&
    readModel.quietHoursTimerRuntimeClaimed === false &&
    readModel.retryExecutionRuntimeClaimed === false &&
    readModel.productionDurableOutboxStorageClaimed === false &&
    readModel.reportDeliveryExecutionClaimed === false
  );
}

function countSurfaceStatus(
  rows: ReadonlyArray<{ readonly parentSurfaceStatus: ParentSurfaceRowInput['parentSurfaceStatus'] }>,
  status: ParentSurfaceRowInput['parentSurfaceStatus']
): number {
  return rows.filter((row) => row.parentSurfaceStatus === status).length;
}

function countPreferenceVisibility(
  rows: ReadonlyArray<{ readonly preferenceVisibility: ParentSurfaceRowInput['preferenceVisibility'] }>,
  visibility: ParentSurfaceRowInput['preferenceVisibility']
): number {
  return rows.filter((row) => row.preferenceVisibility === visibility).length;
}

