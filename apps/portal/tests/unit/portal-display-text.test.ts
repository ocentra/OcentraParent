import { describe, expect, it } from 'vitest';
import { PortalDevTextToken, resolvePortalDevText } from '@ocentra-parent/portal-domain/display-text';
import { decodeDisplayText } from '@ocentra-parent/portal-domain/display-text';

describe('portal display text helpers', () => {
  it('rejects empty display text and preserves portal dev text tokens', () => {
    expect(() => decodeDisplayText('')).toThrow('DisplayText: expected a non-empty string');
    expect(resolvePortalDevText(PortalDevTextToken.ParentPortal)).toBe('Start here');
    expect(resolvePortalDevText(PortalDevTextToken.ProductSurfacePending)).toBe(
      'No family setting is configured for this area yet.'
    );
  });
});
