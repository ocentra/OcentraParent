import { describe, expect, it } from 'vitest';
import { decodeDisplayText } from '../src/contracts';
import { PortalDevText, PortalDevTextToken } from '../src/portal-dev';

describe('portal dev text', () => {
  it('PortalDevText: exposes schema-backed display text tokens', () => {
    const title = PortalDevText[PortalDevTextToken.AppTitle];
    expect(decodeDisplayText(title)).toBe(title);
    expect(title).toBe('Ocentra Parent');
  });
});
