import { describe, expect, it } from 'vitest';
import { PortalRoute } from '@ocentra-parent/portal-domain/contracts';
import { shouldRenderScreenSummaryRoute } from '../src/ScreenSummaryRoutePanel';

describe('screen summary route panel', () => {
  it('renders only on the Screen Analysis route', () => {
    expect(shouldRenderScreenSummaryRoute(PortalRoute.ScreenAnalysis)).toBe(true);
    expect(shouldRenderScreenSummaryRoute(PortalRoute.Activity)).toBe(false);
    expect(shouldRenderScreenSummaryRoute(PortalRoute.Overview)).toBe(false);
  });
});
