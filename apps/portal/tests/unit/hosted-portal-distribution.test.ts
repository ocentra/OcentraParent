import { createElement } from 'react';
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { renderToStaticMarkup } from 'react-dom/server';
import { expect, it } from 'vitest';

import { HostedPortalDistribution } from '../../src/hosted-portal-distribution';
import { resolveHostedPortalDistributionState } from '@ocentra-parent/portal-domain/hosted-portal-distribution';

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
  const preview = resolveHostedPortalDistributionState(
    {
      hash: '',
      origin: 'http://127.0.0.1:4490',
      pathname: '/preview',
      search: '',
    },
    {}
  );
  const wrongRoute = resolveHostedPortalDistributionState(
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
    resolveHostedPortalDistributionState(
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

it('keeps deterministic hosted distribution decisions out of the app shell', () => {
  const source = readFileSync(
    resolve(import.meta.dirname, '..', '..', 'src', 'hosted-portal-distribution.tsx'),
    'utf8'
  );

  expect(source).toContain('@ocentra-parent/portal-domain/hosted-portal-distribution');
  expect(source).not.toContain('function environmentForPath(');
  expect(source).not.toContain('function resolveRequestedRelease(');
  expect(source).not.toContain('function resolveCacheAgeMinutes(');
  expect(source).not.toContain('function openActionLabel(');
  expect(source).not.toContain('function shouldRenderHostedPortalDistribution(');
});

function requireState(pathWithQuery: string) {
  const url = new URL(pathWithQuery, 'http://127.0.0.1:4490');
  const state = resolveHostedPortalDistributionState(
    {
      hash: '',
      origin: url.origin,
      pathname: url.pathname,
      search: url.search,
    },
    hostedEnv
  );

  if (state === null) {
    throw new Error('Expected hosted portal distribution state to be available.');
  }

  return state;
}
