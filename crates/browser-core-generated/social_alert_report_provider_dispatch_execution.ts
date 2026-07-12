/* generated from crates/browser-core/src/social_alert_report_provider_dispatch_execution.rs */

import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  ParentContractSchemaVersion,
  ParentContractSchemaVersionSchema,
  ParentTimestampSchema,
} from '@ocentra-parent/schema-domain/family-reference-primitives';
import {
  RequiredSocialAlertReportProviderReceiptBoundaryNonClaims,
  SocialAlertReportProviderReceiptBoundaryReadModelSchema,
  type SocialAlertReportProviderReceiptBoundaryReadModel,
  type SocialAlertReportProviderReceiptBoundaryRow,
} from './social_alert_report_provider_receipt_boundary_support';
import { V3NotificationProviderChannelSchema } from '@ocentra-parent/schema-domain/notification-v3-provider-retry';
import {
  NotificationLocalOutboxRecordSchema,
  type NotificationLocalOutboxRecord,
} from './social_alert_report_local_outbox_bridge_support';
import { SocialAlertReportReferenceSchema } from './social_alert_report_provider_dispatch_execution_support';
import {
  countRows,
  dispatchExecutionStateFor,
  localOutboxRecordForReceiptRow,
  manualProofRequirementsFor,
  socialAlertReportProviderDispatchExecutionReadModelIsHonest,
  socialAlertReportProviderDispatchExecutionRowIsHonest,
} from './social_alert_report_provider_dispatch_execution_helpers';

export const RequiredSocialAlertReportProviderDispatchExecutionNonClaims = [
  'no-provider-delivery-observed',
  'no-provider-receipt-ingestion-runtime',
  'no-provider-webhook-runtime',
  'no-provider-credentials',
  'no-cloud-routing',
  'no-parent-notification-ui-delivery',
  'no-report-delivery-execution',
  'no-final-policy-execution',
  'no-connector-native-runtime',
  'no-enforcement',
] as const;

export const SocialAlertReportProviderDispatchExecutionState = {
  LocalDispatchPacketReady: 'local-dispatch-packet-ready',
  ManualRequired: 'manual-required',
  ProviderUnavailable: 'provider-unavailable',
} as const;

export const SocialAlertReportProviderDispatchExecutionNonClaimSchema = withParser(
  Schema.Literal(...RequiredSocialAlertReportProviderDispatchExecutionNonClaims)
);
export const SocialAlertReportProviderDispatchExecutionStateSchema = withParser(
  Schema.Literal(...Object.values(SocialAlertReportProviderDispatchExecutionState))
);

const SocialAlertReportProviderDispatchPacketSchema = withParser(
  Schema.Struct({
    dispatchPacketId: SocialAlertReportReferenceSchema,
    outboxEntryRef: SocialAlertReportReferenceSchema,
    providerAttemptRef: SocialAlertReportReferenceSchema,
    providerChannel: V3NotificationProviderChannelSchema,
    alertRef: SocialAlertReportReferenceSchema,
    familyScopeRef: SocialAlertReportReferenceSchema,
    deviceScopeRef: SocialAlertReportReferenceSchema,
    parentActionLinkRef: Schema.Union(SocialAlertReportReferenceSchema, Schema.Null),
    payloadTemplateRef: SocialAlertReportReferenceSchema,
    evidenceRefs: Schema.Array(SocialAlertReportReferenceSchema),
    policyRefs: Schema.Array(SocialAlertReportReferenceSchema),
    auditRefs: Schema.Array(SocialAlertReportReferenceSchema),
    sensitiveDetailMinimized: Schema.Literal(true),
    rawChildEvidenceIncluded: Schema.Literal(false),
    rawUrlOrTitleIncluded: Schema.Literal(false),
    rawMessageTextIncluded: Schema.Literal(false),
    screenshotOrReportIncluded: Schema.Literal(false),
  })
);

const SocialAlertReportProviderDispatchExecutionRowBaseSchema = Schema.Struct({
  dispatchRowId: SocialAlertReportReferenceSchema,
  sourceReceiptRowRef: SocialAlertReportReferenceSchema,
  sourceIntentRef: SocialAlertReportReferenceSchema,
  sourceProviderAttemptRef: SocialAlertReportReferenceSchema,
  sourceReceiptBoundaryState: Schema.Literal(
    'provider-dispatch-required',
    'manual-receipt-required',
    'provider-unavailable'
  ),
  sourceLocalOutboxRecordRef: Schema.Union(SocialAlertReportReferenceSchema, Schema.Null),
  dispatchExecutionState: SocialAlertReportProviderDispatchExecutionStateSchema,
  dispatchPacket: Schema.Union(SocialAlertReportProviderDispatchPacketSchema, Schema.Null),
  manualProofRequirements: Schema.Array(SocialAlertReportReferenceSchema),
  providerDeliveryAttempted: Schema.Literal(false),
  providerDeliveryObserved: Schema.Literal(false),
  providerReceiptIngested: Schema.Literal(false),
  providerWebhookRuntimeClaimed: Schema.Literal(false),
  providerCredentialsClaimed: Schema.Literal(false),
  cloudRoutingClaimed: Schema.Literal(false),
  parentNotificationUiDeliveryClaimed: Schema.Literal(false),
  reportDeliveryExecutionClaimed: Schema.Literal(false),
  finalPolicyExecutionClaimed: Schema.Literal(false),
  connectorNativeRuntimeClaimed: Schema.Literal(false),
  enforcementClaimed: Schema.Literal(false),
});

export const SocialAlertReportProviderDispatchExecutionRowSchema = withParser(
  SocialAlertReportProviderDispatchExecutionRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        socialAlertReportProviderDispatchExecutionRowIsHonest(row) ||
        'Expected provider dispatch execution rows to prepare only local dispatch packets and keep provider delivery unclaimed'
    )
  )
);

const SocialAlertReportProviderDispatchExecutionReadModelBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  dispatchExecutionId: SocialAlertReportReferenceSchema,
  generatedAt: ParentTimestampSchema,
  sourceReceiptBoundaryId: SocialAlertReportReferenceSchema,
  sourceReceiptBoundaryNonClaims: Schema.Array(SocialAlertReportReferenceSchema),
  rows: Schema.Array(SocialAlertReportProviderDispatchExecutionRowSchema),
  localDispatchPacketReadyCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  manualRequiredCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  providerUnavailableCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  dispatchExecutionNonClaims: Schema.Array(SocialAlertReportProviderDispatchExecutionNonClaimSchema),
  providerDeliveryAttempted: Schema.Literal(false),
  providerDeliveryObserved: Schema.Literal(false),
  providerReceiptIngested: Schema.Literal(false),
  providerWebhookRuntimeClaimed: Schema.Literal(false),
  providerCredentialsClaimed: Schema.Literal(false),
  cloudRoutingClaimed: Schema.Literal(false),
  parentNotificationUiDeliveryClaimed: Schema.Literal(false),
  reportDeliveryExecutionClaimed: Schema.Literal(false),
  finalPolicyExecutionClaimed: Schema.Literal(false),
  connectorNativeRuntimeClaimed: Schema.Literal(false),
  enforcementClaimed: Schema.Literal(false),
});

export const SocialAlertReportProviderDispatchExecutionReadModelSchema = withParser(
  SocialAlertReportProviderDispatchExecutionReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        socialAlertReportProviderDispatchExecutionReadModelIsHonest(
          readModel,
          RequiredSocialAlertReportProviderReceiptBoundaryNonClaims,
          RequiredSocialAlertReportProviderDispatchExecutionNonClaims
        ) || 'Expected provider dispatch execution counts and non-claims to match source receipt-boundary rows'
    )
  )
);

export type SocialAlertReportProviderDispatchPacket = Infer<typeof SocialAlertReportProviderDispatchPacketSchema>;
export type SocialAlertReportProviderDispatchExecutionState = Infer<
  typeof SocialAlertReportProviderDispatchExecutionStateSchema
>;
export type SocialAlertReportProviderDispatchExecutionRow = Infer<
  typeof SocialAlertReportProviderDispatchExecutionRowSchema
>;
export type SocialAlertReportProviderDispatchExecutionReadModel = Infer<
  typeof SocialAlertReportProviderDispatchExecutionReadModelSchema
>;

export type SocialAlertReportProviderDispatchExecutionOptions = {
  readonly generatedAt: string;
  readonly dispatchExecutionId: string;
};

export function buildSocialAlertReportProviderDispatchExecutionReadModel(
  options: SocialAlertReportProviderDispatchExecutionOptions,
  sourceReadModel: SocialAlertReportProviderReceiptBoundaryReadModel,
  localOutboxRecords: ReadonlyArray<NotificationLocalOutboxRecord>
): SocialAlertReportProviderDispatchExecutionReadModel {
  const parsedSource = SocialAlertReportProviderReceiptBoundaryReadModelSchema.parse(sourceReadModel);
  const parsedRecords = localOutboxRecords.map((record) => NotificationLocalOutboxRecordSchema.parse(record));
  const rows = parsedSource.rows.map((row) =>
    socialAlertReportProviderDispatchExecutionRowForReceiptRow(row, parsedRecords)
  );

  return SocialAlertReportProviderDispatchExecutionReadModelSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    dispatchExecutionId: options.dispatchExecutionId,
    generatedAt: options.generatedAt,
    sourceReceiptBoundaryId: parsedSource.receiptBoundaryId,
    sourceReceiptBoundaryNonClaims: RequiredSocialAlertReportProviderReceiptBoundaryNonClaims,
    rows,
    localDispatchPacketReadyCount: countRows(
      rows,
      SocialAlertReportProviderDispatchExecutionState.LocalDispatchPacketReady
    ),
    manualRequiredCount: countRows(rows, SocialAlertReportProviderDispatchExecutionState.ManualRequired),
    providerUnavailableCount: countRows(rows, SocialAlertReportProviderDispatchExecutionState.ProviderUnavailable),
    dispatchExecutionNonClaims: RequiredSocialAlertReportProviderDispatchExecutionNonClaims,
    providerDeliveryAttempted: false,
    providerDeliveryObserved: false,
    providerReceiptIngested: false,
    providerWebhookRuntimeClaimed: false,
    providerCredentialsClaimed: false,
    cloudRoutingClaimed: false,
    parentNotificationUiDeliveryClaimed: false,
    reportDeliveryExecutionClaimed: false,
    finalPolicyExecutionClaimed: false,
    connectorNativeRuntimeClaimed: false,
    enforcementClaimed: false,
  });
}

export function summarizeSocialAlertReportProviderDispatchExecution(
  readModel: SocialAlertReportProviderDispatchExecutionReadModel
) {
  return {
    rows: readModel.rows.length,
    localDispatchPacketReadyCount: readModel.localDispatchPacketReadyCount,
    manualRequiredCount: readModel.manualRequiredCount,
    providerUnavailableCount: readModel.providerUnavailableCount,
    providerDeliveryAttempted: readModel.providerDeliveryAttempted,
    providerDeliveryObserved: readModel.providerDeliveryObserved,
    providerReceiptIngested: readModel.providerReceiptIngested,
    enforcementClaimed: readModel.enforcementClaimed,
  };
}

function socialAlertReportProviderDispatchExecutionRowForReceiptRow(
  row: SocialAlertReportProviderReceiptBoundaryRow,
  localOutboxRecords: ReadonlyArray<NotificationLocalOutboxRecord>
): SocialAlertReportProviderDispatchExecutionRow {
  const outboxRecord = localOutboxRecordForReceiptRow(row, localOutboxRecords);
  const dispatchExecutionState = dispatchExecutionStateFor(row, outboxRecord);

  return SocialAlertReportProviderDispatchExecutionRowSchema.parse({
    dispatchRowId: `social-provider-dispatch-execution-${row.receiptRowId}`,
    sourceReceiptRowRef: row.receiptRowId,
    sourceIntentRef: row.sourceIntentRef,
    sourceProviderAttemptRef: row.providerAttemptRef,
    sourceReceiptBoundaryState: row.receiptBoundaryState,
    sourceLocalOutboxRecordRef: row.sourceLocalOutboxRecordRef,
    dispatchExecutionState,
    dispatchPacket:
      dispatchExecutionState === SocialAlertReportProviderDispatchExecutionState.LocalDispatchPacketReady &&
      outboxRecord !== null
        ? dispatchPacketFor(row, outboxRecord)
        : null,
    manualProofRequirements: manualProofRequirementsFor(row, dispatchExecutionState),
    providerDeliveryAttempted: false,
    providerDeliveryObserved: false,
    providerReceiptIngested: false,
    providerWebhookRuntimeClaimed: false,
    providerCredentialsClaimed: false,
    cloudRoutingClaimed: false,
    parentNotificationUiDeliveryClaimed: false,
    reportDeliveryExecutionClaimed: false,
    finalPolicyExecutionClaimed: false,
    connectorNativeRuntimeClaimed: false,
    enforcementClaimed: false,
  });
}

function dispatchPacketFor(
  row: SocialAlertReportProviderReceiptBoundaryRow,
  record: NotificationLocalOutboxRecord
): SocialAlertReportProviderDispatchPacket {
  return SocialAlertReportProviderDispatchPacketSchema.parse({
    dispatchPacketId: `social-provider-dispatch-packet-${record.entryId}`,
    outboxEntryRef: record.entryId,
    providerAttemptRef: row.providerAttemptRef,
    providerChannel: record.envelope.providerChannel,
    alertRef: record.envelope.alertRef,
    familyScopeRef: `social-provider-dispatch-family-${record.envelope.family.familyId}`,
    deviceScopeRef: `social-provider-dispatch-device-${record.envelope.device.deviceId}`,
    parentActionLinkRef: record.envelope.parentAction?.actionReferenceId ?? null,
    payloadTemplateRef: record.envelope.payloadTemplateRef,
    evidenceRefs: record.envelope.evidenceRefs.map(
      (evidence: NotificationLocalOutboxRecord['envelope']['evidenceRefs'][number]) => evidence.evidenceReferenceId
    ),
    policyRefs: record.envelope.policyRefs,
    auditRefs: record.envelope.auditRefs,
    sensitiveDetailMinimized: true,
    rawChildEvidenceIncluded: false,
    rawUrlOrTitleIncluded: false,
    rawMessageTextIncluded: false,
    screenshotOrReportIncluded: false,
  });
}
