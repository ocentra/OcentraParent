import { NonEmptyStringSchema, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { AgentEvent, AgentProtocolDefaults, isAgentProtocolLogText, type AgentEventEnvelope } from './contracts';

const ParentSurfaceStateSchema = Schema.Literal('manual-action-required', 'unavailable-visible');
const HistoryVisibilitySchema = Schema.Literal('history-row-visible', 'manual-review-only', 'unavailable-row-visible');
const PreferenceVisibilitySchema = Schema.Literal('preference-setup-required', 'preference-disabled-visible');
const NonClaimSchema = Schema.Literal(
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
  'no-enforcement'
);

export type SocialAlertReportParentSurfaceReadModelRow = {
  readonly surfaceRowId: string;
  readonly sourceProviderHandoffRowId: string;
  readonly sourcePreferenceHandoffRowId: string;
  readonly sourceIntentRef: string;
  readonly parentSurfaceStatus: 'manual-action-required' | 'unavailable-visible';
  readonly historyVisibility: 'history-row-visible' | 'manual-review-only' | 'unavailable-row-visible';
  readonly preferenceVisibility: 'preference-setup-required' | 'preference-disabled-visible';
  readonly notificationStatusRef: string;
  readonly sourcePreferenceStatusRef: string;
  readonly drillInRefs: readonly string[];
  readonly auditRefs: readonly string[];
  readonly manualProofRequirements: readonly string[];
  readonly minimalSurfacePayloadBoundary: string;
  readonly sensitiveDetailIncluded: false;
  readonly parentNotificationUiRendered: false;
  readonly parentNotificationPreferenceUiRendered: false;
  readonly parentFrequencyControlUiRendered: false;
  readonly parentNotificationHistoryUiRendered: false;
  readonly providerDeliveryClaimed: false;
  readonly providerReceiptClaimed: false;
  readonly parentPreferenceMutationClaimed: false;
  readonly childDeliveryClaimed: false;
  readonly quietHoursTimerRuntimeClaimed: false;
  readonly reportDeliveryExecutionClaimed: false;
  readonly finalPolicyExecutionClaimed: false;
  readonly adapterDispatchClaimed: false;
  readonly enforcementClaimed: false;
};

export type SocialAlertReportParentSurfaceReadModelSnapshot = {
  readonly schemaVersion: 'social-alert-report-parent-surface-read-model';
  readonly intentId: string;
  readonly generatedAt: string;
  readonly sourceProviderStatusHandoffId: string;
  readonly sourcePreferenceStatusHandoffId: string;
  readonly rows: readonly SocialAlertReportParentSurfaceReadModelRow[];
  readonly manualActionRequiredCount: number;
  readonly unavailableVisibleCount: number;
  readonly historyVisibleCount: number;
  readonly preferenceSetupRequiredCount: number;
  readonly parentSurfaceNonClaims: readonly string[];
  readonly parentNotificationUiRendered: false;
  readonly parentNotificationPreferenceUiRendered: false;
  readonly parentFrequencyControlUiRendered: false;
  readonly parentNotificationHistoryUiRendered: false;
  readonly providerDeliveryRuntimeClaimed: false;
  readonly providerReceiptIngestionClaimed: false;
  readonly providerCredentialsClaimed: false;
  readonly cloudRoutingClaimed: false;
  readonly childDeliveryClaimed: false;
  readonly quietHoursTimerRuntimeClaimed: false;
  readonly retryExecutionRuntimeClaimed: false;
  readonly productionDurableOutboxStorageClaimed: false;
  readonly adapterDispatchClaimed: false;
  readonly reportDeliveryExecutionClaimed: false;
  readonly finalPolicyExecutionClaimed: false;
  readonly connectorNativeRuntimeClaimed: false;
  readonly enforcementClaimed: false;
};

const ParentSurfaceRowSchema = Schema.Struct({
  surfaceRowId: NonEmptyStringSchema,
  sourceProviderHandoffRowId: NonEmptyStringSchema,
  sourcePreferenceHandoffRowId: NonEmptyStringSchema,
  sourceIntentRef: NonEmptyStringSchema,
  parentSurfaceStatus: ParentSurfaceStateSchema,
  historyVisibility: HistoryVisibilitySchema,
  preferenceVisibility: PreferenceVisibilitySchema,
  notificationStatusRef: NonEmptyStringSchema,
  sourcePreferenceStatusRef: NonEmptyStringSchema,
  drillInRefs: Schema.Array(NonEmptyStringSchema),
  auditRefs: Schema.Array(NonEmptyStringSchema),
  manualProofRequirements: Schema.Array(NonEmptyStringSchema),
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
}).pipe(
  Schema.filter(
    (row) =>
      parentSurfaceRowIsHonest(row) ||
      'Expected parent-surface stream rows to stay status-only with no delivery or enforcement claims'
  )
);

export const SocialAlertReportParentSurfaceReadModelSnapshotSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal('social-alert-report-parent-surface-read-model'),
    intentId: NonEmptyStringSchema,
    generatedAt: NonEmptyStringSchema,
    sourceProviderStatusHandoffId: NonEmptyStringSchema,
    sourcePreferenceStatusHandoffId: NonEmptyStringSchema,
    rows: Schema.Array(ParentSurfaceRowSchema).pipe(
      Schema.filter((value) => value.length > 0 || 'Expected parent-surface rows')
    ),
    manualActionRequiredCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
    unavailableVisibleCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
    historyVisibleCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
    preferenceSetupRequiredCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
    parentSurfaceNonClaims: Schema.Array(NonClaimSchema),
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
  }).pipe(
    Schema.filter(
      (snapshot) =>
        parentSurfaceSnapshotIsHonest(snapshot) || 'Expected parent-surface counts and no-claim fields to match rows'
    )
  )
);

export type AgentSocialAlertReportParentSurfaceReadModelFailureReason =
  | 'wrong-event'
  | 'missing-json-field'
  | 'invalid-json'
  | 'invalid-payload';

export type AgentSocialAlertReportParentSurfaceReadModelResult =
  | {
      readonly ok: true;
      readonly value: SocialAlertReportParentSurfaceReadModelSnapshot;
    }
  | {
      readonly ok: false;
      readonly reason: AgentSocialAlertReportParentSurfaceReadModelFailureReason;
    };

export function parseAgentSocialAlertReportParentSurfaceReadModelEvent(
  event: AgentEventEnvelope
): AgentSocialAlertReportParentSurfaceReadModelResult {
  if (event.event !== AgentEvent.BrowserSocialAlertReportParentSurfaceReadModelReported) {
    return failure('wrong-event');
  }

  const raw = event.payload[AgentProtocolDefaults.Field.BrowserSocialAlertReportParentSurfaceReadModel];
  if (!isAgentProtocolLogText(raw)) {
    return failure('missing-json-field');
  }

  let decoded: unknown;
  try {
    decoded = JSON.parse(raw);
  } catch {
    return failure('invalid-json');
  }

  const parsed = SocialAlertReportParentSurfaceReadModelSnapshotSchema.safeParse(decoded);
  if (!parsed.success || parsed.data === undefined) {
    return failure('invalid-payload');
  }

  return { ok: true, value: parsed.data };
}

function parentSurfaceRowIsHonest(row: SocialAlertReportParentSurfaceReadModelRow): boolean {
  const rowHasRequiredRefs =
    row.drillInRefs.length > 0 && row.auditRefs.length > 0 && row.manualProofRequirements.length > 0;
  const rowVisibilityIsCoherent =
    row.parentSurfaceStatus === 'manual-action-required'
      ? row.historyVisibility === 'history-row-visible'
      : row.historyVisibility === 'unavailable-row-visible';
  return rowHasRequiredRefs && rowVisibilityIsCoherent && rowClaimsStayFalse(row);
}

function rowClaimsStayFalse(row: SocialAlertReportParentSurfaceReadModelRow): boolean {
  return (
    rowKeepsUiClaimsFalse(row) &&
    rowKeepsDeliveryClaimsFalse(row) &&
    !row.sensitiveDetailIncluded &&
    !row.finalPolicyExecutionClaimed &&
    !row.adapterDispatchClaimed &&
    !row.enforcementClaimed
  );
}

function rowKeepsUiClaimsFalse(row: SocialAlertReportParentSurfaceReadModelRow): boolean {
  return (
    !row.parentNotificationUiRendered &&
    !row.parentNotificationPreferenceUiRendered &&
    !row.parentFrequencyControlUiRendered &&
    !row.parentNotificationHistoryUiRendered
  );
}

function rowKeepsDeliveryClaimsFalse(row: SocialAlertReportParentSurfaceReadModelRow): boolean {
  return (
    !row.providerDeliveryClaimed &&
    !row.providerReceiptClaimed &&
    !row.parentPreferenceMutationClaimed &&
    !row.childDeliveryClaimed &&
    !row.quietHoursTimerRuntimeClaimed &&
    !row.reportDeliveryExecutionClaimed
  );
}

function parentSurfaceSnapshotIsHonest(snapshot: SocialAlertReportParentSurfaceReadModelSnapshot): boolean {
  return (
    snapshot.manualActionRequiredCount === countRows(snapshot, 'manual-action-required') &&
    snapshot.unavailableVisibleCount === countRows(snapshot, 'unavailable-visible') &&
    snapshot.historyVisibleCount === snapshot.rows.length &&
    snapshot.preferenceSetupRequiredCount ===
      snapshot.rows.filter((row) => row.preferenceVisibility === 'preference-setup-required').length &&
    snapshot.parentSurfaceNonClaims.length >= 11 &&
    snapshotClaimsStayFalse(snapshot)
  );
}

function snapshotClaimsStayFalse(snapshot: SocialAlertReportParentSurfaceReadModelSnapshot): boolean {
  return (
    snapshotKeepsUiClaimsFalse(snapshot) &&
    snapshotKeepsDeliveryClaimsFalse(snapshot) &&
    !snapshot.adapterDispatchClaimed &&
    !snapshot.finalPolicyExecutionClaimed &&
    !snapshot.connectorNativeRuntimeClaimed &&
    !snapshot.enforcementClaimed
  );
}

function snapshotKeepsUiClaimsFalse(snapshot: SocialAlertReportParentSurfaceReadModelSnapshot): boolean {
  return (
    !snapshot.parentNotificationUiRendered &&
    !snapshot.parentNotificationPreferenceUiRendered &&
    !snapshot.parentFrequencyControlUiRendered &&
    !snapshot.parentNotificationHistoryUiRendered
  );
}

function snapshotKeepsDeliveryClaimsFalse(snapshot: SocialAlertReportParentSurfaceReadModelSnapshot): boolean {
  return (
    !snapshot.providerDeliveryRuntimeClaimed &&
    !snapshot.providerReceiptIngestionClaimed &&
    !snapshot.providerCredentialsClaimed &&
    !snapshot.cloudRoutingClaimed &&
    !snapshot.childDeliveryClaimed &&
    !snapshot.quietHoursTimerRuntimeClaimed &&
    !snapshot.retryExecutionRuntimeClaimed &&
    !snapshot.productionDurableOutboxStorageClaimed &&
    !snapshot.reportDeliveryExecutionClaimed
  );
}

function countRows(snapshot: SocialAlertReportParentSurfaceReadModelSnapshot, state: string): number {
  return snapshot.rows.filter((row) => row.parentSurfaceStatus === state).length;
}

function failure(reason: AgentSocialAlertReportParentSurfaceReadModelFailureReason) {
  return { ok: false, reason } as const;
}
