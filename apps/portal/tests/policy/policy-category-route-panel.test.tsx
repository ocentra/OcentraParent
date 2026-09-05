import { createElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import { describe, expect, it } from 'vitest';
import { ParentRoute } from '../../generated/parent-ui-bridge';
import { PolicyCategoryRoutePanel, shouldRenderPolicyCategoryRoute } from '../../src/PolicyCategoryRoutePanel';

describe('policy category route panel', () => {
  it('owns only category routes without a service-reported policy editor', () => {
    expect([
      shouldRenderPolicyCategoryRoute(ParentRoute.BrowserSettings),
      shouldRenderPolicyCategoryRoute(ParentRoute.PolicyApps),
      shouldRenderPolicyCategoryRoute(ParentRoute.PolicyGames),
      shouldRenderPolicyCategoryRoute(ParentRoute.PolicyScreen),
      shouldRenderPolicyCategoryRoute(ParentRoute.PolicyRemoteScreen),
      shouldRenderPolicyCategoryRoute(ParentRoute.PolicyNetwork),
    ]).toEqual([true, true, true, false, true, false]);
  });

  it('routes each unavailable category to real operational surfaces', () => {
    for (const [route, title, primaryAction] of [
      [ParentRoute.BrowserSettings, 'Browser policy controls unavailable', 'Open browser activity'],
      [ParentRoute.PolicyApps, 'App policy controls unavailable', 'Open app activity'],
      [ParentRoute.PolicyGames, 'Game policy controls unavailable', 'Open game activity'],
      [ParentRoute.PolicyRemoteScreen, 'Remote screen controls unavailable', 'Open screen analysis'],
    ] as const) {
      const markup = renderToStaticMarkup(
        createElement(PolicyCategoryRoutePanel, {
          onNavigate: () => false,
          route,
        })
      );

      expect(markup).toContain('data-ocentra-policy-category-state="manual-required"');
      expect(markup).toContain(title);
      expect(markup).toContain(primaryAction);
      expect(markup).toContain(route === ParentRoute.PolicyRemoteScreen ? 'Open Start Here' : 'Open rules');
      expect(markup).not.toContain('CHANGES ARE UNAVAILABLE');
      expect(markup).not.toContain('CURRENT POLICY NOT SHOWN HERE');
      expect(markup.match(/<button/g)).toHaveLength(3);
      expect(markup.match(/<article/g)).toHaveLength(3);
      expect(markup).toContain('Current policy');
      expect(markup).toContain('Not reported');
      expect(markup).toContain('Editing authority');
      expect(markup).toContain('Manual required');
      expect(markup).toContain('Review only');
    }
  });
});
