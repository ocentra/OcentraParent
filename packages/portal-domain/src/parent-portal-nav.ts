import type { ParentPortalIconName, ParentPortalTabId, ParentPortalTone } from './parent-portal-data';
import type { PortalRoute as PortalRouteValue } from './routes';

export type ParentPortalNavGroupId = 'quickGlance' | 'guide' | 'manage';

export const PARENT_PORTAL_NAV_LABELS = {
  QuickGlance: 'QUICK GLANCE',
  Guide: 'GUIDE',
  Manage: 'MANAGE',
  Policies: 'POLICY',
  Activity: 'ACTIVITY',
  Devices: 'DEVICES',
  Portal: 'PORTAL',
  DataPrivacy: 'DATA',
  AiMemory: 'AI',
  Account: 'ACCOUNT',
  StartHere: 'START HERE',
  Overview: 'OVERVIEW',
  Web: 'WEB',
  RulesGuide: 'RULES',
  MemoryGuide: 'MEMORY',
  Ai: 'AI',
  ReportsGuide: 'REPORTS',
  Private: 'PRIVATE',
  Browser: 'BROWSER',
  Apps: 'APPS',
  Games: 'GAMES',
  RuleSet: 'RULES',
  Schedules: 'SCHEDULES',
  Approvals: 'APPROVALS',
  Enforce: 'ENFORCE',
  ReportSet: 'REPORTS',
  Lan: 'LAN',
  Capability: 'CAPABILITY',
  Screen: 'SCREEN',
  AppsGames: 'APP USE',
  Network: 'NETWORK',
  Tracking: 'TRACKING',
  Alerts: 'ALERTS',
  Channels: 'CHANNELS',
  Drives: 'DRIVES',
  Export: 'EXPORT',
  AiSetup: 'AI SETUP',
  ApiKeys: 'API KEYS',
  MemorySet: 'MEMORY SET',
  Remote: 'REMOTE',
  Builder: 'BUILDER',
  Audit: 'AUDIT',
  Plan: 'PLAN',
  Access: 'ACCESS',
  Platforms: 'PLATFORMS',
  Updates: 'UPDATES',
  Support: 'SUPPORT',
  Settings: 'SETTINGS',
} as const;

export type ParentPortalHashRoutePath = `#/${PortalRouteValue}`;
export type ParentPortalNavLabel = (typeof PARENT_PORTAL_NAV_LABELS)[keyof typeof PARENT_PORTAL_NAV_LABELS];
export type ParentPortalNavSectionLabel =
  | typeof PARENT_PORTAL_NAV_LABELS.Portal
  | typeof PARENT_PORTAL_NAV_LABELS.Devices
  | typeof PARENT_PORTAL_NAV_LABELS.Activity
  | typeof PARENT_PORTAL_NAV_LABELS.Policies
  | typeof PARENT_PORTAL_NAV_LABELS.DataPrivacy
  | typeof PARENT_PORTAL_NAV_LABELS.AiMemory
  | typeof PARENT_PORTAL_NAV_LABELS.Account;

export type ParentPortalNavGroup = {
  readonly id: ParentPortalNavGroupId;
  readonly label: ParentPortalNavLabel;
  readonly detail: string;
};

export type ParentPortalNavItem = {
  readonly label: ParentPortalNavLabel;
  readonly detail: string;
  readonly icon: ParentPortalIconName;
  readonly tabId: ParentPortalTabId;
  readonly groupId: ParentPortalNavGroupId;
  readonly sectionLabel?: ParentPortalNavSectionLabel;
  readonly tone?: ParentPortalTone;
  readonly routePath?: ParentPortalHashRoutePath;
};

export const PARENT_PORTAL_NAV_GROUPS: readonly ParentPortalNavGroup[] = [
  { id: 'quickGlance', label: PARENT_PORTAL_NAV_LABELS.QuickGlance, detail: 'Current child-device state' },
  { id: 'guide', label: PARENT_PORTAL_NAV_LABELS.Guide, detail: 'Policy, privacy, and local AI' },
  {
    id: 'manage',
    label: PARENT_PORTAL_NAV_LABELS.Manage,
    detail: 'Portal, device, activity, policy, data, AI, account',
  },
] as const;

export const PARENT_PORTAL_NAV_ITEMS: readonly ParentPortalNavItem[] = [
  {
    label: PARENT_PORTAL_NAV_LABELS.StartHere,
    detail: 'Setup and controls map',
    icon: 'start',
    tabId: 'overall',
    groupId: 'guide',
    tone: 'cyan',
    routePath: '#/start',
  },
  {
    label: PARENT_PORTAL_NAV_LABELS.Overview,
    detail: 'Recent child-device state',
    icon: 'overview',
    tabId: 'overall',
    groupId: 'quickGlance',
    tone: 'cyan',
    routePath: '#/overview',
  },
  {
    label: PARENT_PORTAL_NAV_LABELS.Web,
    detail: 'Supported and unsupported',
    icon: 'web',
    tabId: 'controls',
    groupId: 'quickGlance',
    tone: 'cyan',
    routePath: '#/browser',
  },
  {
    label: PARENT_PORTAL_NAV_LABELS.RulesGuide,
    detail: 'Allow, ask, explain, block',
    icon: 'rules',
    tabId: 'controls',
    groupId: 'guide',
    tone: 'gold',
    routePath: '#/policy',
  },
  {
    label: PARENT_PORTAL_NAV_LABELS.MemoryGuide,
    detail: 'Cited local knowledge',
    icon: 'ai-memory',
    tabId: 'aiStatus',
    groupId: 'guide',
    tone: 'purple',
    routePath: '#/memory',
  },
  {
    label: PARENT_PORTAL_NAV_LABELS.Ai,
    detail: 'Local and API AI',
    icon: 'ai-setup',
    tabId: 'aiStatus',
    groupId: 'guide',
    tone: 'cyan',
    routePath: '#/ai-guide',
  },
  {
    label: PARENT_PORTAL_NAV_LABELS.ReportsGuide,
    detail: 'Daily weekly monthly',
    icon: 'report',
    tabId: 'aiStatus',
    groupId: 'guide',
    tone: 'purple',
    routePath: '#/reports-guide',
  },
  {
    label: PARENT_PORTAL_NAV_LABELS.Private,
    detail: 'Data stays local',
    icon: 'privacy',
    tabId: 'aiStatus',
    groupId: 'guide',
    tone: 'gold',
    routePath: '#/privacy-design',
  },
  {
    label: PARENT_PORTAL_NAV_LABELS.Portal,
    detail: 'Portal settings alerts channels',
    icon: 'portal',
    tabId: 'routines',
    groupId: 'manage',
    tone: 'cyan',
    routePath: '#/settings-rules',
  },
  {
    label: PARENT_PORTAL_NAV_LABELS.Devices,
    detail: 'LAN discovery and device management',
    icon: 'lan',
    tabId: 'routines',
    groupId: 'manage',
    tone: 'cyan',
    routePath: '#/lan-pairing',
  },
  {
    label: PARENT_PORTAL_NAV_LABELS.Activity,
    detail: 'Reports and activity management',
    icon: 'activity',
    tabId: 'controls',
    groupId: 'manage',
    tone: 'purple',
    routePath: '#/activity',
  },
  {
    label: PARENT_PORTAL_NAV_LABELS.Browser,
    detail: 'Managed and unmanaged',
    icon: 'browser',
    tabId: 'controls',
    groupId: 'manage',
    sectionLabel: PARENT_PORTAL_NAV_LABELS.Policies,
    tone: 'gold',
    routePath: '#/browser-settings',
  },
  {
    label: PARENT_PORTAL_NAV_LABELS.Apps,
    detail: 'App rules and budgets',
    icon: 'app',
    tabId: 'controls',
    groupId: 'manage',
    sectionLabel: PARENT_PORTAL_NAV_LABELS.Policies,
    tone: 'gold',
    routePath: '#/policy-apps',
  },
  {
    label: PARENT_PORTAL_NAV_LABELS.Games,
    detail: 'Game limits and sessions',
    icon: 'games',
    tabId: 'controls',
    groupId: 'manage',
    sectionLabel: PARENT_PORTAL_NAV_LABELS.Policies,
    tone: 'purple',
    routePath: '#/policy-games',
  },
  {
    label: PARENT_PORTAL_NAV_LABELS.Screen,
    detail: 'Screen capture policy',
    icon: 'screen',
    tabId: 'controls',
    groupId: 'manage',
    sectionLabel: PARENT_PORTAL_NAV_LABELS.Policies,
    tone: 'cyan',
    routePath: '#/policy-screen',
  },
  {
    label: PARENT_PORTAL_NAV_LABELS.Network,
    detail: 'Network metadata policy',
    icon: 'web',
    tabId: 'controls',
    groupId: 'manage',
    sectionLabel: PARENT_PORTAL_NAV_LABELS.Policies,
    tone: 'cyan',
    routePath: '#/policy-network',
  },
  {
    label: PARENT_PORTAL_NAV_LABELS.Tracking,
    detail: 'Device location policy',
    icon: 'devices',
    tabId: 'controls',
    groupId: 'manage',
    sectionLabel: PARENT_PORTAL_NAV_LABELS.Policies,
    tone: 'cyan',
    routePath: '#/policy-tracking',
  },
  {
    label: PARENT_PORTAL_NAV_LABELS.DataPrivacy,
    detail: 'Storage export retention audit',
    icon: 'drives',
    tabId: 'controls',
    groupId: 'manage',
    tone: 'gold',
    routePath: '#/drive-connections',
  },
  {
    label: PARENT_PORTAL_NAV_LABELS.AiMemory,
    detail: 'Models providers memory',
    icon: 'ai-setup',
    tabId: 'aiStatus',
    groupId: 'manage',
    tone: 'purple',
    routePath: '#/ai-runtime',
  },
  {
    label: PARENT_PORTAL_NAV_LABELS.Account,
    detail: 'Plan access support',
    icon: 'account',
    tabId: 'controls',
    groupId: 'manage',
    tone: 'gold',
    routePath: '#/subscription',
  },
] as const;
