import {
  BrowserInterventionActionSchema,
  BrowserBoundaryStateSchema,
  BrowserExactUrlClaimStateSchema,
  BrowserInterventionCapabilityStateSchema,
  BrowserInterventionDecisionSourceSchema,
  BrowserInterventionMechanismSchema,
  BrowserInterventionOutcomeSchema,
  BrowserInterventionTargetTypeSchema,
  BrowserUnmanagedDetectionStateSchema,
  BrowserUnmanagedEnforcementStateSchema,
} from './browser-intervention-schemas';
import type {
  BrowserInterventionAction as BrowserInterventionActionType,
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
} from './browser-intervention-schemas';

export {
  BrowserInterventionActionSchema,
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
} from './browser-intervention-schemas';

export type BrowserInterventionAction = BrowserInterventionActionType;
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
  TimeLimit: BrowserInterventionActionSchema.parse('time-limit'),
  AskParent: BrowserInterventionActionSchema.parse('ask-parent'),
  Monitor: BrowserInterventionActionSchema.parse('monitor'),
  Unknown: BrowserInterventionActionSchema.parse('unknown'),
} as const;

export const BrowserInterventionTargetType = {
  Site: BrowserInterventionTargetTypeSchema.parse('site'),
  Domain: BrowserInterventionTargetTypeSchema.parse('domain'),
  Url: BrowserInterventionTargetTypeSchema.parse('url'),
  Video: BrowserInterventionTargetTypeSchema.parse('video'),
  BrowserProcess: BrowserInterventionTargetTypeSchema.parse('browser-process'),
  BrowserSession: BrowserInterventionTargetTypeSchema.parse('browser-session'),
  Unknown: BrowserInterventionTargetTypeSchema.parse('unknown'),
} as const;

export const BrowserInterventionMechanism = {
  ChromiumCdpFetch: BrowserInterventionMechanismSchema.parse('chromium-cdp-fetch'),
  WebDriverBidiNetwork: BrowserInterventionMechanismSchema.parse('webdriver-bidi-network'),
  ManagedExtension: BrowserInterventionMechanismSchema.parse('managed-extension'),
  OsAppControl: BrowserInterventionMechanismSchema.parse('os-app-control'),
  OwnedWebView: BrowserInterventionMechanismSchema.parse('owned-webview'),
  MonitorOnly: BrowserInterventionMechanismSchema.parse('monitor-only'),
  None: BrowserInterventionMechanismSchema.parse('none'),
} as const;

export const BrowserInterventionOutcome = {
  Applied: BrowserInterventionOutcomeSchema.parse('applied'),
  Allowed: BrowserInterventionOutcomeSchema.parse('allowed'),
  Blocked: BrowserInterventionOutcomeSchema.parse('blocked'),
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
  MonitorOnly: BrowserUnmanagedEnforcementStateSchema.parse('monitor-only'),
  RequiresOsAppControl: BrowserUnmanagedEnforcementStateSchema.parse('requires-os-app-control'),
  ReadyToBlock: BrowserUnmanagedEnforcementStateSchema.parse('ready-to-block'),
  BlockedAndRelaunchedManaged: BrowserUnmanagedEnforcementStateSchema.parse('blocked-and-relaunched-managed'),
  Unsupported: BrowserUnmanagedEnforcementStateSchema.parse('unsupported'),
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
