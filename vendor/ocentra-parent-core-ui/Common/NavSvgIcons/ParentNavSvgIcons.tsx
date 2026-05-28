export type NavSvgIconProps = {
  x?: number;
  y?: number;
  width?: number;
  height?: number;
  color?: string;
  strokeWidth?: number;
};

type IconProps = NavSvgIconProps;

export const parentNavIconAssetUrls = {
  QuickGlanceGlasses: '/parent-nav-quick-glance.svg',
  OverviewListIcon: '/parent-nav-overview-list.svg',
  StartDataAnalysisIcon: '/parent-nav-start-data-analysis.svg',
  GuideBookIcon: '/parent-nav-guide-book.svg',
  ManageFileSettingsIcon: '/parent-nav-manage-file-settings.svg',
  PolicyShieldDocumentIcon: '/parent-nav-policy-shield-document.svg',
  BrowserStackIcon: '/parent-nav-browser-stack.svg',
  WebGlobeIcon: '/parent-nav-web-globe.svg',
  ScheduleCalendarClockIcon: '/parent-nav-schedule-calendar-clock.svg',
  AlertNotificationBellIcon: '/parent-nav-alert-notification-bell.svg',
  ReportDocumentIcon: '/parent-nav-report-document.svg',
  RulesGavelDocumentIcon: '/parent-nav-rules-gavel-document.svg',
  UpdatesSyncDocumentIcon: '/parent-nav-updates-sync-document.svg',
  ActivityNetworkIcon: '/parent-nav-activity-network.svg',
  AppIcon: '/parent-nav-app.svg',
  PortalGatewayIcon: '/parent-nav-portal-gateway.svg',
  FamilyIcon: '/parent-nav-family.svg',
  GamesIcon: '/parent-nav-games.svg',
  DataPrivacyServerShieldIcon: '/parent-nav-data-privacy-server-shield.svg',
  LanNetworkMonitorsIcon: '/parent-nav-lan-network-monitors.svg',
  DevicesMultiScreenIcon: '/parent-nav-devices-multi-screen.svg',
  ScreenAnalysisIcon: '/parent-nav-screen-analysis.svg',
  RemoteAccessMonitorsIcon: '/parent-nav-remote-access-monitors.svg',
  AiSetupSearchIcon: '/parent-nav-ai-setup-search.svg',
  AiGuideIdeaIcon: '/parent-nav-ai-guide-idea.svg',
  AiMemorySetBrainIcon: '/parent-nav-ai-memory-set-brain.svg',
  ApiKeysChipIcon: '/parent-nav-api-keys-chip.svg',
  ExportRetentionIcon: '/parent-nav-export-retention.svg',
  DrivesCloudIcon: '/parent-nav-drives-cloud.svg',
  AuditCloudLogsIcon: '/parent-nav-audit-cloud-logs.svg',
  AiMemoryCircuitIcon: '/parent-nav-ai-memory-circuit.svg',
  AccountProfileIcon: '/parent-nav-account-profile.svg',
  EnforcementOfficerIcon: '/parent-nav-enforcement-officer.svg',
} as const;

function createParentNavAssetIcon(href: string) {
  return ({ x = 0, y = 0, width = 24, height = 24 }: IconProps) => (
    <image
      href={href}
      x={x}
      y={y}
      width={width}
      height={height}
      preserveAspectRatio="xMidYMid meet"
      pointerEvents="none"
    />
  );
}

export const QuickGlanceGlasses = createParentNavAssetIcon(parentNavIconAssetUrls.QuickGlanceGlasses);
export const OverviewListIcon = createParentNavAssetIcon(parentNavIconAssetUrls.OverviewListIcon);
export const StartDataAnalysisIcon = createParentNavAssetIcon(parentNavIconAssetUrls.StartDataAnalysisIcon);
export const GuideBookIcon = createParentNavAssetIcon(parentNavIconAssetUrls.GuideBookIcon);
export const ManageFileSettingsIcon = createParentNavAssetIcon(parentNavIconAssetUrls.ManageFileSettingsIcon);
export const PolicyShieldDocumentIcon = createParentNavAssetIcon(parentNavIconAssetUrls.PolicyShieldDocumentIcon);
export const BrowserStackIcon = createParentNavAssetIcon(parentNavIconAssetUrls.BrowserStackIcon);
export const WebGlobeIcon = createParentNavAssetIcon(parentNavIconAssetUrls.WebGlobeIcon);
export const ScheduleCalendarClockIcon = createParentNavAssetIcon(parentNavIconAssetUrls.ScheduleCalendarClockIcon);
export const AlertNotificationBellIcon = createParentNavAssetIcon(parentNavIconAssetUrls.AlertNotificationBellIcon);
export const ReportDocumentIcon = createParentNavAssetIcon(parentNavIconAssetUrls.ReportDocumentIcon);
export const RulesGavelDocumentIcon = createParentNavAssetIcon(parentNavIconAssetUrls.RulesGavelDocumentIcon);
export const UpdatesSyncDocumentIcon = createParentNavAssetIcon(parentNavIconAssetUrls.UpdatesSyncDocumentIcon);
export const ActivityNetworkIcon = createParentNavAssetIcon(parentNavIconAssetUrls.ActivityNetworkIcon);
export const AppIcon = createParentNavAssetIcon(parentNavIconAssetUrls.AppIcon);
export const PortalGatewayIcon = createParentNavAssetIcon(parentNavIconAssetUrls.PortalGatewayIcon);
export const FamilyIcon = createParentNavAssetIcon(parentNavIconAssetUrls.FamilyIcon);
export const GamesIcon = createParentNavAssetIcon(parentNavIconAssetUrls.GamesIcon);
export const DataPrivacyServerShieldIcon = createParentNavAssetIcon(parentNavIconAssetUrls.DataPrivacyServerShieldIcon);
export const LanNetworkMonitorsIcon = createParentNavAssetIcon(parentNavIconAssetUrls.LanNetworkMonitorsIcon);
export const DevicesMultiScreenIcon = createParentNavAssetIcon(parentNavIconAssetUrls.DevicesMultiScreenIcon);
export const ScreenAnalysisIcon = createParentNavAssetIcon(parentNavIconAssetUrls.ScreenAnalysisIcon);
export const RemoteAccessMonitorsIcon = createParentNavAssetIcon(parentNavIconAssetUrls.RemoteAccessMonitorsIcon);
export const AiSetupSearchIcon = createParentNavAssetIcon(parentNavIconAssetUrls.AiSetupSearchIcon);
export const AiGuideIdeaIcon = createParentNavAssetIcon(parentNavIconAssetUrls.AiGuideIdeaIcon);
export const AiMemorySetBrainIcon = createParentNavAssetIcon(parentNavIconAssetUrls.AiMemorySetBrainIcon);
export const ApiKeysChipIcon = createParentNavAssetIcon(parentNavIconAssetUrls.ApiKeysChipIcon);
export const ExportRetentionIcon = createParentNavAssetIcon(parentNavIconAssetUrls.ExportRetentionIcon);
export const DrivesCloudIcon = createParentNavAssetIcon(parentNavIconAssetUrls.DrivesCloudIcon);
export const AuditCloudLogsIcon = createParentNavAssetIcon(parentNavIconAssetUrls.AuditCloudLogsIcon);
export const AiMemoryCircuitIcon = createParentNavAssetIcon(parentNavIconAssetUrls.AiMemoryCircuitIcon);
export const AccountProfileIcon = createParentNavAssetIcon(parentNavIconAssetUrls.AccountProfileIcon);
export const EnforcementOfficerIcon = createParentNavAssetIcon(parentNavIconAssetUrls.EnforcementOfficerIcon);
