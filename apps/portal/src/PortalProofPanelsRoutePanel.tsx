import { useState, type ReactElement } from 'react';
import { PortalDom } from '@ocentra-parent/portal-domain/contracts';
import { decodeDisplayText } from '@ocentra-parent/schema-domain/text-contracts';
import {
  type ParentAppGameAdapterDispatchPanelSnapshot,
  ParentRoute,
  type ParentAppGameNotificationParentSurfacePanelSnapshot,
  type ParentAppGamePanelSnapshot,
  type ParentAppGameTimerParentSurfacePanelSnapshot,
  type ParentBrowserPanelSnapshot,
  type ParentNetworkEvidenceSummarySnapshot,
  type ParentPolicyPreviewPanelSnapshot,
} from '../generated/parent-ui-bridge';
import { AppGameAdapterDispatchRoutePanel } from './AppGameAdapterDispatchRoutePanel';
import { BrowserParentExplanationRoutePanel } from './BrowserParentExplanationRoutePanel';
import type { PortalRenderActions } from './portal-actions';
import { AppGameChildRuntimeTransportReceiptRoutePanel } from './AppGameChildRuntimeTransportReceiptRoutePanel';
import { AppGameNotificationParentSurfaceRoutePanel } from './AppGameNotificationParentSurfaceRoutePanel';
import { AppGamePlatformProofStatusRoutePanel } from './AppGamePlatformProofStatusRoutePanel';
import { AppGamePolicyReadinessRoutePanel } from './AppGamePolicyReadinessRoutePanel';
import { AppGameTimerParentSurfaceRoutePanel } from './AppGameTimerParentSurfaceRoutePanel';
import type { PortalLiveActivityState } from './live-activity-state';
import { NetworkEvidenceDrawerRoutePanel } from './NetworkEvidenceDrawerRoutePanel';
import { PolicyPreviewRoutePanel } from './PolicyPreviewRoutePanel';
import { SocialAlertReportRoutePanel } from './SocialAlertReportRoutePanel';
import { SocialAuditExplanationRoutePanel } from './SocialAuditExplanationRoutePanel';
import { SocialDashboardRoutePanel } from './SocialDashboardRoutePanel';
import { TrackingStatusRoutePanel } from './TrackingStatusRoutePanel';

const PortalProofPanel = {
  AppGameAdapterDispatch: 'app-game-adapter-dispatch',
  AppGameChildRuntimeTransportReceipt: 'app-game-child-runtime-transport-receipt',
  AppGameNotificationParentSurface: 'app-game-notification-parent-surface',
  AppGamePlatformProofStatus: 'app-game-platform-proof-status',
  AppGamePolicyReadiness: 'app-game-policy-readiness',
  AppGameTimerParentSurface: 'app-game-timer-parent-surface',
  BrowserParentExplanation: 'browser-parent-explanation',
  NetworkActivity: 'network-activity',
  PolicyPreview: 'policy-preview',
  SocialAlertReport: 'social-alert-report',
  SocialAuditExplanation: 'social-audit-explanation',
  SocialDashboard: 'social-dashboard',
  TrackingStatus: 'tracking-status',
} as const;

type PortalProofPanelId = (typeof PortalProofPanel)[keyof typeof PortalProofPanel];

type PortalProofPanelButtonDefinition = {
  readonly panel: PortalProofPanelId;
  readonly label: string;
};

type PortalProofPanelContentProps = {
  readonly actions: PortalRenderActions;
  readonly activePanel: PortalProofPanelId;
  readonly commandEnabled: boolean;
  readonly liveActivity: PortalLiveActivityState;
  readonly networkEvidenceSummary: ParentNetworkEvidenceSummarySnapshot | null;
  readonly policyPreviewPanel: ParentPolicyPreviewPanelSnapshot | null;
  readonly appGameNotificationParentSurfacePanel: ParentAppGameNotificationParentSurfacePanelSnapshot | null;
  readonly appGamePolicyReadinessPanel: ParentAppGamePanelSnapshot | null;
  readonly appGamePlatformProofStatusPanel: ParentAppGamePanelSnapshot | null;
  readonly appGameChildRuntimeTransportReceiptPanel: ParentAppGamePanelSnapshot | null;
  readonly appGameAdapterDispatchPanel: ParentAppGameAdapterDispatchPanelSnapshot | null;
  readonly appGameTimerParentSurfacePanel: ParentAppGameTimerParentSurfacePanelSnapshot | null;
  readonly browserParentExplanationPanel: ParentBrowserPanelSnapshot | null;
  readonly socialAuditExplanationPanel: ParentBrowserPanelSnapshot | null;
  readonly socialDashboardPanel: ParentBrowserPanelSnapshot | null;
  readonly socialAlertReportPanel: ParentBrowserPanelSnapshot | null;
  readonly socialAlertReportParentSurfacePanel: ParentBrowserPanelSnapshot | null;
  readonly socialParentNotificationDeliveryPanel: ParentBrowserPanelSnapshot | null;
  readonly browserActionIntentStreamStatusPanel: ParentBrowserPanelSnapshot | null;
  readonly browserSocialProviderReceiptStreamStatusPanel: ParentBrowserPanelSnapshot | null;
  readonly browserSocialProviderReceiptIngestionReadinessStatusPanel: ParentBrowserPanelSnapshot | null;
};

type PortalProofPanelAppGameProps = {
  readonly actions: PortalRenderActions;
  readonly activePanel: PortalProofPanelId;
  readonly commandEnabled: boolean;
  readonly appGameNotificationParentSurfacePanel: ParentAppGameNotificationParentSurfacePanelSnapshot | null;
  readonly appGamePolicyReadinessPanel: ParentAppGamePanelSnapshot | null;
  readonly appGamePlatformProofStatusPanel: ParentAppGamePanelSnapshot | null;
  readonly appGameChildRuntimeTransportReceiptPanel: ParentAppGamePanelSnapshot | null;
  readonly appGameAdapterDispatchPanel: ParentAppGameAdapterDispatchPanelSnapshot | null;
  readonly appGameTimerParentSurfacePanel: ParentAppGameTimerParentSurfacePanelSnapshot | null;
};

type PortalProofPanelSocialProps = Pick<
  PortalProofPanelContentProps,
  | 'actions'
  | 'activePanel'
  | 'commandEnabled'
  | 'browserActionIntentStreamStatusPanel'
  | 'browserParentExplanationPanel'
  | 'browserSocialProviderReceiptIngestionReadinessStatusPanel'
  | 'browserSocialProviderReceiptStreamStatusPanel'
  | 'socialAlertReportPanel'
  | 'socialAlertReportParentSurfacePanel'
  | 'socialAuditExplanationPanel'
  | 'socialDashboardPanel'
  | 'socialParentNotificationDeliveryPanel'
>;

const PORTAL_PROOF_PANEL_BUTTONS: ReadonlyArray<PortalProofPanelButtonDefinition> = [
  { panel: PortalProofPanel.TrackingStatus, label: decodeDisplayText('Tracking status') },
  { panel: PortalProofPanel.NetworkActivity, label: decodeDisplayText('Network activity') },
  { panel: PortalProofPanel.PolicyPreview, label: decodeDisplayText('Policy decision') },
  { panel: PortalProofPanel.BrowserParentExplanation, label: decodeDisplayText('Browser explanation') },
  { panel: PortalProofPanel.SocialAuditExplanation, label: decodeDisplayText('Social explanation') },
  { panel: PortalProofPanel.SocialDashboard, label: decodeDisplayText('Social dashboard') },
  { panel: PortalProofPanel.SocialAlertReport, label: decodeDisplayText('Social alerts') },
  {
    panel: PortalProofPanel.AppGameNotificationParentSurface,
    label: decodeDisplayText('App/game notifications'),
  },
  { panel: PortalProofPanel.AppGamePolicyReadiness, label: decodeDisplayText('App/game policy') },
  { panel: PortalProofPanel.AppGamePlatformProofStatus, label: decodeDisplayText('App/game platform') },
  {
    panel: PortalProofPanel.AppGameChildRuntimeTransportReceipt,
    label: decodeDisplayText('App/game child runtime'),
  },
  { panel: PortalProofPanel.AppGameAdapterDispatch, label: decodeDisplayText('App/game adapter dispatch') },
  {
    panel: PortalProofPanel.AppGameTimerParentSurface,
    label: decodeDisplayText('App/game timer parent surface'),
  },
];

const DefaultProofPanel = PortalProofPanel.TrackingStatus;

export function PortalProofPanelsRoutePanel({
  actions,
  browserActionIntentStreamStatusPanel,
  browserParentExplanationPanel,
  browserSocialProviderReceiptIngestionReadinessStatusPanel,
  browserSocialProviderReceiptStreamStatusPanel,
  commandEnabled,
  liveActivity,
  networkEvidenceSummary,
  policyPreviewPanel,
  appGameNotificationParentSurfacePanel,
  appGamePolicyReadinessPanel,
  appGamePlatformProofStatusPanel,
  appGameChildRuntimeTransportReceiptPanel,
  appGameAdapterDispatchPanel,
  appGameTimerParentSurfacePanel,
  socialAlertReportPanel,
  socialAlertReportParentSurfacePanel,
  socialAuditExplanationPanel,
  socialDashboardPanel,
  socialParentNotificationDeliveryPanel,
}: {
  readonly actions: PortalRenderActions;
  readonly commandEnabled: boolean;
  readonly liveActivity: PortalLiveActivityState;
  readonly networkEvidenceSummary: ParentNetworkEvidenceSummarySnapshot | null;
  readonly policyPreviewPanel: ParentPolicyPreviewPanelSnapshot | null;
  readonly appGameNotificationParentSurfacePanel: ParentAppGameNotificationParentSurfacePanelSnapshot | null;
  readonly appGamePolicyReadinessPanel: ParentAppGamePanelSnapshot | null;
  readonly appGamePlatformProofStatusPanel: ParentAppGamePanelSnapshot | null;
  readonly appGameChildRuntimeTransportReceiptPanel: ParentAppGamePanelSnapshot | null;
  readonly appGameAdapterDispatchPanel: ParentAppGameAdapterDispatchPanelSnapshot | null;
  readonly appGameTimerParentSurfacePanel: ParentAppGameTimerParentSurfacePanelSnapshot | null;
  readonly browserParentExplanationPanel: ParentBrowserPanelSnapshot | null;
  readonly socialAuditExplanationPanel: ParentBrowserPanelSnapshot | null;
  readonly socialDashboardPanel: ParentBrowserPanelSnapshot | null;
  readonly socialAlertReportPanel: ParentBrowserPanelSnapshot | null;
  readonly socialAlertReportParentSurfacePanel: ParentBrowserPanelSnapshot | null;
  readonly socialParentNotificationDeliveryPanel: ParentBrowserPanelSnapshot | null;
  readonly browserActionIntentStreamStatusPanel: ParentBrowserPanelSnapshot | null;
  readonly browserSocialProviderReceiptStreamStatusPanel: ParentBrowserPanelSnapshot | null;
  readonly browserSocialProviderReceiptIngestionReadinessStatusPanel: ParentBrowserPanelSnapshot | null;
}): ReactElement {
  const [activePanel, setActivePanel] = useState<PortalProofPanelId>(DefaultProofPanel);

  return (
    <section aria-label="Proof panels" className={PortalDom.Classes.DeveloperRoutePanel}>
      <PortalProofPanelToolbar activePanel={activePanel} onSelect={setActivePanel} />
      <div className={PortalDom.Classes.DeveloperRouteContent}>
        {renderPortalProofPanelContent({
          actions,
          activePanel,
          commandEnabled,
          liveActivity,
          networkEvidenceSummary,
          policyPreviewPanel,
          appGameNotificationParentSurfacePanel,
          appGamePolicyReadinessPanel,
          appGamePlatformProofStatusPanel,
          appGameChildRuntimeTransportReceiptPanel,
          appGameAdapterDispatchPanel,
          appGameTimerParentSurfacePanel,
          browserParentExplanationPanel,
          socialAuditExplanationPanel,
          socialDashboardPanel,
          socialAlertReportPanel,
          socialAlertReportParentSurfacePanel,
          socialParentNotificationDeliveryPanel,
          browserActionIntentStreamStatusPanel,
          browserSocialProviderReceiptStreamStatusPanel,
          browserSocialProviderReceiptIngestionReadinessStatusPanel,
        })}
      </div>
    </section>
  );
}

function PortalProofPanelToolbar({
  activePanel,
  onSelect,
}: {
  readonly activePanel: PortalProofPanelId;
  readonly onSelect: (panel: PortalProofPanelId) => void;
}): ReactElement {
  return (
    <div className={PortalDom.Classes.DeveloperRouteToolbar}>
      {PORTAL_PROOF_PANEL_BUTTONS.map(({ panel, label }) => (
        <PortalProofPanelButton
          key={panel}
          active={activePanel === panel}
          label={label}
          onClick={() => onSelect(panel)}
        />
      ))}
    </div>
  );
}

function PortalProofPanelButton({
  active,
  label,
  onClick,
}: {
  readonly active: boolean;
  readonly label: string;
  readonly onClick: () => void;
}): ReactElement {
  return (
    <button
      aria-pressed={active}
      className={PortalDom.Classes.CommandResultTab}
      data-active={active ? PortalDom.Attributes.True : undefined}
      onClick={onClick}
      type={PortalDom.ButtonType.Button}
    >
      {label}
    </button>
  );
}

function renderPortalProofPanelContent({
  actions,
  activePanel,
  browserActionIntentStreamStatusPanel,
  browserParentExplanationPanel,
  browserSocialProviderReceiptIngestionReadinessStatusPanel,
  browserSocialProviderReceiptStreamStatusPanel,
  commandEnabled,
  liveActivity,
  networkEvidenceSummary,
  policyPreviewPanel,
  appGameNotificationParentSurfacePanel,
  appGamePolicyReadinessPanel,
  appGamePlatformProofStatusPanel,
  appGameChildRuntimeTransportReceiptPanel,
  appGameAdapterDispatchPanel,
  appGameTimerParentSurfacePanel,
  socialAlertReportPanel,
  socialAlertReportParentSurfacePanel,
  socialAuditExplanationPanel,
  socialDashboardPanel,
  socialParentNotificationDeliveryPanel,
}: PortalProofPanelContentProps): ReactElement {
  switch (activePanel) {
    case PortalProofPanel.BrowserParentExplanation:
    case PortalProofPanel.SocialAuditExplanation:
    case PortalProofPanel.SocialDashboard:
    case PortalProofPanel.SocialAlertReport:
      return renderPortalProofPanelSocial({
        actions,
        activePanel,
        commandEnabled,
        browserActionIntentStreamStatusPanel,
        browserParentExplanationPanel,
        browserSocialProviderReceiptIngestionReadinessStatusPanel,
        browserSocialProviderReceiptStreamStatusPanel,
        socialAlertReportPanel,
        socialAlertReportParentSurfacePanel,
        socialAuditExplanationPanel,
        socialDashboardPanel,
        socialParentNotificationDeliveryPanel,
      });
    case PortalProofPanel.AppGameNotificationParentSurface:
    case PortalProofPanel.AppGamePolicyReadiness:
    case PortalProofPanel.AppGamePlatformProofStatus:
    case PortalProofPanel.AppGameChildRuntimeTransportReceipt:
    case PortalProofPanel.AppGameAdapterDispatch:
    case PortalProofPanel.AppGameTimerParentSurface:
      return renderAppGameProofPanel({
        actions,
        activePanel,
        commandEnabled,
        appGameNotificationParentSurfacePanel,
        appGamePolicyReadinessPanel,
        appGamePlatformProofStatusPanel,
        appGameChildRuntimeTransportReceiptPanel,
        appGameAdapterDispatchPanel,
        appGameTimerParentSurfacePanel,
      });
    case PortalProofPanel.NetworkActivity:
      return (
        <NetworkEvidenceDrawerRoutePanel
          liveActivity={liveActivity}
          networkEvidenceSummary={networkEvidenceSummary}
          route={ParentRoute.ProofPanels}
        />
      );
    case PortalProofPanel.PolicyPreview:
      return <PolicyPreviewRoutePanel actions={actions} commandEnabled={commandEnabled} panel={policyPreviewPanel} />;
    case PortalProofPanel.TrackingStatus:
    default:
      return <TrackingStatusRoutePanel actions={actions} commandEnabled={commandEnabled} liveActivity={liveActivity} />;
  }
}

function renderPortalProofPanelSocial({
  actions,
  activePanel,
  commandEnabled,
  browserActionIntentStreamStatusPanel,
  browserParentExplanationPanel,
  browserSocialProviderReceiptIngestionReadinessStatusPanel,
  browserSocialProviderReceiptStreamStatusPanel,
  socialAlertReportPanel,
  socialAlertReportParentSurfacePanel,
  socialAuditExplanationPanel,
  socialDashboardPanel,
  socialParentNotificationDeliveryPanel,
}: PortalProofPanelSocialProps): ReactElement {
  switch (activePanel) {
    case PortalProofPanel.BrowserParentExplanation:
      return <BrowserParentExplanationRoutePanel panel={browserParentExplanationPanel} />;
    case PortalProofPanel.SocialAuditExplanation:
      return (
        <SocialAuditExplanationRoutePanel
          actions={actions}
          commandEnabled={commandEnabled}
          panel={socialAuditExplanationPanel}
        />
      );
    case PortalProofPanel.SocialDashboard:
      return <SocialDashboardRoutePanel actions={actions} commandEnabled={commandEnabled} panel={socialDashboardPanel} />;
    case PortalProofPanel.SocialAlertReport:
    default:
      return (
        <SocialAlertReportRoutePanel
          actions={actions}
          commandEnabled={commandEnabled}
          socialAlertReportPanel={socialAlertReportPanel}
          socialAlertReportParentSurfacePanel={socialAlertReportParentSurfacePanel}
          socialParentNotificationDeliveryPanel={socialParentNotificationDeliveryPanel}
          browserActionIntentStreamStatusPanel={browserActionIntentStreamStatusPanel}
          browserSocialProviderReceiptStreamStatusPanel={browserSocialProviderReceiptStreamStatusPanel}
          browserSocialProviderReceiptIngestionReadinessStatusPanel={
            browserSocialProviderReceiptIngestionReadinessStatusPanel
          }
        />
      );
  }
}

function renderAppGameProofPanel({
  actions,
  activePanel,
  commandEnabled,
  appGameNotificationParentSurfacePanel,
  appGamePolicyReadinessPanel,
  appGamePlatformProofStatusPanel,
  appGameChildRuntimeTransportReceiptPanel,
  appGameAdapterDispatchPanel,
  appGameTimerParentSurfacePanel,
}: PortalProofPanelAppGameProps): ReactElement {
  switch (activePanel) {
    case PortalProofPanel.AppGameNotificationParentSurface:
      return <AppGameNotificationParentSurfaceRoutePanel panel={appGameNotificationParentSurfacePanel} />;
    case PortalProofPanel.AppGamePolicyReadiness:
    default:
      return renderAppGameProofPanelDetails({
        actions,
        activePanel,
        commandEnabled,
        appGamePolicyReadinessPanel,
        appGamePlatformProofStatusPanel,
        appGameChildRuntimeTransportReceiptPanel,
        appGameAdapterDispatchPanel,
        appGameTimerParentSurfacePanel,
      });
  }
}

function renderAppGameProofPanelDetails({
  actions,
  activePanel,
  commandEnabled,
  appGamePolicyReadinessPanel,
  appGamePlatformProofStatusPanel,
  appGameChildRuntimeTransportReceiptPanel,
  appGameAdapterDispatchPanel,
  appGameTimerParentSurfacePanel,
}: Omit<PortalProofPanelAppGameProps, 'appGameNotificationParentSurfacePanel'>): ReactElement {
  switch (activePanel) {
    case PortalProofPanel.AppGamePolicyReadiness:
    case PortalProofPanel.AppGamePlatformProofStatus:
      return renderAppGameProofPanelCore({
        actions,
        activePanel,
        commandEnabled,
        appGamePolicyReadinessPanel,
        appGamePlatformProofStatusPanel,
      });
    case PortalProofPanel.AppGameChildRuntimeTransportReceipt:
    case PortalProofPanel.AppGameAdapterDispatch:
    case PortalProofPanel.AppGameTimerParentSurface:
    default:
      return renderAppGameProofPanelTerminal({
        actions,
        activePanel,
        commandEnabled,
        appGameChildRuntimeTransportReceiptPanel,
        appGameAdapterDispatchPanel,
        appGameTimerParentSurfacePanel,
      });
  }
}

function renderAppGameProofPanelCore({
  actions,
  activePanel,
  commandEnabled,
  appGamePolicyReadinessPanel,
  appGamePlatformProofStatusPanel,
}: Pick<
  Omit<PortalProofPanelAppGameProps, 'appGameNotificationParentSurfacePanel' | 'appGameChildRuntimeTransportReceiptPanel' | 'appGameAdapterDispatchPanel' | 'appGameTimerParentSurfacePanel'>,
  'actions' | 'activePanel' | 'commandEnabled' | 'appGamePolicyReadinessPanel' | 'appGamePlatformProofStatusPanel'
>): ReactElement {
  switch (activePanel) {
    case PortalProofPanel.AppGamePolicyReadiness:
      return <AppGamePolicyReadinessRoutePanel actions={actions} commandEnabled={commandEnabled} panel={appGamePolicyReadinessPanel} />;
    case PortalProofPanel.AppGamePlatformProofStatus:
    default:
      return <AppGamePlatformProofStatusRoutePanel actions={actions} commandEnabled={commandEnabled} panel={appGamePlatformProofStatusPanel} />;
  }
}

function renderAppGameProofPanelTerminal({
  actions,
  activePanel,
  commandEnabled,
  appGameChildRuntimeTransportReceiptPanel,
  appGameAdapterDispatchPanel,
  appGameTimerParentSurfacePanel,
}: Pick<
  Omit<PortalProofPanelAppGameProps, 'appGameNotificationParentSurfacePanel' | 'appGamePolicyReadinessPanel' | 'appGamePlatformProofStatusPanel'>,
  'actions' | 'activePanel' | 'commandEnabled' | 'appGameChildRuntimeTransportReceiptPanel' | 'appGameAdapterDispatchPanel' | 'appGameTimerParentSurfacePanel'
>): ReactElement {
  switch (activePanel) {
    case PortalProofPanel.AppGameChildRuntimeTransportReceipt:
      return <AppGameChildRuntimeTransportReceiptRoutePanel actions={actions} commandEnabled={commandEnabled} panel={appGameChildRuntimeTransportReceiptPanel} />;
    case PortalProofPanel.AppGameAdapterDispatch:
      return <AppGameAdapterDispatchRoutePanel actions={actions} commandEnabled={commandEnabled} panel={appGameAdapterDispatchPanel} />;
    case PortalProofPanel.AppGameTimerParentSurface:
    default:
      return <AppGameTimerParentSurfaceRoutePanel actions={actions} commandEnabled={commandEnabled} panel={appGameTimerParentSurfacePanel} />;
  }
}
