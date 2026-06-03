import { describe, expect, it } from 'vitest';
import {
  BrowserParentExplanationText,
  BrowserParentExplanationTextToken,
  resolveBrowserParentExplanationText,
} from '../src/browser-parent-explanation';
import { decodeDisplayText } from '../src/contracts';

describe('browser parent explanation text', () => {
  it('exposes schema-backed parent explanation text tokens', exposesParentExplanationText);
});

function exposesParentExplanationText() {
  for (const token of Object.values(BrowserParentExplanationTextToken)) {
    const text = resolveBrowserParentExplanationText(token);

    expect(decodeDisplayText(text)).toBe(text);
  }

  expect(BrowserParentExplanationText[BrowserParentExplanationTextToken.Title]).toBe('Browser review');
  expect(BrowserParentExplanationText[BrowserParentExplanationTextToken.Audit]).toBe('Audit trail');
}
