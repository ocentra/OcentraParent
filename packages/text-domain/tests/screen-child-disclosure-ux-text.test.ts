import { describe, expect, it } from 'vitest';
import {
  ScreenChildDisclosureUxText,
  ScreenChildDisclosureUxTextToken,
  resolveScreenChildDisclosureUxText,
} from '../src/screen-child-disclosure-ux-text';
import { decodeDisplayText } from '../src/contracts';

const BannedScreenDisclosureCopy = /\b(caught|secret|hidden|surveillance|watched|bad kid|raw screenshot|C:\\|\/Users\/)\b/i;

describe('screen child disclosure UX text', () => {
  it('exposes calm schema-backed child-facing screen disclosure copy', () => {
    expect(ScreenChildDisclosureUxText[ScreenChildDisclosureUxTextToken.Title]).toBe('Screen check status');
    expect(ScreenChildDisclosureUxText[ScreenChildDisclosureUxTextToken.ActiveStatus]).toBe('Running locally');
    expect(ScreenChildDisclosureUxText[ScreenChildDisclosureUxTextToken.ProtectedStatus]).toBe(
      'Skipped protected screen'
    );
  });

  it('keeps disclosure copy free of blame, hidden-capture language, and private diagnostics', () => {
    for (const token of Object.values(ScreenChildDisclosureUxTextToken)) {
      const text = resolveScreenChildDisclosureUxText(token);

      expect(decodeDisplayText(text)).toBe(text);
      expect(text).not.toMatch(BannedScreenDisclosureCopy);
    }
  });
});
