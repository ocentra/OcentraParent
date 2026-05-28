import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  ActivityDeviceIdSchema,
  ActivityEventIdSchema,
  ActivitySourceIdSchema,
  ActivityTimestampSchema,
} from './primitives';
import {
  BrowserChannelSchema,
  BrowserCustodyLabelSchema,
  BrowserDegradedReasonSchema,
  BrowserFamilySchema,
  BrowserManagedSessionIdSchema,
  BrowserProfileIdSchema,
  BrowserQueryVisibilityLabelSchema,
  BrowserUrlSchema,
} from './browser-schemas';

export const BrowserInterventionSchemaVersion = 1;

const NonEmptyBrowserInterventionText = Schema.String.pipe(Schema.minLength(1));

export const BrowserInterventionDecisionSourceSchema = withParser(
  Schema.Literal('parent-rule', 'parent-portal', 'local-ai', 'system', 'manual-test', 'unknown')
);
export const BrowserInterventionActionSchema = withParser(
  Schema.Literal('allow', 'warn', 'block', 'time-limit', 'ask-parent', 'monitor', 'unknown')
);
export const BrowserInterventionTargetTypeSchema = withParser(
  Schema.Literal('site', 'domain', 'url', 'video', 'browser-process', 'browser-session', 'unknown')
);
export const BrowserInterventionMechanismSchema = withParser(
  Schema.Literal(
    'chromium-cdp-fetch',
    'webdriver-bidi-network',
    'managed-extension',
    'os-app-control',
    'owned-webview',
    'monitor-only',
    'none'
  )
);
export const BrowserInterventionOutcomeSchema = withParser(
  Schema.Literal('applied', 'allowed', 'blocked', 'failed', 'unsupported', 'monitor-only')
);
export const BrowserInterventionCapabilityStateSchema = withParser(
  Schema.Literal(
    'ready',
    'needs-managed-session',
    'needs-managed-extension',
    'needs-os-app-control',
    'unsupported-browser',
    'disabled-by-parent',
    'adapter-error'
  )
);
export const BrowserUnmanagedEnforcementStateSchema = withParser(
  Schema.Literal(
    'monitor-only',
    'requires-os-app-control',
    'ready-to-block',
    'blocked-and-relaunched-managed',
    'unsupported'
  )
);
export const BrowserBoundaryStateSchema = withParser(
  Schema.Literal('managed-session', 'unmanaged-browser-process', 'browser-like-process', 'unsupported', 'unknown')
);
export const BrowserExactUrlClaimStateSchema = withParser(
  Schema.Literal('exact-url-proven', 'not-claimed', 'unavailable')
);
export const BrowserUnmanagedDetectionStateSchema = withParser(
  Schema.Literal('none', 'detected', 'warned', 'terminated', 'manual-required', 'unavailable')
);

export const BrowserInterventionIdSchema = withParser(
  NonEmptyBrowserInterventionText.pipe(Schema.brand('BrowserInterventionId'))
);
export const BrowserPolicyDecisionIdSchema = withParser(
  NonEmptyBrowserInterventionText.pipe(Schema.brand('BrowserPolicyDecisionId'))
);
export const BrowserTargetValueSchema = withParser(
  NonEmptyBrowserInterventionText.pipe(Schema.brand('BrowserTargetValue'))
);

export const BrowserInterventionRowSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(BrowserInterventionSchemaVersion),
    browserInterventionId: BrowserInterventionIdSchema,
    observedAt: ActivityTimestampSchema,
    sourceId: ActivitySourceIdSchema,
    deviceId: ActivityDeviceIdSchema,
    browserFamily: Schema.Union(BrowserFamilySchema, Schema.Null),
    browserChannel: Schema.Union(BrowserChannelSchema, Schema.Null),
    managedBrowserSessionId: Schema.Union(BrowserManagedSessionIdSchema, Schema.Null),
    profileId: Schema.Union(BrowserProfileIdSchema, Schema.Null),
    processId: Schema.Union(Schema.Number, Schema.Null),
    policyDecisionId: Schema.Union(BrowserPolicyDecisionIdSchema, Schema.Null),
    decisionSource: BrowserInterventionDecisionSourceSchema,
    interventionAction: BrowserInterventionActionSchema,
    interventionTargetType: BrowserInterventionTargetTypeSchema,
    interventionTargetValue: BrowserTargetValueSchema,
    requestedUrl: Schema.Union(BrowserUrlSchema, Schema.Null),
    observedUrl: Schema.Union(BrowserUrlSchema, Schema.Null),
    interventionMechanism: BrowserInterventionMechanismSchema,
    interventionOutcome: BrowserInterventionOutcomeSchema,
    browserBoundaryState: Schema.optionalWith(BrowserBoundaryStateSchema, {
      default: () => 'unknown' as const,
    }),
    exactUrlClaimState: Schema.optionalWith(BrowserExactUrlClaimStateSchema, {
      default: () => 'not-claimed' as const,
    }),
    unmanagedDetectionState: Schema.optionalWith(BrowserUnmanagedDetectionStateSchema, {
      default: () => 'unavailable' as const,
    }),
    reason: Schema.Union(BrowserDegradedReasonSchema, Schema.Null),
    custodyLabel: BrowserCustodyLabelSchema,
    queryVisibility: BrowserQueryVisibilityLabelSchema,
  })
);

export const BrowserInterventionReadModelSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(BrowserInterventionSchemaVersion),
    generatedAt: ActivityTimestampSchema,
    limit: Schema.Number,
    returned: Schema.Number,
    latestEventId: Schema.Union(ActivityEventIdSchema, Schema.Null),
    latestObservedAt: Schema.Union(ActivityTimestampSchema, Schema.Null),
    managedSessionInterventionCapability: BrowserInterventionCapabilityStateSchema,
    unmanagedBrowserEnforcement: BrowserUnmanagedEnforcementStateSchema,
    rows: Schema.Array(BrowserInterventionRowSchema),
  })
);

export type BrowserInterventionDecisionSource = Infer<typeof BrowserInterventionDecisionSourceSchema>;
export type BrowserInterventionAction = Infer<typeof BrowserInterventionActionSchema>;
export type BrowserInterventionTargetType = Infer<typeof BrowserInterventionTargetTypeSchema>;
export type BrowserInterventionMechanism = Infer<typeof BrowserInterventionMechanismSchema>;
export type BrowserInterventionOutcome = Infer<typeof BrowserInterventionOutcomeSchema>;
export type BrowserInterventionCapabilityState = Infer<typeof BrowserInterventionCapabilityStateSchema>;
export type BrowserUnmanagedEnforcementState = Infer<typeof BrowserUnmanagedEnforcementStateSchema>;
export type BrowserBoundaryState = Infer<typeof BrowserBoundaryStateSchema>;
export type BrowserExactUrlClaimState = Infer<typeof BrowserExactUrlClaimStateSchema>;
export type BrowserUnmanagedDetectionState = Infer<typeof BrowserUnmanagedDetectionStateSchema>;
export type BrowserInterventionRow = Infer<typeof BrowserInterventionRowSchema>;
export type BrowserInterventionReadModel = Infer<typeof BrowserInterventionReadModelSchema>;
