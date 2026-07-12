import { renderToStaticMarkup } from 'react-dom/server';
import { describe, expect, it } from 'vitest';
import type { ParentAppGameAdapterDispatchPanelSnapshot } from '../../generated/parent-ui-bridge';
import { ParentRoute } from '../../generated/parent-ui-bridge';
import type { PortalRenderActions } from '../../src/portal-actions';
import {
  AppGameAdapterDispatchRoutePanel,
  sendAppGameAdapterDispatchExecuteAction,
  shouldRenderAppGameAdapterDispatchRoute,
} from '../../src/AppGameAdapterDispatchRoutePanel';

const AdapterDispatchPanel: ParentAppGameAdapterDispatchPanelSnapshot = {
  eyebrow: 'Runtime reference',
  title: 'App/game adapter dispatch',
  body: 'Service-backed command-result handoff for scoped app/game adapter dispatch.',
  preflightPanel: {
    eyebrow: 'Runtime reference',
    title: 'App/game adapter dispatch preflight',
    body: 'Scoped preflight visibility.',
    loadState: 'ready',
    summaryDetails: [
      { label: 'Capability', value: 'Scoped dispatch ready' },
      { label: 'Product claim', value: 'Preflight remains scoped to Rust-owned adapter dispatch readiness.' },
    ],
    rows: [
      {
        title: 'windows-app-game-owned-process-time-limit',
        details: [
          { label: 'Platform', value: 'windows' },
          { label: 'Platform state', value: 'Not claimed' },
        ],
      },
    ],
    emptyMessage: 'No preflight rows reported.',
    productClaim: 'Preflight remains scoped to Rust-owned adapter dispatch readiness.',
  },
  resultPanel: {
    eyebrow: 'Runtime reference',
    title: 'App/game adapter dispatch result',
    body: 'Scoped adapter dispatch execution visibility.',
    loadState: 'ready',
    summaryDetails: [
      { label: 'Adapter dispatch', value: 'Accepted' },
      {
        label: 'Product claim',
        value:
          'Broad installed-app blocking, platform enforcement, provider delivery, and child delivery remain unclaimed.',
      },
    ],
    rows: [
      {
        title: 'windows-app-game-owned-process-time-limit',
        details: [
          { label: 'Adapter execution', value: 'actually-enforced' },
          { label: 'Platform state', value: 'Not claimed' },
          { label: 'Child delivery', value: 'Not claimed' },
        ],
      },
    ],
    emptyMessage: 'No result rows reported.',
    productClaim:
      'Broad installed-app blocking, platform enforcement, provider delivery, and child delivery remain unclaimed.',
  },
  executeActionLabel: 'Execute scoped adapter dispatch',
};

const NoopPortalRenderActions: PortalRenderActions = {
  reconnect() {},
  selectCommandResult() {},
  async sendCommand() {
    return null;
  },
};

describe('app-game adapter dispatch route panel', () => {
  it('attaches only to App/Game Sessions', () => {
    expect(shouldRenderAppGameAdapterDispatchRoute(ParentRoute.AppGameSessions)).toBe(true);
    expect(shouldRenderAppGameAdapterDispatchRoute(ParentRoute.Overview)).toBe(false);
  });

  it('sends the explicit scoped execute action through the typed command path', () => {
    let requested = 0;
    const actions: PortalRenderActions = {
      ...NoopPortalRenderActions,
      async requestAppGameAdapterDispatchExecute() {
        requested += 1;
        return null;
      },
    };

    sendAppGameAdapterDispatchExecuteAction(actions);

    expect(requested).toBe(1);
  });

  it('renders Rust-owned preflight and result panels without TS intent builders', () => {
    const html = renderToStaticMarkup(
      <AppGameAdapterDispatchRoutePanel
        actions={NoopPortalRenderActions}
        commandEnabled={true}
        panel={AdapterDispatchPanel}
      />
    );

    expect(html).toContain('App/game adapter dispatch');
    expect(html).toContain('Refresh adapter dispatch preflight');
    expect(html).toContain('Refresh adapter dispatch result');
    expect(html).toContain('Execute scoped adapter dispatch');
    expect(html).toContain('windows-app-game-owned-process-time-limit');
    expect(html).toContain('Platform state</dt><dd>Not claimed');
    expect(html).toContain('Child delivery</dt><dd>Not claimed');
    expect(html).toContain(
      'Broad installed-app blocking, platform enforcement, provider delivery, and child delivery remain unclaimed.'
    );
  });
});
