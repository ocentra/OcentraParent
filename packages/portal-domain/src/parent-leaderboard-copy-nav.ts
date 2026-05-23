import type {
  ParentLeaderboardCopyIconName,
  ParentLeaderboardCopyTabId,
  ParentLeaderboardCopyTone,
} from './parent-leaderboard-copy-data';

export type ParentLeaderboardCopyNavGroupId = 'quickGlance' | 'guide' | 'manage';

export type ParentLeaderboardCopyNavGroup = {
  readonly id: ParentLeaderboardCopyNavGroupId;
  readonly label: string;
  readonly detail: string;
};

export type ParentLeaderboardCopyNavItem = {
  readonly label: string;
  readonly detail: string;
  readonly icon: ParentLeaderboardCopyIconName;
  readonly tabId: ParentLeaderboardCopyTabId;
  readonly groupId: ParentLeaderboardCopyNavGroupId;
  readonly tone?: ParentLeaderboardCopyTone;
  readonly routePath?: string;
};

export const PARENT_LEADERBOARD_COPY_NAV_GROUPS: readonly ParentLeaderboardCopyNavGroup[] = [
  { id: 'quickGlance', label: 'QUICK GLANCE', detail: 'Current child-device state' },
  { id: 'guide', label: 'GUIDE', detail: 'Policy, privacy, and local AI' },
  { id: 'manage', label: 'MANAGE', detail: 'Devices, alerts, drives, support' },
] as const;

export const PARENT_LEADERBOARD_COPY_NAV_ITEMS: readonly ParentLeaderboardCopyNavItem[] = [
  {
    label: 'START HERE',
    detail: 'Setup and controls map',
    icon: 'home',
    tabId: 'overall',
    groupId: 'guide',
    tone: 'cyan',
  },
  {
    label: 'OVERVIEW',
    detail: 'Recent child-device state',
    icon: 'activity',
    tabId: 'overall',
    groupId: 'quickGlance',
    tone: 'cyan',
    routePath: '#/overview',
  },
  {
    label: 'ACTIVITY',
    detail: 'Stored activity timeline',
    icon: 'activity',
    tabId: 'overall',
    groupId: 'quickGlance',
    tone: 'purple',
    routePath: '#/activity',
  },
  {
    label: 'WEB',
    detail: 'Supported and unsupported',
    icon: 'shield',
    tabId: 'perGame',
    groupId: 'quickGlance',
    tone: 'cyan',
    routePath: '#/browser',
  },
  {
    label: 'RULES',
    detail: 'Allow, ask, explain, block',
    icon: 'grid',
    tabId: 'perGame',
    groupId: 'guide',
    tone: 'gold',
  },
  {
    label: 'MEMORY',
    detail: 'Cited local knowledge',
    icon: 'circle',
    tabId: 'aiBenchmarks',
    groupId: 'guide',
    tone: 'purple',
  },
  {
    label: 'AI',
    detail: 'Local and API AI',
    icon: 'bot',
    tabId: 'aiBenchmarks',
    groupId: 'guide',
    tone: 'cyan',
  },
  {
    label: 'REPORTS',
    detail: 'Daily weekly monthly',
    icon: 'grid',
    tabId: 'aiBenchmarks',
    groupId: 'guide',
    tone: 'purple',
  },
  {
    label: 'PRIVATE',
    detail: 'Data stays local',
    icon: 'shield',
    tabId: 'aiBenchmarks',
    groupId: 'guide',
    tone: 'gold',
  },
  {
    label: 'DEVICES',
    detail: 'Child device pairing',
    icon: 'gamepad',
    tabId: 'tournaments',
    groupId: 'manage',
    tone: 'cyan',
    routePath: '#/devices',
  },
  {
    label: 'ALERTS',
    detail: 'Parent notifications',
    icon: 'gift',
    tabId: 'tournaments',
    groupId: 'manage',
    tone: 'red',
    routePath: '#/notifications',
  },
  {
    label: 'DRIVES',
    detail: 'Connect your drives',
    icon: 'trophy',
    tabId: 'friends',
    groupId: 'manage',
    tone: 'gold',
    routePath: '#/drive-connections',
  },
  {
    label: 'AI SETUP',
    detail: 'Models and API providers',
    icon: 'bot',
    tabId: 'aiBenchmarks',
    groupId: 'manage',
    tone: 'purple',
    routePath: '#/ai-runtime',
  },
  {
    label: 'SUBSCRIPTION',
    detail: 'Plans and device limits',
    icon: 'coins',
    tabId: 'friends',
    groupId: 'manage',
    tone: 'gold',
    routePath: '#/settings-rules',
  },
  {
    label: 'SUPPORT',
    detail: 'Diagnostics and help',
    icon: 'users',
    tabId: 'friends',
    groupId: 'manage',
    tone: 'cyan',
    routePath: '#/diagnostics',
  },
  {
    label: 'SETTINGS',
    detail: 'Family defaults',
    icon: 'crown',
    tabId: 'tournaments',
    groupId: 'manage',
    tone: 'cyan',
    routePath: '#/settings-rules',
  },
] as const;
