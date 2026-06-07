import {
  SocialAlertReportIntentSchema,
  type SocialAlertReportIntent,
} from '@ocentra-parent/parent-domain/social-alert-report-intent';
import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { AgentEvent, AgentProtocolDefaults, isAgentProtocolLogText, type AgentEventEnvelope } from './contracts';

const SocialAlertReportReadModelClaimBoundariesSchema = Schema.Struct({
  providerDelivery: Schema.Literal('not-claimed'),
  reportDelivery: Schema.Literal('not-claimed'),
  parentNotificationUi: Schema.Literal('not-claimed'),
  finalPolicyDecision: Schema.Literal('not-claimed'),
  enforcement: Schema.Literal('not-claimed'),
});

export const SocialAlertReportReadModelSnapshotSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal('social-alert-report-read-model'),
    familyId: Schema.String.pipe(Schema.minLength(1)),
    childProfileId: Schema.String.pipe(Schema.minLength(1)),
    generatedAt: Schema.String.pipe(Schema.minLength(1)),
    intents: Schema.Array(SocialAlertReportIntentSchema).pipe(
      Schema.filter((value) => value.length > 0 || 'Expected social alert/report intent rows')
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

function adapterFailure(reason: AgentSocialAlertReportReadModelFailureReason): AgentSocialAlertReportReadModelResult {
  return {
    ok: false,
    reason,
  };
}
