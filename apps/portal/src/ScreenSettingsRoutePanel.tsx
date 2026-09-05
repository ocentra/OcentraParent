import type { ReactElement } from 'react';
import { PortalDom } from '@ocentra-parent/portal-domain/contracts';
import {
  decodeDisplayText,
  PortalDevTextToken,
  resolvePortalDevText,
} from '@ocentra-parent/portal-domain/display-text';
import { isParentScreenSettingsRoute, type ParentRouteId } from '../generated/parent-ui-bridge';
import type { PortalRenderActions } from './portal-actions';
import { ScreenSettingsWritableControls } from './ScreenSettingsWritableControls';

const SCREEN_SETTINGS_ROUTE_TEXT = {
  ariaLabel: decodeDisplayText('Screen analysis settings'),
  eyebrow: decodeDisplayText('Parent controls'),
  title: decodeDisplayText('Screen analysis settings'),
  body: decodeDisplayText(
    'Choose how supported child devices may create local, redacted screen summaries. Raw capture, hosted processing, and live view stay off unless separately approved and supported.'
  ),
} as const;

export function shouldRenderScreenSettingsRoute(route: ParentRouteId): boolean {
  return isParentScreenSettingsRoute(route);
}

export function ScreenSettingsRoutePanel({
  actions,
  commandEnabled,
  serviceResponseSnapshot,
}: {
  readonly actions: PortalRenderActions;
  readonly commandEnabled: boolean;
  readonly serviceResponseSnapshot: unknown | null;
}): ReactElement {
  return (
    <section
      aria-label={SCREEN_SETTINGS_ROUTE_TEXT.ariaLabel}
      className={PortalDom.Classes.TrackingStatusOverlay}
      data-ocentra-screen-settings-connection={commandEnabled ? 'connected' : 'unavailable'}
    >
      <div className={PortalDom.Classes.TrackingStatusOverlayContent}>
        <header className={PortalDom.Classes.TrackingStatusOverlayHeader}>
          <p className={PortalDom.Classes.ProductEyebrow}>{SCREEN_SETTINGS_ROUTE_TEXT.eyebrow}</p>
          <h2>{SCREEN_SETTINGS_ROUTE_TEXT.title}</h2>
          <p>{SCREEN_SETTINGS_ROUTE_TEXT.body}</p>
          {commandEnabled ? null : (
            <button
              className={PortalDom.Classes.CommandResultTab}
              onClick={actions.reconnect}
              type={PortalDom.ButtonType.Button}
            >
              {resolvePortalDevText(PortalDevTextToken.RetryStatus)}
            </button>
          )}
        </header>
        <div
          className={[PortalDom.Classes.ProductDashboard, PortalDom.Classes.TrackingStatusOverlayGrid].join(
            PortalDom.Classes.ClassNameSeparator
          )}
        >
          <ScreenSettingsWritableControls
            actions={actions}
            commandEnabled={commandEnabled}
            serviceResponseSnapshot={serviceResponseSnapshot}
          />
        </div>
      </div>
    </section>
  );
}
