import { decodeDisplayText, decodeTextTokenId, type DisplayText } from './text-contracts';

export const PortalProductTextToken = {
  ProductStatusLive: decodeTextTokenId('portal.dev.productStatusLive'),
  ProductStatusLocalOnly: decodeTextTokenId('portal.dev.productStatusLocalOnly'),
  ProductStatusPreviewOnly: decodeTextTokenId('portal.dev.productStatusPreviewOnly'),
  NavGroupMonitor: decodeTextTokenId('portal.dev.navGroup.monitor'),
  NavGroupGuide: decodeTextTokenId('portal.dev.navGroup.guide'),
  NavGroupOperate: decodeTextTokenId('portal.dev.navGroup.operate'),
  NavGroupDevTools: decodeTextTokenId('portal.dev.navGroup.devTools'),
  Overview: decodeTextTokenId('portal.dev.route.overview'),
  ParentPortal: decodeTextTokenId('portal.dev.route.parentPortal'),
  Activity: decodeTextTokenId('portal.dev.route.activity'),
  Browser: decodeTextTokenId('portal.dev.route.browser'),
  Policy: decodeTextTokenId('portal.dev.route.policy'),
  Memory: decodeTextTokenId('portal.dev.route.memory'),
  AiRuntime: decodeTextTokenId('portal.dev.route.aiRuntime'),
  Devices: decodeTextTokenId('portal.dev.route.devices'),
  Diagnostics: decodeTextTokenId('portal.dev.route.diagnostics'),
  ProofPanels: decodeTextTokenId('portal.dev.route.proofPanels'),
  SettingsRules: decodeTextTokenId('portal.dev.route.settingsRules'),
  FrameTuner: decodeTextTokenId('portal.dev.route.frameTuner'),
  Commands: decodeTextTokenId('portal.dev.route.commands'),
  Events: decodeTextTokenId('portal.dev.route.events'),
  Logs: decodeTextTokenId('portal.dev.route.logs'),
  OverviewDescription: decodeTextTokenId('portal.dev.route.overview.description'),
  ParentPortalDescription: decodeTextTokenId('portal.dev.route.parentPortal.description'),
  ActivityDescription: decodeTextTokenId('portal.dev.route.activity.description'),
  BrowserDescription: decodeTextTokenId('portal.dev.route.browser.description'),
  PolicyDescription: decodeTextTokenId('portal.dev.route.policy.description'),
  MemoryDescription: decodeTextTokenId('portal.dev.route.memory.description'),
  AiRuntimeDescription: decodeTextTokenId('portal.dev.route.aiRuntime.description'),
  DevicesDescription: decodeTextTokenId('portal.dev.route.devices.description'),
  DiagnosticsDescription: decodeTextTokenId('portal.dev.route.diagnostics.description'),
  ProofPanelsDescription: decodeTextTokenId('portal.dev.route.proofPanels.description'),
  SettingsRulesDescription: decodeTextTokenId('portal.dev.route.settingsRules.description'),
  FrameTunerDescription: decodeTextTokenId('portal.dev.route.frameTuner.description'),
  CommandsDescription: decodeTextTokenId('portal.dev.route.commands.description'),
  EventsDescription: decodeTextTokenId('portal.dev.route.events.description'),
  LogsDescription: decodeTextTokenId('portal.dev.route.logs.description'),
  Connected: decodeTextTokenId('portal.dev.connected'),
  Unavailable: decodeTextTokenId('portal.dev.unavailable'),
  PendingTypedIntent: decodeTextTokenId('portal.dev.pendingTypedIntent'),
  PendingServiceReadModel: decodeTextTokenId('portal.dev.pendingServiceReadModel'),
  ProductionShellReady: decodeTextTokenId('portal.dev.productionShellReady'),
  LocalDataOnly: decodeTextTokenId('portal.dev.localDataOnly'),
  ParentControls: decodeTextTokenId('portal.dev.parentControls'),
  RuleBuilder: decodeTextTokenId('portal.dev.ruleBuilder'),
  SchedulesBudgets: decodeTextTokenId('portal.dev.schedulesBudgets'),
  Approvals: decodeTextTokenId('portal.dev.approvals'),
  ScreenAnalysis: decodeTextTokenId('portal.dev.screenAnalysis'),
  RemoteScreen: decodeTextTokenId('portal.dev.remoteScreen'),
  AppGameSessions: decodeTextTokenId('portal.dev.appGameSessions'),
  DeviceInventory: decodeTextTokenId('portal.dev.deviceInventory'),
  Pairing: decodeTextTokenId('portal.dev.pairing'),
  ExportSync: decodeTextTokenId('portal.dev.exportSync'),
  Notifications: decodeTextTokenId('portal.dev.notifications'),
  BillingEntitlements: decodeTextTokenId('portal.dev.billingEntitlements'),
  DesktopApp: decodeTextTokenId('portal.dev.desktopApp'),
  MobileApp: decodeTextTokenId('portal.dev.mobileApp'),
  ProductSurfacePending: decodeTextTokenId('portal.dev.productSurfacePending'),
  ProductSurfaceWired: decodeTextTokenId('portal.dev.productSurfaceWired'),
  ThemeLight: decodeTextTokenId('portal.dev.theme.light'),
  ThemeDark: decodeTextTokenId('portal.dev.theme.dark'),
  HeaderHome: decodeTextTokenId('portal.dev.header.home'),
  HeaderLogin: decodeTextTokenId('portal.dev.header.login'),
  HeaderTagline: decodeTextTokenId('portal.dev.header.tagline'),
  HeaderBrandLeft: decodeTextTokenId('portal.dev.header.brandLeft'),
  HeaderBrandRight: decodeTextTokenId('portal.dev.header.brandRight'),
  FooterMadeWith: decodeTextTokenId('portal.dev.footer.madeWith'),
  FooterBy: decodeTextTokenId('portal.dev.footer.by'),
  FooterHeart: decodeTextTokenId('portal.dev.footer.heart'),
  FooterLink: decodeTextTokenId('portal.dev.footer.link'),
  FooterVersion: decodeTextTokenId('portal.dev.footer.version'),
  AuthClose: decodeTextTokenId('portal.dev.auth.close'),
  AuthEyebrow: decodeTextTokenId('portal.dev.auth.eyebrow'),
  AuthTitle: decodeTextTokenId('portal.dev.auth.title'),
  AuthBody: decodeTextTokenId('portal.dev.auth.body'),
  AuthSignIn: decodeTextTokenId('portal.dev.auth.signIn'),
  AuthSignUp: decodeTextTokenId('portal.dev.auth.signUp'),
  AuthParentEmail: decodeTextTokenId('portal.dev.auth.parentEmail'),
  AuthParentName: decodeTextTokenId('portal.dev.auth.parentName'),
  AuthPassword: decodeTextTokenId('portal.dev.auth.password'),
  AuthConfirmPassword: decodeTextTokenId('portal.dev.auth.confirmPassword'),
  AuthPrimaryAction: decodeTextTokenId('portal.dev.auth.primaryAction'),
  AuthSocialTitle: decodeTextTokenId('portal.dev.auth.socialTitle'),
  AuthGoogle: decodeTextTokenId('portal.dev.auth.google'),
  AuthFacebook: decodeTextTokenId('portal.dev.auth.facebook'),
  AuthGuest: decodeTextTokenId('portal.dev.auth.guest'),
  AuthTrustTitle: decodeTextTokenId('portal.dev.auth.trustTitle'),
  AuthTrustBody: decodeTextTokenId('portal.dev.auth.trustBody'),
  AuthUnavailable: decodeTextTokenId('portal.dev.auth.unavailable'),
  OpenSettings: decodeTextTokenId('portal.dev.openSettings'),
  ChildDevice: decodeTextTokenId('portal.dev.childDevice'),
  FamilyDefault: decodeTextTokenId('portal.dev.familyDefault'),
  DeviceRuleScope: decodeTextTokenId('portal.dev.deviceRuleScope'),
  DeviceRuleScopeBody: decodeTextTokenId('portal.dev.deviceRuleScopeBody'),
  DeviceRuleScopeTip: decodeTextTokenId('portal.dev.deviceRuleScopeTip'),
  ManagedWeb: decodeTextTokenId('portal.dev.managedWeb'),
  DeviceRuleOverride: decodeTextTokenId('portal.dev.deviceRuleOverride'),
  PolicyModeAdvisory: decodeTextTokenId('portal.dev.policyModeAdvisory'),
  PolicyModeActive: decodeTextTokenId('portal.dev.policyModeActive'),
  PolicyDecisionLinked: decodeTextTokenId('portal.dev.policyDecisionLinked'),
  ActivityRecordLinked: decodeTextTokenId('portal.dev.activityRecordLinked'),
  FamilyRulesTitle: decodeTextTokenId('portal.dev.familyRulesTitle'),
  FamilyRulesBody: decodeTextTokenId('portal.dev.familyRulesBody'),
  HowItWorks: decodeTextTokenId('portal.dev.howItWorks'),
  RiskToKnow: decodeTextTokenId('portal.dev.riskToKnow'),
  NotConfiguredStatus: decodeTextTokenId('portal.dev.notConfiguredStatus'),
  LocalOnlyStatus: decodeTextTokenId('portal.dev.localOnlyStatus'),
  BrowserSupportedTitle: decodeTextTokenId('portal.dev.browserSupportedTitle'),
  BrowserSupportedBody: decodeTextTokenId('portal.dev.browserSupportedBody'),
  BrowserSupportedTip: decodeTextTokenId('portal.dev.browserSupportedTip'),
  BrowserUnsupportedTitle: decodeTextTokenId('portal.dev.browserUnsupportedTitle'),
  BrowserUnsupportedBody: decodeTextTokenId('portal.dev.browserUnsupportedBody'),
  BrowserUnsupportedTip: decodeTextTokenId('portal.dev.browserUnsupportedTip'),
  BrowserBlockTitle: decodeTextTokenId('portal.dev.browserBlockTitle'),
  BrowserBlockBody: decodeTextTokenId('portal.dev.browserBlockBody'),
  BrowserBlockTip: decodeTextTokenId('portal.dev.browserBlockTip'),
  BrowserRiskBody: decodeTextTokenId('portal.dev.browserRiskBody'),
  BrowserControls: decodeTextTokenId('portal.dev.browserControls'),
  DataCustodyTitle: decodeTextTokenId('portal.dev.dataCustodyTitle'),
  DataCustodyBody: decodeTextTokenId('portal.dev.dataCustodyBody'),
  DataCustodyTip: decodeTextTokenId('portal.dev.dataCustodyTip'),
  DisplayTheme: decodeTextTokenId('portal.dev.displayTheme'),
  DriveConnectionsTitle: decodeTextTokenId('portal.dev.driveConnectionsTitle'),
  DriveConnectionsBody: decodeTextTokenId('portal.dev.driveConnectionsBody'),
  DriveConnectionsTip: decodeTextTokenId('portal.dev.driveConnectionsTip'),
  RuleBuilderBody: decodeTextTokenId('portal.dev.ruleBuilderBody'),
  SchedulesBudgetsBody: decodeTextTokenId('portal.dev.schedulesBudgetsBody'),
  ApprovalsBody: decodeTextTokenId('portal.dev.approvalsBody'),
  NotificationsBody: decodeTextTokenId('portal.dev.notificationsBody'),
  DeviceInventoryBody: decodeTextTokenId('portal.dev.deviceInventoryBody'),
  PairingBody: decodeTextTokenId('portal.dev.pairingBody'),
  DesktopAppBody: decodeTextTokenId('portal.dev.desktopAppBody'),
  MobileAppBody: decodeTextTokenId('portal.dev.mobileAppBody'),
  MemoryBody: decodeTextTokenId('portal.dev.memoryBody'),
  AiRuntimeBody: decodeTextTokenId('portal.dev.aiRuntimeBody'),
} as const;

export type PortalProductTextTokenValue = (typeof PortalProductTextToken)[keyof typeof PortalProductTextToken];

export const PortalProductText: Record<PortalProductTextTokenValue, DisplayText> = {
  [PortalProductTextToken.ProductStatusLive]: decodeDisplayText('Child device connected'),
  [PortalProductTextToken.ProductStatusLocalOnly]: decodeDisplayText('Private device data'),
  [PortalProductTextToken.ProductStatusPreviewOnly]: decodeDisplayText('Advisory mode'),
  [PortalProductTextToken.NavGroupMonitor]: decodeDisplayText('Today'),
  [PortalProductTextToken.NavGroupGuide]: decodeDisplayText('Guide'),
  [PortalProductTextToken.NavGroupOperate]: decodeDisplayText('Manage'),
  [PortalProductTextToken.NavGroupDevTools]: decodeDisplayText('Dev tools'),
  [PortalProductTextToken.Overview]: decodeDisplayText('Overview'),
  [PortalProductTextToken.ParentPortal]: decodeDisplayText('Start here'),
  [PortalProductTextToken.Activity]: decodeDisplayText('Activity'),
  [PortalProductTextToken.Browser]: decodeDisplayText('Web'),
  [PortalProductTextToken.Policy]: decodeDisplayText('Policy'),
  [PortalProductTextToken.Memory]: decodeDisplayText('Memory'),
  [PortalProductTextToken.AiRuntime]: decodeDisplayText('Local AI'),
  [PortalProductTextToken.Devices]: decodeDisplayText('Devices'),
  [PortalProductTextToken.Diagnostics]: decodeDisplayText('Support'),
  [PortalProductTextToken.ProofPanels]: decodeDisplayText('Proof panels'),
  [PortalProductTextToken.SettingsRules]: decodeDisplayText('Settings'),
  [PortalProductTextToken.FrameTuner]: decodeDisplayText('App layout'),
  [PortalProductTextToken.Commands]: decodeDisplayText('Controls'),
  [PortalProductTextToken.Events]: decodeDisplayText('Audit'),
  [PortalProductTextToken.Logs]: decodeDisplayText('Logs'),
  [PortalProductTextToken.OverviewDescription]: decodeDisplayText('Daily command'),
  [PortalProductTextToken.ParentPortalDescription]: decodeDisplayText('Setup and controls map'),
  [PortalProductTextToken.ActivityDescription]: decodeDisplayText('Stored activity'),
  [PortalProductTextToken.BrowserDescription]: decodeDisplayText('Browser evidence'),
  [PortalProductTextToken.PolicyDescription]: decodeDisplayText('Rules and approvals'),
  [PortalProductTextToken.MemoryDescription]: decodeDisplayText('Cited local memory'),
  [PortalProductTextToken.AiRuntimeDescription]: decodeDisplayText('Local model privacy'),
  [PortalProductTextToken.DevicesDescription]: decodeDisplayText('Device control'),
  [PortalProductTextToken.DiagnosticsDescription]: decodeDisplayText('Exports and logs'),
  [PortalProductTextToken.ProofPanelsDescription]: decodeDisplayText('Tracking, network, and policy proof panels.'),
  [PortalProductTextToken.SettingsRulesDescription]: decodeDisplayText('Schedules and budgets'),
  [PortalProductTextToken.FrameTunerDescription]: decodeDisplayText(
    'Layout and content editor for parent portal app surfaces.'
  ),
  [PortalProductTextToken.CommandsDescription]: decodeDisplayText(
    'Safe device refresh actions that use the real child-device connection.'
  ),
  [PortalProductTextToken.EventsDescription]: decodeDisplayText(
    'Validated child-device audit entries from the local service.'
  ),
  [PortalProductTextToken.LogsDescription]: decodeDisplayText('Local portal and agent service log snapshots.'),
  [PortalProductTextToken.Connected]: decodeDisplayText('Child device connected'),
  [PortalProductTextToken.Unavailable]: decodeDisplayText('Child device not connected'),
  [PortalProductTextToken.PendingTypedIntent]: decodeDisplayText('Setup'),
  [PortalProductTextToken.PendingServiceReadModel]: decodeDisplayText('Not connected'),
  [PortalProductTextToken.ProductionShellReady]: decodeDisplayText('Family command center'),
  [PortalProductTextToken.LocalDataOnly]: decodeDisplayText(
    'Your house, your rules. Child activity stays local unless you connect a parent-owned export.'
  ),
  [PortalProductTextToken.ParentControls]: decodeDisplayText('Parent controls'),
  [PortalProductTextToken.RuleBuilder]: decodeDisplayText('Rule builder'),
  [PortalProductTextToken.SchedulesBudgets]: decodeDisplayText('Schedules and budgets'),
  [PortalProductTextToken.Approvals]: decodeDisplayText('Approvals'),
  [PortalProductTextToken.ScreenAnalysis]: decodeDisplayText('Screen analysis'),
  [PortalProductTextToken.RemoteScreen]: decodeDisplayText('Remote screen'),
  [PortalProductTextToken.AppGameSessions]: decodeDisplayText('App and game sessions'),
  [PortalProductTextToken.DeviceInventory]: decodeDisplayText('Device inventory'),
  [PortalProductTextToken.Pairing]: decodeDisplayText('Pairing'),
  [PortalProductTextToken.ExportSync]: decodeDisplayText('Export and sync'),
  [PortalProductTextToken.Notifications]: decodeDisplayText('Notifications'),
  [PortalProductTextToken.BillingEntitlements]: decodeDisplayText('Billing and entitlements'),
  [PortalProductTextToken.DesktopApp]: decodeDisplayText('Desktop app'),
  [PortalProductTextToken.MobileApp]: decodeDisplayText('Mobile app'),
  [PortalProductTextToken.ProductSurfacePending]: decodeDisplayText(
    'No family setting is configured for this area yet.'
  ),
  [PortalProductTextToken.ProductSurfaceWired]: decodeDisplayText('Connected to the local child device.'),
  [PortalProductTextToken.ThemeLight]: decodeDisplayText('Light'),
  [PortalProductTextToken.ThemeDark]: decodeDisplayText('Dark'),
  [PortalProductTextToken.HeaderHome]: decodeDisplayText('Home'),
  [PortalProductTextToken.HeaderLogin]: decodeDisplayText('Login'),
  [PortalProductTextToken.HeaderTagline]: decodeDisplayText('Your House Your Rule'),
  [PortalProductTextToken.HeaderBrandLeft]: decodeDisplayText("O'centra"),
  [PortalProductTextToken.HeaderBrandRight]: decodeDisplayText('Parent'),
  [PortalProductTextToken.FooterMadeWith]: decodeDisplayText('Made with'),
  [PortalProductTextToken.FooterBy]: decodeDisplayText('by'),
  [PortalProductTextToken.FooterHeart]: decodeDisplayText('heart'),
  [PortalProductTextToken.FooterLink]: decodeDisplayText('Ocentra'),
  [PortalProductTextToken.FooterVersion]: decodeDisplayText('[ alpha v0.1.1 ]'),
  [PortalProductTextToken.AuthClose]: decodeDisplayText('Close parent sign in'),
  [PortalProductTextToken.AuthEyebrow]: decodeDisplayText('Parent access'),
  [PortalProductTextToken.AuthTitle]: decodeDisplayText('Protect the family console'),
  [PortalProductTextToken.AuthBody]: decodeDisplayText(
    'Use a parent session before changing rules, approvals, drives, or child-device trust.'
  ),
  [PortalProductTextToken.AuthSignIn]: decodeDisplayText('Sign in'),
  [PortalProductTextToken.AuthSignUp]: decodeDisplayText('Create account'),
  [PortalProductTextToken.AuthParentEmail]: decodeDisplayText('Parent email'),
  [PortalProductTextToken.AuthParentName]: decodeDisplayText('Parent display name'),
  [PortalProductTextToken.AuthPassword]: decodeDisplayText('Password'),
  [PortalProductTextToken.AuthConfirmPassword]: decodeDisplayText('Confirm password'),
  [PortalProductTextToken.AuthPrimaryAction]: decodeDisplayText('Continue'),
  [PortalProductTextToken.AuthSocialTitle]: decodeDisplayText('Trusted sign-in options'),
  [PortalProductTextToken.AuthGoogle]: decodeDisplayText('Continue with Google'),
  [PortalProductTextToken.AuthFacebook]: decodeDisplayText('Continue with Facebook'),
  [PortalProductTextToken.AuthGuest]: decodeDisplayText('Continue as local parent'),
  [PortalProductTextToken.AuthTrustTitle]: decodeDisplayText('Private by design'),
  [PortalProductTextToken.AuthTrustBody]: decodeDisplayText(
    'Parent identity protects this console. Child activity stays local unless you choose an export.'
  ),
  [PortalProductTextToken.AuthUnavailable]: decodeDisplayText('Parent identity is not connected on this device yet.'),
  [PortalProductTextToken.OpenSettings]: decodeDisplayText('Settings'),
  [PortalProductTextToken.ChildDevice]: decodeDisplayText('Child device'),
  [PortalProductTextToken.FamilyDefault]: decodeDisplayText('Family default'),
  [PortalProductTextToken.DeviceRuleScope]: decodeDisplayText('Device rule scope'),
  [PortalProductTextToken.DeviceRuleScopeBody]: decodeDisplayText(
    'Choose a child device first, then apply web, app, schedule, approval, and explanation rules to that device.'
  ),
  [PortalProductTextToken.DeviceRuleScopeTip]: decodeDisplayText(
    'Device rules should override family defaults only for the selected child device and remain visible in the audit.'
  ),
  [PortalProductTextToken.ManagedWeb]: decodeDisplayText('Managed web'),
  [PortalProductTextToken.DeviceRuleOverride]: decodeDisplayText('Device override'),
  [PortalProductTextToken.PolicyModeAdvisory]: decodeDisplayText('Advisory'),
  [PortalProductTextToken.PolicyModeActive]: decodeDisplayText('Active'),
  [PortalProductTextToken.PolicyDecisionLinked]: decodeDisplayText('Policy decision linked'),
  [PortalProductTextToken.ActivityRecordLinked]: decodeDisplayText('Stored activity linked'),
  [PortalProductTextToken.FamilyRulesTitle]: decodeDisplayText('Your house, your rules'),
  [PortalProductTextToken.FamilyRulesBody]: decodeDisplayText(
    'Choose where the child can browse, when to ask you, what to explain, and what to block.'
  ),
  [PortalProductTextToken.HowItWorks]: decodeDisplayText('How it works'),
  [PortalProductTextToken.RiskToKnow]: decodeDisplayText('Risk to know'),
  [PortalProductTextToken.NotConfiguredStatus]: decodeDisplayText('Not configured'),
  [PortalProductTextToken.LocalOnlyStatus]: decodeDisplayText('Local only'),
  [PortalProductTextToken.BrowserSupportedTitle]: decodeDisplayText('Supported browsers'),
  [PortalProductTextToken.BrowserSupportedBody]: decodeDisplayText(
    'Use the managed browser path for the strongest visibility, policy checks, and explain-before-block flow.'
  ),
  [PortalProductTextToken.BrowserSupportedTip]: decodeDisplayText(
    'When the child device is connected, browser events carry typed evidence instead of private page dumps.'
  ),
  [PortalProductTextToken.BrowserUnsupportedTitle]: decodeDisplayText('Unsupported browsers'),
  [PortalProductTextToken.BrowserUnsupportedBody]: decodeDisplayText(
    'Show browsers that are visible but not controllable yet, so a parent can close the gap instead of guessing.'
  ),
  [PortalProductTextToken.BrowserUnsupportedTip]: decodeDisplayText(
    'Unsupported does not mean ignored; it means the app should explain the weaker protection boundary.'
  ),
  [PortalProductTextToken.BrowserBlockTitle]: decodeDisplayText('Block or allow'),
  [PortalProductTextToken.BrowserBlockBody]: decodeDisplayText(
    'Turn a supported browser rule into allow, ask-parent, explain-first, schedule-limit, or block behavior.'
  ),
  [PortalProductTextToken.BrowserBlockTip]: decodeDisplayText(
    'Blocking belongs on typed policy decisions with evidence references, never on hidden browser-side guesses.'
  ),
  [PortalProductTextToken.BrowserRiskBody]: decodeDisplayText(
    'A child may try another browser, private mode, or a different device; the portal should show that risk plainly.'
  ),
  [PortalProductTextToken.BrowserControls]: decodeDisplayText('Browser controls'),
  [PortalProductTextToken.DataCustodyTitle]: decodeDisplayText('Private by design'),
  [PortalProductTextToken.DataCustodyBody]: decodeDisplayText(
    'The child device keeps evidence locally. You decide whether diagnostics or exports leave the device.'
  ),
  [PortalProductTextToken.DataCustodyTip]: decodeDisplayText(
    'Exports are parent-owned. Raw private content is not shared with Ocentra by default.'
  ),
  [PortalProductTextToken.DisplayTheme]: decodeDisplayText('Display theme'),
  [PortalProductTextToken.DriveConnectionsTitle]: decodeDisplayText('Connect your drives'),
  [PortalProductTextToken.DriveConnectionsBody]: decodeDisplayText(
    'Prepare backups for a parent-owned drive, school archive, or support bundle when you choose.'
  ),
  [PortalProductTextToken.DriveConnectionsTip]: decodeDisplayText(
    'Drive sync must stay opt-in, revocable, and separate from core child safety decisions.'
  ),
  [PortalProductTextToken.RuleBuilderBody]: decodeDisplayText(
    'Create family rules for web, apps, games, schedules, local AI explanations, and exception requests.'
  ),
  [PortalProductTextToken.SchedulesBudgetsBody]: decodeDisplayText(
    'Set school, sleep, homework, and weekend windows without hiding what happened outside the window.'
  ),
  [PortalProductTextToken.ApprovalsBody]: decodeDisplayText(
    'Route ask-parent moments into a clear approve, deny, or explain outcome.'
  ),
  [PortalProductTextToken.NotificationsBody]: decodeDisplayText(
    'Choose which events deserve a parent alert and which should stay in the daily audit.'
  ),
  [PortalProductTextToken.DeviceInventoryBody]: decodeDisplayText(
    'See each child device, platform, service state, and whether protection is connected.'
  ),
  [PortalProductTextToken.PairingBody]: decodeDisplayText(
    'Pair desktop and mobile apps with a parent-owned local trust step before controls are enabled.'
  ),
  [PortalProductTextToken.DesktopAppBody]: decodeDisplayText(
    'The desktop app hosts the child-device service and native capabilities for Windows, macOS, and Linux.'
  ),
  [PortalProductTextToken.MobileAppBody]: decodeDisplayText(
    'The mobile app should surface parent decisions and platform-limited controls without overclaiming access.'
  ),
  [PortalProductTextToken.MemoryBody]: decodeDisplayText(
    'Derived memory links must cite stored evidence, selected policy versions, or parent actions.'
  ),
  [PortalProductTextToken.AiRuntimeBody]: decodeDisplayText(
    'Local AI should explain and summarize when available, while safety decisions remain typed and auditable.'
  ),
};
