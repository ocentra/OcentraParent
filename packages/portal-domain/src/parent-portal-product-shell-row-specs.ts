import type { ParentPortalTone } from './parent-portal-data';

const PRODUCT_SHELL_LABELS = {
  HouseholdSetup: 'Household setup',
  BrowserInventory: 'Browser inventory',
  ExactUrlCapability: 'Exact URL capability',
  ActiveTabProof: 'Active tab proof',
  UnmanagedFallback: 'Unmanaged fallback',
  ManagedWeb: 'Managed web path',
  BrowserSetup: 'Browser setup',
  ActivityStore: 'Activity store',
  AppGameSessions: 'App and game sessions',
  ReportsSurface: 'Reports surface',
  AppPolicy: 'App policy',
  GamePolicy: 'Game policy',
  ScreenAnalysis: 'Screen analysis',
  NetworkActivity: 'Network activity',
  TrackingPolicy: 'Tracking policy',
  RemoteScreenPolicy: 'Remote screen policy',
  SchedulePlan: 'Schedule plan',
  ApprovalQueue: 'Approval queue',
  EnforcementReadiness: 'Enforcement readiness',
  AssistantEntry: 'Assistant entry',
  ApiProviders: 'API providers',
  MemorySetup: 'Memory setup',
  DataCustody: 'Data custody',
  ExportRetention: 'Export retention',
  Alerts: 'Alerts',
  NotificationChannels: 'Notification channels',
  RemoteAccess: 'Remote access',
  AuditHistory: 'Audit history',
  Support: 'Support',
  Subscription: 'Subscription',
  Entitlements: 'Entitlements',
  DevicePairing: 'Device pairing',
  LanDiscovery: 'LAN discovery',
} as const;

const PRODUCT_SHELL_PRIMARY_AREAS = {
  FamilySettings: 'Family Settings',
  Settings: 'SETTINGS',
  ManagedWeb: 'Managed Web',
  BrowserSetup: 'BROWSER SETUP',
  ActivityStore: 'Activity Store',
  AppGameSessions: 'APP AND GAME SESSIONS',
  Reports: 'REPORTS',
  AppPolicy: 'APP POLICY',
  GamePolicy: 'GAME POLICY',
  ScreenAnalysis: 'SCREEN ANALYSIS',
  NetworkActivity: 'NETWORK ACTIVITY',
  TrackingPolicy: 'TRACKING POLICY',
  RemoteScreenPolicy: 'REMOTE SCREEN POLICY',
  Schedules: 'SCHEDULES',
  Approvals: 'APPROVALS',
  Enforcement: 'ENFORCEMENT',
  AiSetup: 'AI SETUP',
  ApiProviders: 'API Providers',
  ApiKeys: 'API KEYS',
  MemorySetup: 'MEMORY SETUP',
  DriveExports: 'Drive Exports',
  Drives: 'DRIVES',
  Alerts: 'ALERTS',
  NotificationChannels: 'NOTIFICATION CHANNELS',
  RemoteAccess: 'REMOTE ACCESS',
  ExportRetention: 'EXPORT DELETE RETENTION',
  AuditHistory: 'AUDIT HISTORY',
  Support: 'SUPPORT',
  Subscription: 'SUBSCRIPTION',
  Entitlements: 'ENTITLEMENTS',
  Device: 'DEVICE',
  LanPairing: 'LAN PAIRING',
  CapabilityStatus: 'CAPABILITY STATUS',
} as const;

export type ProductShellSignalKind =
  | 'household'
  | 'browser'
  | 'browserInventory'
  | 'browserExactUrl'
  | 'browserActiveTab'
  | 'browserUnmanagedFallback'
  | 'activity'
  | 'network'
  | 'policy'
  | 'remotePolicy'
  | 'assistant'
  | 'manual';

export type ProductShellRowSpec = {
  readonly label: string;
  readonly order: number;
  readonly primaryArea: string;
  readonly tone: ParentPortalTone;
  readonly signalKind: ProductShellSignalKind;
};

type ProductShellRowSpecDefinition = {
  readonly labelKey: keyof typeof PRODUCT_SHELL_LABELS;
  readonly order: number;
  readonly areaKey: keyof typeof PRODUCT_SHELL_PRIMARY_AREAS;
  readonly tone: ParentPortalTone;
  readonly signalKind: ProductShellSignalKind;
};

const PRODUCT_SHELL_ROW_SPEC_DEFINITIONS: readonly ProductShellRowSpecDefinition[] = [
  rowSpec('HouseholdSetup', 7, 'FamilySettings', 'cyan', 'household'),
  rowSpec('HouseholdSetup', 7.5, 'Settings', 'cyan', 'household'),
  rowSpec('BrowserInventory', 7.6, 'ManagedWeb', 'gold', 'browserInventory'),
  rowSpec('ExactUrlCapability', 7.7, 'ManagedWeb', 'gold', 'browserExactUrl'),
  rowSpec('ActiveTabProof', 7.8, 'ManagedWeb', 'gold', 'browserActiveTab'),
  rowSpec('UnmanagedFallback', 7.9, 'ManagedWeb', 'cyan', 'browserUnmanagedFallback'),
  rowSpec('ManagedWeb', 8, 'ManagedWeb', 'gold', 'browser'),
  rowSpec('BrowserSetup', 9, 'BrowserSetup', 'gold', 'browser'),
  rowSpec('ActivityStore', 10, 'ActivityStore', 'purple', 'activity'),
  rowSpec('AppGameSessions', 11, 'AppGameSessions', 'purple', 'activity'),
  rowSpec('ReportsSurface', 12, 'Reports', 'purple', 'activity'),
  rowSpec('AppPolicy', 13, 'AppPolicy', 'gold', 'policy'),
  rowSpec('GamePolicy', 14, 'GamePolicy', 'purple', 'policy'),
  rowSpec('ScreenAnalysis', 15, 'ScreenAnalysis', 'cyan', 'activity'),
  rowSpec('NetworkActivity', 16, 'NetworkActivity', 'cyan', 'network'),
  rowSpec('TrackingPolicy', 17, 'TrackingPolicy', 'cyan', 'policy'),
  rowSpec('RemoteScreenPolicy', 18, 'RemoteScreenPolicy', 'purple', 'remotePolicy'),
  rowSpec('SchedulePlan', 19, 'Schedules', 'purple', 'policy'),
  rowSpec('ApprovalQueue', 20, 'Approvals', 'cyan', 'policy'),
  rowSpec('EnforcementReadiness', 21, 'Enforcement', 'red', 'policy'),
  rowSpec('AssistantEntry', 22, 'AiSetup', 'purple', 'assistant'),
  rowSpec('ApiProviders', 23, 'ApiProviders', 'purple', 'assistant'),
  rowSpec('ApiProviders', 23.5, 'ApiKeys', 'purple', 'assistant'),
  rowSpec('MemorySetup', 24, 'MemorySetup', 'purple', 'manual'),
  rowSpec('DataCustody', 25, 'DriveExports', 'gold', 'manual'),
  rowSpec('DataCustody', 25.5, 'Drives', 'gold', 'manual'),
  rowSpec('ExportRetention', 26, 'ExportRetention', 'gold', 'manual'),
  rowSpec('Alerts', 27, 'Alerts', 'red', 'manual'),
  rowSpec('NotificationChannels', 28, 'NotificationChannels', 'red', 'manual'),
  rowSpec('RemoteAccess', 29, 'RemoteAccess', 'cyan', 'manual'),
  rowSpec('AuditHistory', 30, 'AuditHistory', 'purple', 'manual'),
  rowSpec('Support', 31, 'Support', 'cyan', 'manual'),
  rowSpec('Subscription', 32, 'Subscription', 'gold', 'manual'),
  rowSpec('Entitlements', 33, 'Entitlements', 'gold', 'manual'),
  rowSpec('DevicePairing', 34, 'Device', 'cyan', 'household'),
  rowSpec('LanDiscovery', 35, 'LanPairing', 'cyan', 'household'),
  rowSpec('HouseholdSetup', 36, 'CapabilityStatus', 'cyan', 'household'),
];

export function productShellRowSpecs(): readonly ProductShellRowSpec[] {
  return PRODUCT_SHELL_ROW_SPEC_DEFINITIONS.map((definition) => ({
    label: PRODUCT_SHELL_LABELS[definition.labelKey],
    order: definition.order,
    primaryArea: PRODUCT_SHELL_PRIMARY_AREAS[definition.areaKey],
    tone: definition.tone,
    signalKind: definition.signalKind,
  }));
}

function rowSpec(
  labelKey: keyof typeof PRODUCT_SHELL_LABELS,
  order: number,
  areaKey: keyof typeof PRODUCT_SHELL_PRIMARY_AREAS,
  tone: ParentPortalTone,
  signalKind: ProductShellSignalKind
): ProductShellRowSpecDefinition {
  return {
    labelKey,
    order,
    areaKey,
    tone,
    signalKind,
  };
}
