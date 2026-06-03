import type { ParentPortalTone } from './parent-portal-data';
import { PARENT_PORTAL_SERVICE_STATE } from './parent-portal-service-state-constants';

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

export const PRODUCT_SHELL_ROW_SPECS: readonly ProductShellRowSpec[] = [
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

function rowSpec(
  labelKey: keyof typeof PARENT_PORTAL_SERVICE_STATE.Label,
  order: number,
  areaKey: keyof typeof PARENT_PORTAL_SERVICE_STATE.Area,
  tone: ParentPortalTone,
  signalKind: ProductShellSignalKind
): ProductShellRowSpec {
  return {
    label: PARENT_PORTAL_SERVICE_STATE.Label[labelKey],
    order,
    primaryArea: PARENT_PORTAL_SERVICE_STATE.Area[areaKey],
    tone,
    signalKind,
  };
}
