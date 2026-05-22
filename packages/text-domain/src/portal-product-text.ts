import { decodeDisplayText, type DisplayText } from './contracts';
import { PortalProductTextToken, type PortalProductTextTokenValue } from './portal-product-text-tokens';

export { PortalProductTextToken };

export const PortalProductText: Record<PortalProductTextTokenValue, DisplayText> = {
  [PortalProductTextToken.ProductStatusLive]: decodeDisplayText('Child device connected'),
  [PortalProductTextToken.ProductStatusLocalOnly]: decodeDisplayText('Private device data'),
  [PortalProductTextToken.ProductStatusPreviewOnly]: decodeDisplayText('Advisory mode'),
  [PortalProductTextToken.NavGroupMonitor]: decodeDisplayText('Today'),
  [PortalProductTextToken.NavGroupGuide]: decodeDisplayText('Guide'),
  [PortalProductTextToken.NavGroupOperate]: decodeDisplayText('Manage'),
  [PortalProductTextToken.Overview]: decodeDisplayText('Overview'),
  [PortalProductTextToken.Activity]: decodeDisplayText('Activity'),
  [PortalProductTextToken.Browser]: decodeDisplayText('Web'),
  [PortalProductTextToken.Policy]: decodeDisplayText('Policy'),
  [PortalProductTextToken.Memory]: decodeDisplayText('Memory'),
  [PortalProductTextToken.AiRuntime]: decodeDisplayText('Local AI'),
  [PortalProductTextToken.Devices]: decodeDisplayText('Devices'),
  [PortalProductTextToken.Diagnostics]: decodeDisplayText('Support'),
  [PortalProductTextToken.SettingsRules]: decodeDisplayText('Settings'),
  [PortalProductTextToken.Commands]: decodeDisplayText('Controls'),
  [PortalProductTextToken.Events]: decodeDisplayText('Audit'),
  [PortalProductTextToken.OverviewDescription]: decodeDisplayText(
    'The daily command center for device health, activity, web visibility, and family rules.'
  ),
  [PortalProductTextToken.ActivityDescription]: decodeDisplayText(
    'Stored activity, app focus, network visibility, and source custody from the child device.'
  ),
  [PortalProductTextToken.BrowserDescription]: decodeDisplayText(
    'Managed web state, recent URL evidence, and browser protection readiness.'
  ),
  [PortalProductTextToken.PolicyDescription]: decodeDisplayText(
    'Family rule decisions, reason codes, evidence references, and protection mode.'
  ),
  [PortalProductTextToken.MemoryDescription]: decodeDisplayText(
    'Evidence-cited memory and activity graph visibility derived from stored local facts.'
  ),
  [PortalProductTextToken.AiRuntimeDescription]: decodeDisplayText(
    'Local provider readiness, model cache, privacy mode, and execution state.'
  ),
  [PortalProductTextToken.DevicesDescription]: decodeDisplayText(
    'Child device identity, platform status, LAN pairing direction, and device scope.'
  ),
  [PortalProductTextToken.DiagnosticsDescription]: decodeDisplayText(
    'Support evidence, service logs, export readiness, and copyable diagnostics.'
  ),
  [PortalProductTextToken.SettingsRulesDescription]: decodeDisplayText(
    'Parent-authored settings, schedules, budgets, approvals, and sensitive capability controls.'
  ),
  [PortalProductTextToken.CommandsDescription]: decodeDisplayText(
    'Safe device refresh actions that use the real child-device connection.'
  ),
  [PortalProductTextToken.EventsDescription]: decodeDisplayText(
    'Validated child-device audit entries from the local service.'
  ),
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
