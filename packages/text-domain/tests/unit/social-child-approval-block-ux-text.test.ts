import { describe, expect, it } from 'vitest';
import {
  SocialChildApprovalBlockUxText,
  SocialChildApprovalBlockUxTextToken,
  resolveSocialChildApprovalBlockUxText,
} from '@ocentra-parent/schema-domain/text-social-ux';
import { decodeDisplayText } from '@ocentra-parent/schema-domain/text-contracts';

const BannedSocialChildCopyFragments = [
  'watched',
  'surveillance',
  'caught',
  'bad kid',
  'AI blocked',
  'secret',
  'credential',
  'message content',
] as const;

describe('social child approval block UX text', () => {
  it('exposes calm schema-backed child social approval and block copy', () => {
    for (const token of Object.values(SocialChildApprovalBlockUxTextToken)) {
      const text = resolveSocialChildApprovalBlockUxText(token);

      expect(decodeDisplayText(text)).toBe(text);
      for (const bannedFragment of BannedSocialChildCopyFragments) {
        expect(text.includes(bannedFragment)).toBe(false);
      }
    }
  });

  it('keeps child-facing copy aligned with manual-required social proof', () => {
    expect(SocialChildApprovalBlockUxText[SocialChildApprovalBlockUxTextToken.ApprovalPendingBody]).toBe(
      'A parent needs to review this social account step before you continue.'
    );
    expect(SocialChildApprovalBlockUxText[SocialChildApprovalBlockUxTextToken.BlockedRouteBody]).toBe(
      'This route is limited by your family rules right now.'
    );
    expect(SocialChildApprovalBlockUxText[SocialChildApprovalBlockUxTextToken.NativeUnavailableBody]).toBe(
      'Ocentra needs more device proof before it can change this native social app.'
    );
  });
});
