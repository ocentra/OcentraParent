import { createElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import { describe, expect, it } from 'vitest';
import { ParentRoute } from '../../generated/parent-ui-bridge';
import {
  NetworkEvidenceDrawerRoutePanel,
  shouldRenderNetworkEvidenceDrawerRoute,
} from '../../src/NetworkEvidenceDrawerRoutePanel';
import { resolveSnapshotLiveActivityState } from '../../src/route-live-activity-state';
import { NetworkEvidenceDrawerRoutePanelFixture } from '../fixtures/network/network-evidence-drawer-route-panel-fixture';

describe('network evidence drawer portal route panel', () => {
  it('attaches the network evidence drawer only to network routes', () => {
    expect(shouldRenderNetworkEvidenceDrawerRoute(ParentRoute.Activity)).toBe(true);
    expect(shouldRenderNetworkEvidenceDrawerRoute(ParentRoute.NetworkActivity)).toBe(true);
    expect(shouldRenderNetworkEvidenceDrawerRoute(ParentRoute.Overview)).toBe(false);
  });

  it('renders network flow as metadata and leaves exact URL claims unexpanded', () => {
    const html = renderToStaticMarkup(
      createElement(NetworkEvidenceDrawerRoutePanel, {
        liveActivity: NetworkEvidenceDrawerRoutePanelFixture.liveActivity,
        networkEvidenceSummary: NetworkEvidenceDrawerRoutePanelFixture.networkEvidenceSummary,
        route: ParentRoute.Activity,
      })
    );

    expect(html).toContain('Network activity');
    expect(html).toContain('Network protocol</dt><dd>tcp');
    expect(html).toContain('TCP state</dt><dd>established');
    expect(html).toContain('Process</dt><dd>notepad.exe | 4242 | process-attributed');
    expect(html).toContain('Domain</dt><dd>browser-network-metadata.example.test');
    expect(html).toContain('domain-observed');
    expect(html).toContain('Connections</dt><dd>1');
    expect(html).toContain('Exact URL claim</dt><dd>Not reported');
    expect(html).toContain('Performance state</dt><dd>degraded');
    expect(html).toContain('Local AI result</dt><dd>ai-audit-1');
    expect(html).not.toContain('page content');
  });

  it('keeps unknown network-flow states explicit instead of inventing flow details', () => {
    const html = renderToStaticMarkup(
      createElement(NetworkEvidenceDrawerRoutePanel, {
        liveActivity: resolveSnapshotLiveActivityState({
          browserInterventionReadModel: null,
          browserManagedStatus: null,
          lanAddDeviceReadModel: null,
          networkFlowReadModel: null,
        }),
        route: ParentRoute.NetworkActivity,
      })
    );

    expect(html).toContain('No network activity is available yet.');
    expect(html).toContain('Exact URL claim</dt><dd>Not reported');
    expect(html).toContain('Performance state</dt><dd>Not reported');
  });
});
