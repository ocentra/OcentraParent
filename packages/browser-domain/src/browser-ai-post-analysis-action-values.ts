import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';

const NonEmptyPostAnalysisText = Schema.String.pipe(Schema.minLength(1));

export const BrowserAiPostAnalysisActionPlanIdSchema = withParser(
  NonEmptyPostAnalysisText.pipe(Schema.brand('BrowserAiPostAnalysisActionPlanId'))
);
export const BrowserAiPostAnalysisActionAuditRefSchema = withParser(
  NonEmptyPostAnalysisText.pipe(Schema.brand('BrowserAiPostAnalysisActionAuditRef'))
);

export const BrowserAiPostAnalysisActionLabelSchema = withParser(
  Schema.Literal(
    'background_reviewed',
    'continue_allowed',
    'warning_shown_after_review',
    'playback_stopped_after_review',
    'parent_approval_requested_after_review',
    'future_visits_blocked',
    'remembered_with_expiry',
    'manual_required',
    'no_action'
  )
);
export const BrowserAiPostAnalysisActionTriggerSchema = withParser(
  Schema.Literal('policy_decision', 'timeout_fallback', 'parent_override', 'memory_refresh')
);
export const BrowserAiPostAnalysisActionTimingSchema = withParser(
  Schema.Literal('before_playback', 'after_playback_started', 'background_only', 'future_visit_only')
);
export const BrowserAiPostAnalysisDeliveryStateSchema = withParser(
  Schema.Literal('modeled_only', 'pending', 'delivered', 'failed', 'manual_required', 'unavailable')
);

export type BrowserAiPostAnalysisActionLabel = Infer<typeof BrowserAiPostAnalysisActionLabelSchema>;
export type BrowserAiPostAnalysisDeliveryState = Infer<typeof BrowserAiPostAnalysisDeliveryStateSchema>;
