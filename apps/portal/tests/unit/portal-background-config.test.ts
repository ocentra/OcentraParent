import { describe, expect, it } from 'vitest';
import {
  DEFAULT_PORTAL_BACKGROUND_CONFIG,
  normalizePortalBackgroundConfig,
} from '@ocentra-parent/portal-domain/portal-background';
import { readDefaultPortalBackgroundConfig } from '../../src/portal-background-config';

describe('portal background configuration', () => {
  it('uses the domain default before the runtime static asset is loaded', () => {
    expect(readDefaultPortalBackgroundConfig()).toEqual(
      normalizePortalBackgroundConfig(DEFAULT_PORTAL_BACKGROUND_CONFIG)
    );
  });
});
