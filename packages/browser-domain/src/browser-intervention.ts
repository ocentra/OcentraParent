import {
  BrowserInterventionActionSchema,
  BrowserInterventionDeliveryStateSchema,
  BrowserBoundaryStateSchema,
  BrowserExactUrlClaimStateSchema,
  BrowserInterventionCapabilityStateSchema,
  BrowserInterventionDecisionSourceSchema,
  BrowserInterventionMechanismSchema,
  BrowserInterventionOutcomeSchema,
  BrowserInterventionTargetTypeSchema,
  BrowserUnmanagedDetectionStateSchema,
  BrowserUnmanagedEnforcementStateSchema,
  BrowserUnmanagedFallbackActionStateSchema,
} from './browser-intervention-schemas';
import type {
  BrowserInterventionAction as BrowserInterventionActionType,
  BrowserInterventionDeliveryState as BrowserInterventionDeliveryStateType,
  BrowserBoundaryState as BrowserBoundaryStateType,
  BrowserExactUrlClaimState as BrowserExactUrlClaimStateType,
  BrowserInterventionCapabilityState as BrowserInterventionCapabilityStateType,
  BrowserInterventionDecisionSource as BrowserInterventionDecisionSourceType,
  BrowserInterventionMechanism as BrowserInterventionMechanismType,
  BrowserInterventionOutcome as BrowserInterventionOutcomeType,
  BrowserInterventionReadModel,
  BrowserInterventionRow,
  BrowserInterventionTargetType as BrowserInterventionTargetTypeType,
  BrowserUnmanagedDetectionState as BrowserUnmanagedDetectionStateType,
  BrowserUnmanagedEnforcementState as BrowserUnmanagedEnforcementStateType,
  BrowserUnmanagedFallbackActionState as BrowserUnmanagedFallbackActionStateType,
} from './browser-intervention-schemas';

export {
  BrowserInterventionActionSchema,
  BrowserInterventionActionIdSchema,
  BrowserInterventionAuditIdSchema,
  BrowserInterventionDeliveryStateSchema,
  BrowserBoundaryStateSchema,
  BrowserExactUrlClaimStateSchema,
  BrowserInterventionCapabilityStateSchema,
  BrowserInterventionDecisionSourceSchema,
  BrowserInterventionIdSchema,
  BrowserInterventionMechanismSchema,
  BrowserInterventionOutcomeSchema,
  BrowserInterventionReadModelSchema,
  BrowserInterventionRowSchema,
  BrowserInterventionSchemaVersion,
  BrowserInterventionTargetTypeSchema,
  BrowserPolicyDecisionIdSchema,
  BrowserTargetValueSchema,
  BrowserUnmanagedDetectionStateSchema,
  BrowserUnmanagedEnforcementStateSchema,
  BrowserUnmanagedFallbackActionStateSchema,
} from './browser-intervention-schemas';

export type BrowserInterventionAction = BrowserInterventionActionType;
export type BrowserInterventionDeliveryState = BrowserInterventionDeliveryStateType;
export type BrowserBoundaryState = BrowserBoundaryStateType;
export type BrowserExactUrlClaimState = BrowserExactUrlClaimStateType;
export type BrowserInterventionCapabilityState = BrowserInterventionCapabilityStateType;
export type BrowserInterventionDecisionSource = BrowserInterventionDecisionSourceType;
export type BrowserInterventionMechanism = BrowserInterventionMechanismType;
export type BrowserInterventionOutcome = BrowserInterventionOutcomeType;
export type { BrowserInterventionReadModel, BrowserInterventionRow };
export type BrowserInterventionTargetType = BrowserInterventionTargetTypeType;
export type BrowserUnmanagedDetectionState = BrowserUnmanagedDetectionStateType;
export type BrowserUnmanagedEnforcementState = BrowserUnmanagedEnforcementStateType;
export type BrowserUnmanagedFallbackActionState = BrowserUnmanagedFallbackActionStateType;

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
