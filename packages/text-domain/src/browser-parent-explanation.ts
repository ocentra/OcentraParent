import { decodeDisplayText, decodeTextTokenId, type DisplayText } from './contracts';
import { resolveDisplayTextFromMap } from './text-family';

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
