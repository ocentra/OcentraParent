import { describe, expect, it } from 'vitest';
import { BrowserChildUxText, BrowserChildUxTextToken, resolveBrowserChildUxText } from '../../src/browser-child-ux';
import { decodeDisplayText } from '../../src/contracts';

describe('browser child UX text', () => {
  it('exposes calm schema-backed browser child UX copy', exposesCalmChildCopy);
  it('keeps checking copy aligned with the browser AI plan', keepsCheckingCopyAligned);
});

function exposesCalmChildCopy() {
  const unsafeTerms = /\b(watched|surveillance|caught|shame|bad kid)\b/i;

  for (const token of Object.values(BrowserChildUxTextToken)) {
    const text = resolveBrowserChildUxText(token);

    expect(decodeDisplayText(text)).toBe(text);
    expect(text).not.toMatch(unsafeTerms);
  }
}

function keepsCheckingCopyAligned() {
  expect(BrowserChildUxText[BrowserChildUxTextToken.Checking]).toBe(
    'Ocentra is checking whether this page matches your family rules.'
  );
  expect(BrowserChildUxText[BrowserChildUxTextToken.Blocked]).toBe('This page is blocked by your parent rule.');
  expect(BrowserChildUxText[BrowserChildUxTextToken.Unclassified]).toBe('This page could not be classified.');
}
