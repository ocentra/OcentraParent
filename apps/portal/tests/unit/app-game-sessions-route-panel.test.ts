import { createElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import { describe, expect, it } from 'vitest';
import {
  ParentRoute,
  type ParentAppGameNotificationParentSurfacePanelSnapshot,
  type ParentAppGamePanelSnapshot,
} from '../../generated/parent-ui-bridge';
import { AppGameSessionsRoutePanel, shouldRenderAppGameSessionsRoutePanel } from '../../src/AppGameSessionsRoutePanel';
import type { PortalRenderActions } from '../../src/portal-actions';

const actions: PortalRenderActions = {
  reconnect() {},
  selectCommandResult() {},
  async sendCommand() {
    return null;
  },
};

const notificationPanel: ParentAppGameNotificationParentSurfacePanelSnapshot = {
  eyebrow: 'Rust-owned notifications',
  title: 'Notification readiness from Rust',
  body: 'Notification readiness is projected by the parent runtime.',
  state: 'manual-required',
  summary: 'Notifications require manual setup',
  productClaim: 'Provider delivery remains unclaimed.',
  metrics: [{ label: 'Rows returned', value: '0' }],
  rows: [],
  emptyMessage: 'No notification delivery rows reported.',
};

const policyPanel: ParentAppGamePanelSnapshot = {
  eyebrow: 'Rust-owned policy',
  title: 'Policy readiness from Rust',
  body: 'Policy readiness is projected by the parent runtime.',
  loadState: 'manual-required',
  summaryDetails: [{ label: 'Status', value: 'manual-required' }],
  rows: [],
  emptyMessage: 'No policy readiness rows reported.',
  productClaim: 'Adapter dispatch remains unclaimed.',
};

describe('app/game sessions route panel', () => {
  it('keeps reported notification and policy status in collapsed state-labelled drawers', () => {
    const html = renderToStaticMarkup(
      createElement(AppGameSessionsRoutePanel, {
        actions,
        commandEnabled: false,
        notificationPanel,
        policyPanel,
      })
    );

    expect(html).toContain('aria-label="App and game sessions"');
    expect(html).toContain('data-ocentra-app-game-status-panel="open-on-unavailable"');
    expect(html).toContain('data-ocentra-app-game-route-state="reported"');
    expect(html).toContain('class="app-game-sessions-route-panel-shell"');
    expect(html).toContain('data-ocentra-app-game-notification-state="manual-required"');
    expect(html).toContain('data-ocentra-app-game-policy-state="manual-required"');
    expect(html).toContain('App activity status');
    expect(html).toContain('Retry status');
    expect(html.match(/<details/g)).toHaveLength(3);
    expect(html).not.toContain('<details open=""');
    expect(html).toContain('Notification readiness from Rust');
    expect(html).toContain('Policy readiness from Rust');
    expect(html).toContain('No notification delivery rows reported.');
    expect(html).toContain('No policy readiness rows reported.');
    expect(html.match(/<button/g)).toHaveLength(1);
  });

  it('opens unavailable status so the honest retry action is visible without expanding a bottom dock', () => {
    const html = renderToStaticMarkup(
      createElement(AppGameSessionsRoutePanel, {
        actions,
        commandEnabled: false,
        notificationPanel: null,
        policyPanel: null,
      })
    );

    expect(html).toContain('data-ocentra-app-game-route-state="unavailable"');
    expect(html).toContain('class="app-game-sessions-route-panel-shell" open=""');
    expect(html).toContain('Retry status to load app use, game, notification, and policy status.');
    expect(html).toContain('>Retry status</button>');
    expect(html).not.toContain('status-dock');
  });

  it('exposes refresh outside the collapsed drawers when the service is connected', () => {
    const html = renderToStaticMarkup(
      createElement(AppGameSessionsRoutePanel, {
        actions: {
          ...actions,
          async refreshRouteSnapshot() {
            return null;
          },
        },
        commandEnabled: true,
        notificationPanel,
        policyPanel,
      })
    );

    expect(html).toContain('Refresh app activity');
    expect(html).not.toContain('>Retry status<');
    expect(html.match(/<button/g)).toHaveLength(1);
  });

  it('is owned only by the App/Game Sessions route', () => {
    expect(shouldRenderAppGameSessionsRoutePanel(ParentRoute.AppGameSessions)).toBe(true);
    expect(shouldRenderAppGameSessionsRoutePanel(ParentRoute.ProofPanels)).toBe(false);
  });
});
