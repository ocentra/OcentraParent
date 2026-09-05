import React, { type ReactElement } from 'react';
import {
  decodeDisplayText,
  PortalDevTextToken,
  resolvePortalDevText,
} from '@ocentra-parent/portal-domain/display-text';
import { PortalDom } from '@ocentra-parent/portal-domain/contracts';
import {
  decodeParentTrackingStatusPanelSnapshot,
  isParentTrackingStatusRoute,
  type ParentRouteId,
  type ParentTrackingStatusPanelSnapshot,
} from '../generated/parent-ui-bridge';
import type { PortalLiveActivityState } from './live-activity-state';
import type { PortalRenderActions } from './portal-actions';
import { renderTrackingStatusRoutePanelBody } from './tracking-status-route-panel-body';

const TRACKING_STATUS_UNAVAILABLE_TEXT = {
  body: decodeDisplayText(
    'Tracking is not connected to the local service. No child location or activity is being shown.'
  ),
  cardBody: decodeDisplayText(
    'Retry status to load the Rust-owned tracking read model, including device, custody, freshness, and evidence rows.'
  ),
  label: decodeDisplayText('Tracking status'),
  title: decodeDisplayText('Tracking status unavailable'),
} as const;

const TRACKING_STATUS_INVALID_TEXT = {
  body: decodeDisplayText(
    'The local service returned tracking data that does not match the Rust-owned contract. No tracking rows or actions are available from this response.'
  ),
  cardBody: decodeDisplayText('Retry status after the local service reports a valid tracking status payload.'),
  label: decodeDisplayText('Tracking status'),
  title: decodeDisplayText('Tracking status data rejected'),
} as const;

type TrackingStatusPanelResolution =
  | { readonly state: 'available'; readonly panel: ParentTrackingStatusPanelSnapshot }
  | { readonly state: 'invalid-contract' | 'unavailable'; readonly panel: null };

export function shouldRenderTrackingStatusRoute(route: ParentRouteId): boolean {
  return isParentTrackingStatusRoute(route);
}

export function TrackingStatusRoutePanel({
  actions,
  commandEnabled,
  liveActivity,
  showUnavailable = false,
  surface = 'proof',
}: {
  readonly actions: PortalRenderActions;
  readonly commandEnabled: boolean;
  readonly liveActivity: PortalLiveActivityState;
  readonly showUnavailable?: boolean;
  readonly surface?: 'product' | 'proof';
}): ReactElement | null {
  const resolution = resolveTrackingStatusPanel(liveActivity.activityTrackingPanel);
  if (resolution.state !== 'available') {
    return surface === 'product' || showUnavailable
      ? renderTrackingStatusUnavailablePanel(actions, commandEnabled, resolution.state, surface)
      : null;
  }
  const panel = resolution.panel;
  const routeAction = trackingStatusRouteAction(actions, commandEnabled);
  return (
    <section
      aria-label={surface === 'product' ? panel.title : resolvePortalDevText(PortalDevTextToken.TrackingStatusSurface)}
      className={PortalDom.Classes.TrackingStatusOverlay}
      data-ocentra-tracking-surface={surface}
    >
      <div className={PortalDom.Classes.TrackingStatusOverlayContent}>
        <header className={PortalDom.Classes.TrackingStatusOverlayHeader}>
          <p className={PortalDom.Classes.ProductEyebrow}>{panel.eyebrow}</p>
          <h2>{panel.title}</h2>
          <p>{panel.body}</p>
          <button
            className={PortalDom.Classes.CommandResultTab}
            type={PortalDom.ButtonType.Button}
            onClick={routeAction.run}
          >
            {routeAction.label}
          </button>
        </header>
        {renderTrackingStatusRoutePanelBody(panel)}
      </div>
    </section>
  );
}

function renderTrackingStatusUnavailablePanel(
  actions: PortalRenderActions,
  commandEnabled: boolean,
  state: 'invalid-contract' | 'unavailable',
  surface: 'product' | 'proof'
): ReactElement {
  const text = state === 'invalid-contract' ? TRACKING_STATUS_INVALID_TEXT : TRACKING_STATUS_UNAVAILABLE_TEXT;
  const routeAction = trackingStatusRouteAction(actions, commandEnabled);
  return (
    <section
      aria-label={text.label}
      className={PortalDom.Classes.TrackingStatusOverlay}
      data-ocentra-tracking-route-state={state}
      data-ocentra-tracking-surface={surface}
    >
      <div className={PortalDom.Classes.TrackingStatusOverlayContent}>
        <header className={PortalDom.Classes.TrackingStatusOverlayHeader}>
          <p className={PortalDom.Classes.ProductEyebrow}>
            {resolvePortalDevText(PortalDevTextToken.TrackingServiceReadModel)}
          </p>
          <h2>{text.title}</h2>
          <p>{text.body}</p>
          <button
            className={PortalDom.Classes.CommandResultTab}
            type={PortalDom.ButtonType.Button}
            onClick={routeAction.run}
          >
            {routeAction.label}
          </button>
        </header>
        <div
          className={[PortalDom.Classes.ProductDashboard, PortalDom.Classes.TrackingStatusOverlayGrid].join(
            PortalDom.Classes.ClassNameSeparator
          )}
        >
          {renderTrackingStatusUnavailableCards(text.cardBody, state)}
        </div>
      </div>
    </section>
  );
}

function renderTrackingStatusUnavailableCards(
  serviceBody: string,
  state: 'invalid-contract' | 'unavailable'
): ReactElement[] {
  const cards = [
    {
      title: resolvePortalDevText(PortalDevTextToken.TrackingServiceReadModel),
      body: serviceBody,
    },
    {
      title: decodeDisplayText('Location and devices'),
      body: decodeDisplayText(
        'No location, accuracy, device freshness, or child status is displayed without a valid service row.'
      ),
    },
    {
      title: decodeDisplayText('Tracking controls'),
      body: decodeDisplayText(
        'Check-in, exception, live-tracking, missing-device, and notification actions stay unavailable until the service supplies owner-authorized inputs.'
      ),
    },
  ];
  return cards.map((card) => (
    <article
      className={trackingStatusUnavailableCardClassName()}
      data-ocentra-tracking-card-state={state}
      key={card.title}
    >
      <h2>{card.title}</h2>
      <p>{card.body}</p>
    </article>
  ));
}

function resolveTrackingStatusPanel(value: unknown | null): TrackingStatusPanelResolution {
  if (value == null) {
    return { state: 'unavailable', panel: null };
  }
  try {
    return { state: 'available', panel: decodeParentTrackingStatusPanelSnapshot(value) };
  } catch {
    return { state: 'invalid-contract', panel: null };
  }
}

function trackingStatusUnavailableCardClassName(): string {
  return [PortalDom.Classes.Summary, PortalDom.Classes.ProductStatusCard].join(PortalDom.Classes.ClassNameSeparator);
}

function trackingStatusRouteAction(
  actions: PortalRenderActions,
  commandEnabled: boolean
): { readonly label: string; readonly run: () => void } {
  const refreshRouteSnapshot = actions.refreshRouteSnapshot;
  if (!commandEnabled || refreshRouteSnapshot === undefined) {
    return {
      label: resolvePortalDevText(PortalDevTextToken.RetryStatus),
      run: actions.reconnect,
    };
  }
  return {
    label: resolvePortalDevText(PortalDevTextToken.GetActivityTrackingReadModel),
    run: () => {
      void refreshRouteSnapshot();
    },
  };
}
