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
  navItems: Array<{
    label: string;
    detail: string;
    icon: ParentLeaderboardCopyIconName;
    tabId: ParentLeaderboardCopyTabId;
    tone?: ParentLeaderboardCopyTone;
  }>;
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
    bestGame: 'Activity',
    trend: '+2',
    tone: 'purple',
  },
  {
    user_id: 'Screen Analysis',
    rank: 5,
    score: 2865,
    wins: 12,
    losses: 5,
    bestGame: 'Local AI',
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
];

export const PARENT_LEADERBOARD_COPY_CONTENT: ParentLeaderboardCopyContent = {
  tabs: [
    { id: 'overall', label: 'OVERVIEW', title: 'PARENT COMMAND OVERVIEW' },
    { id: 'perGame', label: 'CONTROL AREAS', title: 'DEVICE CONTROL AREAS' },
    { id: 'aiBenchmarks', label: 'LOCAL AI', title: 'LOCAL AI READINESS' },
    { id: 'tournaments', label: 'SCHEDULES', title: 'SCHEDULES AND APPROVALS' },
    { id: 'friends', label: 'SUPPORT', title: 'SUPPORT AND EXPORTS' },
  ],
  navItems: [
    { label: 'TODAY', detail: 'Recent child-device state', icon: 'activity', tabId: 'overall', tone: 'cyan' },
    { label: 'BROWSERS', detail: 'Supported and unsupported', icon: 'shield', tabId: 'perGame', tone: 'cyan' },
    { label: 'RULES', detail: 'Allow, ask, explain, block', icon: 'grid', tabId: 'perGame', tone: 'cyan' },
    { label: 'MEMORY', detail: 'Cited local knowledge', icon: 'circle', tabId: 'aiBenchmarks', tone: 'purple' },
    { label: 'LOCAL AI', detail: 'Private explanations', icon: 'bot', tabId: 'aiBenchmarks', tone: 'cyan' },
    { label: 'DEVICES', detail: 'Child device pairing', icon: 'gamepad', tabId: 'tournaments', tone: 'cyan' },
    { label: 'EXPORTS', detail: 'Diagnostics and drives', icon: 'trophy', tabId: 'friends', tone: 'cyan' },
    { label: 'SETTINGS', detail: 'Family defaults', icon: 'crown', tabId: 'tournaments', tone: 'cyan' },
  ],
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
      eyebrow: 'Local AI',
      title: 'Private explanations and memory',
      summary: 'Use local AI for explanation and summaries when available, without making hidden policy decisions.',
      primary: 'Evidence cited',
      secondary: 'Raw private content stays local',
      action: 'Open local AI',
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
      routePath: '/parent/managed-web',
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
      routePath: '/parent/browser-gap',
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
      routePath: '/parent/policy-action',
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
      routePath: '/parent/activity-store',
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
      routePath: '/parent/drive-exports',
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
      routePath: '/parent/managed-web',
    },
    {
      id: 'policy-action',
      name: 'POLICY ACTION',
      detail: 'Allow, ask, explain, block',
      icon: 'grid',
      tone: 'red',
      category: 'Policy',
      subcategory: 'Rules and approvals',
      gameType: 3,
      routePath: '/parent/policy-action',
    },
    {
      id: 'local-ai',
      name: 'LOCAL AI',
      detail: 'Private explanations',
      icon: 'bot',
      tone: 'purple',
      category: 'AI',
      subcategory: 'Evidence summaries',
      gameType: 6,
      routePath: '/parent/local-ai',
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
      routePath: '/parent/device-pairing',
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
      routePath: '/parent/support-exports',
    },
  ],
  fallbackRows: PARENT_LEADERBOARD_COPY_ROWS,
  aiBenchmarkRows: [
    {
      user_id: 'Local Explain',
      rank: 1,
      score: 9812,
      wins: 18,
      losses: 1,
      bestGame: 'Evidence summary',
      trend: 'Ready',
      tone: 'red',
    },
    {
      user_id: 'Cited Memory',
      rank: 2,
      score: 9381,
      wins: 16,
      losses: 2,
      bestGame: 'Memory links',
      trend: 'Local',
      tone: 'purple',
    },
    {
      user_id: 'Policy Preview',
      rank: 3,
      score: 9034,
      wins: 14,
      losses: 3,
      bestGame: 'Typed decisions',
      trend: 'Advisory',
      tone: 'gold',
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
    queueLabel: 'REQUEST',
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
      routeLabel: '/parent',
      rowSource: 'fallbackRows',
    },
    gameLeaderboard: {
      defaultTab: 'perGame',
      selectedGameId: 'managed-web',
      title: 'Control Area',
      routeLabel: '/parent/:area',
      rowSource: 'fallbackRows',
    },
    aiBenchmarkLeaderboard: {
      defaultTab: 'aiBenchmarks',
      selectedGameId: 'local-ai',
      title: 'Local AI',
      routeLabel: '/parent/local-ai',
      rowSource: 'aiBenchmarkRows',
    },
  },
};
