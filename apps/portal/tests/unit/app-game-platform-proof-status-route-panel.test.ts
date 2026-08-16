import { createElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import { describe, expect, it } from 'vitest';
import { ParentRoute, type ParentAppGamePanelSnapshot } from '../../generated/parent-ui-bridge';
import type { PortalRenderActions } from '../../src/portal-actions';
import {
  AppGamePlatformProofStatusRoutePanel,
  shouldRenderAppGamePlatformProofStatusRoute,
} from '../../src/AppGamePlatformProofStatusRoutePanel';

const PlatformProofStatusPanel: ParentAppGamePanelSnapshot = {
  eyebrow: 'Rust-owned panel',
  title: 'App/game platform proof status',
  body: 'Rust-owned platform proof rows are rendered directly in the portal.',
  loadState: 'ready',
  summaryDetails: [
    { label: 'Platform proofs', value: '2' },
    { label: 'Host-visible rows', value: '1' },
    { label: 'Not-applicable rows', value: '1' },
    { label: 'Enforcement-ready rows', value: '0' },
  ],
  rows: [
    {
      title: 'windows',
      details: [
        { label: 'Host capability', value: 'available' },
        { label: 'Adapter dispatch', value: 'Not claimed' },
        { label: 'Platform state', value: 'Not claimed' },
      ],
    },
    {
      title: 'ios',
      details: [
        { label: 'Host capability', value: 'not-applicable' },
        { label: 'Adapter dispatch', value: 'Not claimed' },
        { label: 'Platform state', value: 'Not claimed' },
      ],
    },
  ],
  emptyMessage: 'No app/game platform proof-status panel has been reported yet.',
  productClaim: 'Broad blocking, platform enforcement, and child delivery remain unclaimed.',
};

describe('app-game platform proof status portal route panel', () => {
  it('attaches the renderer only to App/Game Sessions', () => {
    expect(shouldRenderAppGamePlatformProofStatusRoute(ParentRoute.AppGameSessions)).toBe(true);
    expect(shouldRenderAppGamePlatformProofStatusRoute(ParentRoute.Overview)).toBe(false);
  });

  it('renders the Rust-owned platform proof panel without TS normalization logic', () => {
    const html = renderToStaticMarkup(
      createElement(AppGamePlatformProofStatusRoutePanel, {
        actions: testActions(),
        commandEnabled: true,
        panel: PlatformProofStatusPanel,
      })
    );

    expect(html).toContain('App/game platform proof status');
    expect(html).toContain('Platform proofs');
    expect(html).toContain('Host-visible rows');
    expect(html).toContain('windows');
    expect(html).toContain('ios');
    expect(html).toContain('Host capability');
    expect(html).toContain('not-applicable');
  });

  it('keeps the Rust-owned empty state explicit when the panel snapshot is absent', () => {
    const html = renderToStaticMarkup(
      createElement(AppGamePlatformProofStatusRoutePanel, {
        actions: testActions(),
        commandEnabled: false,
        panel: null,
      })
    );

    expect(html).toContain('No app/game platform proof-status panel has been reported yet.');
    expect(html).toContain('Broad blocking, platform enforcement, and child delivery remain unclaimed.');
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
