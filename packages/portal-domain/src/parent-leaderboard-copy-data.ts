import { PortalRoute, type PortalRoute as PortalRouteValue } from './routes';
import {
  PARENT_LEADERBOARD_COPY_NAV_GROUPS,
  PARENT_LEADERBOARD_COPY_NAV_ITEMS,
  type ParentLeaderboardCopyNavGroup,
  type ParentLeaderboardCopyNavItem,
} from './parent-leaderboard-copy-nav';
import {
  PARENT_LEADERBOARD_COPY_GUIDE_TOPICS,
  type ParentLeaderboardCopyGuideTopic,
} from './parent-leaderboard-copy-guides';
import {
  PARENT_LEADERBOARD_COPY_MANAGE_QUICK_GAMES,
  PARENT_LEADERBOARD_COPY_MANAGE_ROWS,
} from './parent-leaderboard-copy-manage-data';

export type ParentLeaderboardCopyTone = 'cyan' | 'gold' | 'purple' | 'red' | 'muted';
export type ParentLeaderboardCopyTabId = 'overall' | 'perGame' | 'aiBenchmarks' | 'tournaments' | 'friends';
export type ParentLeaderboardCopyIconName =
  | 'activity'
  | 'bot'
  | 'calendar'
  | 'circle'
  | 'coins'
  | 'crown'
  | 'gamepad'
  | 'gift'
  | 'grid'
  | 'home'
  | 'medal'
  | 'shield'
  | 'swords'
  | 'trophy'
  | 'users';
export type ParentLeaderboardCopyRowSource = 'api' | 'fallbackRows' | 'aiBenchmarkRows';
export type ParentLeaderboardCopyPageMode = 'leaderboard' | 'gameLeaderboard' | 'aiBenchmarkLeaderboard';

export type ParentLeaderboardCopyRow = {
  user_id: string;
  rank: number;
  score: number;
  wins?: number;
  losses?: number;
  bestGame?: string;
  trend?: string;
  tone?: ParentLeaderboardCopyTone;
};

export type ParentLeaderboardCopyContent = {
  tabs: Array<{
    id: ParentLeaderboardCopyTabId;
    label: string;
    title: string;
  }>;
  navGroups: readonly ParentLeaderboardCopyNavGroup[];
  navItems: readonly ParentLeaderboardCopyNavItem[];
  tabDetails: Record<
    ParentLeaderboardCopyTabId,
    {
      eyebrow: string;
      title: string;
      summary: string;
      primary: string;
      secondary: string;
      action: string;
      tone: ParentLeaderboardCopyTone;
    }
  >;
  topGames: Array<{
    id: string;
    rank: number;
    name: string;
    matches: string;
    growth: string;
    tone: ParentLeaderboardCopyTone;
    category: string;
    subcategory: string;
    gameType: number;
    routePath: string;
  }>;
  quickGames: Array<{
    id: string;
    name: string;
    detail: string;
    icon: ParentLeaderboardCopyIconName;
    tone: ParentLeaderboardCopyTone;
    category: string;
    subcategory: string;
    gameType: number;
    routePath: string;
  }>;
  guideTopics: readonly ParentLeaderboardCopyGuideTopic[];
  fallbackRows: ParentLeaderboardCopyRow[];
  aiBenchmarkRows: ParentLeaderboardCopyRow[];
  distributionLabels: string[];
  season: {
    label: string;
    title: string;
    dateRange: string;
    actionLabel: string;
    detailTitle: string;
    detailSubtitle: string;
    stats: Array<{ label: string; value: string }>;
  };
  metricLabels: {
    totalPlayers: string;
    totalGames: string;
    rankedWins: string;
    nearbyPlayers: string;
    season: string;
    updated: string;
  };
  uiCopy: {
    hubTitle: string;
    topGamesTitle: string;
    distributionTitle: string;
    distributionCenterLabel: string;
    feedTitle: string;
    liveLabel: string;
    viewAllLabel: string;
    refreshLabel: string;
    queueLabel: string;
    showLabel: string;
    pageLabel: string;
    selectedPlayerLabel: string;
    detailSnapshotTitle: string;
    detailSnapshotLines: string[];
    loadingTitle: string;
    loadingBody: string;
    errorTitle: string;
  };
  modes: {
    leaderboard: {
      defaultTab: ParentLeaderboardCopyTabId;
      selectedGameId: string;
      title: string;
      routeLabel: string;
      rowSource: ParentLeaderboardCopyRowSource;
    };
    gameLeaderboard: {
      defaultTab: ParentLeaderboardCopyTabId;
      selectedGameId: string;
      title: string;
      routeLabel: string;
      rowSource: ParentLeaderboardCopyRowSource;
    };
    aiBenchmarkLeaderboard: {
      defaultTab: ParentLeaderboardCopyTabId;
      selectedGameId: string;
      title: string;
      routeLabel: string;
      rowSource: ParentLeaderboardCopyRowSource;
    };
  };
};

export type ParentLeaderboardCopyRouteContext = {
  readonly pageMode: ParentLeaderboardCopyPageMode;
  readonly navLabel: string;
  readonly selectedControlId: string;
};

export const PARENT_LEADERBOARD_COPY_ROUTE = {
  ClassName: 'parent-leaderboard-copy-route',
  PageMode: 'leaderboard',
  EmptyTimestamp: '',
  StatusText: {
    Local: 'LOCAL',
    Connecting: 'CONNECTING',
    CheckService: 'CHECK SERVICE',
    Offline: 'OFFLINE',
  },
} as const;

export const PARENT_LEADERBOARD_COPY_ROUTE_CONTEXT: Readonly<
  Partial<Record<PortalRouteValue, ParentLeaderboardCopyRouteContext>>
> = {
  [PortalRoute.Overview]: routeContext('leaderboard', 'OVERVIEW', 'activity-store'),
  [PortalRoute.LeaderboardCopy]: routeContext('leaderboard', 'START HERE', 'setup-overall'),
  [PortalRoute.Activity]: routeContext('leaderboard', 'ACTIVITY', 'activity-store'),
  [PortalRoute.Browser]: routeContext('gameLeaderboard', 'WEB', 'managed-web'),
  [PortalRoute.BrowserSettings]: routeContext('gameLeaderboard', 'BROWSER SETUP', 'browser-settings'),
  [PortalRoute.Policy]: routeContext('gameLeaderboard', 'RULES', 'policy-action'),
  [PortalRoute.RuleManagement]: routeContext('gameLeaderboard', 'RULE SETUP', 'rules-management'),
  [PortalRoute.Schedules]: routeContext('gameLeaderboard', 'SCHEDULES', 'schedules-budgets'),
  [PortalRoute.Approvals]: routeContext('gameLeaderboard', 'APPROVALS', 'approvals'),
  [PortalRoute.Enforcement]: routeContext('gameLeaderboard', 'ENFORCE', 'enforcement-readiness'),
  [PortalRoute.PrivacyDesign]: routeContext('aiBenchmarkLeaderboard', 'PRIVATE', 'privacy-design'),
  [PortalRoute.Memory]: routeContext('aiBenchmarkLeaderboard', 'MEMORY', 'memory-citations'),
  [PortalRoute.MemorySettings]: routeContext('aiBenchmarkLeaderboard', 'MEMORY SETUP', 'memory-settings'),
  [PortalRoute.AiRuntime]: routeContext('aiBenchmarkLeaderboard', 'AI SETUP', 'api-providers'),
  [PortalRoute.ApiProviders]: routeContext('aiBenchmarkLeaderboard', 'API KEYS', 'api-providers'),
  [PortalRoute.ReportSettings]: routeContext('gameLeaderboard', 'REPORT SETUP', 'reports-settings'),
  [PortalRoute.ScreenAnalysis]: routeContext('gameLeaderboard', 'SCREEN', 'screen-analysis'),
  [PortalRoute.AppGameSessions]: routeContext('gameLeaderboard', 'APPS/GAMES', 'app-game-sessions'),
  [PortalRoute.NetworkActivity]: routeContext('gameLeaderboard', 'NETWORK', 'network-activity'),
  [PortalRoute.Devices]: routeContext('gameLeaderboard', 'DEVICES', 'device-pairing'),
  [PortalRoute.Notifications]: routeContext('gameLeaderboard', 'ALERTS', 'notifications'),
  [PortalRoute.DriveConnections]: routeContext('gameLeaderboard', 'DRIVES', 'drive-exports'),
  [PortalRoute.RemoteAccess]: routeContext('gameLeaderboard', 'REMOTE', 'remote-access'),
  [PortalRoute.Subscription]: routeContext('gameLeaderboard', 'SUBSCRIPTION', 'subscription-plans'),
  [PortalRoute.PlatformsInstall]: routeContext('gameLeaderboard', 'PLATFORMS', 'platforms-install'),
  [PortalRoute.Diagnostics]: routeContext('gameLeaderboard', 'SUPPORT', 'support-api-status'),
  [PortalRoute.SettingsRules]: routeContext('gameLeaderboard', 'SETTINGS', 'family-settings'),
} as const;

export function parentLeaderboardCopyRouteContext(route: PortalRouteValue): ParentLeaderboardCopyRouteContext {
  return PARENT_LEADERBOARD_COPY_ROUTE_CONTEXT[route] ?? routeContext('leaderboard', 'OVERVIEW', 'managed-web');
}

export const PARENT_LEADERBOARD_COPY_ROWS: ParentLeaderboardCopyRow[] = [
  {
    user_id: 'Supported Browsers',
    rank: 1,
    score: 4928,
    wins: 24,
    losses: 0,
    bestGame: 'Managed Web',
    trend: 'Local',
    tone: 'gold',
  },
  {
    user_id: 'Unsupported Browsers',
    rank: 2,
    score: 3640,
    wins: 18,
    losses: 2,
    bestGame: 'Browser Gap',
    trend: 'Review',
    tone: 'cyan',
  },
  {
    user_id: 'Block or Allow',
    rank: 3,
    score: 3215,
    wins: 16,
    losses: 3,
    bestGame: 'Policy Action',
    trend: 'Ready',
    tone: 'red',
  },
  {
    user_id: 'App Sessions',
    rank: 4,
    score: 2980,
    wins: 14,
    losses: 4,
    bestGame: 'APP AND GAME SESSIONS',
    trend: '+2',
    tone: 'purple',
  },
  {
    user_id: 'Screen Analysis',
    rank: 5,
    score: 2865,
    wins: 12,
    losses: 5,
    bestGame: 'SCREEN ANALYSIS',
    trend: '+1',
    tone: 'cyan',
  },
  {
    user_id: 'Rule Builder',
    rank: 6,
    score: 2754,
    wins: 10,
    losses: 6,
    bestGame: 'Family Rules',
    trend: 'Draft',
    tone: 'gold',
  },
  {
    user_id: 'Device Pairing',
    rank: 7,
    score: 2645,
    wins: 8,
    losses: 7,
    bestGame: 'Child Device',
    trend: 'Local',
    tone: 'purple',
  },
  {
    user_id: 'Drive Exports',
    rank: 8,
    score: 2523,
    wins: 6,
    losses: 8,
    bestGame: 'Parent Owned',
    trend: 'Opt in',
    tone: 'cyan',
  },
  {
    user_id: 'Notifications',
    rank: 9,
    score: 2400,
    wins: 4,
    losses: 1,
    bestGame: 'Parent Alerts',
    trend: 'Opt in',
    tone: 'red',
  },
  {
    user_id: 'Private by Design',
    rank: 10,
    score: 2320,
    wins: 4,
    losses: 0,
    bestGame: 'Data Custody',
    trend: 'Local',
    tone: 'gold',
  },
];

export const PARENT_LEADERBOARD_COPY_CONTENT: ParentLeaderboardCopyContent = {
  tabs: [
    { id: 'overall', label: 'OVERVIEW', title: 'PARENT COMMAND OVERVIEW' },
    { id: 'perGame', label: 'CONTROL AREAS', title: 'DEVICE CONTROL AREAS' },
    { id: 'aiBenchmarks', label: 'AI', title: 'AI READINESS' },
    { id: 'tournaments', label: 'SCHEDULES', title: 'SCHEDULES AND APPROVALS' },
    { id: 'friends', label: 'SUPPORT', title: 'SUPPORT AND EXPORTS' },
  ],
  navGroups: PARENT_LEADERBOARD_COPY_NAV_GROUPS,
  navItems: PARENT_LEADERBOARD_COPY_NAV_ITEMS,
  tabDetails: {
    overall: {
      eyebrow: 'Family command',
      title: 'Your house, your rules',
      summary: 'See local child-device state, recent evidence, and which controls are ready to wire.',
      primary: 'Local device visibility',
      secondary: 'No cloud sharing by default',
      action: 'Review today',
      tone: 'cyan',
    },
    perGame: {
      eyebrow: 'Control surface',
      title: 'Browser, app, and policy controls',
      summary: 'Choose which areas the parent can configure per child device.',
      primary: 'Per-device rules',
      secondary: 'Allow, ask, explain, schedule, or block',
      action: 'Open controls',
      tone: 'gold',
    },
    aiBenchmarks: {
      eyebrow: 'AI',
      title: 'Local models, API providers, and memory',
      summary: 'Use local AI first, allow API AI only when a parent chooses provider, data scope, and device policy.',
      primary: 'Evidence cited',
      secondary: 'Per-device model and provider choices',
      action: 'Open AI setup',
      tone: 'red',
    },
    tournaments: {
      eyebrow: 'Family routine',
      title: 'Schedules, approvals, and devices',
      summary: 'Pair each child device and apply family defaults or per-device overrides.',
      primary: 'Child device first',
      secondary: 'Rules stay auditable',
      action: 'Open settings',
      tone: 'purple',
    },
    friends: {
      eyebrow: 'Parent owned',
      title: 'Support, exports, and drive connections',
      summary: 'Diagnostics and backups are parent-owned, opt-in, and revocable.',
      primary: 'Private by design',
      secondary: 'Export only when parent chooses',
      action: 'Open support',
      tone: 'cyan',
    },
  },
  topGames: [
    {
      id: 'managed-web',
      rank: 1,
      name: 'Managed Web',
      matches: 'Ready path',
      growth: 'Advisory',
      tone: 'gold',
      category: 'Browser',
      subcategory: 'Supported browsers',
      gameType: 1,
      routePath: '#/browser',
    },
    {
      id: 'browser-gap',
      rank: 2,
      name: 'Browser Gap',
      matches: 'Visible',
      growth: 'Not configured',
      tone: 'cyan',
      category: 'Browser',
      subcategory: 'Unsupported browsers',
      gameType: 2,
      routePath: '#/browser',
    },
    {
      id: 'policy-action',
      rank: 3,
      name: 'Policy Action',
      matches: 'Allow Ask Block',
      growth: 'Ready',
      tone: 'red',
      category: 'Policy',
      subcategory: 'Block or allow',
      gameType: 3,
      routePath: '#/policy',
    },
    {
      id: 'activity-store',
      rank: 4,
      name: 'Activity Store',
      matches: 'Local only',
      growth: 'Evidence cited',
      tone: 'purple',
      category: 'Activity',
      subcategory: 'Recent events',
      gameType: 4,
      routePath: '#/activity',
    },
    {
      id: 'drive-exports',
      rank: 5,
      name: 'Drive Exports',
      matches: 'Parent owned',
      growth: 'Opt in',
      tone: 'cyan',
      category: 'Support',
      subcategory: 'Connect your drives',
      gameType: 5,
      routePath: '#/drive-connections',
    },
    {
      id: 'privacy-design',
      rank: 6,
      name: 'Private by Design',
      matches: 'Local first',
      growth: 'No cloud share',
      tone: 'gold',
      category: 'Privacy',
      subcategory: 'Data custody',
      gameType: 6,
      routePath: '#/privacy-design',
    },
    {
      id: 'memory-citations',
      rank: 7,
      name: 'Memory Citations',
      matches: 'Cited links',
      growth: 'Freshness gated',
      tone: 'purple',
      category: 'Memory',
      subcategory: 'Local knowledge',
      gameType: 7,
      routePath: '#/memory',
    },
    {
      id: 'notifications',
      rank: 8,
      name: 'Notifications',
      matches: 'Parent only',
      growth: 'Opt in',
      tone: 'red',
      category: 'Devices',
      subcategory: 'Parent alerts',
      gameType: 8,
      routePath: '#/notifications',
    },
    {
      id: 'family-settings',
      rank: 9,
      name: 'Family Settings',
      matches: 'Defaults',
      growth: 'Per device',
      tone: 'cyan',
      category: 'Devices',
      subcategory: 'Family defaults',
      gameType: 9,
      routePath: '#/settings-rules',
    },
    {
      id: 'api-providers',
      rank: 10,
      name: 'API Providers',
      matches: 'Optional',
      growth: 'Parent scoped',
      tone: 'purple',
      category: 'AI',
      subcategory: 'External AI setup',
      gameType: 10,
      routePath: '#/api-providers',
    },
    {
      id: 'device-pairing',
      rank: 11,
      name: 'Device Pairing',
      matches: 'Trusted',
      growth: 'Per child',
      tone: 'cyan',
      category: 'Devices',
      subcategory: 'Pairing and status',
      gameType: 11,
      routePath: '#/devices',
    },
  ],
  quickGames: [
    {
      id: 'managed-web',
      name: 'MANAGED WEB',
      detail: 'Supported browser path',
      icon: 'shield',
      tone: 'gold',
      category: 'Browser',
      subcategory: 'Supported browsers',
      gameType: 1,
      routePath: '#/browser',
    },
    ...PARENT_LEADERBOARD_COPY_MANAGE_QUICK_GAMES,
    {
      id: 'policy-action',
      name: 'POLICY ACTION',
      detail: 'Allow, ask, explain, block',
      icon: 'grid',
      tone: 'red',
      category: 'Policy',
      subcategory: 'Rules and approvals',
      gameType: 3,
      routePath: '#/policy',
    },
    {
      id: 'local-ai',
      name: 'LOCAL AI',
      detail: 'On-device evaluator',
      icon: 'bot',
      tone: 'purple',
      category: 'AI',
      subcategory: 'Evidence summaries',
      gameType: 6,
      routePath: '#/ai-runtime',
    },
    {
      id: 'api-providers',
      name: 'API PROVIDERS',
      detail: 'Optional external AI',
      icon: 'bot',
      tone: 'purple',
      category: 'AI',
      subcategory: 'Provider setup',
      gameType: 6,
      routePath: '#/api-providers',
    },
    {
      id: 'local-ai-hub',
      name: 'LOCAL AI HUB',
      detail: 'Shared home model queue',
      icon: 'bot',
      tone: 'cyan',
      category: 'AI',
      subcategory: 'Local hub',
      gameType: 6,
      routePath: '#/ai-runtime',
    },
    {
      id: 'privacy-design',
      name: 'PRIVATE BY DESIGN',
      detail: 'Local-first custody',
      icon: 'shield',
      tone: 'gold',
      category: 'Privacy',
      subcategory: 'Data custody',
      gameType: 6,
      routePath: '#/privacy-design',
    },
    {
      id: 'memory-citations',
      name: 'MEMORY CITATIONS',
      detail: 'Cited local knowledge',
      icon: 'circle',
      tone: 'purple',
      category: 'Memory',
      subcategory: 'Freshness gated',
      gameType: 7,
      routePath: '#/memory',
    },
    {
      id: 'device-pairing',
      name: 'DEVICE PAIRING',
      detail: 'Child device trust',
      icon: 'gamepad',
      tone: 'cyan',
      category: 'Devices',
      subcategory: 'Pairing and status',
      gameType: 7,
      routePath: '#/devices',
    },
    {
      id: 'notifications',
      name: 'NOTIFICATIONS',
      detail: 'Parent alert routing',
      icon: 'gift',
      tone: 'red',
      category: 'Devices',
      subcategory: 'Opt-in parent alerts',
      gameType: 8,
      routePath: '#/notifications',
    },
    {
      id: 'family-settings',
      name: 'FAMILY SETTINGS',
      detail: 'Defaults and overrides',
      icon: 'crown',
      tone: 'cyan',
      category: 'Devices',
      subcategory: 'Per-device rules',
      gameType: 9,
      routePath: '#/settings-rules',
    },
    {
      id: 'support-exports',
      name: 'SUPPORT EXPORTS',
      detail: 'Parent-owned bundles',
      icon: 'trophy',
      tone: 'gold',
      category: 'Support',
      subcategory: 'Diagnostics and drives',
      gameType: 8,
      routePath: '#/diagnostics',
    },
    {
      id: 'support-api-status',
      name: 'SUPPORT AND API STATUS',
      detail: 'Routes and capability gaps',
      icon: 'users',
      tone: 'cyan',
      category: 'Support',
      subcategory: 'Protocol and platform',
      gameType: 8,
      routePath: '#/diagnostics',
    },
  ],
  guideTopics: PARENT_LEADERBOARD_COPY_GUIDE_TOPICS,
  fallbackRows: [...PARENT_LEADERBOARD_COPY_ROWS, ...PARENT_LEADERBOARD_COPY_MANAGE_ROWS],
  aiBenchmarkRows: [
    {
      user_id: 'Local Models',
      rank: 1,
      score: 9812,
      wins: 18,
      losses: 1,
      bestGame: 'Local AI',
      trend: 'Ready',
      tone: 'red',
    },
    {
      user_id: 'API Providers',
      rank: 2,
      score: 9381,
      wins: 16,
      losses: 2,
      bestGame: 'External AI',
      trend: 'Optional',
      tone: 'purple',
    },
    {
      user_id: 'Local AI Hub',
      rank: 3,
      score: 9034,
      wins: 14,
      losses: 3,
      bestGame: 'Model queue',
      trend: 'Planned',
      tone: 'gold',
    },
    {
      user_id: 'Cited Memory',
      rank: 4,
      score: 8810,
      wins: 12,
      losses: 4,
      bestGame: 'Memory links',
      trend: 'Local',
      tone: 'purple',
    },
  ],
  distributionLabels: ['LOCAL 100%', 'CLOUD 0%', 'EXPORT OPT-IN', 'RAW PRIVATE'],
  season: {
    label: 'LOCAL',
    title: 'YOUR HOUSE YOUR RULE',
    dateRange: 'Child device service',
    actionLabel: 'SETTINGS',
    detailTitle: 'FAMILY DEFAULTS',
    detailSubtitle: 'Device rules, schedules, approvals, and drive exports',
    stats: [
      { label: 'DATA', value: 'LOCAL' },
      { label: 'EXPORT', value: 'OPT-IN' },
      { label: 'MODE', value: 'ADVISORY' },
    ],
  },
  metricLabels: {
    totalPlayers: 'CONTROL AREAS',
    totalGames: 'DEVICES',
    rankedWins: 'READY PATHS',
    nearbyPlayers: 'EVENTS',
    season: 'MODE',
    updated: 'UPDATED',
  },
  uiCopy: {
    hubTitle: 'PARENT HUB',
    topGamesTitle: 'CONTROL AREAS',
    distributionTitle: 'DATA CUSTODY',
    distributionCenterLabel: 'LOCAL',
    feedTitle: 'LIVE DEVICE FEED',
    liveLabel: 'LIVE',
    viewAllLabel: 'VIEW ALL',
    refreshLabel: 'REFRESH',
    queueLabel: 'RECONNECT',
    showLabel: 'SHOW',
    pageLabel: 'PAGE',
    selectedPlayerLabel: 'SELECTED CONTROL',
    detailSnapshotTitle: 'CONTROL SNAPSHOT',
    detailSnapshotLines: [
      'Each parent control should map to a typed child-device event, policy decision, or setup gap.',
      'Evidence references stay local unless a parent chooses a diagnostic export or drive connection.',
    ],
    loadingTitle: 'LOADING PARENT SURFACE',
    loadingBody: 'Refreshing local child-device visibility.',
    errorTitle: 'PARENT SURFACE UNAVAILABLE',
  },
  modes: {
    leaderboard: {
      defaultTab: 'overall',
      selectedGameId: 'managed-web',
      title: 'Parent Command Deck',
      routeLabel: '#/overview',
      rowSource: 'fallbackRows',
    },
    gameLeaderboard: {
      defaultTab: 'perGame',
      selectedGameId: 'managed-web',
      title: 'Control Detail',
      routeLabel: '#/browser',
      rowSource: 'fallbackRows',
    },
    aiBenchmarkLeaderboard: {
      defaultTab: 'aiBenchmarks',
      selectedGameId: 'api-providers',
      title: 'AI',
      routeLabel: '#/ai-runtime',
      rowSource: 'aiBenchmarkRows',
    },
  },
};

function routeContext(
  pageMode: ParentLeaderboardCopyPageMode,
  navLabel: string,
  selectedControlId: string
): ParentLeaderboardCopyRouteContext {
  return { pageMode, navLabel, selectedControlId };
}
