import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';

export const BrowserAiRemoteApprovalIdSchema = withParser(
  brandedNonEmptyStringSchema('BrowserAiRemoteApprovalId')
);
export const BrowserAiRemoteProviderIdSchema = withParser(
  brandedNonEmptyStringSchema('BrowserAiRemoteProviderId')
);
export const BrowserAiRemoteRouteIdSchema = withParser(
  brandedNonEmptyStringSchema('BrowserAiRemoteRouteId')
);

export const BrowserAiRemoteDataScopeSchema = withParser(
  Schema.Literal('url-shape', 'metadata-summary', 'memory-refs', 'graph-refs', 'parent-rule-refs', 'schedule-refs')
);
export const BrowserAiRemoteRetentionModeSchema = withParser(
  Schema.Literal('no-retention', 'vendor-retention-disabled', 'manual-required')
);
export const BrowserAiRemoteCapabilityStateSchema = withParser(
  Schema.Literal('available', 'disabled-by-parent', 'approval-missing', 'provider-unavailable', 'data-scope-rejected')
);
export const BrowserAiRemoteDegradedStateSchema = withParser(
  Schema.Literal(
    'none',
    'parent-approval-missing',
    'provider-unavailable',
    'data-scope-rejected',
    'retention-not-proved',
    'local-safety-fallback-missing',
    'unsupported-task',
    'manual-required'
  )
);
export const BrowserAiRemoteExecutionStateSchema = withParser(
  Schema.Literal('selected', 'manual-required', 'unavailable')
);

export type BrowserAiRemoteDegradedState = Infer<typeof BrowserAiRemoteDegradedStateSchema>;

