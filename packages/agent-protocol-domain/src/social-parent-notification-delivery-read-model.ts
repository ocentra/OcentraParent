import { Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { AgentEvent, AgentProtocolDefaults, isAgentProtocolLogText, type AgentEventEnvelope } from './contracts';

const NonClaimSchema = Schema.Literal(
  'no-parent-notification-ui-delivery',
  'no-external-runtime-report-delivery',
  'no-provider-delivery',
  'no-provider-receipt-ingestion',
  'no-final-policy-execution',
  'no-enforcement'
);

const DeliveryReadinessStateSchema = Schema.Literal('parent-report-status-ready', 'manual-required', 'unavailable');
const ReportExecutionStateSchema = Schema.Literal('parent-owned-report-ready', 'manual-required', 'unavailable');
const NonEmptyRefSchema = Schema.String.pipe(Schema.minLength(1));

type DeliveryReadinessState = 'parent-report-status-ready' | 'manual-required' | 'unavailable';
type ReportExecutionState = 'parent-owned-report-ready' | 'manual-required' | 'unavailable';
type DeliveryReadinessRow = {
  readonly notificationDeliveryReadinessRowId: string;
  readonly sourceReportWriterDeliveryRowRef: string;
  readonly sourceIntentRef: string;
  readonly parentVisibleReportStatusRef: string | null;
  readonly parentNotificationUiRef: string | null;
  readonly parentReportRef: string | null;
  readonly reportArtifactRef: string | null;
  readonly reportReceiptRef: string | null;
  readonly sourceEvidenceRefs: readonly string[];
  readonly sourcePolicyRefs: readonly string[];
  readonly sourceAuditRefs: readonly string[];
  readonly manualProofRequirements: readonly string[];
  readonly notificationDeliveryReadinessState: DeliveryReadinessState;
  readonly reportDeliveryExecutionState: ReportExecutionState;
  readonly parentOwnedReportArtifactWritten: boolean;
  readonly parentOwnedReportReceiptRecorded: boolean;
  readonly parentNotificationUiDelivered: false;
  readonly externalRuntimeReportDeliveryClaimed: false;
  readonly providerDeliveryAttempted: false;
  readonly providerReceiptIngested: false;
  readonly finalPolicyDecisionClaimed: false;
  readonly enforcementClaimed: false;
  readonly createdAt: string;
};
type DeliveryReadinessSnapshot = {
  readonly schemaVersion: 'social-parent-notification-delivery-read-model';
  readonly readinessId: string;
  readonly generatedAt: string;
  readonly sourceReportWriterProofRef: string;
  readonly rows: readonly DeliveryReadinessRow[];
  readonly nonClaims: readonly string[];
  readonly parentReportStatusReadyCount: number;
  readonly manualRequiredCount: number;
  readonly unavailableCount: number;
  readonly parentNotificationUiDeliveryClaimed: false;
  readonly externalRuntimeReportDeliveryClaimed: false;
  readonly finalPolicyExecutionClaimed: false;
  readonly enforcementClaimed: false;
};

const DeliveryReadinessRowSchema = Schema.Struct({
  notificationDeliveryReadinessRowId: NonEmptyRefSchema,
  sourceReportWriterDeliveryRowRef: NonEmptyRefSchema,
  sourceIntentRef: NonEmptyRefSchema,
  parentVisibleReportStatusRef: Schema.Union(NonEmptyRefSchema, Schema.Null),
  parentNotificationUiRef: Schema.Union(NonEmptyRefSchema, Schema.Null),
  parentReportRef: Schema.Union(NonEmptyRefSchema, Schema.Null),
  reportArtifactRef: Schema.Union(NonEmptyRefSchema, Schema.Null),
  reportReceiptRef: Schema.Union(NonEmptyRefSchema, Schema.Null),
  sourceEvidenceRefs: Schema.Array(NonEmptyRefSchema).pipe(
    Schema.filter((value) => value.length > 0 || 'Expected source evidence refs')
  ),
  sourcePolicyRefs: Schema.Array(NonEmptyRefSchema).pipe(
    Schema.filter((value) => value.length > 0 || 'Expected source policy refs')
  ),
  sourceAuditRefs: Schema.Array(NonEmptyRefSchema).pipe(
    Schema.filter((value) => value.length > 0 || 'Expected source audit refs')
  ),
  manualProofRequirements: Schema.Array(NonEmptyRefSchema),
  notificationDeliveryReadinessState: DeliveryReadinessStateSchema,
  reportDeliveryExecutionState: ReportExecutionStateSchema,
  parentOwnedReportArtifactWritten: Schema.Boolean,
  parentOwnedReportReceiptRecorded: Schema.Boolean,
  parentNotificationUiDelivered: Schema.Literal(false),
  externalRuntimeReportDeliveryClaimed: Schema.Literal(false),
  providerDeliveryAttempted: Schema.Literal(false),
  providerReceiptIngested: Schema.Literal(false),
  finalPolicyDecisionClaimed: Schema.Literal(false),
  enforcementClaimed: Schema.Literal(false),
  createdAt: NonEmptyRefSchema,
}).pipe(
  Schema.filter(
    (row) =>
      deliveryReadinessRowIsHonest(row) ||
      'Expected social parent notification delivery readiness rows to preserve no-delivery boundaries'
  )
);

export const SocialParentNotificationDeliveryReadModelSnapshotSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal('social-parent-notification-delivery-read-model'),
    readinessId: NonEmptyRefSchema,
    generatedAt: NonEmptyRefSchema,
    sourceReportWriterProofRef: NonEmptyRefSchema,
    rows: Schema.Array(DeliveryReadinessRowSchema).pipe(
      Schema.filter((value) => value.length > 0 || 'Expected parent notification delivery readiness rows')
    ),
    nonClaims: Schema.Array(NonClaimSchema),
    parentReportStatusReadyCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
    manualRequiredCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
    unavailableCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
    parentNotificationUiDeliveryClaimed: Schema.Literal(false),
    externalRuntimeReportDeliveryClaimed: Schema.Literal(false),
    finalPolicyExecutionClaimed: Schema.Literal(false),
    enforcementClaimed: Schema.Literal(false),
  }).pipe(
    Schema.filter(
      (snapshot) =>
        deliveryReadinessSnapshotIsHonest(snapshot) ||
        'Expected parent notification delivery readiness counts and non-claims to match rows'
    )
  )
);

export type SocialParentNotificationDeliveryReadModelSnapshot = DeliveryReadinessSnapshot;
export type SocialParentNotificationDeliveryReadModelRow = DeliveryReadinessRow;

export type AgentSocialParentNotificationDeliveryReadModelFailureReason =
  | 'wrong-event'
  | 'missing-json-field'
  | 'invalid-json'
  | 'invalid-payload';

export type AgentSocialParentNotificationDeliveryReadModelResult =
  | {
      readonly ok: true;
      readonly value: SocialParentNotificationDeliveryReadModelSnapshot;
    }
  | {
      readonly ok: false;
      readonly reason: AgentSocialParentNotificationDeliveryReadModelFailureReason;
    };

export function parseAgentSocialParentNotificationDeliveryReadModelEvent(
  event: AgentEventEnvelope
): AgentSocialParentNotificationDeliveryReadModelResult {
  if (event.event !== AgentEvent.BrowserSocialParentNotificationDeliveryReadModelReported) {
    return adapterFailure('wrong-event');
  }

  const raw = event.payload[AgentProtocolDefaults.Field.BrowserSocialParentNotificationDeliveryReadModel];
  if (!isAgentProtocolLogText(raw)) {
    return adapterFailure('missing-json-field');
  }

  let decoded: unknown;
  try {
    decoded = JSON.parse(raw);
  } catch {
    return adapterFailure('invalid-json');
  }

  const parsed = SocialParentNotificationDeliveryReadModelSnapshotSchema.safeParse(decoded);
  if (!parsed.success || parsed.data === undefined) {
    return adapterFailure('invalid-payload');
  }

  return {
    ok: true,
    value: parsed.data,
  };
}

function deliveryReadinessRowIsHonest(row: DeliveryReadinessRow): boolean {
  if (row.notificationDeliveryReadinessState === 'parent-report-status-ready') {
    return reportReadyRowIsHonest(row);
  }
  if (row.notificationDeliveryReadinessState === 'unavailable') {
    return unavailableRowIsHonest(row);
  }
  return manualRequiredRowIsHonest(row);
}

function reportReadyRowIsHonest(row: DeliveryReadinessRow): boolean {
  return (
    row.parentVisibleReportStatusRef !== null &&
    row.parentNotificationUiRef === null &&
    row.parentReportRef !== null &&
    row.reportArtifactRef !== null &&
    row.reportReceiptRef !== null &&
    row.reportDeliveryExecutionState === 'parent-owned-report-ready' &&
    row.manualProofRequirements.length === 0 &&
    row.parentOwnedReportArtifactWritten &&
    row.parentOwnedReportReceiptRecorded &&
    rowClaimsStayFalse(row)
  );
}

function manualRequiredRowIsHonest(row: DeliveryReadinessRow): boolean {
  return (
    row.parentNotificationUiRef === null &&
    row.reportDeliveryExecutionState === 'manual-required' &&
    row.manualProofRequirements.length > 0 &&
    rowClaimsStayFalse(row)
  );
}

function unavailableRowIsHonest(row: DeliveryReadinessRow): boolean {
  return (
    row.parentNotificationUiRef === null &&
    row.parentReportRef === null &&
    row.reportArtifactRef === null &&
    row.reportReceiptRef === null &&
    row.reportDeliveryExecutionState === 'unavailable' &&
    row.manualProofRequirements.length > 0 &&
    rowClaimsStayFalse(row)
  );
}

function deliveryReadinessSnapshotIsHonest(snapshot: DeliveryReadinessSnapshot): boolean {
  return (
    snapshot.parentReportStatusReadyCount === countRows(snapshot.rows, 'parent-report-status-ready') &&
    snapshot.manualRequiredCount === countRows(snapshot.rows, 'manual-required') &&
    snapshot.unavailableCount === countRows(snapshot.rows, 'unavailable') &&
    snapshotHasRequiredNonClaims(snapshot.nonClaims) &&
    !snapshot.parentNotificationUiDeliveryClaimed &&
    !snapshot.externalRuntimeReportDeliveryClaimed &&
    !snapshot.finalPolicyExecutionClaimed &&
    !snapshot.enforcementClaimed
  );
}

function snapshotHasRequiredNonClaims(nonClaims: readonly string[]): boolean {
  return requiredNonClaims().every((nonClaim) => nonClaims.includes(nonClaim));
}

function requiredNonClaims(): readonly string[] {
  return [
    'no-parent-notification-ui-delivery',
    'no-external-runtime-report-delivery',
    'no-provider-delivery',
    'no-provider-receipt-ingestion',
    'no-final-policy-execution',
    'no-enforcement',
  ];
}

function rowClaimsStayFalse(row: {
  readonly parentNotificationUiDelivered: false;
  readonly externalRuntimeReportDeliveryClaimed: false;
  readonly providerDeliveryAttempted: false;
  readonly providerReceiptIngested: false;
  readonly finalPolicyDecisionClaimed: false;
  readonly enforcementClaimed: false;
}): boolean {
  return (
    !row.parentNotificationUiDelivered &&
    !row.externalRuntimeReportDeliveryClaimed &&
    !row.providerDeliveryAttempted &&
    !row.providerReceiptIngested &&
    !row.finalPolicyDecisionClaimed &&
    !row.enforcementClaimed
  );
}

function countRows(rows: readonly { readonly notificationDeliveryReadinessState: string }[], state: string): number {
  return rows.filter((row) => row.notificationDeliveryReadinessState === state).length;
}

function adapterFailure(
  reason: AgentSocialParentNotificationDeliveryReadModelFailureReason
): AgentSocialParentNotificationDeliveryReadModelResult {
  return {
    ok: false,
    reason,
  };
}
