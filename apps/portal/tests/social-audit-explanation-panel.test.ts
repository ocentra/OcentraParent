import { describe, expect, it } from 'vitest';
import {
  createSocialAuditExplanationPanelIntent,
} from '@ocentra-parent/portal-domain/social-audit-explanation-panel';
import { PortalRoute } from '@ocentra-parent/portal-domain/routes';
import { shouldRenderSocialAuditExplanationRoute } from '../src/SocialAuditExplanationRoutePanel';

describe('social audit explanation portal route panel', () => {
  it('mounts only on the Browser route', () => {
    expect(shouldRenderSocialAuditExplanationRoute(PortalRoute.Browser)).toBe(true);
    expect(shouldRenderSocialAuditExplanationRoute(PortalRoute.AppGameSessions)).toBe(false);
  });

  it('keeps the live route empty until a proof or future service-backed explanation snapshot exists', () => {
    const intent = createSocialAuditExplanationPanelIntent(null);

    expect(intent.rows).toEqual([]);
    expect(intent.emptyMessage).toBe('No social audit explanation snapshot has been reported yet.');
    expect(intent.productClaim).toContain('runtime audit-store delivery');
    expect(intent.productClaim).toContain('enforcement remain unclaimed');
  });
});
