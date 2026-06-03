import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';

const NonEmptySocialAuditExplanationText = Schema.String.pipe(Schema.minLength(1));

export const SocialAuditExplanationReadModelSchemaVersionSchema = withParser(
  Schema.Literal('social-audit-explanation-read-model')
);

export const SocialAuditExplanationEventIdSchema = withParser(
  NonEmptySocialAuditExplanationText.pipe(Schema.brand('SocialAuditExplanationEventId'))
);

export const SocialAuditExplanationSnapshotIdSchema = withParser(
  NonEmptySocialAuditExplanationText.pipe(Schema.brand('SocialAuditExplanationSnapshotId'))
);

export const SocialAuditExplanationSubjectKindSchema = withParser(
  Schema.Literal(
    'account-approval',
    'feed-video-gate',
    'native-app-gap',
    'connector-boundary',
    'decision-memory',
    'manual-required-gap'
  )
);

export const SocialAuditExplanationStatusSchema = withParser(
  Schema.Literal('ready-for-parent', 'manual-required', 'contract-only', 'unavailable')
);

export const SocialAuditExplanationDecisionStateSchema = withParser(
  Schema.Literal('candidate-only', 'parent-recorded', 'manual-required', 'unavailable')
);

export const SocialAuditExplanationEvidenceKindSchema = withParser(
  Schema.Literal(
    'route-evidence',
    'policy-candidate',
    'parent-approval',
    'decision-memory',
    'connector-boundary',
    'native-capability',
    'manual-gap'
  )
);

export const SocialAuditExplanationAudienceSchema = withParser(
  Schema.Literal('parent', 'audit-log', 'support-redacted')
);

export const SocialAuditExplanationReasonSchema = withParser(
  Schema.Literal(
    'evidence-linked',
    'policy-candidate-linked',
    'parent-decision-linked',
    'memory-linked',
    'connector-boundary-linked',
    'native-app-manual-required',
    'manual-review-required',
    'missing-runtime-proof'
  )
);

export type SocialAuditExplanationEvidenceKind = Infer<typeof SocialAuditExplanationEvidenceKindSchema>;
export type SocialAuditExplanationSubjectKind = Infer<typeof SocialAuditExplanationSubjectKindSchema>;
