import { createElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import { describe, expect, it } from 'vitest';
import { ParentRoute, type ParentAppGamePanelSnapshot } from '../../generated/parent-ui-bridge';
import type { PortalRenderActions } from '../../src/portal-actions';
import {
  AppGameChildRuntimeTransportReceiptRoutePanel,
  shouldRenderAppGameChildRuntimeTransportReceiptRoute,
} from '../../src/AppGameChildRuntimeTransportReceiptRoutePanel';

const ChildRuntimeTransportReceiptPanel: ParentAppGamePanelSnapshot = {
  eyebrow: 'Rust-owned panel',
  title: 'App/game child runtime transport receipt',
  body: 'Rust-owned child runtime transport rows are rendered directly in the portal.',
  loadState: 'ready',
  summaryDetails: [
    { label: 'Transport rows', value: '2' },
    { label: 'Transport-required rows', value: '1' },
    { label: 'Manual-required rows', value: '1' },
    { label: 'Runtime transport', value: 'Not claimed' },
    { label: 'Runtime receipt', value: 'Not claimed' },
  ],
  rows: [
    {
      title: 'app-game-child-runtime-transport-receipt-warning',
      details: [
        { label: 'Boundary state', value: 'child-runtime-transport-required' },
        { label: 'Open gaps', value: 'child-runtime-transport-not-executed' },
      ],
    },
    {
      title: 'app-game-child-runtime-transport-receipt-apple',
      details: [
        { label: 'Boundary state', value: 'manual-required' },
        { label: 'Open gaps', value: 'child-runtime-receipt-not-ingested' },
      ],
    },
  ],
  emptyMessage: 'No app/game child runtime transport receipt panel has been reported yet.',
  productClaim: 'Runtime transport, runtime receipt, and provider delivery remain unclaimed.',
};

describe('app-game child runtime transport receipt portal route panel', () => {
  it('attaches the renderer only to App/Game Sessions', () => {
    expect(shouldRenderAppGameChildRuntimeTransportReceiptRoute(ParentRoute.AppGameSessions)).toBe(true);
    expect(shouldRenderAppGameChildRuntimeTransportReceiptRoute(ParentRoute.Overview)).toBe(false);
  });

  it('renders the Rust-owned child runtime transport panel without TS read-model reconstruction', () => {
    const html = renderToStaticMarkup(
      createElement(AppGameChildRuntimeTransportReceiptRoutePanel, {
        actions: testActions(),
        commandEnabled: true,
        panel: ChildRuntimeTransportReceiptPanel,
      })
    );

    expect(html).toContain('App/game child runtime transport receipt');
    expect(html).toContain('Transport rows');
    expect(html).toContain('app-game-child-runtime-transport-receipt-warning');
    expect(html).toContain('child-runtime-transport-required');
    expect(html).toContain('app-game-child-runtime-transport-receipt-apple');
    expect(html).toContain('child-runtime-receipt-not-ingested');
  });

  it('keeps the Rust-owned empty state explicit when the panel snapshot is absent', () => {
    const html = renderToStaticMarkup(
      createElement(AppGameChildRuntimeTransportReceiptRoutePanel, {
        actions: testActions(),
        commandEnabled: false,
        panel: null,
      })
    );

    expect(html).toContain('No app/game child runtime transport receipt panel has been reported yet.');
    expect(html).toContain('Runtime transport, runtime receipt, and provider delivery remain unclaimed.');
    expect(html).toContain('unavailable');
    expect(html).toContain('disabled=""');
  });
});

function testActions(): PortalRenderActions {
  return {
    reconnect() {},
    selectCommandResult() {},
    async sendCommand() {
      return null;
    },
    async refreshRouteSnapshot() {
      return null;
    },
  };
}
