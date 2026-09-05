import { createElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import { describe, expect, it } from 'vitest';
import { ParentRoute } from '../../generated/parent-ui-bridge';
import { CapabilityStatusRoutePanel, shouldRenderCapabilityStatusRoute } from '../../src/CapabilityStatusRoutePanel';
import type { PortalRenderActions } from '../../src/portal-actions';
import { resolveSnapshotLiveActivityState } from '../../src/route-live-activity-state';
import { networkFlowReadModelSnapshot } from '../live-activity/live-activity-state-test-support';

const actions: PortalRenderActions = {
  reconnect() {},
  selectCommandResult() {},
  async sendCommand() {
    return null;
  },
};

describe('capability status route panel', () => {
  it('renders only on the dedicated Capability Status route', () => {
    expect([
      shouldRenderCapabilityStatusRoute(ParentRoute.CapabilityStatus),
      shouldRenderCapabilityStatusRoute(ParentRoute.Overview),
    ]).toEqual([true, false]);
  });

  it('offers reconnection and explicit unavailable domain boundaries without fabricating reported capability', () => {
    const markup = renderToStaticMarkup(
      createElement(CapabilityStatusRoutePanel, {
        actions,
        commandEnabled: false,
        liveActivity: resolveSnapshotLiveActivityState(null),
        shellStatus: null,
      })
    );

    expect(markup).toContain('data-ocentra-capability-status-state="unavailable"');
    expect(markup).toContain('aria-label="Capability domains"');
    expect(markup).toContain('Capability status unavailable');
    expect(markup).toContain('Retry status');
    expect(markup).not.toContain('Refresh capability status');
    expect(markup.match(/data-ocentra-capability-card-state="unavailable"/g)).toHaveLength(6);
    expect(markup).toContain('<h3>Browser</h3>');
    expect(markup).toContain('<h3>Tracking</h3>');
    expect(markup).toContain('<h3>Screen activity</h3>');
    expect(markup).toContain('<h3>App activity</h3>');
    expect(markup).toContain('<h3>Game activity</h3>');
    expect(markup).toContain('<h3>Network activity</h3>');
    expect(markup).toContain('No browser capability is reported. Browser controls remain unavailable.');
    expect(markup).not.toContain('data-ocentra-capability-card-state="reported"');
  });

  it('renders service-reported shell status when domain activity is unavailable', () => {
    const markup = renderToStaticMarkup(
      createElement(CapabilityStatusRoutePanel, {
        actions,
        commandEnabled: false,
        liveActivity: resolveSnapshotLiveActivityState(null),
        shellStatus: {
          routeLabel: 'Capability status',
          parentAccessState: 'proof-missing',
          globalConnectionState: 'offline',
          routeCapabilityState: 'unavailable',
          dataSourceLabel: 'unavailable',
          cards: [
            {
              id: 'parent-access',
              label: 'Parent access',
              value: 'proof-missing',
              detail: 'No LAN authority proof is attached because the local agent-service route is unavailable.',
              tone: 'muted',
            },
            {
              id: 'connection',
              label: 'Connection',
              value: 'offline',
              detail: 'route: Capability status',
              tone: 'red',
            },
          ],
        },
      })
    );

    expect(markup).toContain('data-ocentra-capability-status-state="reported"');
    expect(markup).toContain('Capability and service status');
    expect(markup).toContain('<h3>Parent access</h3>');
    expect(markup).toContain('<dd>proof-missing</dd>');
    expect(markup).toContain('<h3>Connection</h3>');
    expect(markup).toContain('<dd>offline</dd>');
    expect(markup.match(/data-ocentra-capability-card-state="unavailable"/g)).toHaveLength(6);
    expect(markup).toContain('No browser capability is reported. Browser controls remain unavailable.');
    expect(markup).toContain('No network capability is reported. Network evidence and controls remain unavailable.');
    expect(markup).not.toContain('No capability or service read model has been reported');
  });

  it('renders only strictly decoded service-reported domain states', () => {
    const markup = renderToStaticMarkup(
      createElement(CapabilityStatusRoutePanel, {
        actions: {
          ...actions,
          async refreshRouteSnapshot() {
            return null;
          },
        },
        commandEnabled: true,
        liveActivity: resolveSnapshotLiveActivityState({
          activityScreenReadModel: { ok: false, state: 'unavailable', reason: 'invalid-payload' },
          activityAppUseReadModel: { ok: false, state: 'unavailable', reason: 'wrong-event' },
        }),
        shellStatus: null,
      })
    );

    expect(markup).toContain('data-ocentra-capability-status-state="reported"');
    expect(markup).toContain('<h3>Screen activity</h3>');
    expect(markup).toContain('<h3>App activity</h3>');
    expect(markup).toContain('invalid-payload');
    expect(markup).toContain('wrong-event');
    expect(markup).toContain('Refresh capability status');
    expect(markup.match(/data-ocentra-capability-card-state="unavailable"/g)).toHaveLength(6);
    expect(markup).toContain('<h3>Browser</h3>');
    expect(markup).toContain('<h3>Tracking</h3>');
    expect(markup).toContain('<h3>Game activity</h3>');
    expect(markup).toContain('<h3>Network activity</h3>');
    expect(markup).toContain('No browser capability is reported. Browser controls remain unavailable.');
  });

  it('renders network capability from the service network-flow read model', () => {
    const markup = renderToStaticMarkup(
      createElement(CapabilityStatusRoutePanel, {
        actions,
        commandEnabled: true,
        liveActivity: resolveSnapshotLiveActivityState({
          networkFlowReadModel: networkFlowReadModelSnapshot(),
        }),
        shellStatus: null,
      })
    );

    expect(markup).toContain('<h3>Network activity</h3>');
    expect(markup).toContain('<dd>available</dd>');
    expect(markup).toContain('1 network flow row reported; custody child-device-query-store');
    expect(markup).not.toContain('No network capability is reported');
  });
});
