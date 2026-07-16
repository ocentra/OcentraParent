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

const NoopPortalRenderActions: PortalRenderActions = {
  reconnect() {},
  selectCommandResult() {},
  async sendCommand() {
    return null;
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
      ...NoopPortalRenderActions,
      async requestAppGameTimerParentPreferenceSetup(payload) {
        payloads.push(payload);
        return null;
      },
    };

    sendAppGameTimerParentPreferenceSetupAction(actions, PreferenceSetupPayload);

    expect(payloads).toEqual([PreferenceSetupPayload]);
  });

  it('renders Rust-owned timer parent-surface rows and action controls', () => {
    const html = renderToStaticMarkup(
      createElement(AppGameTimerParentSurfaceRoutePanel, {
        actions: NoopPortalRenderActions,
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

  it('keeps the absent Rust panel explicit instead of inventing rows', () => {
    const html = renderToStaticMarkup(
      createElement(AppGameTimerParentSurfaceRoutePanel, {
        actions: NoopPortalRenderActions,
        commandEnabled: false,
        panel: null,
      })
    );

    expect(html).toContain('Rust has not reported an app/game timer parent surface panel yet.');
    expect(html).toContain('No timer parent surface rows have been reported yet.');
  });
});
