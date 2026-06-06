import { describe, expect, it } from 'vitest';
import {
  SocialDashboardUxText,
  SocialDashboardUxTextToken,
  resolveSocialDashboardUxText,
} from '../src/social-dashboard-ux-text';
import { decodeDisplayText } from '../src/contracts';

const BannedSocialDashboardCopyFragments = [
  'watching',
  'surveillance',
  'secret',
  'caught',
  'guaranteed block',
  'connector connected',
] as const;

describe('social dashboard UX text', () => {
  it('exposes schema-backed parent social dashboard copy tokens', () => {
    for (const token of Object.values(SocialDashboardUxTextToken)) {
      const text = resolveSocialDashboardUxText(token);

      expect(decodeDisplayText(text)).toBe(text);
      for (const bannedFragment of BannedSocialDashboardCopyFragments) {
        expect(text.includes(bannedFragment)).toBe(false);
      }
    }
  });

  it('keeps parent dashboard labels aligned with the social proof boundary', () => {
    expect(SocialDashboardUxText[SocialDashboardUxTextToken.Title]).toBe('Social review');
    expect(SocialDashboardUxText[SocialDashboardUxTextToken.NativeAppCapability]).toBe('Native app capability');
    expect(SocialDashboardUxText[SocialDashboardUxTextToken.SettingsCustody]).toBe('Settings and custody');
    expect(SocialDashboardUxText[SocialDashboardUxTextToken.ManualRequiredStatus]).toBe('Manual proof required');
    expect(SocialDashboardUxText[SocialDashboardUxTextToken.ContractOnlyStatus]).toBe('Contract proof only');
  });
});
