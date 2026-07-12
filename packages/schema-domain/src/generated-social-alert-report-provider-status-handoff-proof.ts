/* generated from crates/browser-core/src/social_schema_generated_alert_report.rs */

import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';
import {
  SocialAlertReportProviderPreflightReadModelSchema,
  SocialAlertReportProviderPreflightStatus,
  SocialAlertReportProviderPreflightStatusSchema,
  type SocialAlertReportProviderPreflightReadModel,
  type SocialAlertReportProviderPreflightRow,
} from './generated-social-alert-report-provider-preflight-proof';
import {
  ParentContractSchemaVersion,
  ParentContractSchemaVersionSchema,
  ParentTimestampSchema,
} from '@ocentra-parent/schema-domain/family-reference-primitives';
import {
  V08NotificationProviderStatusBoundaryEntrySchema,
  V08NotificationProviderStatusBoundaryReadModel,
  type V08NotificationProviderStatus,
} from '@ocentra-parent/schema-domain/v0-8-notification-provider-status-boundary';

export const RequiredSocialAlertReportProviderStatusHandoffNonClaims = [
  'no-provider-delivery-execution',
  'no-provider-receipt-ingestion',
  'no-provider-credentials',
  'no-cloud-routing',
  'no-parent-notification-ui-delivery',
  'no-report-delivery-execution',
  'no-final-policy-execution',
  'no-connector-native-runtime',
  'no-enforcement',
] as const;

export const SocialAlertReportProviderStatusHandoffNonClaimSchema = withParser(
  Schema.Literal(...RequiredSocialAlertReportProviderStatusHandoffNonClaims)
);
export const SocialAlertReportProviderStatusHandoffIdSchema = withParser(
  brandedNonEmptyStringSchema('SocialAlertReportProviderStatusHandoffId')
);
export const SocialAlertReportProviderStatusHandoffReferenceSchema = withParser(
  brandedNonEmptyStringSchema('SocialAlertReportProviderStatusHandoffReference')
);

const SocialAlertReportProviderStatusHandoffRowBaseSchema = Schema.Struct({
  handoffRowId: SocialAlertReportProviderStatusHandoffReferenceSchema,
  sourcePreflightRowId: SocialAlertReportProviderStatusHandoffReferenceSchema,
  sourceIntentRef: SocialAlertReportProviderStatusHandoffReferenceSchema,
  sourcePreflightStatus: SocialAlertReportProviderPreflightStatusSchema,
  sourceLocalOutboxRecordRef: Schema.Union(SocialAlertReportProviderStatusHandoffReferenceSchema, Schema.Null),
  sourceProviderChannelRef: Schema.Union(SocialAlertReportProviderStatusHandoffReferenceSchema, Schema.Null),
  providerStatusBoundaryEntry: V08NotificationProviderStatusBoundaryEntrySchema,
  manualProofRequirements: Schema.Array(SocialAlertReportProviderStatusHandoffReferenceSchema),
});

export const SocialAlertReportProviderStatusHandoffRowSchema = withParser(
  SocialAlertReportProviderStatusHandoffRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        socialProviderStatusHandoffRowIsHonest(row) ||
        'Expected social alert/report provider status handoff rows to map preflight rows into manual-required or unavailable provider status boundary entries without delivery claims'
    )
  )
);

const SocialAlertReportProviderStatusHandoffReadModelBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  handoffId: SocialAlertReportProviderStatusHandoffIdSchema,
  generatedAt: ParentTimestampSchema,
  sourceProviderPreflightId: SocialAlertReportProviderStatusHandoffReferenceSchema,
  sourceContractRefs: Schema.Array(SocialAlertReportProviderStatusHandoffReferenceSchema),
  providerStatusBoundaryReadModelRef: SocialAlertReportProviderStatusHandoffReferenceSchema,
  providerStatusBoundaryCoverageRefs: Schema.Array(SocialAlertReportProviderStatusHandoffReferenceSchema),
  rows: Schema.Array(SocialAlertReportProviderStatusHandoffRowSchema),
  providerStatusManualRequiredCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  providerStatusUnavailableCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  handoffNonClaims: Schema.Array(SocialAlertReportProviderStatusHandoffNonClaimSchema),
  providerDeliveryRuntimeClaimed: Schema.Literal(false),
  providerReceiptIngestionClaimed: Schema.Literal(false),
  providerCredentialsClaimed: Schema.Literal(false),
  cloudRoutingClaimed: Schema.Literal(false),
  parentNotificationUiDeliveryClaimed: Schema.Literal(false),
  reportDeliveryExecutionClaimed: Schema.Literal(false),
  finalPolicyExecutionClaimed: Schema.Literal(false),
  connectorNativeRuntimeClaimed: Schema.Literal(false),
  enforcementClaimed: Schema.Literal(false),
});

export const SocialAlertReportProviderStatusHandoffReadModelSchema = withParser(
  SocialAlertReportProviderStatusHandoffReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        socialProviderStatusHandoffReadModelIsHonest(readModel) ||
        'Expected social alert/report provider status handoff counts and non-claims to match provider status boundary rows'
    )
  )
);

export type SocialAlertReportProviderStatusHandoffRow = Infer<typeof SocialAlertReportProviderStatusHandoffRowSchema>;
export type SocialAlertReportProviderStatusHandoffReadModel = Infer<
  typeof SocialAlertReportProviderStatusHandoffReadModelSchema
>;

type SocialProviderStatusHandoffRowInput = Infer<typeof SocialAlertReportProviderStatusHandoffRowBaseSchema>;
type ProviderStatusBoundaryEntry = SocialProviderStatusHandoffRowInput['providerStatusBoundaryEntry'];
type ProviderStatusBoundaryExpectation = Pick<
  ProviderStatusBoundaryEntry,
  'providerStatus' | 'statusProofState' | 'quietHoursReadiness' | 'escalationReadiness'
>;

const UnavailableProviderStatusBoundaryExpectation: ProviderStatusBoundaryExpectation = {
  providerStatus: 'unavailable',
  statusProofState: 'provider-unavailable-contract',
  quietHoursReadiness: 'unavailable',
  escalationReadiness: 'unavailable',
};

const ManualRequiredProviderStatusBoundaryExpectation: ProviderStatusBoundaryExpectation = {
  providerStatus: 'manual-required',
  statusProofState: 'manual-action-required',
  quietHoursReadiness: 'manual-required',
  escalationReadiness: 'manual-required',
};

export type SocialAlertReportProviderStatusHandoffOptions = {
  readonly generatedAt: string;
  readonly handoffId: string;
  readonly sourceContractRefs: readonly string[];
};

export function buildSocialAlertReportProviderStatusHandoffReadModel(
  options: SocialAlertReportProviderStatusHandoffOptions,
  sourceReadModel: SocialAlertReportProviderPreflightReadModel
): SocialAlertReportProviderStatusHandoffReadModel {
  const parsedSource = SocialAlertReportProviderPreflightReadModelSchema.parse(sourceReadModel);
  const rows = parsedSource.rows.map((row) => socialProviderStatusHandoffRowForPreflightRow(options, row));

  return SocialAlertReportProviderStatusHandoffReadModelSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    handoffId: options.handoffId,
    generatedAt: options.generatedAt,
    sourceProviderPreflightId: parsedSource.providerPreflightId,
    sourceContractRefs: options.sourceContractRefs,
    providerStatusBoundaryReadModelRef: V08NotificationProviderStatusBoundaryReadModel.readModelId,
    providerStatusBoundaryCoverageRefs: V08NotificationProviderStatusBoundaryReadModel.entries.map(
      (entry) => entry.statusEntryId
    ),
    rows,
    providerStatusManualRequiredCount: countProviderStatus(rows, 'manual-required'),
    providerStatusUnavailableCount: countProviderStatus(rows, 'unavailable'),
    handoffNonClaims: RequiredSocialAlertReportProviderStatusHandoffNonClaims,
    providerDeliveryRuntimeClaimed: false,
    providerReceiptIngestionClaimed: false,
    providerCredentialsClaimed: false,
    cloudRoutingClaimed: false,
    parentNotificationUiDeliveryClaimed: false,
    reportDeliveryExecutionClaimed: false,
    finalPolicyExecutionClaimed: false,
    connectorNativeRuntimeClaimed: false,
    enforcementClaimed: false,
  });
}

export function summarizeSocialAlertReportProviderStatusHandoff(
  readModel: SocialAlertReportProviderStatusHandoffReadModel
) {
  return {
    rows: readModel.rows.length,
    providerStatusManualRequiredCount: readModel.providerStatusManualRequiredCount,
    providerStatusUnavailableCount: readModel.providerStatusUnavailableCount,
    providerDeliveryRuntimeClaimed: readModel.providerDeliveryRuntimeClaimed,
    providerReceiptIngestionClaimed: readModel.providerReceiptIngestionClaimed,
    finalPolicyExecutionClaimed: readModel.finalPolicyExecutionClaimed,
    enforcementClaimed: readModel.enforcementClaimed,
  };
}

function socialProviderStatusHandoffRowForPreflightRow(
  options: SocialAlertReportProviderStatusHandoffOptions,
  row: SocialAlertReportProviderPreflightRow
): SocialAlertReportProviderStatusHandoffRow {
  return SocialAlertReportProviderStatusHandoffRowSchema.parse({
    handoffRowId: `social-provider-status-handoff-${row.preflightRowId}`,
    sourcePreflightRowId: row.preflightRowId,
    sourceIntentRef: row.sourceIntentRef,
    sourcePreflightStatus: row.status,
    sourceLocalOutboxRecordRef: row.sourceLocalOutboxRecordRef,
    sourceProviderChannelRef: row.providerChannelRef,
    providerStatusBoundaryEntry: providerStatusBoundaryEntryForPreflightRow(options, row),
    manualProofRequirements: row.manualProofRequirements,
  });
}

function providerStatusBoundaryEntryForPreflightRow(
  options: SocialAlertReportProviderStatusHandoffOptions,
  row: SocialAlertReportProviderPreflightRow
) {
  const unavailable = row.status === SocialAlertReportProviderPreflightStatus.Unavailable;

  return V08NotificationProviderStatusBoundaryEntrySchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    statusEntryId: `social-provider-status-${row.preflightRowId}`,
    providerStatus: unavailable ? 'unavailable' : 'manual-required',
    statusProofState: unavailable ? 'provider-unavailable-contract' : 'manual-action-required',
    quietHoursReadiness: unavailable ? 'unavailable' : 'manual-required',
    escalationReadiness: unavailable ? 'unavailable' : 'manual-required',
    deliveryClaimState: unavailable ? 'not-implemented' : 'not-observed',
    notificationIntentRef: `social-provider-status-intent-${row.sourceIntentRef}`,
    notificationStatusRef: `social-provider-status-ref-${row.preflightRowId}`,
    providerAttemptRef: `social-provider-attempt-not-started-${row.preflightRowId}`,
    auditRefs: row.auditRefs,
    preferenceRefs: providerPreferenceRefsForRow(row),
    readinessRefs: providerReadinessRefsForRow(row),
    providerReceiptRefs: [],
    manualProofRequirements: row.manualProofRequirements,
    minimalPayloadBoundary: unavailable
      ? 'Provider unavailable keeps social alert/report delivery unclaimed and visible for manual review.'
      : 'Provider manual-required keeps social alert/report delivery blocked until adapter, credentials, preferences, and smoke proof exist.',
    providerDeliveryImplemented: false,
    providerDeliveryObserved: false,
    deliveredNotificationClaimed: false,
    sensitiveProviderPayloadClaimed: false,
    providerStoresChildEvidenceClaimed: false,
    lastCheckedAt: options.generatedAt,
  });
}

function providerPreferenceRefsForRow(row: SocialAlertReportProviderPreflightRow): readonly string[] {
  return row.providerChannelRef === null
    ? ['social-provider-preference-manual-review']
    : [`social-provider-preference-${row.providerChannelRef}`];
}

function providerReadinessRefsForRow(row: SocialAlertReportProviderPreflightRow): readonly string[] {
  if (row.status === SocialAlertReportProviderPreflightStatus.Unavailable) {
    return ['social-provider-readiness-unavailable'];
  }
  return row.adapterRequirementRefs.length === 0
    ? ['social-provider-readiness-manual-required']
    : row.adapterRequirementRefs;
}

function socialProviderStatusHandoffRowIsHonest(row: SocialProviderStatusHandoffRowInput): boolean {
  const entry = row.providerStatusBoundaryEntry;

  return (
    providerStatusBoundaryMatchesPreflight(row) &&
    providerStatusBoundaryKeepsDeliveryUnclaimed(entry) &&
    row.manualProofRequirements.length > 0 &&
    entry.manualProofRequirements.length > 0
  );
}

function providerStatusBoundaryMatchesPreflight(row: SocialProviderStatusHandoffRowInput): boolean {
  const entry = row.providerStatusBoundaryEntry;
  const expected = providerStatusBoundaryExpectationFor(row.sourcePreflightStatus);

  return (
    entry.providerStatus === expected.providerStatus &&
    entry.statusProofState === expected.statusProofState &&
    entry.quietHoursReadiness === expected.quietHoursReadiness &&
    entry.escalationReadiness === expected.escalationReadiness
  );
}

function providerStatusBoundaryExpectationFor(
  status: SocialAlertReportProviderPreflightStatus
): ProviderStatusBoundaryExpectation {
  if (status === SocialAlertReportProviderPreflightStatus.Unavailable) {
    return UnavailableProviderStatusBoundaryExpectation;
  }
  return ManualRequiredProviderStatusBoundaryExpectation;
}

function providerStatusBoundaryKeepsDeliveryUnclaimed(entry: ProviderStatusBoundaryEntry): boolean {
  const deliveryClaims = [
    entry.providerDeliveryImplemented,
    entry.providerDeliveryObserved,
    entry.deliveredNotificationClaimed,
    entry.sensitiveProviderPayloadClaimed,
    entry.providerStoresChildEvidenceClaimed,
  ];

  return entry.providerReceiptRefs.length === 0 && deliveryClaims.every((claim) => claim === false);
}

function socialProviderStatusHandoffReadModelIsHonest(
  readModel: Infer<typeof SocialAlertReportProviderStatusHandoffReadModelBaseSchema>
): boolean {
  return (
    readModel.providerStatusManualRequiredCount === countProviderStatus(readModel.rows, 'manual-required') &&
    readModel.providerStatusUnavailableCount === countProviderStatus(readModel.rows, 'unavailable') &&
    RequiredSocialAlertReportProviderStatusHandoffNonClaims.every((claim) =>
      readModel.handoffNonClaims.includes(claim)
    ) &&
    readModel.providerStatusBoundaryCoverageRefs.length ===
      V08NotificationProviderStatusBoundaryReadModel.entries.length
  );
}

const countProviderStatus = (
  rows: ReadonlyArray<{
    readonly providerStatusBoundaryEntry: { readonly providerStatus: V08NotificationProviderStatus };
  }>,
  providerStatus: V08NotificationProviderStatus
): number => rows.filter((row) => row.providerStatusBoundaryEntry.providerStatus === providerStatus).length;
