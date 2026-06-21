import { type Infer, NonEmptyStringSchema, Schema, withParser } from './effect';
import { SocialVideoSourceCustodySettingsSchema } from './agent-social-video-source-custody-settings';

export const SocialSourceCustodyMutationSnapshotSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal('social-source-custody-mutation-proof'),
    mutationId: NonEmptyStringSchema,
    requestedAt: NonEmptyStringSchema,
    appliedAt: NonEmptyStringSchema,
    mutationState: Schema.Literal('applied'),
    settings: SocialVideoSourceCustodySettingsSchema,
    evidenceRefs: Schema.Array(NonEmptyStringSchema).pipe(
      Schema.filter((value) => value.length > 0 || 'Expected social source custody mutation evidence refs')
    ),
    auditRefs: Schema.Array(NonEmptyStringSchema).pipe(
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
