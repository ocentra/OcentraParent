import { createElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import { describe, expect, it } from 'vitest';
import { ParentRoute } from '../../generated/parent-ui-bridge';
import { RemoteAccessRoutePanel, shouldRenderRemoteAccessRoute } from '../../src/RemoteAccessRoutePanel';

describe('remote access route panel', () => {
  it('owns only the remote access product route', () => {
    expect([
      shouldRenderRemoteAccessRoute(ParentRoute.RemoteAccess),
      shouldRenderRemoteAccessRoute(ParentRoute.PolicyRemoteScreen),
      shouldRenderRemoteAccessRoute(ParentRoute.Devices),
    ]).toEqual([true, false, false]);
  });

  it('offers only real navigation while owner-backed remote authority is unavailable', () => {
    const markup = renderToStaticMarkup(
      createElement(RemoteAccessRoutePanel, {
        onNavigate: () => false,
      })
    );

    expect(markup).toContain('data-ocentra-remote-access-state="manual-required"');
    expect(markup).toContain('Remote access unavailable');
    expect(markup).toContain('Open Start Here');
    expect(markup).toContain('Open devices');
    expect(markup).toContain('Review remote screen policy');
    expect(markup).toContain('Remote session');
    expect(markup).toContain('Trusted target');
    expect(markup).toContain('Control authority');
    expect(markup.match(/<article class="summary product-status-card">/gu)).toHaveLength(3);
    expect(markup).not.toContain('REMOTE TARGET NOT REPORTED');
    expect(markup).not.toContain('REMOTE ACCESS NOT AVAILABLE');
    expect(markup.match(/<button/g)).toHaveLength(3);
  });
});
