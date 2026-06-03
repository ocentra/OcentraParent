import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';

const NonEmptyRemoteAiValueText = Schema.String.pipe(Schema.minLength(1));

export const BrowserAiRemoteApprovalIdSchema = withParser(
  NonEmptyRemoteAiValueText.pipe(Schema.brand('BrowserAiRemoteApprovalId'))
);
export const BrowserAiRemoteProviderIdSchema = withParser(
  NonEmptyRemoteAiValueText.pipe(Schema.brand('BrowserAiRemoteProviderId'))
);
export const BrowserAiRemoteRouteIdSchema = withParser(
  NonEmptyRemoteAiValueText.pipe(Schema.brand('BrowserAiRemoteRouteId'))
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
