import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  NotificationLocalOutboxRecordSchema,
  type NotificationLocalOutboxRecord,
} from '@ocentra-parent/schema-domain/notification-local-outbox';
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
} from '@ocentra-parent/schema-domain/social-alert-report-provider-receipt-boundary-proof';
import { SocialAlertReportReferenceSchema } from '@ocentra-parent/schema-domain/social-alert-report-intent-values';
import { V3NotificationProviderChannelSchema } from '@ocentra-parent/schema-domain/notification-v3-provider-retry';

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
        socialAlertReportProviderDispatchExecutionReadModelIsHonest(readModel) ||
        'Expected provider dispatch execution counts and non-claims to match source receipt-boundary rows'
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

type DispatchExecutionRowInput = Infer<typeof SocialAlertReportProviderDispatchExecutionRowBaseSchema>;
type DispatchExecutionReadModelInput = Infer<typeof SocialAlertReportProviderDispatchExecutionReadModelBaseSchema>;

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

function localOutboxRecordForReceiptRow(
  row: SocialAlertReportProviderReceiptBoundaryRow,
  localOutboxRecords: ReadonlyArray<NotificationLocalOutboxRecord>
): NotificationLocalOutboxRecord | null {
  if (row.sourceLocalOutboxRecordRef === null) {
    return null;
  }

  return localOutboxRecords.find((record) => String(record.entryId) === String(row.sourceLocalOutboxRecordRef)) ?? null;
}

function dispatchExecutionStateFor(
  row: SocialAlertReportProviderReceiptBoundaryRow,
  outboxRecord: NotificationLocalOutboxRecord | null
): SocialAlertReportProviderDispatchExecutionState {
  if (row.receiptBoundaryState === 'provider-unavailable') {
    return SocialAlertReportProviderDispatchExecutionState.ProviderUnavailable;
  }
  if (row.receiptBoundaryState !== 'provider-dispatch-required' || outboxRecord === null) {
    return SocialAlertReportProviderDispatchExecutionState.ManualRequired;
  }
  return SocialAlertReportProviderDispatchExecutionState.LocalDispatchPacketReady;
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

function manualProofRequirementsFor(
  row: SocialAlertReportProviderReceiptBoundaryRow,
  state: SocialAlertReportProviderDispatchExecutionState
): readonly string[] {
  if (state === SocialAlertReportProviderDispatchExecutionState.LocalDispatchPacketReady) {
    return [];
  }
  if (state === SocialAlertReportProviderDispatchExecutionState.ProviderUnavailable) {
    return [`social-provider-dispatch-provider-unavailable-${row.sourceIntentRef}`];
  }
  if (row.receiptBoundaryState === 'provider-dispatch-required') {
    return [`social-provider-dispatch-local-outbox-record-required-${row.sourceIntentRef}`];
  }
  return row.manualProofRequirements;
}

function socialAlertReportProviderDispatchExecutionRowIsHonest(row: DispatchExecutionRowInput): boolean {
  if (row.dispatchExecutionState === SocialAlertReportProviderDispatchExecutionState.LocalDispatchPacketReady) {
    return dispatchPacketReadyRowIsHonest(row);
  }
  if (row.dispatchExecutionState === SocialAlertReportProviderDispatchExecutionState.ProviderUnavailable) {
    return providerUnavailableRowIsHonest(row);
  }
  return manualRequiredRowIsHonest(row);
}

function dispatchPacketReadyRowIsHonest(row: DispatchExecutionRowInput): boolean {
  return (
    row.sourceReceiptBoundaryState === 'provider-dispatch-required' &&
    row.sourceLocalOutboxRecordRef !== null &&
    row.dispatchPacket !== null &&
    String(row.dispatchPacket.outboxEntryRef) === String(row.sourceLocalOutboxRecordRef) &&
    String(row.dispatchPacket.providerAttemptRef) === String(row.sourceProviderAttemptRef) &&
    row.manualProofRequirements.length === 0 &&
    providerDispatchClaimsStayFalse(row)
  );
}

function manualRequiredRowIsHonest(row: DispatchExecutionRowInput): boolean {
  return row.dispatchPacket === null && row.manualProofRequirements.length > 0 && providerDispatchClaimsStayFalse(row);
}

function providerUnavailableRowIsHonest(row: DispatchExecutionRowInput): boolean {
  return (
    row.sourceReceiptBoundaryState === 'provider-unavailable' &&
    row.dispatchPacket === null &&
    row.manualProofRequirements.length > 0 &&
    providerDispatchClaimsStayFalse(row)
  );
}

function providerDispatchClaimsStayFalse(row: DispatchExecutionRowInput): boolean {
  return [
    row.providerDeliveryAttempted,
    row.providerDeliveryObserved,
    row.providerReceiptIngested,
    row.providerWebhookRuntimeClaimed,
    row.providerCredentialsClaimed,
    row.cloudRoutingClaimed,
    row.parentNotificationUiDeliveryClaimed,
    row.reportDeliveryExecutionClaimed,
    row.finalPolicyExecutionClaimed,
    row.connectorNativeRuntimeClaimed,
    row.enforcementClaimed,
  ].every((claim) => claim === false);
}

function socialAlertReportProviderDispatchExecutionReadModelIsHonest(
  readModel: DispatchExecutionReadModelInput
): boolean {
  const sourceNonClaims: readonly string[] = readModel.sourceReceiptBoundaryNonClaims;

  return (
    readModel.localDispatchPacketReadyCount ===
      countRows(readModel.rows, SocialAlertReportProviderDispatchExecutionState.LocalDispatchPacketReady) &&
    readModel.manualRequiredCount ===
      countRows(readModel.rows, SocialAlertReportProviderDispatchExecutionState.ManualRequired) &&
    readModel.providerUnavailableCount ===
      countRows(readModel.rows, SocialAlertReportProviderDispatchExecutionState.ProviderUnavailable) &&
    RequiredSocialAlertReportProviderReceiptBoundaryNonClaims.every((claim) => sourceNonClaims.includes(claim)) &&
    RequiredSocialAlertReportProviderDispatchExecutionNonClaims.every((claim) =>
      readModel.dispatchExecutionNonClaims.includes(claim)
    ) &&
    readModel.providerDeliveryAttempted === false &&
    readModel.providerDeliveryObserved === false &&
    readModel.providerReceiptIngested === false &&
    readModel.enforcementClaimed === false
  );
}

function countRows(
  rows: ReadonlyArray<{ readonly dispatchExecutionState: SocialAlertReportProviderDispatchExecutionState }>,
  state: SocialAlertReportProviderDispatchExecutionState
): number {
  return rows.filter((row) => row.dispatchExecutionState === state).length;
}
