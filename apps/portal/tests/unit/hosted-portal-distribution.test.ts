import { createElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import { expect, it } from 'vitest';

import {
  HostedPortalDistribution,
  resolveHostedPortalDistributionState as resolveHostedPortalDistributionStateFromApp,
} from '../../src/hosted-portal-distribution';
import {
  resolveHostedPortalDistributionState as resolveHostedPortalDistributionStateFromDomain,
  type HostedPortalEnv,
  type HostedPortalLocation,
} from '@ocentra-parent/portal-domain/hosted-portal-distribution';

const hostedEnv = {
  VITE_OCENTRA_PARENT_HOSTED_PORTAL_MODE: 'hosted',
} as const;

it('separates preview, staging, and production hosted paths', () => {
  const preview = requireState('/preview');
  const staging = requireState('/staging');
  const production = requireState('/production');

  expect(preview.environment).toBe('preview');
  expect(staging.environment).toBe('staging');
  expect(production.environment).toBe('production');
});

it('labels preview, staging, and production hosted paths distinctly', () => {
  const preview = requireState('/preview');
  const staging = requireState('/staging');
  const production = requireState('/production');

  expect(preview.openActionLabel).toBe('Open parent portal preview');
  expect(staging.openActionLabel).toBe('Open parent portal staging');
  expect(production.openActionLabel).toBe('Open parent portal production');
});

it('reports missing hosted runtime ownership without rendering a dead action button', () => {
  const state = requireState('/preview');
  const markup = renderToStaticMarkup(createElement(HostedPortalDistribution, { state }));

  expect(state.controlsEnabled).toBe(true);
  expect(markup).toContain('data-hosted-action-state="runtime-owner-unavailable"');
  expect(markup).toContain(
    'Open parent portal preview is unavailable until an authenticated hosted runtime owner is connected.'
  );
  expect(markup).not.toContain('<button');
});

it('rejects unsupported hosted paths instead of falling through to a child or setup route', () => {
  const state = requireState('/child-runtime');
  const markup = renderToStaticMarkup(createElement(HostedPortalDistribution, { state }));

  expect(state.routeState).toBe('wrong-route');
  expect(markup).toContain('Unsupported hosted parent portal route');
  expect(markup).toContain('/preview, /staging, /production');
  expect(markup).not.toContain('Device controls');
});

it('blocks parent-only controls when auth is missing', () => {
  const state = requireState('/production?auth=missing');
  const markup = renderToStaticMarkup(createElement(HostedPortalDistribution, { state }));

  expect(state.authState).toBe('missing');
  expect(state.controlsEnabled).toBe(false);
  expect(markup).toContain('Parent sign-in is required');
  expect(markup).toContain('data-hosted-action-state="blocked"');
  expect(markup).toContain('Parent release action blocked');
});

it('marks stale cache as non-fresh and blocks fresh install claims', () => {
  const state = requireState('/staging?cache=stale&cacheAgeMinutes=240');
  const markup = renderToStaticMarkup(createElement(HostedPortalDistribution, { state }));

  expect(state.cacheState).toBe('stale');
  expect(state.cacheAgeMinutes).toBe(240);
  expect(state.controlsEnabled).toBe(false);
  expect(markup).toContain('Fresh install or release claims are blocked');
});

it('prevents preview or staging routes from presenting a production release claim', () => {
  const preview = requireState('/preview?release=production');
  const staging = requireState('/staging?release=production');

  expect(preview.productionClaimBlocked).toBe(true);
  expect(staging.productionClaimBlocked).toBe(true);
  expect(renderToStaticMarkup(createElement(HostedPortalDistribution, { state: preview }))).toContain(
    'Production release claim blocked'
  );
});

it('activates hosted distribution from browser pathname routes without a hosted env toggle', () => {
  const preview = resolveHostedPortalDistributionStateFromDomain(
    {
      hash: '',
      origin: 'http://127.0.0.1:4490',
      pathname: '/preview',
      search: '',
    },
    {}
  );
  const wrongRoute = resolveHostedPortalDistributionStateFromDomain(
    {
      hash: '',
      origin: 'http://127.0.0.1:4490',
      pathname: '/child-runtime',
      search: '',
    },
    {}
  );

  expect(preview?.routeState).toBe('matched');
  expect(preview?.environment).toBe('preview');
  expect(wrongRoute?.routeState).toBe('wrong-route');
});

it('keeps the root hash-route shell when hosted distribution mode is disabled', () => {
  expect(
    resolveHostedPortalDistributionStateFromDomain(
      {
        hash: '#/overview',
        origin: 'http://127.0.0.1:4490',
        pathname: '/',
        search: '',
      },
      {}
    )
  ).toBeNull();
});

it('keeps the app resolver wrapper behaviorally aligned with the domain resolver', () => {
  const cases: ReadonlyArray<{
    readonly location: HostedPortalLocation;
    readonly env: HostedPortalEnv;
    readonly defaultNowMinutes?: number;
  }> = [
    {
      location: locationFor('/preview'),
      env: hostedEnv,
      defaultNowMinutes: 4,
    },
    {
      location: locationFor('/child-runtime?auth=missing&cache=stale&cacheAgeMinutes=240&release=production'),
      env: hostedEnv,
      defaultNowMinutes: 9,
    },
    {
      location: locationFor('/staging?auth=missing&cache=stale&cacheAgeMinutes=240&release=production'),
      env: hostedEnv,
      defaultNowMinutes: 9,
    },
    {
      location: {
        hash: '#/overview',
        origin: 'http://127.0.0.1:4490',
        pathname: '/',
        search: '',
      },
      env: {},
    },
  ];

  for (const testCase of cases) {
    expect(
      resolveHostedPortalDistributionStateFromApp(testCase.location, testCase.env, testCase.defaultNowMinutes)
    ).toEqual(
      resolveHostedPortalDistributionStateFromDomain(testCase.location, testCase.env, testCase.defaultNowMinutes)
    );
  }
});

function requireState(pathWithQuery: string) {
  const state = resolveHostedPortalDistributionStateFromDomain(locationFor(pathWithQuery), hostedEnv);

  if (state === null) {
    throw new Error('Expected hosted portal distribution state to be available.');
  }

  return state;
}

function locationFor(pathWithQuery: string): HostedPortalLocation {
  const url = new URL(pathWithQuery, 'http://127.0.0.1:4490');
  return {
    hash: '',
    origin: url.origin,
    pathname: url.pathname,
    search: url.search,
  };
}
