import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  ParentContractSchemaVersion,
  ParentContractSchemaVersionSchema,
  ParentTimestampSchema,
} from './reference-primitives';
import { FamilyReferenceSchema, ParentDeviceReferenceSchema } from './references';

const TrackingRetentionWriterBoundaryProofText = Schema.String.pipe(Schema.minLength(1));
const TrackingRetentionWriterBoundaryCount = Schema.Number.pipe(Schema.int(), Schema.nonNegative());

export const TrackingRetentionWriterBoundarySettingKindSchema = withParser(
  Schema.Literal('retention-window', 'delete-after-alert-resolved', 'parent-export', 'remote-sync', 'remote-ai')
);

export const TrackingRetentionWriterBoundaryStateSchema = withParser(
  Schema.Literal(
    'accepted-for-contract',
    'rejected-invalid-input',
    'manual-service-mutation-required',
    'remote-sync-disabled',
    'remote-ai-disabled'
  )
);

export const TrackingRetentionWriterBoundaryNonClaimSchema = withParser(
  Schema.Literal(
    'no-live-service-mutation',
    'no-platform-retention-writer',
    'no-child-device-delivery',
    'no-provider-delivery',
    'no-notification-receipt',
    'no-remote-sync-runtime',
    'no-remote-ai-runtime',
    'no-portal-settings-ui',
    'no-physical-device-proof',
    'no-product-ready-claim'
  )
);

export const RequiredTrackingRetentionWriterBoundaryNonClaims = [
  'no-live-service-mutation',
  'no-platform-retention-writer',
  'no-child-device-delivery',
  'no-provider-delivery',
  'no-notification-receipt',
  'no-remote-sync-runtime',
  'no-remote-ai-runtime',
  'no-portal-settings-ui',
  'no-physical-device-proof',
  'no-product-ready-claim',
] as const;

export const TrackingRetentionWriterBoundaryProofIdSchema = TrackingRetentionWriterBoundaryProofText.pipe(
  Schema.brand('TrackingRetentionWriterBoundaryProofId')
);

export const TrackingRetentionWriterBoundaryReferenceSchema = TrackingRetentionWriterBoundaryProofText.pipe(
  Schema.brand('TrackingRetentionWriterBoundaryReference')
);

const TrackingRetentionWriterBoundaryRequestBaseSchema = Schema.Struct({
  requestId: TrackingRetentionWriterBoundaryReferenceSchema,
  settingKind: TrackingRetentionWriterBoundarySettingKindSchema,
  requestedValueRef: TrackingRetentionWriterBoundaryReferenceSchema,
  parentActionRef: TrackingRetentionWriterBoundaryReferenceSchema,
  sourceProofRefs: Schema.Array(TrackingRetentionWriterBoundaryReferenceSchema),
  evidenceRefs: Schema.Array(TrackingRetentionWriterBoundaryReferenceSchema),
  auditRefs: Schema.Array(TrackingRetentionWriterBoundaryReferenceSchema),
  requestedAt: ParentTimestampSchema,
  remoteSyncEnabled: Schema.Literal(false),
  remoteAiEnabled: Schema.Literal(false),
});

export const TrackingRetentionWriterBoundaryRequestSchema = withParser(
  TrackingRetentionWriterBoundaryRequestBaseSchema.pipe(
    Schema.filter(
      (request) =>
        trackingRetentionWriterBoundaryRequestIsHonest(request) ||
        'Retention writer requests need source/evidence/audit refs and must keep remote sync and remote AI disabled'
    )
  )
);

const TrackingRetentionWriterBoundaryRowBaseSchema = Schema.Struct({
  rowId: TrackingRetentionWriterBoundaryReferenceSchema,
  request: TrackingRetentionWriterBoundaryRequestSchema,
  state: TrackingRetentionWriterBoundaryStateSchema,
  validationRef: TrackingRetentionWriterBoundaryReferenceSchema,
  mutationEnvelopeRef: TrackingRetentionWriterBoundaryReferenceSchema,
  readModelUpdateRef: TrackingRetentionWriterBoundaryReferenceSchema,
  serviceMutationClaimed: Schema.Literal(false),
  platformRetentionWriterClaimed: Schema.Literal(false),
  childDeviceDeliveryClaimed: Schema.Literal(false),
  providerDeliveryClaimed: Schema.Literal(false),
  notificationReceiptClaimed: Schema.Literal(false),
  portalSettingsUiClaimed: Schema.Literal(false),
  physicalDeviceProofClaimed: Schema.Literal(false),
  productReadyClaimed: Schema.Literal(false),
});

export const TrackingRetentionWriterBoundaryRowSchema = withParser(
  TrackingRetentionWriterBoundaryRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        trackingRetentionWriterBoundaryRowIsHonest(row) ||
        'Retention writer rows need validation, mutation-envelope, read-model refs, and all runtime/product claims false'
    )
  )
);

const TrackingRetentionWriterBoundaryReadModelBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  proofId: TrackingRetentionWriterBoundaryProofIdSchema,
  generatedAt: ParentTimestampSchema,
  family: FamilyReferenceSchema,
  device: ParentDeviceReferenceSchema,
  sourceFeatureRefs: Schema.Array(TrackingRetentionWriterBoundaryReferenceSchema),
  rows: Schema.Array(TrackingRetentionWriterBoundaryRowSchema),
  acceptedForContractCount: TrackingRetentionWriterBoundaryCount,
  manualServiceMutationRequiredCount: TrackingRetentionWriterBoundaryCount,
  disabledRemoteRuntimeCount: TrackingRetentionWriterBoundaryCount,
  nonClaims: Schema.Array(TrackingRetentionWriterBoundaryNonClaimSchema),
  serviceMutationClaimed: Schema.Literal(false),
  platformRetentionWriterClaimed: Schema.Literal(false),
  childDeviceDeliveryClaimed: Schema.Literal(false),
  providerDeliveryClaimed: Schema.Literal(false),
  notificationReceiptClaimed: Schema.Literal(false),
  remoteSyncRuntimeClaimed: Schema.Literal(false),
  remoteAiRuntimeClaimed: Schema.Literal(false),
  portalSettingsUiClaimed: Schema.Literal(false),
  physicalDeviceProofClaimed: Schema.Literal(false),
  productReadyClaimed: Schema.Literal(false),
});

export const TrackingRetentionWriterBoundaryReadModelSchema = withParser(
  TrackingRetentionWriterBoundaryReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        trackingRetentionWriterBoundaryReadModelIsHonest(readModel) ||
        'Retention writer read model counts and non-claims must match rows without runtime/product overclaims'
    )
  )
);

export type TrackingRetentionWriterBoundarySettingKind = Infer<typeof TrackingRetentionWriterBoundarySettingKindSchema>;
export type TrackingRetentionWriterBoundaryRequest = Infer<typeof TrackingRetentionWriterBoundaryRequestSchema>;
export type TrackingRetentionWriterBoundaryRow = Infer<typeof TrackingRetentionWriterBoundaryRowSchema>;
export type TrackingRetentionWriterBoundaryReadModel = Infer<typeof TrackingRetentionWriterBoundaryReadModelSchema>;

export type TrackingRetentionWriterBoundaryOptions = {
  readonly generatedAt: string;
  readonly proofId: string;
  readonly familyId: string;
  readonly childProfileId: string;
  readonly deviceId: string;
  readonly deviceLabel: string;
  readonly platform: 'windows' | 'linux' | 'macos' | 'android' | 'ios';
  readonly sourceFeatureRefs: readonly string[];
};

type TrackingRetentionWriterBoundaryRequestInput = Infer<typeof TrackingRetentionWriterBoundaryRequestBaseSchema>;
type TrackingRetentionWriterBoundaryRowInput = Infer<typeof TrackingRetentionWriterBoundaryRowBaseSchema>;
type TrackingRetentionWriterBoundaryReadModelInput = Infer<typeof TrackingRetentionWriterBoundaryReadModelBaseSchema>;

export function buildTrackingRetentionWriterBoundaryReadModel(
  options: TrackingRetentionWriterBoundaryOptions,
  requests: readonly TrackingRetentionWriterBoundaryRequestInput[]
): TrackingRetentionWriterBoundaryReadModel {
  const parsedRequests = requests.map((request) => TrackingRetentionWriterBoundaryRequestSchema.parse(request));
  const rows = parsedRequests.map((request) => retentionWriterBoundaryRowForRequest(request));

  return TrackingRetentionWriterBoundaryReadModelSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    proofId: options.proofId,
    generatedAt: options.generatedAt,
    family: { familyId: options.familyId },
    device: {
      deviceId: options.deviceId,
      childProfileId: options.childProfileId,
      label: options.deviceLabel,
      platform: options.platform,
    },
    sourceFeatureRefs: options.sourceFeatureRefs,
    rows,
    acceptedForContractCount: countRows(rows, 'accepted-for-contract'),
    manualServiceMutationRequiredCount: countRows(rows, 'manual-service-mutation-required'),
    disabledRemoteRuntimeCount: countRows(rows, 'remote-sync-disabled') + countRows(rows, 'remote-ai-disabled'),
    nonClaims: RequiredTrackingRetentionWriterBoundaryNonClaims,
    serviceMutationClaimed: false,
    platformRetentionWriterClaimed: false,
    childDeviceDeliveryClaimed: false,
    providerDeliveryClaimed: false,
    notificationReceiptClaimed: false,
    remoteSyncRuntimeClaimed: false,
    remoteAiRuntimeClaimed: false,
    portalSettingsUiClaimed: false,
    physicalDeviceProofClaimed: false,
    productReadyClaimed: false,
  });
}

function retentionWriterBoundaryRowForRequest(
  request: TrackingRetentionWriterBoundaryRequest
): TrackingRetentionWriterBoundaryRow {
  return TrackingRetentionWriterBoundaryRowSchema.parse({
    rowId: `tracking-retention-writer-${request.requestId}`,
    request,
    state: stateForRequest(request),
    validationRef: `tracking-retention-writer-validation-${request.requestId}`,
    mutationEnvelopeRef: `tracking-retention-writer-envelope-${request.requestId}`,
    readModelUpdateRef: `tracking-retention-writer-read-model-update-${request.requestId}`,
    serviceMutationClaimed: false,
    platformRetentionWriterClaimed: false,
    childDeviceDeliveryClaimed: false,
    providerDeliveryClaimed: false,
    notificationReceiptClaimed: false,
    portalSettingsUiClaimed: false,
    physicalDeviceProofClaimed: false,
    productReadyClaimed: false,
  });
}

function stateForRequest(request: TrackingRetentionWriterBoundaryRequest): TrackingRetentionWriterBoundaryRow['state'] {
  if (request.settingKind === 'remote-sync') {
    return 'remote-sync-disabled';
  }
  if (request.settingKind === 'remote-ai') {
    return 'remote-ai-disabled';
  }
  if (request.settingKind === 'parent-export') {
    return 'manual-service-mutation-required';
  }
  return 'accepted-for-contract';
}

function trackingRetentionWriterBoundaryRequestIsHonest(request: TrackingRetentionWriterBoundaryRequestInput): boolean {
  return (
    request.sourceProofRefs.length >= 2 &&
    request.evidenceRefs.length > 0 &&
    request.auditRefs.length > 0 &&
    request.remoteSyncEnabled === false &&
    request.remoteAiEnabled === false
  );
}

function trackingRetentionWriterBoundaryRowIsHonest(row: TrackingRetentionWriterBoundaryRowInput): boolean {
  return (
    row.validationRef.length > 0 &&
    row.mutationEnvelopeRef.length > 0 &&
    row.readModelUpdateRef.length > 0 &&
    row.serviceMutationClaimed === false &&
    row.platformRetentionWriterClaimed === false &&
    row.childDeviceDeliveryClaimed === false &&
    row.providerDeliveryClaimed === false &&
    row.notificationReceiptClaimed === false &&
    row.portalSettingsUiClaimed === false &&
    row.physicalDeviceProofClaimed === false &&
    row.productReadyClaimed === false
  );
}

function trackingRetentionWriterBoundaryReadModelIsHonest(
  readModel: TrackingRetentionWriterBoundaryReadModelInput
): boolean {
  return (
    readModel.rows.length === 5 &&
    readModel.acceptedForContractCount === countRows(readModel.rows, 'accepted-for-contract') &&
    readModel.manualServiceMutationRequiredCount === countRows(readModel.rows, 'manual-service-mutation-required') &&
    readModel.disabledRemoteRuntimeCount ===
      countRows(readModel.rows, 'remote-sync-disabled') + countRows(readModel.rows, 'remote-ai-disabled') &&
    RequiredTrackingRetentionWriterBoundaryNonClaims.every((claim) => readModel.nonClaims.includes(claim))
  );
}

const countRows = (
  rows: ReadonlyArray<{ readonly state: string }>,
  state: TrackingRetentionWriterBoundaryRow['state']
): number => rows.filter((row) => row.state === state).length;

export const decodeTrackingRetentionWriterBoundaryReadModel = Schema.decodeUnknownSync(
  TrackingRetentionWriterBoundaryReadModelSchema
);
