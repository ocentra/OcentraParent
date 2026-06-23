import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';
import {
  ActivityDeviceIdSchema,
  ActivityEvidenceIdSchema,
  ActivityEventIdSchema,
  ActivitySourceIdSchema,
  ActivityTimestampSchema,
} from '@ocentra-parent/schema-domain/evidence-primitives';
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

export const BrowserInterventionIdSchema = withParser(brandedNonEmptyStringSchema('BrowserInterventionId'));
export const BrowserInterventionActionIdSchema = withParser(brandedNonEmptyStringSchema('BrowserInterventionActionId'));
export const BrowserInterventionAuditIdSchema = withParser(brandedNonEmptyStringSchema('BrowserInterventionAuditId'));
export const BrowserPolicyDecisionIdSchema = withParser(brandedNonEmptyStringSchema('BrowserPolicyDecisionId'));
export const BrowserTargetValueSchema = withParser(brandedNonEmptyStringSchema('BrowserTargetValue'));

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

export const BrowserInterventionDecisionSource = {
  ParentRule: BrowserInterventionDecisionSourceSchema.parse('parent-rule'),
  ParentPortal: BrowserInterventionDecisionSourceSchema.parse('parent-portal'),
  LocalAi: BrowserInterventionDecisionSourceSchema.parse('local-ai'),
  System: BrowserInterventionDecisionSourceSchema.parse('system'),
  ManualTest: BrowserInterventionDecisionSourceSchema.parse('manual-test'),
  Unknown: BrowserInterventionDecisionSourceSchema.parse('unknown'),
} as const;

export const BrowserInterventionAction = {
  Allow: BrowserInterventionActionSchema.parse('allow'),
  Warn: BrowserInterventionActionSchema.parse('warn'),
  Block: BrowserInterventionActionSchema.parse('block'),
  Redirect: BrowserInterventionActionSchema.parse('redirect'),
  TimeLimit: BrowserInterventionActionSchema.parse('time-limit'),
  AskParent: BrowserInterventionActionSchema.parse('parent-review'),
  ApprovalHold: BrowserInterventionActionSchema.parse('approval-hold'),
  CheckingHold: BrowserInterventionActionSchema.parse('checking-hold'),
  TerminateProcess: BrowserInterventionActionSchema.parse('terminate-process'),
  RelaunchManaged: BrowserInterventionActionSchema.parse('relaunch-managed'),
  Monitor: BrowserInterventionActionSchema.parse('monitor'),
  Unknown: BrowserInterventionActionSchema.parse('unknown'),
} as const;

export const BrowserInterventionTargetType = {
  Site: BrowserInterventionTargetTypeSchema.parse('site'),
  Domain: BrowserInterventionTargetTypeSchema.parse('domain'),
  Url: BrowserInterventionTargetTypeSchema.parse('url'),
  Video: BrowserInterventionTargetTypeSchema.parse('video'),
  SocialAccountCreation: BrowserInterventionTargetTypeSchema.parse('social-account-creation'),
  SocialFeed: BrowserInterventionTargetTypeSchema.parse('social-feed'),
  SocialShortVideoFeed: BrowserInterventionTargetTypeSchema.parse('social-short-video-feed'),
  SocialMessaging: BrowserInterventionTargetTypeSchema.parse('social-messaging'),
  SocialUploadPost: BrowserInterventionTargetTypeSchema.parse('social-upload-post'),
  SocialLivestream: BrowserInterventionTargetTypeSchema.parse('social-livestream'),
  UnknownSocialSite: BrowserInterventionTargetTypeSchema.parse('unknown-social-site'),
  BrowserGame: BrowserInterventionTargetTypeSchema.parse('browser-game'),
  GameAccount: BrowserInterventionTargetTypeSchema.parse('game-account'),
  GamePurchase: BrowserInterventionTargetTypeSchema.parse('game-purchase'),
  CloudGaming: BrowserInterventionTargetTypeSchema.parse('cloud-gaming'),
  UnknownGame: BrowserInterventionTargetTypeSchema.parse('unknown-game'),
  UnblockedGameSite: BrowserInterventionTargetTypeSchema.parse('unblocked-game-site'),
  BrowserProcess: BrowserInterventionTargetTypeSchema.parse('browser-process'),
  BrowserSession: BrowserInterventionTargetTypeSchema.parse('browser-session'),
  Unknown: BrowserInterventionTargetTypeSchema.parse('unknown'),
} as const;

export const BrowserInterventionMechanism = {
  ChromiumCdpFetch: BrowserInterventionMechanismSchema.parse('chromium-cdp-fetch'),
  WebDriverBidiNetwork: BrowserInterventionMechanismSchema.parse('webdriver-bidi-network'),
  ManagedExtension: BrowserInterventionMechanismSchema.parse('managed-extension'),
  ManagedBlockPage: BrowserInterventionMechanismSchema.parse('managed-block-page'),
  ApprovalHoldPage: BrowserInterventionMechanismSchema.parse('approval-hold-page'),
  CheckingHoldPage: BrowserInterventionMechanismSchema.parse('checking-hold-page'),
  OsAppControl: BrowserInterventionMechanismSchema.parse('os-app-control'),
  OwnedWebView: BrowserInterventionMechanismSchema.parse('owned-webview'),
  MonitorOnly: BrowserInterventionMechanismSchema.parse('monitor-only'),
  None: BrowserInterventionMechanismSchema.parse('none'),
} as const;

export const BrowserInterventionOutcome = {
  Applied: BrowserInterventionOutcomeSchema.parse('applied'),
  Allowed: BrowserInterventionOutcomeSchema.parse('allowed'),
  Warned: BrowserInterventionOutcomeSchema.parse('warned'),
  Blocked: BrowserInterventionOutcomeSchema.parse('blocked'),
  Redirected: BrowserInterventionOutcomeSchema.parse('redirected'),
  ApprovalRequired: BrowserInterventionOutcomeSchema.parse('approval-required'),
  Held: BrowserInterventionOutcomeSchema.parse('held'),
  Terminated: BrowserInterventionOutcomeSchema.parse('terminated'),
  RelaunchStarted: BrowserInterventionOutcomeSchema.parse('relaunch-started'),
  ManualRequired: BrowserInterventionOutcomeSchema.parse('manual-required'),
  Failed: BrowserInterventionOutcomeSchema.parse('failed'),
  Unsupported: BrowserInterventionOutcomeSchema.parse('unsupported'),
  MonitorOnly: BrowserInterventionOutcomeSchema.parse('monitor-only'),
} as const;

export const BrowserInterventionCapabilityState = {
  Ready: BrowserInterventionCapabilityStateSchema.parse('ready'),
  NeedsManagedSession: BrowserInterventionCapabilityStateSchema.parse('needs-managed-session'),
  NeedsManagedExtension: BrowserInterventionCapabilityStateSchema.parse('needs-managed-extension'),
  NeedsOsAppControl: BrowserInterventionCapabilityStateSchema.parse('needs-os-app-control'),
  UnsupportedBrowser: BrowserInterventionCapabilityStateSchema.parse('unsupported-browser'),
  DisabledByParent: BrowserInterventionCapabilityStateSchema.parse('disabled-by-parent'),
  AdapterError: BrowserInterventionCapabilityStateSchema.parse('adapter-error'),
} as const;

export const BrowserUnmanagedEnforcementState = {
  ReportOnly: BrowserUnmanagedEnforcementStateSchema.parse('report-only'),
  WarnChild: BrowserUnmanagedEnforcementStateSchema.parse('warn-child'),
  AskParent: BrowserUnmanagedEnforcementStateSchema.parse('parent-review'),
  TerminateProcess: BrowserUnmanagedEnforcementStateSchema.parse('terminate-process'),
  RelaunchManagedBrowser: BrowserUnmanagedEnforcementStateSchema.parse('relaunch-managed-browser'),
  OsBlockConfigured: BrowserUnmanagedEnforcementStateSchema.parse('os-block-configured'),
  OsBlockManualRequired: BrowserUnmanagedEnforcementStateSchema.parse('os-block-manual-required'),
  AllowedUnmanagedException: BrowserUnmanagedEnforcementStateSchema.parse('allowed-unmanaged-exception'),
  Degraded: BrowserUnmanagedEnforcementStateSchema.parse('degraded'),
  Unavailable: BrowserUnmanagedEnforcementStateSchema.parse('unavailable'),
  MonitorOnly: BrowserUnmanagedEnforcementStateSchema.parse('monitor-only'),
  RequiresOsAppControl: BrowserUnmanagedEnforcementStateSchema.parse('requires-os-app-control'),
  ReadyToBlock: BrowserUnmanagedEnforcementStateSchema.parse('ready-to-block'),
  BlockedAndRelaunchedManaged: BrowserUnmanagedEnforcementStateSchema.parse('blocked-and-relaunched-managed'),
  Unsupported: BrowserUnmanagedEnforcementStateSchema.parse('unsupported'),
} as const;

export const BrowserUnmanagedFallbackActionState = {
  ReportOnly: BrowserUnmanagedFallbackActionStateSchema.parse('report-only'),
  WarnChild: BrowserUnmanagedFallbackActionStateSchema.parse('warn-child'),
  AskParent: BrowserUnmanagedFallbackActionStateSchema.parse('parent-review'),
  TerminateProcess: BrowserUnmanagedFallbackActionStateSchema.parse('terminate-process'),
  RelaunchManagedBrowser: BrowserUnmanagedFallbackActionStateSchema.parse('relaunch-managed-browser'),
  OsBlockConfigured: BrowserUnmanagedFallbackActionStateSchema.parse('os-block-configured'),
  OsBlockManualRequired: BrowserUnmanagedFallbackActionStateSchema.parse('os-block-manual-required'),
  AllowedUnmanagedException: BrowserUnmanagedFallbackActionStateSchema.parse('allowed-unmanaged-exception'),
  Degraded: BrowserUnmanagedFallbackActionStateSchema.parse('degraded'),
  Unavailable: BrowserUnmanagedFallbackActionStateSchema.parse('unavailable'),
} as const;

export const BrowserBoundaryState = {
  ManagedSession: BrowserBoundaryStateSchema.parse('managed-session'),
  UnmanagedBrowserProcess: BrowserBoundaryStateSchema.parse('unmanaged-browser-process'),
  BrowserLikeProcess: BrowserBoundaryStateSchema.parse('browser-like-process'),
  Unsupported: BrowserBoundaryStateSchema.parse('unsupported'),
  Unknown: BrowserBoundaryStateSchema.parse('unknown'),
} as const;

export const BrowserExactUrlClaimState = {
  ExactUrlProven: BrowserExactUrlClaimStateSchema.parse('exact-url-proven'),
  NotClaimed: BrowserExactUrlClaimStateSchema.parse('not-claimed'),
  Unavailable: BrowserExactUrlClaimStateSchema.parse('unavailable'),
} as const;

export const BrowserUnmanagedDetectionState = {
  None: BrowserUnmanagedDetectionStateSchema.parse('none'),
  Detected: BrowserUnmanagedDetectionStateSchema.parse('detected'),
  Warned: BrowserUnmanagedDetectionStateSchema.parse('warned'),
  Terminated: BrowserUnmanagedDetectionStateSchema.parse('terminated'),
  ManualRequired: BrowserUnmanagedDetectionStateSchema.parse('manual-required'),
  Unavailable: BrowserUnmanagedDetectionStateSchema.parse('unavailable'),
} as const;

export const BrowserInterventionDeliveryState = {
  NotDelivered: BrowserInterventionDeliveryStateSchema.parse('not-delivered'),
  WarnPageRendered: BrowserInterventionDeliveryStateSchema.parse('warn-page-rendered'),
  BlockPageRendered: BrowserInterventionDeliveryStateSchema.parse('block-page-rendered'),
  ApprovalHoldRendered: BrowserInterventionDeliveryStateSchema.parse('approval-hold-rendered'),
  CheckingHoldRendered: BrowserInterventionDeliveryStateSchema.parse('checking-hold-rendered'),
  PortalRowOnly: BrowserInterventionDeliveryStateSchema.parse('portal-row-only'),
  ManualRequired: BrowserInterventionDeliveryStateSchema.parse('manual-required'),
} as const;

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
