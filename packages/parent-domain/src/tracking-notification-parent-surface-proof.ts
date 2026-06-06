import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  ParentContractSchemaVersion,
  ParentContractSchemaVersionSchema,
  ParentTimestampSchema,
} from './reference-primitives';
import { FamilyReferenceSchema } from './references';
import {
  RequiredTrackingProviderNotificationProofNonClaims,
  TrackingProviderNotificationProofReadModelSchema,
  type TrackingProviderNotificationProofReadModel,
  type TrackingProviderNotificationProofRow,
} from './tracking-provider-notification-proof';

const TrackingNotificationParentSurfaceText = Schema.String.pipe(Schema.minLength(1));

export const RequiredTrackingNotificationParentSurfaceNonClaims = [
  'no-parent-notification-ui-rendered',
  'no-parent-preference-ui-rendered',
  'no-parent-history-ui-rendered',
  'no-parent-preference-mutation-runtime',
  'no-provider-delivery-execution',
  'no-provider-receipt-ingestion',
  'no-provider-credentials',
  'no-adapter-dispatch',
  'no-child-device-delivery',
  'no-mobile-physical-device-proof',
  'no-authority-proof',
  'no-production-durable-outbox-storage',
] as const;

export const TrackingNotificationParentSurfaceNonClaimSchema = withParser(
  Schema.Literal(...RequiredTrackingNotificationParentSurfaceNonClaims)
);

export const TrackingNotificationParentSurfaceProofIdSchema = TrackingNotificationParentSurfaceText.pipe(
  Schema.brand('TrackingNotificationParentSurfaceProofId')
);
export const TrackingNotificationParentSurfaceReferenceSchema = TrackingNotificationParentSurfaceText.pipe(
  Schema.brand('TrackingNotificationParentSurfaceReference')
);

export const TrackingNotificationParentSurfaceStatusSchema = withParser(
  Schema.Literal('history-row-ready', 'manual-action-required', 'unavailable-visible')
);
export const TrackingNotificationHistoryVisibilitySchema = withParser(
  Schema.Literal('status-history-ready', 'manual-review-only', 'unavailable-row-visible')
);
export const TrackingNotificationPreferenceVisibilitySchema = withParser(
  Schema.Literal('preference-setup-required', 'preference-unavailable-visible')
);

const TrackingNotificationParentSurfaceRowBaseSchema = Schema.Struct({
  surfaceRowId: TrackingNotificationParentSurfaceReferenceSchema,
  sourceProviderProofRowRef: TrackingNotificationParentSurfaceReferenceSchema,
  sourceAlertId: TrackingNotificationParentSurfaceReferenceSchema,
  sourcePolicyDecisionId: TrackingNotificationParentSurfaceReferenceSchema,
  sourceProviderStatusEntryRef: TrackingNotificationParentSurfaceReferenceSchema,
  sourceProviderStatusKind: TrackingNotificationParentSurfaceReferenceSchema,
  parentSurfaceStatus: TrackingNotificationParentSurfaceStatusSchema,
  historyVisibility: TrackingNotificationHistoryVisibilitySchema,
  preferenceVisibility: TrackingNotificationPreferenceVisibilitySchema,
  parentVisibleNotificationStatusRef: TrackingNotificationParentSurfaceReferenceSchema,
  providerAttemptRef: TrackingNotificationParentSurfaceReferenceSchema,
  evidenceRefs: Schema.Array(TrackingNotificationParentSurfaceReferenceSchema),
  sourceNotificationStatusRefs: Schema.Array(TrackingNotificationParentSurfaceReferenceSchema),
  reasonCodeRefs: Schema.Array(TrackingNotificationParentSurfaceReferenceSchema),
  drillInRefs: Schema.Array(TrackingNotificationParentSurfaceReferenceSchema),
  auditRefs: Schema.Array(TrackingNotificationParentSurfaceReferenceSchema),
  preferenceRefs: Schema.Array(TrackingNotificationParentSurfaceReferenceSchema),
  manualProofRequirements: Schema.Array(TrackingNotificationParentSurfaceReferenceSchema),
  minimalSurfacePayloadBoundary: TrackingNotificationParentSurfaceText,
  sensitiveDetailIncluded: Schema.Literal(false),
  parentNotificationUiRendered: Schema.Literal(false),
  parentPreferenceUiRendered: Schema.Literal(false),
  parentHistoryUiRendered: Schema.Literal(false),
  parentPreferenceMutationClaimed: Schema.Literal(false),
  providerDeliveryClaimed: Schema.Literal(false),
  providerReceiptIngestionClaimed: Schema.Literal(false),
  childDeviceDeliveryClaimed: Schema.Literal(false),
  authorityProofClaimed: Schema.Literal(false),
  mobilePhysicalDeviceProofClaimed: Schema.Literal(false),
});

export const TrackingNotificationParentSurfaceRowSchema = withParser(
  TrackingNotificationParentSurfaceRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        trackingNotificationParentSurfaceRowIsHonest(row) ||
        'Expected tracking notification parent-surface rows to preserve alert/provider refs and keep UI, mutation, delivery, receipt, child-device, authority, and physical-device claims false'
    )
  )
);

const TrackingNotificationParentSurfaceReadModelBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  proofId: TrackingNotificationParentSurfaceProofIdSchema,
  generatedAt: ParentTimestampSchema,
  family: FamilyReferenceSchema,
  sourceProviderNotificationProofRef: TrackingNotificationParentSurfaceReferenceSchema,
  sourceContractRefs: Schema.Array(TrackingNotificationParentSurfaceReferenceSchema),
  sourceProviderProofNonClaims: Schema.Array(TrackingNotificationParentSurfaceReferenceSchema),
  rows: Schema.Array(TrackingNotificationParentSurfaceRowSchema),
  historyRowReadyCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  manualActionRequiredCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  unavailableVisibleCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  preferenceSetupRequiredCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  parentSurfaceNonClaims: Schema.Array(TrackingNotificationParentSurfaceNonClaimSchema),
  parentNotificationUiRendered: Schema.Literal(false),
  parentPreferenceUiRendered: Schema.Literal(false),
  parentHistoryUiRendered: Schema.Literal(false),
  parentPreferenceMutationRuntimeClaimed: Schema.Literal(false),
  providerDeliveryRuntimeClaimed: Schema.Literal(false),
  providerReceiptIngestionClaimed: Schema.Literal(false),
  providerCredentialsClaimed: Schema.Literal(false),
  adapterDispatchClaimed: Schema.Literal(false),
  childDeviceDeliveryClaimed: Schema.Literal(false),
  mobilePhysicalDeviceProofClaimed: Schema.Literal(false),
  authorityProofClaimed: Schema.Literal(false),
  productionDurableOutboxStorageClaimed: Schema.Literal(false),
});

export const TrackingNotificationParentSurfaceReadModelSchema = withParser(
  TrackingNotificationParentSurfaceReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        trackingNotificationParentSurfaceReadModelIsHonest(readModel) ||
        'Expected tracking notification parent-surface counts and non-claims to match row state'
    )
  )
);

export type TrackingNotificationParentSurfaceRow = Infer<typeof TrackingNotificationParentSurfaceRowSchema>;
export type TrackingNotificationParentSurfaceReadModel = Infer<typeof TrackingNotificationParentSurfaceReadModelSchema>;

export type TrackingNotificationParentSurfaceProofOptions = {
  readonly generatedAt: string;
  readonly proofId: string;
  readonly sourceProviderNotificationProofRef: string;
  readonly sourceContractRefs: readonly string[];
};

type TrackingNotificationParentSurfaceRowInput = Infer<typeof TrackingNotificationParentSurfaceRowBaseSchema>;
type TrackingNotificationParentSurfaceReadModelInput = Infer<
  typeof TrackingNotificationParentSurfaceReadModelBaseSchema
>;

export function buildTrackingNotificationParentSurfaceReadModel(
  options: TrackingNotificationParentSurfaceProofOptions,
  sourceProviderProof: TrackingProviderNotificationProofReadModel
): TrackingNotificationParentSurfaceReadModel {
  const parsedSource = TrackingProviderNotificationProofReadModelSchema.parse(sourceProviderProof);
  const rows = parsedSource.rows.map((row) => trackingNotificationParentSurfaceRowForProviderRow(row));

  return TrackingNotificationParentSurfaceReadModelSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    proofId: options.proofId,
    generatedAt: options.generatedAt,
    family: parsedSource.family,
    sourceProviderNotificationProofRef: options.sourceProviderNotificationProofRef,
    sourceContractRefs: options.sourceContractRefs,
    sourceProviderProofNonClaims: RequiredTrackingProviderNotificationProofNonClaims,
    rows,
    historyRowReadyCount: countSurfaceStatus(rows, 'history-row-ready'),
    manualActionRequiredCount: countSurfaceStatus(rows, 'manual-action-required'),
    unavailableVisibleCount: countSurfaceStatus(rows, 'unavailable-visible'),
    preferenceSetupRequiredCount: countPreferenceVisibility(rows, 'preference-setup-required'),
    parentSurfaceNonClaims: RequiredTrackingNotificationParentSurfaceNonClaims,
    parentNotificationUiRendered: false,
    parentPreferenceUiRendered: false,
    parentHistoryUiRendered: false,
    parentPreferenceMutationRuntimeClaimed: false,
    providerDeliveryRuntimeClaimed: false,
    providerReceiptIngestionClaimed: false,
    providerCredentialsClaimed: false,
    adapterDispatchClaimed: false,
    childDeviceDeliveryClaimed: false,
    mobilePhysicalDeviceProofClaimed: false,
    authorityProofClaimed: false,
    productionDurableOutboxStorageClaimed: false,
  });
}

function trackingNotificationParentSurfaceRowForProviderRow(
  row: TrackingProviderNotificationProofRow
): TrackingNotificationParentSurfaceRow {
  const providerEntry = row.providerStatusBoundaryEntry;

  return TrackingNotificationParentSurfaceRowSchema.parse({
    surfaceRowId: `tracking-notification-parent-surface-${row.sourceAlertId}`,
    sourceProviderProofRowRef: row.rowId,
    sourceAlertId: row.sourceAlertId,
    sourcePolicyDecisionId: row.sourcePolicyDecisionId,
    sourceProviderStatusEntryRef: providerEntry.statusEntryId,
    sourceProviderStatusKind: row.providerStatusKind,
    parentSurfaceStatus: parentSurfaceStatusFor(row),
    historyVisibility: historyVisibilityFor(row),
    preferenceVisibility:
      row.providerStatusKind === 'unavailable' ? 'preference-unavailable-visible' : 'preference-setup-required',
    parentVisibleNotificationStatusRef: providerEntry.notificationStatusRef,
    providerAttemptRef: providerEntry.providerAttemptRef,
    evidenceRefs: row.evidenceRefs,
    sourceNotificationStatusRefs: row.notificationStatusRefs,
    reasonCodeRefs: row.reasonCodeRefs,
    drillInRefs: [row.sourceAlertId, row.sourcePolicyDecisionId, providerEntry.notificationStatusRef],
    auditRefs: providerEntry.auditRefs,
    preferenceRefs: providerEntry.preferenceRefs,
    manualProofRequirements: row.manualProofRequirements,
    minimalSurfacePayloadBoundary:
      row.sensitiveDetailMode === 'authenticated-drill-in-only'
        ? 'Parent surface row may show manual status and authenticated drill-in refs only; precise location evidence stays behind authenticated tracking detail surfaces.'
        : 'Parent surface row may show minimal alert status, history, and setup requirements while evidence detail stays behind authenticated tracking detail surfaces.',
    sensitiveDetailIncluded: false,
    parentNotificationUiRendered: false,
    parentPreferenceUiRendered: false,
    parentHistoryUiRendered: false,
    parentPreferenceMutationClaimed: false,
    providerDeliveryClaimed: false,
    providerReceiptIngestionClaimed: false,
    childDeviceDeliveryClaimed: false,
    authorityProofClaimed: false,
    mobilePhysicalDeviceProofClaimed: false,
  });
}

function parentSurfaceStatusFor(
  row: TrackingProviderNotificationProofRow
): TrackingNotificationParentSurfaceRowInput['parentSurfaceStatus'] {
  if (row.providerStatusKind === 'unavailable') {
    return 'unavailable-visible';
  }
  if (row.providerStatusKind === 'manual-required') {
    return 'manual-action-required';
  }
  return 'history-row-ready';
}

function historyVisibilityFor(
  row: TrackingProviderNotificationProofRow
): TrackingNotificationParentSurfaceRowInput['historyVisibility'] {
  if (row.providerStatusKind === 'unavailable') {
    return 'unavailable-row-visible';
  }
  if (row.providerStatusKind === 'manual-required') {
    return 'manual-review-only';
  }
  return 'status-history-ready';
}

function trackingNotificationParentSurfaceRowIsHonest(row: TrackingNotificationParentSurfaceRowInput): boolean {
  return (
    row.evidenceRefs.length > 0 &&
    row.reasonCodeRefs.length > 0 &&
    row.drillInRefs.length > 0 &&
    row.auditRefs.length > 0 &&
    row.preferenceRefs.length > 0 &&
    row.manualProofRequirements.length > 0 &&
    row.minimalSurfacePayloadBoundary.trim().length > 0 &&
    trackingNotificationParentSurfaceClaimsStayFalse(row)
  );
}

function trackingNotificationParentSurfaceClaimsStayFalse(row: TrackingNotificationParentSurfaceRowInput): boolean {
  return [
    row.sensitiveDetailIncluded,
    row.parentNotificationUiRendered,
    row.parentPreferenceUiRendered,
    row.parentHistoryUiRendered,
    row.parentPreferenceMutationClaimed,
    row.providerDeliveryClaimed,
    row.providerReceiptIngestionClaimed,
    row.childDeviceDeliveryClaimed,
    row.authorityProofClaimed,
    row.mobilePhysicalDeviceProofClaimed,
  ].every((claim) => claim === false);
}

function trackingNotificationParentSurfaceReadModelIsHonest(
  readModel: TrackingNotificationParentSurfaceReadModelInput
): boolean {
  const sourceProviderProofNonClaims = new Set<string>(readModel.sourceProviderProofNonClaims);

  return (
    readModel.historyRowReadyCount === countSurfaceStatus(readModel.rows, 'history-row-ready') &&
    readModel.manualActionRequiredCount === countSurfaceStatus(readModel.rows, 'manual-action-required') &&
    readModel.unavailableVisibleCount === countSurfaceStatus(readModel.rows, 'unavailable-visible') &&
    readModel.preferenceSetupRequiredCount === countPreferenceVisibility(readModel.rows, 'preference-setup-required') &&
    RequiredTrackingProviderNotificationProofNonClaims.every((claim) => sourceProviderProofNonClaims.has(claim)) &&
    RequiredTrackingNotificationParentSurfaceNonClaims.every((claim) =>
      readModel.parentSurfaceNonClaims.includes(claim)
    )
  );
}

const countSurfaceStatus = (
  rows: ReadonlyArray<{
    readonly parentSurfaceStatus: TrackingNotificationParentSurfaceRowInput['parentSurfaceStatus'];
  }>,
  status: TrackingNotificationParentSurfaceRowInput['parentSurfaceStatus']
): number => rows.filter((row) => row.parentSurfaceStatus === status).length;

const countPreferenceVisibility = (
  rows: ReadonlyArray<{
    readonly preferenceVisibility: TrackingNotificationParentSurfaceRowInput['preferenceVisibility'];
  }>,
  visibility: TrackingNotificationParentSurfaceRowInput['preferenceVisibility']
): number => rows.filter((row) => row.preferenceVisibility === visibility).length;

export const decodeTrackingNotificationParentSurfaceReadModel = Schema.decodeUnknownSync(
  TrackingNotificationParentSurfaceReadModelSchema
);
