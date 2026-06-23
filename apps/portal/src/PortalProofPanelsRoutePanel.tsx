import { useState, type ReactElement } from 'react';
import type { PortalShellParentAccessState } from '@ocentra-parent/portal-domain/parent-portal-shell-status';
import { PortalDom } from '@ocentra-parent/portal-domain/contracts';
import { PortalRoute } from '@ocentra-parent/schema-domain/portal-contracts';
import { decodeDisplayText } from '@ocentra-parent/schema-domain/text-contracts';
import type { PortalRenderActions } from './portal-actions';
import type { PortalLiveActivityState } from './live-activity-state';
import { NetworkEvidenceDrawerRoutePanel } from './NetworkEvidenceDrawerRoutePanel';
import { PolicyPreviewRoutePanel } from './PolicyPreviewRoutePanel';
import { TrackingStatusRoutePanel } from './TrackingStatusRoutePanel';

const PortalProofPanel = {
  NetworkActivity: 'network-activity',
  PolicyPreview: 'policy-preview',
  TrackingStatus: 'tracking-status',
} as const;

type PortalProofPanelId = (typeof PortalProofPanel)[keyof typeof PortalProofPanel];

const DefaultProofPanel = PortalProofPanel.TrackingStatus;

export function PortalProofPanelsRoutePanel({
  actions,
  commandEnabled,
  liveActivity,
  parentAccessState,
}: {
  readonly actions: PortalRenderActions;
  readonly commandEnabled: boolean;
  readonly liveActivity: PortalLiveActivityState;
  readonly parentAccessState: PortalShellParentAccessState;
}): ReactElement {
  const [activePanel, setActivePanel] = useState<PortalProofPanelId>(DefaultProofPanel);

  return (
    <section aria-label="Proof panels" className={PortalDom.Classes.DeveloperRoutePanel}>
      <div className={PortalDom.Classes.DeveloperRouteToolbar}>
        <PortalProofPanelButton
          active={activePanel === PortalProofPanel.TrackingStatus}
          label={decodeDisplayText('Tracking status')}
          onClick={() => setActivePanel(PortalProofPanel.TrackingStatus)}
        />
        <PortalProofPanelButton
          active={activePanel === PortalProofPanel.NetworkActivity}
          label={decodeDisplayText('Network activity')}
          onClick={() => setActivePanel(PortalProofPanel.NetworkActivity)}
        />
        <PortalProofPanelButton
          active={activePanel === PortalProofPanel.PolicyPreview}
          label={decodeDisplayText('Policy decision')}
          onClick={() => setActivePanel(PortalProofPanel.PolicyPreview)}
        />
      </div>
      <div className={PortalDom.Classes.DeveloperRouteContent}>
        {renderPortalProofPanel({
          actions,
          activePanel,
          commandEnabled,
          liveActivity,
          parentAccessState,
        })}
      </div>
    </section>
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

function renderPortalProofPanel({
  actions,
  activePanel,
  commandEnabled,
  liveActivity,
  parentAccessState,
}: {
  readonly actions: PortalRenderActions;
  readonly activePanel: PortalProofPanelId;
  readonly commandEnabled: boolean;
  readonly liveActivity: PortalLiveActivityState;
  readonly parentAccessState: PortalShellParentAccessState;
}): ReactElement {
  switch (activePanel) {
    case PortalProofPanel.NetworkActivity:
      return <NetworkEvidenceDrawerRoutePanel liveActivity={liveActivity} route={PortalRoute.ProofPanels} />;
    case PortalProofPanel.PolicyPreview:
      return (
        <PolicyPreviewRoutePanel
          actions={actions}
          commandEnabled={commandEnabled}
          liveActivity={liveActivity}
          parentAccessState={parentAccessState}
        />
      );
    case PortalProofPanel.TrackingStatus:
    default:
      return <TrackingStatusRoutePanel actions={actions} commandEnabled={commandEnabled} liveActivity={liveActivity} />;
  }
}
