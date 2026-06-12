import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';

const NonEmptySocialChildUxText = Schema.String.pipe(Schema.minLength(1));

export const SocialChildApprovalBlockUxSchemaVersionSchema = withParser(
  Schema.Literal('social-child-approval-block-ux-contract')
);

export const SocialChildApprovalBlockSurfaceIdSchema = withParser(
  NonEmptySocialChildUxText.pipe(Schema.brand('SocialChildApprovalBlockSurfaceId'))
);

export const SocialChildApprovalBlockSurfaceKindSchema = withParser(
  Schema.Literal(
    'approval-request-pending',
    'blocked-social-route-candidate',
    'warning-social-route-candidate',
    'manual-review-required',
    'time-limit-candidate',
    'native-app-unavailable'
  )
);

export const SocialChildApprovalBlockStateSchema = withParser(
  Schema.Literal('waiting-parent', 'blocked-contract-only', 'child-readable', 'manual-required', 'unavailable')
);

export const SocialChildApprovalBlockActionSchema = withParser(
  Schema.Literal(
    'parent-review',
    'acknowledge-warning',
    'open-safe-back',
    'wait-for-parent',
    'manual-review',
    'no-action'
  )
);

export const SocialChildApprovalBlockReasonSchema = withParser(
  Schema.Literal(
    'parent-approval-needed',
    'route-block-candidate',
    'route-warning-candidate',
    'manual-review-needed',
    'time-limit-not-applied',
    'native-app-proof-unavailable'
  )
);

export type SocialChildApprovalBlockSurfaceKind = Infer<typeof SocialChildApprovalBlockSurfaceKindSchema>;
