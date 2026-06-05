import { describe, expect, it } from 'vitest';
import { PortalRoute } from '@ocentra-parent/portal-domain/contracts';
import { createSocialDashboardPanelIntent } from '../src/social-dashboard-panel';
import { shouldRenderSocialDashboardRoute } from '../src/SocialDashboardRoutePanel';

describe('social dashboard portal route panel', () => {
  it('mounts only on the Browser route', () => {
    expect(shouldRenderSocialDashboardRoute(PortalRoute.Browser)).toBe(true);
    expect(shouldRenderSocialDashboardRoute(PortalRoute.AppGameSessions)).toBe(false);
  });

  it('keeps the live route empty until a service-backed social snapshot exists', () => {
    const intent = createSocialDashboardPanelIntent(null);

    expect(intent.rows).toEqual([]);
    expect(intent.emptyMessage).toBe('No social dashboard snapshot has been reported yet.');
    expect(intent.productClaim).toContain('social runtime data fetch');
    expect(intent.productClaim).toContain('enforcement remain unclaimed');
  });
});
