import {
  SocialVideoSourceCustodySettingsSchema,
  type SocialVideoSourceCustodySettings,
} from '@ocentra-parent/activity-domain/social-video-source-custody-settings';
import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { AgentEvent, AgentProtocolDefaults, isAgentProtocolLogText, type AgentEventEnvelope } from './contracts';

const NonEmptyTextSchema = Schema.String.pipe(Schema.minLength(1));

export const SocialSourceCustodyMutationSnapshotSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal('social-source-custody-mutation-proof'),
    mutationId: NonEmptyTextSchema,
    requestedAt: NonEmptyTextSchema,
    appliedAt: NonEmptyTextSchema,
    mutationState: Schema.Literal('applied'),
    settings: SocialVideoSourceCustodySettingsSchema,
    evidenceRefs: Schema.Array(NonEmptyTextSchema).pipe(
      Schema.filter((value) => value.length > 0 || 'Expected social source custody mutation evidence refs')
    ),
    auditRefs: Schema.Array(NonEmptyTextSchema).pipe(
      Schema.filter((value) => value.length > 0 || 'Expected social source custody mutation audit refs')
    ),
    serviceMutationExecuted: Schema.Literal(true),
    runtimeCustodyMutationApplied: Schema.Literal(true),
    rawContentCustodyClaimed: Schema.Literal(false),
    connectorApiCalled: Schema.Literal(false),
    finalPolicyDecisionClaimed: Schema.Literal(false),
    enforcementClaimed: Schema.Literal(false),
    productClaimReady: Schema.Literal(false),
  }).pipe(
    Schema.filter(
      (snapshot) =>
        (!snapshot.settings.rawMessageContentAllowed &&
          !snapshot.settings.rawVideoContentAllowed &&
          !snapshot.settings.screenshotCustodyAllowed &&
          !snapshot.settings.connectorApiCalled &&
          !snapshot.settings.finalPolicyDecisionClaimed &&
          !snapshot.settings.enforcementClaimed) ||
        'Expected source custody mutation proof to preserve ref-only no-policy no-enforcement settings'
    )
  )
);

export type SocialSourceCustodyMutationSnapshot = Infer<typeof SocialSourceCustodyMutationSnapshotSchema>;

export type AgentSocialSourceCustodyMutationFailureReason =
  | 'wrong-event'
  | 'missing-json-field'
  | 'invalid-json'
  | 'invalid-payload';

export type AgentSocialSourceCustodyMutationResult =
  | {
      readonly ok: true;
      readonly value: SocialSourceCustodyMutationSnapshot;
    }
  | {
      readonly ok: false;
      readonly reason: AgentSocialSourceCustodyMutationFailureReason;
    };

export function parseAgentSocialSourceCustodyMutationEvent(
  event: AgentEventEnvelope
): AgentSocialSourceCustodyMutationResult {
  if (event.event !== AgentEvent.BrowserSocialSourceCustodyMutationApplied) {
    return adapterFailure('wrong-event');
  }

  const raw = event.payload[AgentProtocolDefaults.Field.BrowserSocialSourceCustodyMutation];
  if (!isAgentProtocolLogText(raw)) {
    return adapterFailure('missing-json-field');
  }

  let decoded: unknown;
  try {
    decoded = JSON.parse(raw);
  } catch {
    return adapterFailure('invalid-json');
  }

  const parsed = SocialSourceCustodyMutationSnapshotSchema.safeParse(decoded);
  if (!parsed.success || parsed.data === undefined) {
    return adapterFailure('invalid-payload');
  }

  return {
    ok: true,
    value: parsed.data,
  };
}

export type SocialSourceCustodyMutationSettings = SocialVideoSourceCustodySettings;

function adapterFailure(reason: AgentSocialSourceCustodyMutationFailureReason): AgentSocialSourceCustodyMutationResult {
  return {
    ok: false,
    reason,
  };
}
