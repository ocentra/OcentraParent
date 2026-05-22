import {
  BrowserInterventionActionSchema,
  BrowserInterventionCapabilityStateSchema,
  BrowserInterventionDecisionSourceSchema,
  BrowserInterventionMechanismSchema,
  BrowserInterventionOutcomeSchema,
  BrowserInterventionTargetTypeSchema,
  BrowserUnmanagedEnforcementStateSchema,
} from './browser-intervention-schemas';
import type {
  BrowserInterventionAction as BrowserInterventionActionType,
  BrowserInterventionCapabilityState as BrowserInterventionCapabilityStateType,
  BrowserInterventionDecisionSource as BrowserInterventionDecisionSourceType,
  BrowserInterventionMechanism as BrowserInterventionMechanismType,
  BrowserInterventionOutcome as BrowserInterventionOutcomeType,
  BrowserInterventionReadModel,
  BrowserInterventionRow,
  BrowserInterventionTargetType as BrowserInterventionTargetTypeType,
  BrowserUnmanagedEnforcementState as BrowserUnmanagedEnforcementStateType,
} from './browser-intervention-schemas';

export {
  BrowserInterventionActionSchema,
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
  BrowserUnmanagedEnforcementStateSchema,
} from './browser-intervention-schemas';

export type BrowserInterventionAction = BrowserInterventionActionType;
export type BrowserInterventionCapabilityState = BrowserInterventionCapabilityStateType;
export type BrowserInterventionDecisionSource = BrowserInterventionDecisionSourceType;
export type BrowserInterventionMechanism = BrowserInterventionMechanismType;
export type BrowserInterventionOutcome = BrowserInterventionOutcomeType;
export type { BrowserInterventionReadModel, BrowserInterventionRow };
export type BrowserInterventionTargetType = BrowserInterventionTargetTypeType;
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
