import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import {
  AppGameChildUxLocalHandoffArtifactReadModelSchema,
  AppGameChildUxLocalHandoffArtifactRecordSchema,
  type AppGameChildUxLocalHandoffArtifactReadModel,
  type AppGameChildUxLocalHandoffArtifactRecord,
} from './app-game-child-facing-ux-local-handoff';
import { AppGameChildUxSurfaceState } from './app-game-child-facing-ux-rules';
import {
  NotificationLocalOutboxRecordSchema,
  type NotificationLocalOutboxRecord,
} from '@ocentra-parent/notification-domain/notification-local-outbox-adapter-proof';
import { FamilyReferenceSchema, type FamilyReference, type ParentActionReference } from '@ocentra-parent/family-domain/references';
import {
  ParentContractSchemaVersion,
  ParentContractSchemaVersionSchema,
  ParentTimestampSchema,
} from '@ocentra-parent/schema-domain/family-reference-primitives';
import { V3NotificationRuleReasonCodeSchema } from '@ocentra-parent/notification-domain/v3-notification-rule-provider-retry-contract';

export const AppGameChildUxLocalOutboxBridgeStatus = {
  Linked: 'linked-local-outbox-record',
  ManualRequired: 'manual-required',
  Unavailable: 'unavailable',
} as const;

export const AppGameChildUxLocalOutboxBridgeStatusSchema = withParser(
  Schema.Literal(...Object.values(AppGameChildUxLocalOutboxBridgeStatus))
);
export const AppGameChildUxLocalOutboxBridgeIdSchema = brandedNonEmptyStringSchema('AppGameChildUxLocalOutboxBridgeId');
export const AppGameChildUxLocalOutboxBridgeReferenceSchema = brandedNonEmptyStringSchema('AppGameChildUxLocalOutboxBridgeReference');

const AppGameChildUxLocalOutboxBridgeRowBaseSchema = Schema.Struct({
  bridgeRecordId: AppGameChildUxLocalOutboxBridgeReferenceSchema,
  status: AppGameChildUxLocalOutboxBridgeStatusSchema,
  sourceArtifactRecord: AppGameChildUxLocalHandoffArtifactRecordSchema,
  outboxRecord: Schema.Union(NotificationLocalOutboxRecordSchema, Schema.Null),
  blockedReasonRefs: Schema.Array(AppGameChildUxLocalOutboxBridgeReferenceSchema),
});

export const AppGameChildUxLocalOutboxBridgeRowSchema = withParser(
  AppGameChildUxLocalOutboxBridgeRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        appGameChildUxLocalOutboxBridgeRowIsHonest(row) ||
        'Expected child UX local outbox bridge rows to queue only deliverable local artifacts and keep manual or unavailable states out of queued records'
    )
  )
);

const AppGameChildUxLocalOutboxBridgeReadModelBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  bridgeId: AppGameChildUxLocalOutboxBridgeIdSchema,
  generatedAt: ParentTimestampSchema,
  family: FamilyReferenceSchema,
  sourceArtifactReadModel: AppGameChildUxLocalHandoffArtifactReadModelSchema,
  outboxRootRef: AppGameChildUxLocalOutboxBridgeReferenceSchema,
  rows: Schema.Array(AppGameChildUxLocalOutboxBridgeRowSchema),
  linkedRecordCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  manualRequiredCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  unavailableCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  childDeliveryRuntimeClaimed: Schema.Literal(false),
  providerDeliveryRuntimeClaimed: Schema.Literal(false),
  providerReceiptIngestionClaimed: Schema.Literal(false),
  schedulerRuntimeClaimed: Schema.Literal(false),
  cloudRoutingClaimed: Schema.Literal(false),
  parentNotificationUiClaimed: Schema.Literal(false),
  adapterDispatchClaimed: Schema.Literal(false),
  platformEnforcementClaimed: Schema.Literal(false),
  rawPrivateSourceRowsIncluded: Schema.Literal(false),
});

export const AppGameChildUxLocalOutboxBridgeReadModelSchema = withParser(
  AppGameChildUxLocalOutboxBridgeReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        appGameChildUxLocalOutboxBridgeReadModelCountsMatch(readModel) ||
        'Expected child UX local outbox bridge counts and no-claim flags to match linked manual-required and unavailable rows'
    )
  )
);

export type AppGameChildUxLocalOutboxBridgeStatus = Infer<typeof AppGameChildUxLocalOutboxBridgeStatusSchema>;
export type AppGameChildUxLocalOutboxBridgeRow = Infer<typeof AppGameChildUxLocalOutboxBridgeRowSchema>;
export type AppGameChildUxLocalOutboxBridgeReadModel = Infer<typeof AppGameChildUxLocalOutboxBridgeReadModelSchema>;

export type AppGameChildUxLocalOutboxBridgeOptions = {
  readonly family: FamilyReference;
  readonly parentAction: ParentActionReference;
  readonly generatedAt: string;
  readonly bridgeId: string;
  readonly outboxRootRef: string;
  readonly outboxFileRef: string;
  readonly localDataPathRef: string;
};

export function buildAppGameChildUxLocalOutboxBridgeReadModel(
  options: AppGameChildUxLocalOutboxBridgeOptions,
  artifactReadModel: AppGameChildUxLocalHandoffArtifactReadModel
): AppGameChildUxLocalOutboxBridgeReadModel {
  const source = AppGameChildUxLocalHandoffArtifactReadModelSchema.parse(artifactReadModel);
  const rows = source.records.map((record) => appGameChildUxLocalArtifactToOutboxBridgeRow(options, record));

  return AppGameChildUxLocalOutboxBridgeReadModelSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    bridgeId: options.bridgeId,
    generatedAt: options.generatedAt,
    family: options.family,
    sourceArtifactReadModel: source,
    outboxRootRef: options.outboxRootRef,
    rows,
    linkedRecordCount: countRows(rows, AppGameChildUxLocalOutboxBridgeStatus.Linked),
    manualRequiredCount: countRows(rows, AppGameChildUxLocalOutboxBridgeStatus.ManualRequired),
    unavailableCount: countRows(rows, AppGameChildUxLocalOutboxBridgeStatus.Unavailable),
    childDeliveryRuntimeClaimed: false,
    providerDeliveryRuntimeClaimed: false,
    providerReceiptIngestionClaimed: false,
    schedulerRuntimeClaimed: false,
    cloudRoutingClaimed: false,
    parentNotificationUiClaimed: false,
    adapterDispatchClaimed: false,
    platformEnforcementClaimed: false,
    rawPrivateSourceRowsIncluded: false,
  });
}

export function appGameChildUxLocalArtifactToOutboxBridgeRow(
  options: AppGameChildUxLocalOutboxBridgeOptions,
  candidate: AppGameChildUxLocalHandoffArtifactRecord
): AppGameChildUxLocalOutboxBridgeRow {
  const sourceArtifactRecord = AppGameChildUxLocalHandoffArtifactRecordSchema.parse(candidate);
  const status = bridgeStatusForArtifact(sourceArtifactRecord);

  return AppGameChildUxLocalOutboxBridgeRowSchema.parse({
    bridgeRecordId: `app-game-child-ux-local-outbox-bridge-${sourceArtifactRecord.recordId}`,
    status,
    sourceArtifactRecord,
    outboxRecord:
      status === AppGameChildUxLocalOutboxBridgeStatus.Linked
        ? localOutboxRecordForArtifact(options, sourceArtifactRecord)
        : null,
    blockedReasonRefs:
      status === AppGameChildUxLocalOutboxBridgeStatus.Linked
        ? []
        : [`app-game-child-ux-local-outbox-blocked-${sourceArtifactRecord.recordId}`],
  });
}

export function serializeAppGameChildUxLocalOutboxJsonl(readModel: AppGameChildUxLocalOutboxBridgeReadModel): string {
  return `${readModel.rows
    .flatMap((row) => (row.outboxRecord === null ? [] : [row.outboxRecord]))
    .map((record) => JSON.stringify(record))
    .join('\n')}\n`;
}

export function parseAppGameChildUxLocalOutboxJsonl(jsonl: string): NotificationLocalOutboxRecord[] {
  return jsonl
    .split('\n')
    .filter((line) => line.trim().length > 0)
    .map((line) => NotificationLocalOutboxRecordSchema.parse(JSON.parse(line)));
}

function bridgeStatusForArtifact(
  artifact: AppGameChildUxLocalHandoffArtifactRecord
): AppGameChildUxLocalOutboxBridgeStatus {
  if (artifact.card.surfaceState === AppGameChildUxSurfaceState.Unavailable) {
    return AppGameChildUxLocalOutboxBridgeStatus.Unavailable;
  }
  if (artifact.card.surfaceState === AppGameChildUxSurfaceState.ManualRequired) {
    return AppGameChildUxLocalOutboxBridgeStatus.ManualRequired;
  }
  return AppGameChildUxLocalOutboxBridgeStatus.Linked;
}

function localOutboxRecordForArtifact(
  options: AppGameChildUxLocalOutboxBridgeOptions,
  artifact: AppGameChildUxLocalHandoffArtifactRecord
): NotificationLocalOutboxRecord {
  return NotificationLocalOutboxRecordSchema.parse({
    entryId: `app-game-child-ux-local-outbox-${artifact.recordId}`,
    state: 'queued-local',
    envelope: {
      alertRef: `app-game-child-ux-local-outbox-alert-${artifact.recordId}`,
      family: options.family,
      device: artifact.card.device,
      parentAction: options.parentAction,
      severity: childUxSeverity(artifact),
      reasonCode: childUxReasonCode(artifact),
      providerChannel: 'in-app',
      evidenceRefs: artifact.card.evidenceReferences,
      policyRefs: artifact.childReasonReferences,
      auditRefs: artifact.childStatusReferences,
      payloadTemplateRef: `app-game-child-ux-local-outbox-template-${artifact.card.surfaceState}`,
      providerPayloadPreview:
        'child UX local outbox payload carries alert id, family device scope, severity, reason code, evidence ref, policy ref, parent action link, and child UX copy token refs',
      sensitiveDetailMinimized: true,
      rawChildEvidenceIncluded: false,
      rawUrlOrTitleIncluded: false,
      rawMessageTextIncluded: false,
      screenshotOrReportIncluded: false,
    },
    outboxFileRef: options.outboxFileRef,
    localDataPathRef: options.localDataPathRef,
    deliveryClaimState: 'local-outbox-only',
    visibleAfterAt: null,
    retryAttemptCount: 0,
    quietHoursRef: null,
    retryPolicyRef: null,
    deadLetterRef: null,
    providerReceiptRef: null,
    manualProofRequirements: [],
    manualActionRequired: false,
    providerDeliveryAttempted: false,
    providerDeliveryObserved: false,
    providerReceiptIngested: false,
    providerCredentialsStored: false,
    cloudRoutingClaimed: false,
    parentNotificationUiClaimed: false,
    sensitiveProviderMetadataStored: false,
  });
}

function childUxSeverity(artifact: AppGameChildUxLocalHandoffArtifactRecord) {
  if (artifact.card.surfaceState === AppGameChildUxSurfaceState.TimeLimitReached) {
    return 'urgent';
  }
  if (
    artifact.card.surfaceState === AppGameChildUxSurfaceState.FamilyRuleWarning ||
    artifact.card.surfaceState === AppGameChildUxSurfaceState.NewAppApprovalNeeded ||
    artifact.card.surfaceState === AppGameChildUxSurfaceState.TimeAlmostFinished
  ) {
    return 'attention';
  }
  return 'info';
}

function childUxReasonCode(artifact: AppGameChildUxLocalHandoffArtifactRecord) {
  if (
    artifact.card.surfaceState === AppGameChildUxSurfaceState.TimeLimitReached ||
    artifact.card.surfaceState === AppGameChildUxSurfaceState.TimeAlmostFinished ||
    artifact.card.surfaceState === AppGameChildUxSurfaceState.FamilyRuleWarning
  ) {
    return V3NotificationRuleReasonCodeSchema.parse('policy-violation');
  }
  return V3NotificationRuleReasonCodeSchema.parse('parent-request');
}

function appGameChildUxLocalOutboxBridgeRowIsHonest(
  row: Infer<typeof AppGameChildUxLocalOutboxBridgeRowBaseSchema>
): boolean {
  if (row.status === AppGameChildUxLocalOutboxBridgeStatus.Linked) {
    return (
      row.outboxRecord !== null &&
      String(row.outboxRecord.entryId).startsWith('app-game-child-ux-local-outbox-') &&
      row.blockedReasonRefs.length === 0 &&
      !row.sourceArtifactRecord.notificationDeliveryClaimed &&
      !row.sourceArtifactRecord.childDeliveryRuntimeClaimed
    );
  }
  return row.outboxRecord === null && row.blockedReasonRefs.length > 0;
}

function appGameChildUxLocalOutboxBridgeReadModelCountsMatch(
  readModel: Infer<typeof AppGameChildUxLocalOutboxBridgeReadModelBaseSchema>
): boolean {
  return (
    readModel.linkedRecordCount === countRows(readModel.rows, AppGameChildUxLocalOutboxBridgeStatus.Linked) &&
    readModel.manualRequiredCount === countRows(readModel.rows, AppGameChildUxLocalOutboxBridgeStatus.ManualRequired) &&
    readModel.unavailableCount === countRows(readModel.rows, AppGameChildUxLocalOutboxBridgeStatus.Unavailable) &&
    !readModel.childDeliveryRuntimeClaimed &&
    !readModel.providerDeliveryRuntimeClaimed &&
    !readModel.providerReceiptIngestionClaimed &&
    !readModel.schedulerRuntimeClaimed &&
    !readModel.cloudRoutingClaimed &&
    !readModel.parentNotificationUiClaimed &&
    !readModel.adapterDispatchClaimed &&
    !readModel.platformEnforcementClaimed &&
    !readModel.rawPrivateSourceRowsIncluded
  );
}

function countRows(
  rows: ReadonlyArray<{ readonly status: AppGameChildUxLocalOutboxBridgeStatus }>,
  status: AppGameChildUxLocalOutboxBridgeStatus
): number {
  return rows.filter((row) => row.status === status).length;
}

