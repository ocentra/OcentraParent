import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from './effect';
import { ParentEvidenceReferenceIdSchema } from './family-reference-primitives';

export const BrowserGameApprovalRequestIdSchema = withParser(
  brandedNonEmptyStringSchema('BrowserGameApprovalRequestId')
);

export const BrowserGameApprovalDecisionIdSchema = withParser(
  brandedNonEmptyStringSchema('BrowserGameApprovalDecisionId')
);

export const BrowserGameApprovalRequestKindSchema = withParser(
  Schema.Literal(
    'game-account-creation',
    'game-login',
    'secondary-game-account',
    'game-purchase',
    'subscription-purchase',
    'loot-box-purchase',
    'virtual-currency-purchase',
    'game-download',
    'install-prompt',
    'wallet-or-gambling-payment',
    'cloud-gaming-start',
    'unknown-game-start'
  )
);

export const BrowserGameApprovalRequestStateSchema = withParser(
  Schema.Literal('pending-contract-only', 'blocked-candidate', 'manual-required', 'unavailable')
);

export const BrowserGameApprovalDecisionKindSchema = withParser(
  Schema.Literal(
    'approve-once-candidate',
    'approve-account-candidate',
    'approve-purchase-candidate',
    'deny-candidate',
    'block-candidate',
    'manual-required'
  )
);

export const BrowserGameApprovalDecisionStateSchema = withParser(
  Schema.Literal('recorded-contract-only', 'manual-required')
);

export const BrowserGameApprovalConfidenceSchema = withParser(Schema.Literal('high', 'medium', 'low', 'unknown'));

export const BrowserGameApprovalReasonCodeSchema = withParser(
  Schema.Literal(
    'account-creation-route',
    'login-route',
    'secondary-account-route',
    'purchase-route',
    'subscription-route',
    'loot-box-route',
    'virtual-currency-route',
    'download-or-install-route',
    'wallet-payment-risk',
    'gambling-like-payment-risk',
    'unknown-game-route',
    'cloud-gaming-route',
    'educational-account-requires-approval',
    'parent-rule-requires-approval',
    'parent-rule-blocks-flow',
    'missing-route-proof',
    'manual-required'
  )
);

export const BrowserGameApprovalEvidenceRefsSchema = Schema.Array(ParentEvidenceReferenceIdSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected browser-game approval evidence refs')
);

export const BrowserGameApprovalReasonCodesSchema = Schema.Array(BrowserGameApprovalReasonCodeSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected browser-game approval reason codes')
);

export type BrowserGameApprovalRequestKind = Infer<typeof BrowserGameApprovalRequestKindSchema>;
export type BrowserGameApprovalRequestState = Infer<typeof BrowserGameApprovalRequestStateSchema>;
export type BrowserGameApprovalDecisionKind = Infer<typeof BrowserGameApprovalDecisionKindSchema>;
export type BrowserGameApprovalDecisionState = Infer<typeof BrowserGameApprovalDecisionStateSchema>;
export type BrowserGameApprovalReasonCode = Infer<typeof BrowserGameApprovalReasonCodeSchema>;
