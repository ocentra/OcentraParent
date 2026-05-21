import { describe, expect, it } from 'vitest';
import { decodeDisplayText } from '../src/contracts';
import { PortalDevText, PortalDevTextToken } from '../src/portal-dev';

describe('portal dev text', () => {
  it('PortalDevText: exposes schema-backed display text tokens', () => {
    const title = PortalDevText[PortalDevTextToken.AppTitle];
    expect(decodeDisplayText(title)).toBe(title);
    expect(title).toBe('Ocentra Parent');
    expect(PortalDevText[PortalDevTextToken.LiveActivity]).toBe('Live activity');
    expect(PortalDevText[PortalDevTextToken.RecentActivity]).toBe('Recent activity');
    expect(PortalDevText[PortalDevTextToken.PolicyPreview]).toBe('Policy preview');
    expect(PortalDevText[PortalDevTextToken.PolicyPreviewNoEnforcement]).toBe('Enforcement disabled; preview only.');
    expect(PortalDevText[PortalDevTextToken.CommandResult]).toBe('Command result');
    expect(PortalDevText[PortalDevTextToken.CopyResult]).toBe('Copy result');
  });
});
