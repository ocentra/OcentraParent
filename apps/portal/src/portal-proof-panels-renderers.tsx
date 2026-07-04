import { useState, type ReactElement } from 'react';
import { PortalDom } from '@ocentra-parent/portal-domain/contracts';
import { decodeDisplayText } from '@ocentra-parent/schema-domain/text-contracts';
import {
  type ParentAppGameAdapterDispatchPanelSnapshot,
  type ParentAppGameNotificationParentSurfacePanelSnapshot,
  type ParentAppGamePanelSnapshot,
  type ParentAppGameTimerParentSurfacePanelSnapshot,
  type ParentBrowserPanelSnapshot,
  type ParentNetworkEvidenceSummarySnapshot,
  type ParentPolicyPreviewPanelSnapshot,
} from '../generated/parent-ui-bridge';
import type { PortalRenderActions } from './portal-actions';
import type { PortalLiveActivityState } from './live-activity-state';
import { renderPortalProofPanelContent } from './portal-proof-panels-content';

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

export type PortalProofPanelsRoutePanelProps = {
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
  readonly onSelectPanel: (panel: PortalProofPanelId) => void;
};

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

export function renderPortalProofPanelsRoutePanel({
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
  activePanel,
  onSelectPanel,
}: PortalProofPanelsRoutePanelProps): ReactElement {
  return (
    <section aria-label="Proof panels" className={PortalDom.Classes.DeveloperRoutePanel}>
      <PortalProofPanelToolbar activePanel={activePanel} onSelect={onSelectPanel} />
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

export function usePortalProofPanelId(): [PortalProofPanelId, (panel: PortalProofPanelId) => void] {
  return useState<PortalProofPanelId>(DefaultProofPanel);
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
