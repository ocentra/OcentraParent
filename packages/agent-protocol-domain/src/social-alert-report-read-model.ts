import {
  SocialAlertReportIntentSchema,
  type SocialAlertReportIntent,
} from '@ocentra-parent/browser-domain/social-alert-report-intent';
import {
  type Infer,
  NonEmptyStringSchema,
  Schema,
  withParser,
} from '@ocentra-parent/schema-domain/effect';
import { AgentEvent, AgentProtocolDefaults, isAgentProtocolLogText, type AgentEventEnvelope } from './contracts';

const SocialAlertReportReadModelClaimBoundariesSchema = Schema.Struct({
  providerDelivery: Schema.Literal('not-claimed'),
  reportDelivery: Schema.Literal('not-claimed'),
  parentNotificationUi: Schema.Literal('not-claimed'),
  finalPolicyDecision: Schema.Literal('not-claimed'),
  enforcement: Schema.Literal('not-claimed'),
});

const SocialAlertReportProviderStatusRowSchema = Schema.Struct({
  statusEntryId: NonEmptyStringSchema,
  sourceIntentRef: NonEmptyStringSchema,
  sourcePreflightStatus: Schema.Literal('provider-adapter-required', 'manual-required', 'unavailable'),
  providerStatus: Schema.Literal('manual-required', 'unavailable'),
  statusProofState: Schema.Literal('manual-action-required', 'provider-unavailable-contract'),
  deliveryClaimState: Schema.Literal('not-observed', 'not-implemented'),
  providerAttemptRef: NonEmptyStringSchema,
  readinessRefs: Schema.Array(NonEmptyStringSchema).pipe(
    Schema.filter((value) => value.length > 0 || 'Expected provider status readiness refs')
  ),
  providerReceiptRefs: Schema.Array(NonEmptyStringSchema),
  manualProofRequirements: Schema.Array(NonEmptyStringSchema).pipe(
    Schema.filter((value) => value.length > 0 || 'Expected provider status manual proof requirements')
  ),
  providerDeliveryImplemented: Schema.Literal(false),
  providerDeliveryObserved: Schema.Literal(false),
  deliveredNotificationClaimed: Schema.Literal(false),
  sensitiveProviderPayloadClaimed: Schema.Literal(false),
  providerStoresChildEvidenceClaimed: Schema.Literal(false),
  lastCheckedAt: NonEmptyStringSchema,
}).pipe(
  Schema.filter(
    (row) =>
      socialAlertReportProviderStatusRowIsHonest(row) ||
      'Expected social alert/report provider status rows to preserve manual-required/unavailable ' +
        'no-delivery boundaries'
  )
);

export const SocialAlertReportReadModelSnapshotSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal('social-alert-report-read-model'),
    familyId: NonEmptyStringSchema,
    childProfileId: NonEmptyStringSchema,
    generatedAt: NonEmptyStringSchema,
    intents: Schema.Array(SocialAlertReportIntentSchema).pipe(
      Schema.filter((value) => value.length > 0 || 'Expected social alert/report intent rows')
    ),
    providerStatusRows: Schema.Array(SocialAlertReportProviderStatusRowSchema).pipe(
      Schema.filter((value) => value.length > 0 || 'Expected social alert/report provider status rows')
    ),
    claimBoundaries: SocialAlertReportReadModelClaimBoundariesSchema,
  }).pipe(
    Schema.filter(
      (snapshot) =>
        snapshot.intents.every(
          (intent) =>
            !intent.providerDeliveryAttempted &&
            !intent.providerDeliveryObserved &&
            !intent.providerReceiptIngested &&
            !intent.parentNotificationUiClaimed &&
            !intent.reportDeliveryClaimed &&
            !intent.finalPolicyDecisionClaimed &&
            !intent.enforcementClaimed
        ) || 'Expected social alert/report read model to preserve no-delivery no-enforcement boundaries'
    )
  )
);

export type SocialAlertReportReadModelSnapshot = Infer<typeof SocialAlertReportReadModelSnapshotSchema>;
export type SocialAlertReportProviderStatusRow = Infer<typeof SocialAlertReportProviderStatusRowSchema>;

export type AgentSocialAlertReportReadModelFailureReason =
  | 'wrong-event'
  | 'missing-json-field'
  | 'invalid-json'
  | 'invalid-payload';

export type AgentSocialAlertReportReadModelResult =
  | {
      readonly ok: true;
      readonly value: SocialAlertReportReadModelSnapshot;
    }
  | {
      readonly ok: false;
      readonly reason: AgentSocialAlertReportReadModelFailureReason;
    };

export function parseAgentSocialAlertReportReadModelEvent(
  event: AgentEventEnvelope
): AgentSocialAlertReportReadModelResult {
  if (event.event !== AgentEvent.BrowserSocialAlertReportReadModelReported) {
    return adapterFailure('wrong-event');
  }

  const raw = event.payload[AgentProtocolDefaults.Field.BrowserSocialAlertReportReadModel];
  if (!isAgentProtocolLogText(raw)) {
    return adapterFailure('missing-json-field');
  }

  let decoded: unknown;
  try {
    decoded = JSON.parse(raw);
  } catch {
    return adapterFailure('invalid-json');
  }

  const parsed = SocialAlertReportReadModelSnapshotSchema.safeParse(decoded);
  if (!parsed.success || parsed.data === undefined) {
    return adapterFailure('invalid-payload');
  }

  return {
    ok: true,
    value: parsed.data,
  };
}

export type SocialAlertReportReadModelIntent = SocialAlertReportIntent;

function socialAlertReportProviderStatusRowIsHonest(row: {
  readonly sourcePreflightStatus: 'provider-adapter-required' | 'manual-required' | 'unavailable';
  readonly providerStatus: 'manual-required' | 'unavailable';
  readonly statusProofState: 'manual-action-required' | 'provider-unavailable-contract';
  readonly deliveryClaimState: 'not-observed' | 'not-implemented';
  readonly providerReceiptRefs: readonly string[];
  readonly providerDeliveryImplemented: false;
  readonly providerDeliveryObserved: false;
  readonly deliveredNotificationClaimed: false;
  readonly sensitiveProviderPayloadClaimed: false;
  readonly providerStoresChildEvidenceClaimed: false;
}): boolean {
  const unavailable = row.sourcePreflightStatus === 'unavailable';
  const expectedProviderStatus = unavailable ? 'unavailable' : 'manual-required';
  const expectedProofState = unavailable ? 'provider-unavailable-contract' : 'manual-action-required';
  const expectedDeliveryClaim = unavailable ? 'not-implemented' : 'not-observed';

  return (
    row.providerStatus === expectedProviderStatus &&
    row.statusProofState === expectedProofState &&
    row.deliveryClaimState === expectedDeliveryClaim &&
    row.providerReceiptRefs.length === 0 &&
    row.providerDeliveryImplemented === false &&
    row.providerDeliveryObserved === false &&
    row.deliveredNotificationClaimed === false &&
    row.sensitiveProviderPayloadClaimed === false &&
    row.providerStoresChildEvidenceClaimed === false
  );
}

function adapterFailure(reason: AgentSocialAlertReportReadModelFailureReason): AgentSocialAlertReportReadModelResult {
  return {
    ok: false,
    reason,
  };
}
