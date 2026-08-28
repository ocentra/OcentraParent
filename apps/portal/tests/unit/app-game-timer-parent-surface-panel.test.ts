import { createElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import { describe, expect, it } from 'vitest';
import type { ParentUiActionPayload } from '../../generated/parent-ui-bridge';
import { ParentRoute } from '../../generated/parent-ui-bridge';
import type { PortalRenderActions } from '../../src/portal-actions';
import {
  AppGameTimerParentSurfaceRoutePanel,
  sendAppGameTimerParentPreferenceSetupAction,
  shouldRenderAppGameTimerParentSurfaceRoute,
} from '../../src/AppGameTimerParentSurfaceRoutePanel';
import { AppGameTimerParentSurfacePanelFixture } from '../fixtures/app-game/app-game-timer-parent-surface-panel-fixture';

const PreferenceSetupPayload: ParentUiActionPayload = {
  ActivityAppGameTimerParentPreferenceSetupRequest:
    '{"parentSurfaceIntentReferenceId":"app-game-child-ux-parent-surface-action-result-app-game-1","parentPreferenceSetupReferenceId":"app-game-child-ux-parent-preference-setup-action-result-app-game-1","requestReferenceIds":["app-game-child-ux-local-handoff-action-result-app-game-1","parent-approved","child-status-limit-reached"]}',
};

const GuardedPortalRenderActions: PortalRenderActions = {
  reconnect() {
    throw new Error('unexpected reconnect during timer parent-surface render');
  },
  selectCommandResult() {
    throw new Error('unexpected command-result selection during timer parent-surface render');
  },
  async sendCommand() {
    throw new Error('unexpected command dispatch during timer parent-surface render');
  },
};

describe('app-game timer parent-surface portal route panel', () => {
  it('attaches the renderer only to App/Game Sessions', () => {
    expect(shouldRenderAppGameTimerParentSurfaceRoute(ParentRoute.AppGameSessions)).toBe(true);
    expect(shouldRenderAppGameTimerParentSurfaceRoute(ParentRoute.Overview)).toBe(false);
  });

  it('dispatches parent preference setup through the typed Rust-owned bridge action payload', () => {
    const payloads: ParentUiActionPayload[] = [];
    const actions: PortalRenderActions = {
      ...GuardedPortalRenderActions,
      async requestAppGameTimerParentPreferenceSetup(payload) {
        payloads.push(payload);
        return null;
      },
    };

    sendAppGameTimerParentPreferenceSetupAction(actions, PreferenceSetupPayload);

    expect(payloads).toEqual([PreferenceSetupPayload]);
  });

  it('renders Rust-owned timer parent-surface rows', () => {
    const html = renderToStaticMarkup(
      createElement(AppGameTimerParentSurfaceRoutePanel, {
        actions: GuardedPortalRenderActions,
        commandEnabled: true,
        panel: AppGameTimerParentSurfacePanelFixture,
      })
    );

    expect(html).toContain('App/game timer parent surface');
    expect(html).toContain('Refresh timer parent surface');
    expect(html).toContain('Timer runtime</dt><dd>Ready');
    expect(html).toContain('Session duration</dt><dd>15 min from stored evidence');
    expect(html).toContain('Session duration</dt><dd>15 min');
    expect(html).toContain('Evidence source</dt><dd>stored journal rows');
  });

  it('withholds parent preference controls when the Rust-owned action is unavailable', () => {
    const panel = {
      ...AppGameTimerParentSurfacePanelFixture,
      parentPreferenceSetupRows: [
        {
          title: 'Parent preference setup',
          details: [{ label: 'Status', value: 'Manual required' }],
          actionLabel: 'Set up parent preference',
          actionPayload: PreferenceSetupPayload,
        },
      ],
    };
    const html = renderToStaticMarkup(
      createElement(AppGameTimerParentSurfaceRoutePanel, {
        actions: GuardedPortalRenderActions,
        commandEnabled: true,
        panel,
      })
    );

    expect(html).toContain('Parent preference setup');
    expect(html).toContain('Manual required');
    expect(html).not.toContain('Set up parent preference');
  });

  it('keeps the absent Rust panel explicit instead of inventing rows', () => {
    const html = renderToStaticMarkup(
      createElement(AppGameTimerParentSurfaceRoutePanel, {
        actions: GuardedPortalRenderActions,
        commandEnabled: false,
        panel: null,
      })
    );

    expect(html).toContain('Rust has not reported an app/game timer parent surface panel yet.');
    expect(html).toContain('No timer parent surface rows have been reported yet.');
  });
});
