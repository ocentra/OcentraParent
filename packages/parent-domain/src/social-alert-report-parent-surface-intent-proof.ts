import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { SocialAlertReportProviderPreflightStatusSchema } from './social-alert-report-provider-preflight-proof';
import {
  SocialAlertReportProviderStatusHandoffReadModelSchema,
  type SocialAlertReportProviderStatusHandoffReadModel,
  type SocialAlertReportProviderStatusHandoffRow,
} from './social-alert-report-provider-status-handoff-proof';
import {
  ParentContractSchemaVersion,
  ParentContractSchemaVersionSchema,
  ParentTimestampSchema,
} from './reference-primitives';
import {
  V08NotificationProviderStatusSchema,
  type V08NotificationProviderStatus,
} from './v0-8-notification-provider-status-boundary';

const SurfaceText = Schema.String.pipe(Schema.minLength(1));

export const RequiredSocialAlertReportParentSurfaceIntentNonClaims = [
  'no-parent-notification-ui-rendered',
  'no-provider-delivery-execution',
  'no-provider-receipt-ingestion',
  'no-provider-credentials',
  'no-cloud-routing',
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
export const SocialAlertReportParentSurfaceIntentIdSchema = SurfaceText.pipe(
  Schema.brand('SocialAlertReportParentSurfaceIntentId')
);
export const SocialAlertReportParentSurfaceIntentReferenceSchema = SurfaceText.pipe(
  Schema.brand('SocialAlertReportParentSurfaceIntentReference')
);

const SocialAlertReportParentSurfaceIntentRowBaseSchema = Schema.Struct({
  surfaceRowId: SocialAlertReportParentSurfaceIntentReferenceSchema,
  sourceProviderHandoffRowId: SocialAlertReportParentSurfaceIntentReferenceSchema,
  sourceIntentRef: SocialAlertReportParentSurfaceIntentReferenceSchema,
  sourceLocalOutboxRecordRef: Schema.Union(SocialAlertReportParentSurfaceIntentReferenceSchema, Schema.Null),
  sourceProviderChannelRef: Schema.Union(SocialAlertReportParentSurfaceIntentReferenceSchema, Schema.Null),
  sourcePreflightStatus: SocialAlertReportProviderPreflightStatusSchema,
  providerStatus: V08NotificationProviderStatusSchema,
  notificationStatusRef: SocialAlertReportParentSurfaceIntentReferenceSchema,
  parentSurfaceStatus: SocialAlertReportParentSurfaceStatusSchema,
  historyVisibility: SocialAlertReportParentSurfaceHistoryVisibilitySchema,
  drillInRefs: Schema.Array(SocialAlertReportParentSurfaceIntentReferenceSchema),
  auditRefs: Schema.Array(SocialAlertReportParentSurfaceIntentReferenceSchema),
  manualProofRequirements: Schema.Array(SocialAlertReportParentSurfaceIntentReferenceSchema),
  minimalSurfacePayloadBoundary: SurfaceText,
  sensitiveDetailIncluded: Schema.Literal(false),
  parentNotificationUiRendered: Schema.Literal(false),
  providerDeliveryClaimed: Schema.Literal(false),
  providerReceiptClaimed: Schema.Literal(false),
  reportDeliveryExecutionClaimed: Schema.Literal(false),
  finalPolicyExecutionClaimed: Schema.Literal(false),
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
  sourceContractRefs: Schema.Array(SocialAlertReportParentSurfaceIntentReferenceSchema),
  rows: Schema.Array(SocialAlertReportParentSurfaceIntentRowSchema),
  manualActionRequiredCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  unavailableVisibleCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  historyVisibleCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  parentSurfaceNonClaims: Schema.Array(SocialAlertReportParentSurfaceIntentNonClaimSchema),
  parentNotificationUiRendered: Schema.Literal(false),
  providerDeliveryRuntimeClaimed: Schema.Literal(false),
  providerReceiptIngestionClaimed: Schema.Literal(false),
  providerCredentialsClaimed: Schema.Literal(false),
  cloudRoutingClaimed: Schema.Literal(false),
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
  providerReadModel: SocialAlertReportProviderStatusHandoffReadModel
): SocialAlertReportParentSurfaceIntentReadModel {
  const parsedProvider = SocialAlertReportProviderStatusHandoffReadModelSchema.parse(providerReadModel);
  const rows = parsedProvider.rows.map(socialParentSurfaceIntentRowForProviderStatusRow);

  return SocialAlertReportParentSurfaceIntentReadModelSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    intentId: options.intentId,
    generatedAt: options.generatedAt,
    sourceProviderStatusHandoffId: parsedProvider.handoffId,
    sourceContractRefs: options.sourceContractRefs,
    rows,
    manualActionRequiredCount: countSurfaceStatus(rows, 'manual-action-required'),
    unavailableVisibleCount: countSurfaceStatus(rows, 'unavailable-visible'),
    historyVisibleCount: rows.length,
    parentSurfaceNonClaims: RequiredSocialAlertReportParentSurfaceIntentNonClaims,
    parentNotificationUiRendered: false,
    providerDeliveryRuntimeClaimed: false,
    providerReceiptIngestionClaimed: false,
    providerCredentialsClaimed: false,
    cloudRoutingClaimed: false,
    reportDeliveryExecutionClaimed: false,
    finalPolicyExecutionClaimed: false,
    connectorNativeRuntimeClaimed: false,
    enforcementClaimed: false,
  });
}

function socialParentSurfaceIntentRowForProviderStatusRow(
  providerRow: SocialAlertReportProviderStatusHandoffRow
): SocialAlertReportParentSurfaceIntentRow {
  const providerEntry = providerRow.providerStatusBoundaryEntry;

  return SocialAlertReportParentSurfaceIntentRowSchema.parse({
    surfaceRowId: `social-alert-report-parent-surface-${providerRow.handoffRowId}`,
    sourceProviderHandoffRowId: providerRow.handoffRowId,
    sourceIntentRef: providerRow.sourceIntentRef,
    sourceLocalOutboxRecordRef: providerRow.sourceLocalOutboxRecordRef,
    sourceProviderChannelRef: providerRow.sourceProviderChannelRef,
    sourcePreflightStatus: providerRow.sourcePreflightStatus,
    providerStatus: providerEntry.providerStatus,
    notificationStatusRef: providerEntry.notificationStatusRef,
    parentSurfaceStatus:
      providerEntry.providerStatus === 'unavailable' ? 'unavailable-visible' : 'manual-action-required',
    historyVisibility: historyVisibilityFor(providerEntry.providerStatus),
    drillInRefs: [providerEntry.notificationStatusRef, ...providerEntry.readinessRefs],
    auditRefs: providerEntry.auditRefs,
    manualProofRequirements: [...providerRow.manualProofRequirements, ...providerEntry.manualProofRequirements],
    minimalSurfacePayloadBoundary:
      'Parent surface intent contains social alert/report status refs, readiness refs, and manual requirements only; sensitive social evidence stays behind authenticated drill-in.',
    sensitiveDetailIncluded: false,
    parentNotificationUiRendered: false,
    providerDeliveryClaimed: false,
    providerReceiptClaimed: false,
    reportDeliveryExecutionClaimed: false,
    finalPolicyExecutionClaimed: false,
    enforcementClaimed: false,
  });
}

function historyVisibilityFor(
  providerStatus: V08NotificationProviderStatus
): ParentSurfaceRowInput['historyVisibility'] {
  return providerStatus === 'unavailable' ? 'unavailable-row-visible' : 'manual-review-only';
}

function socialParentSurfaceRowIsHonest(row: ParentSurfaceRowInput): boolean {
  return (
    row.drillInRefs.length > 0 &&
    row.auditRefs.length > 0 &&
    row.manualProofRequirements.length > 0 &&
    row.sensitiveDetailIncluded === false &&
    row.parentNotificationUiRendered === false &&
    row.providerDeliveryClaimed === false &&
    row.providerReceiptClaimed === false &&
    row.reportDeliveryExecutionClaimed === false &&
    row.finalPolicyExecutionClaimed === false &&
    row.enforcementClaimed === false
  );
}

function socialParentSurfaceReadModelIsHonest(readModel: ParentSurfaceReadModelInput): boolean {
  return (
    readModel.manualActionRequiredCount === countSurfaceStatus(readModel.rows, 'manual-action-required') &&
    readModel.unavailableVisibleCount === countSurfaceStatus(readModel.rows, 'unavailable-visible') &&
    readModel.historyVisibleCount === readModel.rows.length &&
    readModel.parentSurfaceNonClaims.length === RequiredSocialAlertReportParentSurfaceIntentNonClaims.length &&
    readModel.parentNotificationUiRendered === false &&
    readModel.providerDeliveryRuntimeClaimed === false &&
    readModel.providerReceiptIngestionClaimed === false &&
    readModel.reportDeliveryExecutionClaimed === false &&
    readModel.finalPolicyExecutionClaimed === false &&
    readModel.connectorNativeRuntimeClaimed === false &&
    readModel.enforcementClaimed === false
  );
}

function countSurfaceStatus(
  rows: ReadonlyArray<{ readonly parentSurfaceStatus: ParentSurfaceRowInput['parentSurfaceStatus'] }>,
  status: ParentSurfaceRowInput['parentSurfaceStatus']
): number {
  return rows.filter((row) => row.parentSurfaceStatus === status).length;
}
