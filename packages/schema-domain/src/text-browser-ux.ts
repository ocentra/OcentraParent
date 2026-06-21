import { decodeDisplayText, decodeTextTokenId, type DisplayText } from './text-contracts';

function resolveDisplayTextFromMap<Token extends string>(
  textMap: Readonly<Record<Token, DisplayText>>,
  token: Token,
  missingMessage: DisplayText
): DisplayText {
  const text = textMap[token];
  if (text === undefined) {
    throw new Error(missingMessage);
  }
  return text;
}

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
  return resolveDisplayTextFromMap(BrowserChildUxText, token, MissingBrowserChildUxTextTokenMessage);
}

export const BrowserParentExplanationTextToken = {
  Title: decodeTextTokenId('browser.parent.explanation.title'),
  Summary: decodeTextTokenId('browser.parent.explanation.summary'),
  Evidence: decodeTextTokenId('browser.parent.explanation.evidence'),
  Ai: decodeTextTokenId('browser.parent.explanation.ai'),
  Policy: decodeTextTokenId('browser.parent.explanation.policy'),
  Action: decodeTextTokenId('browser.parent.explanation.action'),
  ChildExperience: decodeTextTokenId('browser.parent.explanation.childExperience'),
  Degraded: decodeTextTokenId('browser.parent.explanation.degraded'),
  Audit: decodeTextTokenId('browser.parent.explanation.audit'),
} as const;

export type BrowserParentExplanationTextTokenValue =
  (typeof BrowserParentExplanationTextToken)[keyof typeof BrowserParentExplanationTextToken];

export const BrowserParentExplanationText: Record<BrowserParentExplanationTextTokenValue, DisplayText> = {
  [BrowserParentExplanationTextToken.Title]: decodeDisplayText('Browser review'),
  [BrowserParentExplanationTextToken.Summary]: decodeDisplayText('What happened'),
  [BrowserParentExplanationTextToken.Evidence]: decodeDisplayText('Evidence used'),
  [BrowserParentExplanationTextToken.Ai]: decodeDisplayText('AI and model details'),
  [BrowserParentExplanationTextToken.Policy]: decodeDisplayText('Rule matched'),
  [BrowserParentExplanationTextToken.Action]: decodeDisplayText('Action taken'),
  [BrowserParentExplanationTextToken.ChildExperience]: decodeDisplayText('What the child saw'),
  [BrowserParentExplanationTextToken.Degraded]: decodeDisplayText('Fallback or manual review needed'),
  [BrowserParentExplanationTextToken.Audit]: decodeDisplayText('Audit trail'),
};

const MissingBrowserParentExplanationTextTokenMessage = decodeDisplayText(
  'Missing browser parent explanation text token.'
);

export function resolveBrowserParentExplanationText(token: BrowserParentExplanationTextTokenValue): DisplayText {
  return resolveDisplayTextFromMap(
    BrowserParentExplanationText,
    token,
    MissingBrowserParentExplanationTextTokenMessage
  );
}
