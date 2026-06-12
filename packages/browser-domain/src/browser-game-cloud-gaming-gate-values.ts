import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentEvidenceReferenceIdSchema } from '@ocentra-parent/family-domain/reference-primitives';

const NonEmptyBrowserGameCloudGateText = Schema.String.pipe(Schema.minLength(1));

export const BrowserGameCloudGateRequestIdSchema = withParser(
  NonEmptyBrowserGameCloudGateText.pipe(Schema.brand('BrowserGameCloudGateRequestId'))
);

export const BrowserGameCloudGateDecisionIdSchema = withParser(
  NonEmptyBrowserGameCloudGateText.pipe(Schema.brand('BrowserGameCloudGateDecisionId'))
);

export const BrowserGameCloudPlatformSchema = withParser(
  Schema.Literal(
    'xbox-cloud-gaming',
    'geforce-now',
    'amazon-luna',
    'boosteroid',
    'playstation-cloud',
    'shadow-cloud-pc',
    'now-gg',
    'unknown-cloud-gaming'
  )
);

export const BrowserGameCloudGateSubjectSchema = withParser(
  Schema.Literal(
    'cloud-platform-session',
    'unknown-cloud-game',
    'mature-cloud-game',
    'school-night-cloud-gaming',
    'time-budget-cloud-gaming'
  )
);

export const BrowserGameCloudGateStateSchema = withParser(
  Schema.Literal('candidate', 'manual-required', 'unavailable')
);

export const BrowserGameCloudGateActionCandidateSchema = withParser(
  Schema.Literal(
    'allow-window-candidate',
    'parent-review-candidate',
    'block-candidate',
    'time-limit-candidate',
    'manual-review-candidate',
    'unknown-fallback-candidate'
  )
);

export const BrowserGameCloudGateDecisionKindSchema = withParser(
  Schema.Literal(
    'allow-session-candidate',
    'parent-review-candidate',
    'deny-session-candidate',
    'time-limit-session-candidate',
    'manual-required'
  )
);

export const BrowserGameCloudGateDecisionStateSchema = withParser(
  Schema.Literal('recorded-contract-only', 'manual-required')
);

export const BrowserGameCloudGateConfidenceSchema = withParser(Schema.Literal('high', 'medium', 'low', 'unknown'));

export const BrowserGameCloudGateSignalKindSchema = withParser(
  Schema.Literal(
    'known-cloud-domain',
    'streaming-session-route',
    'gamepad-api',
    'fullscreen-pointer-lock',
    'high-bandwidth-stream',
    'low-latency-network',
    'platform-title-metadata',
    'platform-rating-metadata',
    'unknown-title-fallback'
  )
);

export const BrowserGameCloudGateReasonCodeSchema = withParser(
  Schema.Literal(
    'known-cloud-domain',
    'streaming-route',
    'unknown-cloud-title',
    'title-metadata-present',
    'rating-metadata-present',
    'mature-title-risk',
    'parent-approval-required',
    'schedule-blocked',
    'time-budget-candidate',
    'missing-platform-proof',
    'content-frame-unavailable',
    'cloud-title-unavailable',
    'runtime-signal-unavailable',
    'manual-required'
  )
);

export const BrowserGameCloudGateEvidenceRefsSchema = Schema.Array(ParentEvidenceReferenceIdSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected browser-game cloud gate evidence refs')
);

export const BrowserGameCloudGateSignalKindsSchema = Schema.Array(BrowserGameCloudGateSignalKindSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected browser-game cloud gate signal kinds')
);

export const BrowserGameCloudGateReasonCodesSchema = Schema.Array(BrowserGameCloudGateReasonCodeSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected browser-game cloud gate reason codes')
);

export type BrowserGameCloudGateActionCandidate = Infer<typeof BrowserGameCloudGateActionCandidateSchema>;
export type BrowserGameCloudGateDecisionKind = Infer<typeof BrowserGameCloudGateDecisionKindSchema>;
export type BrowserGameCloudGateReasonCode = Infer<typeof BrowserGameCloudGateReasonCodeSchema>;
export type BrowserGameCloudGateState = Infer<typeof BrowserGameCloudGateStateSchema>;
export type BrowserGameCloudGateSubject = Infer<typeof BrowserGameCloudGateSubjectSchema>;
export type BrowserGameCloudGateSignalKind = Infer<typeof BrowserGameCloudGateSignalKindSchema>;
