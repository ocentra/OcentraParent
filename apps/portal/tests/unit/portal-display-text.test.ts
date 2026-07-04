import { describe, expect, it } from 'vitest';
import { PortalDevTextToken, resolvePortalDevText } from '../../src/portal-dev-text';
import { decodeDisplayText } from '../../src/portal-display-text';

describe('portal display text helpers', () => {
  it('rejects empty display text and preserves portal dev text tokens', () => {
    expect(() => decodeDisplayText('')).toThrow('DisplayText must be non-empty');
    expect(resolvePortalDevText(PortalDevTextToken.ParentPortal)).toBe('Start here');
    expect(resolvePortalDevText(PortalDevTextToken.ProductSurfacePending)).toBe(
      'No family setting is configured for this area yet.'
    );
  });
});
