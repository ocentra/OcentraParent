import { describe, expect, it } from 'vitest';

import { resolveHostedPortalDistributionState } from '../../src/hosted-portal-distribution';

const hostedEnv = {
  VITE_OCENTRA_PARENT_HOSTED_PORTAL_MODE: 'hosted',
} as const;

describe('hosted portal distribution', () => {
  it('resolves preview staging and production outside the app shell', () => {
    const preview = requireState('/preview');
    const staging = requireState('/staging');
    const production = requireState('/production');

    expect(preview.environment).toBe('preview');
    expect(staging.environment).toBe('staging');
    expect(production.environment).toBe('production');
    expect(preview.openActionLabel).toBe('Open parent portal preview');
    expect(staging.openActionLabel).toBe('Open parent portal staging');
    expect(production.openActionLabel).toBe('Open parent portal production');
  });

  it('blocks production release claims on preview or staging routes', () => {
    expect(requireState('/preview?release=production').productionClaimBlocked).toBe(true);
    expect(requireState('/staging?release=production').productionClaimBlocked).toBe(true);
  });

  it('keeps the root hash route on the host-bridge shell when hosted mode is disabled', () => {
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
