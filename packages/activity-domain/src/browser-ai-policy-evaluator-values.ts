import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';

const NonEmptyPolicyEvaluatorText = Schema.String.pipe(Schema.minLength(1));

export const BrowserAiPolicyEvaluatorRequestIdSchema = withParser(
  NonEmptyPolicyEvaluatorText.pipe(Schema.brand('BrowserAiPolicyEvaluatorRequestId'))
);
export const BrowserPolicyDecisionIdSchema = withParser(
  NonEmptyPolicyEvaluatorText.pipe(Schema.brand('BrowserPolicyDecisionId'))
);
export const BrowserPolicyDecisionAuditRefSchema = withParser(
  NonEmptyPolicyEvaluatorText.pipe(Schema.brand('BrowserPolicyDecisionAuditRef'))
);
export const BrowserPolicyAdapterProofRefSchema = withParser(
  NonEmptyPolicyEvaluatorText.pipe(Schema.brand('BrowserPolicyAdapterProofRef'))
);

export const BrowserPolicyDecisionOutcomeSchema = withParser(
  Schema.Literal('allow', 'warn', 'ask_parent', 'time_limit', 'block', 'unknown')
);
export const BrowserPolicyEvaluatorModeSchema = withParser(
  Schema.Literal('observe_only', 'dry_run', 'active', 'manual_required', 'unavailable')
);
export const BrowserPolicyEvaluatorHandoffStateSchema = withParser(
  Schema.Literal('ready', 'degraded', 'manual_required', 'unavailable')
);
export const BrowserPolicyDecisionReasonCodeSchema = withParser(
  Schema.Literal(
    'explicit_parent_rule',
    'schedule_match',
    'ai_high_confidence',
    'ai_low_confidence',
    'memory_hit',
    'graph_ref',
    'adapter_proof_missing',
    'parent_fallback',
    'degraded_provider',
    'unknown_evidence'
  )
);

export type BrowserPolicyDecisionOutcome = Infer<typeof BrowserPolicyDecisionOutcomeSchema>;
export type BrowserPolicyEvaluatorMode = Infer<typeof BrowserPolicyEvaluatorModeSchema>;
export type BrowserPolicyDecisionReasonCode = Infer<typeof BrowserPolicyDecisionReasonCodeSchema>;
