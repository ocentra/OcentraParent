import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import {
  ActivityDeviceIdSchema,
  ActivityEvidenceIdSchema,
  ActivityEventIdSchema,
  ActivitySourceIdSchema,
  ActivityTimestampSchema,
} from '@ocentra-parent/evidence-domain/primitives';
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

export const BrowserInterventionDecisionSourceSchema = withParser(
  Schema.Literal('parent-rule', 'parent-portal', 'local-ai', 'system', 'manual-test', 'unknown')
);
export const BrowserInterventionActionSchema = withParser(
  Schema.Literal(
    'allow',
    'warn',
    'block',
    'redirect',
    'time-limit',
    'parent-review',
    'approval-hold',
    'checking-hold',
    'terminate-process',
    'relaunch-managed',
    'monitor',
    'unknown'
  )
);
export const BrowserInterventionTargetTypeSchema = withParser(
  Schema.Literal(
    'site',
    'domain',
    'url',
    'video',
    'social-account-creation',
    'social-feed',
    'social-short-video-feed',
    'social-messaging',
    'social-upload-post',
    'social-livestream',
    'unknown-social-site',
    'browser-game',
    'game-account',
    'game-purchase',
    'cloud-gaming',
    'unknown-game',
    'unblocked-game-site',
    'browser-process',
    'browser-session',
    'unknown'
  )
);
export const BrowserInterventionMechanismSchema = withParser(
  Schema.Literal(
    'chromium-cdp-fetch',
    'webdriver-bidi-network',
    'managed-extension',
    'managed-block-page',
    'approval-hold-page',
    'checking-hold-page',
    'os-app-control',
    'owned-webview',
    'monitor-only',
    'none'
  )
);
export const BrowserInterventionOutcomeSchema = withParser(
  Schema.Literal(
    'applied',
    'allowed',
    'warned',
    'blocked',
    'redirected',
    'approval-required',
    'held',
    'terminated',
    'relaunch-started',
    'manual-required',
    'failed',
    'unsupported',
    'monitor-only'
  )
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
    'report-only',
    'warn-child',
    'parent-review',
    'terminate-process',
    'relaunch-managed-browser',
    'os-block-configured',
    'os-block-manual-required',
    'allowed-unmanaged-exception',
    'degraded',
    'unavailable',
    'monitor-only',
    'requires-os-app-control',
    'ready-to-block',
    'blocked-and-relaunched-managed',
    'unsupported'
  )
);
export const BrowserUnmanagedFallbackActionStateSchema = withParser(
  Schema.Literal(
    'report-only',
    'warn-child',
    'parent-review',
    'terminate-process',
    'relaunch-managed-browser',
    'os-block-configured',
    'os-block-manual-required',
    'allowed-unmanaged-exception',
    'degraded',
    'unavailable'
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
export const BrowserInterventionDeliveryStateSchema = withParser(
  Schema.Literal(
    'not-delivered',
    'warn-page-rendered',
    'block-page-rendered',
    'approval-hold-rendered',
    'checking-hold-rendered',
    'portal-row-only',
    'manual-required'
  )
);

export const BrowserInterventionIdSchema = withParser(
  brandedNonEmptyStringSchema('BrowserInterventionId')
);
export const BrowserInterventionActionIdSchema = withParser(
  brandedNonEmptyStringSchema('BrowserInterventionActionId')
);
export const BrowserInterventionAuditIdSchema = withParser(
  brandedNonEmptyStringSchema('BrowserInterventionAuditId')
);
export const BrowserPolicyDecisionIdSchema = withParser(
  brandedNonEmptyStringSchema('BrowserPolicyDecisionId')
);
export const BrowserTargetValueSchema = withParser(
  brandedNonEmptyStringSchema('BrowserTargetValue')
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
    interventionActionId: Schema.optionalWith(Schema.Union(BrowserInterventionActionIdSchema, Schema.Null), {
      default: () => null,
    }),
    interventionAuditId: Schema.optionalWith(Schema.Union(BrowserInterventionAuditIdSchema, Schema.Null), {
      default: () => null,
    }),
    evidenceReferenceIds: Schema.optionalWith(Schema.Array(ActivityEvidenceIdSchema), {
      default: () => [],
    }),
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
    unmanagedFallbackAction: Schema.optionalWith(BrowserUnmanagedFallbackActionStateSchema, {
      default: () => 'unavailable' as const,
    }),
    childDeliveryState: Schema.optionalWith(BrowserInterventionDeliveryStateSchema, {
      default: () => 'not-delivered' as const,
    }),
    reason: Schema.Union(BrowserDegradedReasonSchema, Schema.Null),
    custodyLabel: BrowserCustodyLabelSchema,
    queryVisibility: BrowserQueryVisibilityLabelSchema,
  }).pipe(
    Schema.filter(
      (row) =>
        browserInterventionRowDoesNotOverclaimUnmanaged(row) ||
        'Expected unmanaged browser intervention rows to omit URL fields and exact URL proof'
    )
  )
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
    unmanagedFallbackAction: Schema.optionalWith(BrowserUnmanagedFallbackActionStateSchema, {
      default: () => 'os-block-manual-required' as const,
    }),
    rows: Schema.Array(BrowserInterventionRowSchema),
  })
);

function browserInterventionRowDoesNotOverclaimUnmanaged(row: {
  readonly browserBoundaryState: BrowserBoundaryState;
  readonly requestedUrl: unknown;
  readonly observedUrl: unknown;
  readonly exactUrlClaimState: BrowserExactUrlClaimState;
}): boolean {
  const isUnmanagedBoundary =
    row.browserBoundaryState === 'unmanaged-browser-process' || row.browserBoundaryState === 'browser-like-process';
  if (!isUnmanagedBoundary) {
    return true;
  }
  return row.requestedUrl === null && row.observedUrl === null && row.exactUrlClaimState !== 'exact-url-proven';
}

export type BrowserInterventionDecisionSource = Infer<typeof BrowserInterventionDecisionSourceSchema>;
export type BrowserInterventionAction = Infer<typeof BrowserInterventionActionSchema>;
export type BrowserInterventionTargetType = Infer<typeof BrowserInterventionTargetTypeSchema>;
export type BrowserInterventionMechanism = Infer<typeof BrowserInterventionMechanismSchema>;
export type BrowserInterventionOutcome = Infer<typeof BrowserInterventionOutcomeSchema>;
export type BrowserInterventionCapabilityState = Infer<typeof BrowserInterventionCapabilityStateSchema>;
export type BrowserUnmanagedEnforcementState = Infer<typeof BrowserUnmanagedEnforcementStateSchema>;
export type BrowserUnmanagedFallbackActionState = Infer<typeof BrowserUnmanagedFallbackActionStateSchema>;
export type BrowserBoundaryState = Infer<typeof BrowserBoundaryStateSchema>;
export type BrowserExactUrlClaimState = Infer<typeof BrowserExactUrlClaimStateSchema>;
export type BrowserUnmanagedDetectionState = Infer<typeof BrowserUnmanagedDetectionStateSchema>;
export type BrowserInterventionDeliveryState = Infer<typeof BrowserInterventionDeliveryStateSchema>;
export type BrowserInterventionRow = Infer<typeof BrowserInterventionRowSchema>;
export type BrowserInterventionReadModel = Infer<typeof BrowserInterventionReadModelSchema>;

