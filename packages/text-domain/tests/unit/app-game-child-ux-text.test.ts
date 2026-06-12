import { describe, expect, it } from 'vitest';
import { AppGameChildUxText, AppGameChildUxTextToken } from '../../src/app-game-child-ux-text';
import { decodeDisplayText } from '../../src/contracts';

const BannedChildCopyFragments = ['caught', 'dangerous', 'AI blocked', 'C:\\', '/Users/', '.exe'] as const;

describe('app/game child UX text', () => {
  it('exposes calm schema-backed child-facing warning and request copy', () => {
    expect(AppGameChildUxText[AppGameChildUxTextToken.FamilyRuleBody]).toBe(
      'This app is limited by your family rules.'
    );
    expect(AppGameChildUxText[AppGameChildUxTextToken.NewAppBody]).toBe(
      'This new app needs parent approval before you can use it.'
    );
    expect(AppGameChildUxText[AppGameChildUxTextToken.LimitReachedBody]).toBe(
      'This app is blocked right now. You can ask your parent for more time.'
    );
    expect(AppGameChildUxText[AppGameChildUxTextToken.ManualRequiredBody]).toBe(
      'This device needs parent setup before Ocentra can change this app.'
    );
  });

  it('keeps child copy free of blame, AI attribution, and private diagnostics', () => {
    const copyValues = Object.values(AppGameChildUxText);

    for (const copyValue of copyValues) {
      expect(decodeDisplayText(copyValue)).toBe(copyValue);
      for (const bannedFragment of BannedChildCopyFragments) {
        expect(copyValue.includes(bannedFragment)).toBe(false);
      }
    }
  });
});
