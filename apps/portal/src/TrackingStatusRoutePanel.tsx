import React, { type ReactElement } from 'react';
import { PortalDevTextToken, resolvePortalDevText } from '@ocentra-parent/portal-domain/display-text';
import { PortalDom } from '@ocentra-parent/portal-domain/contracts';
import {
  isParentTrackingStatusRoute,
  type ParentRouteId,
  type ParentTrackingStatusPanelSnapshot,
} from '../generated/parent-ui-bridge';
import type { PortalLiveActivityState } from './live-activity-state';
import type { PortalRenderActions } from './portal-actions';
import { renderTrackingStatusRoutePanelBody } from './tracking-status-route-panel-body';

export function shouldRenderTrackingStatusRoute(route: ParentRouteId): boolean {
  return isParentTrackingStatusRoute(route);
}

export function TrackingStatusRoutePanel({
  actions,
  commandEnabled,
  liveActivity,
}: {
  readonly actions: PortalRenderActions;
  readonly commandEnabled: boolean;
  readonly liveActivity: PortalLiveActivityState;
}): ReactElement | null {
  const panel = liveActivity.activityTrackingPanel as ParentTrackingStatusPanelSnapshot | null;
  if (panel == null) {
    return null;
  }
  return (
    <section
      aria-label={resolvePortalDevText(PortalDevTextToken.TrackingStatusSurface)}
      className={PortalDom.Classes.TrackingStatusOverlay}
    >
      <div className={PortalDom.Classes.TrackingStatusOverlayContent}>
        <header className={PortalDom.Classes.TrackingStatusOverlayHeader}>
          <p className={PortalDom.Classes.ProductEyebrow}>{panel.eyebrow}</p>
          <h2>{panel.title}</h2>
          <p>{panel.body}</p>
          <button
            className={PortalDom.Classes.CommandResultTab}
            disabled={!commandEnabled}
            type={PortalDom.ButtonType.Button}
            onClick={createTrackingStatusRefreshHandler(actions)}
          >
            {resolvePortalDevText(PortalDevTextToken.GetActivityTrackingReadModel)}
          </button>
        </header>
        {renderTrackingStatusRoutePanelBody(panel)}
      </div>
    </section>
  );
}

function createTrackingStatusRefreshHandler(actions: PortalRenderActions): () => void {
  return () => {
    void actions.refreshRouteSnapshot?.();
  };
}
