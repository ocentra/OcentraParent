import type { AgentEventEnvelope } from '@ocentra-parent/agent-protocol-domain/contracts';
import type { ParentPortalContent, ParentPortalRow } from './parent-portal-data';
import { parentPortalServiceRows } from './parent-portal-service-state-rows';

export const PARENT_PORTAL_SERVICE_STATE = {
  Empty: '',
  RowSource: {
    Api: 'api',
  },
  Connection: {
    Connected: 'connected',
    Connecting: 'connecting',
    Error: 'error',
  },
  Label: {
    LocalAgent: 'Local agent',
    LanDiscovery: 'LAN discovery',
    DevicePairing: 'Device pairing',
    BrowserActivity: 'Browser activity',
    BrowserInventory: 'Browser inventory',
    ExactUrlCapability: 'Exact URL capability',
    ActiveTabProof: 'Active tab proof',
    UnmanagedFallback: 'Unmanaged fallback',
    ActivityReports: 'Activity reports',
    NetworkTracking: 'Network tracking',
    HouseholdSetup: 'Household setup',
    ManagedWeb: 'Managed web path',
    BrowserSetup: 'Browser setup',
    ActivityStore: 'Activity store',
    AppGameSessions: 'App and game sessions',
    AppPolicy: 'App policy',
    GamePolicy: 'Game policy',
    ScreenAnalysis: 'Screen analysis',
    NetworkActivity: 'Network activity',
    TrackingPolicy: 'Tracking policy',
    RemoteScreenPolicy: 'Remote screen policy',
    SchedulePlan: 'Schedule plan',
    ApprovalQueue: 'Approval queue',
    EnforcementReadiness: 'Enforcement readiness',
    ReportsSurface: 'Reports surface',
    AssistantEntry: 'Assistant entry',
    DataCustody: 'Data custody',
    Alerts: 'Alerts',
    NotificationChannels: 'Notification channels',
    RemoteAccess: 'Remote access',
    ExportRetention: 'Export retention',
    AuditHistory: 'Audit history',
    Support: 'Support',
    Subscription: 'Subscription',
    Entitlements: 'Entitlements',
    MemorySetup: 'Memory setup',
    ApiProviders: 'API providers',
  },
  Area: {
    Service: 'Service',
    Runtime: 'Runtime',
    Lan: 'LAN',
    CurrentDevice: 'Current device',
    Browser: 'Browser',
    BrowserInventory: 'BROWSER INVENTORY',
    ExactUrlCapability: 'BROWSER EXACT URL CAPABILITY',
    ActiveTabProof: 'BROWSER ACTIVE TAB PROOF',
    UnmanagedFallback: 'BROWSER UNMANAGED FALLBACK',
    Activity: 'Activity',
    Network: 'Network',
    FamilySettings: 'Family Settings',
    Settings: 'SETTINGS',
    ManagedWeb: 'Managed Web',
    BrowserSetup: 'BROWSER SETUP',
    ActivityStore: 'Activity Store',
    AppGameSessions: 'APP AND GAME SESSIONS',
    AppPolicy: 'APP POLICY',
    GamePolicy: 'GAME POLICY',
    ScreenAnalysis: 'SCREEN ANALYSIS',
    NetworkActivity: 'NETWORK ACTIVITY',
    TrackingPolicy: 'TRACKING POLICY',
    RemoteScreenPolicy: 'REMOTE SCREEN POLICY',
    Schedules: 'SCHEDULES',
    Approvals: 'APPROVALS',
    Enforcement: 'ENFORCEMENT',
    Reports: 'REPORTS',
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
  },
  Trend: {
    NotReported: 'not-reported',
    Offline: 'offline',
    Reported: 'reported',
    Unavailable: 'unavailable',
    NotClaimed: 'not-claimed',
    ManualRequired: 'manual-required',
    PermissionRequired: 'permission-required',
    ScaffoldOnly: 'scaffold-only',
    BackendNotConnected: 'backend-not-connected',
  },
  Field: {
    LanScanSummary: 'scanSummary',
    ScannedDeviceCount: 'scannedDeviceCount',
  },
} as const;

export const SERVICE_BACKED_CONTENT: ParentPortalContent = {
  ...PARENT_PORTAL_CONTENT,
  uiCopy: {
    ...PARENT_PORTAL_CONTENT.uiCopy,
    detailSnapshotLines: [
      'Visible rows use real service events first, then honest manual-required or unavailable gaps.',
      'State labels stay explicit: paired, pending, observer-only, controller, degraded, or backend-not-connected.',
    ],
  },
  modes: {
    ...PARENT_PORTAL_CONTENT.modes,
    parentOverview: {
      ...PARENT_PORTAL_CONTENT.modes.parentOverview,
      rowSource: PARENT_PORTAL_SERVICE_STATE.RowSource.Api,
    },
    parentManage: {
      ...PARENT_PORTAL_CONTENT.modes.parentManage,
      rowSource: PARENT_PORTAL_SERVICE_STATE.RowSource.Api,
    },
  },
};

export type ParentPortalServiceConnectionState = 'connected' | 'connecting' | 'disconnected' | 'error';

export type ParentPortalServiceStateInput = {
  readonly connectionState: ParentPortalServiceConnectionState;
  readonly events: readonly AgentEventEnvelope[];
};

export type ParentPortalServiceState = {
  readonly content: ParentPortalContent;
  readonly parentPortalRows: ParentPortalRow[];
  readonly userEntry: ParentPortalRow | null;
};

export function resolveParentPortalServiceState(input: ParentPortalServiceStateInput): ParentPortalServiceState {
  const parentPortalRows = parentPortalServiceRows(input);

  return {
    content: SERVICE_BACKED_CONTENT,
    parentPortalRows,
    userEntry: parentPortalRows[0] ?? null,
  };
}
