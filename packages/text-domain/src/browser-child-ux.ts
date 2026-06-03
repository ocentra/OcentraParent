import { decodeDisplayText, decodeTextTokenId, type DisplayText } from './contracts';

export const BrowserChildUxTextToken = {
  Opening: decodeTextTokenId('browser.child.opening.title'),
  Checking: decodeTextTokenId('browser.child.checking.title'),
  Allowed: decodeTextTokenId('browser.child.allowed.title'),
  Warning: decodeTextTokenId('browser.child.warning.title'),
  Approval: decodeTextTokenId('browser.child.approval.title'),
  Limited: decodeTextTokenId('browser.child.limited.title'),
  Blocked: decodeTextTokenId('browser.child.blocked.title'),
  Unclassified: decodeTextTokenId('browser.child.unclassified.title'),
  Manual: decodeTextTokenId('browser.child.manual.title'),
  Unavailable: decodeTextTokenId('browser.child.unavailable.title'),
} as const;

export type BrowserChildUxTextTokenValue = (typeof BrowserChildUxTextToken)[keyof typeof BrowserChildUxTextToken];

export const BrowserChildUxText: Record<BrowserChildUxTextTokenValue, DisplayText> = {
  [BrowserChildUxTextToken.Opening]: decodeDisplayText('Opening this page.'),
  [BrowserChildUxTextToken.Checking]: decodeDisplayText(
    'Ocentra is checking whether this page matches your family rules.'
  ),
  [BrowserChildUxTextToken.Allowed]: decodeDisplayText('This looks educational and is allowed.'),
  [BrowserChildUxTextToken.Warning]: decodeDisplayText('This page may need a different choice right now.'),
  [BrowserChildUxTextToken.Approval]: decodeDisplayText('This video needs parent approval.'),
  [BrowserChildUxTextToken.Limited]: decodeDisplayText('This site is limited right now.'),
  [BrowserChildUxTextToken.Blocked]: decodeDisplayText('This page is blocked by your parent rule.'),
  [BrowserChildUxTextToken.Unclassified]: decodeDisplayText('This page could not be classified.'),
  [BrowserChildUxTextToken.Manual]: decodeDisplayText('A parent needs to choose what happens next.'),
  [BrowserChildUxTextToken.Unavailable]: decodeDisplayText('The browser check is not available right now.'),
};

const MissingBrowserChildUxTextTokenMessage = decodeDisplayText('Missing browser child UX text token.');

export function resolveBrowserChildUxText(token: BrowserChildUxTextTokenValue): DisplayText {
  const text = BrowserChildUxText[token];
  if (text === undefined) {
    throw new Error(MissingBrowserChildUxTextTokenMessage);
  }
  return text;
}
