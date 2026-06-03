import { decodeDisplayText, decodeTextTokenId, type DisplayText } from './contracts';

export const AppGameChildUxTextToken = {
  FamilyRuleTitle: decodeTextTokenId('appGame.childUx.familyRule.title'),
  FamilyRuleBody: decodeTextTokenId('appGame.childUx.familyRule.body'),
  NewAppTitle: decodeTextTokenId('appGame.childUx.newApp.title'),
  NewAppBody: decodeTextTokenId('appGame.childUx.newApp.body'),
  AlmostFinishedTitle: decodeTextTokenId('appGame.childUx.almostFinished.title'),
  AlmostFinishedBody: decodeTextTokenId('appGame.childUx.almostFinished.body'),
  LimitReachedTitle: decodeTextTokenId('appGame.childUx.limitReached.title'),
  LimitReachedBody: decodeTextTokenId('appGame.childUx.limitReached.body'),
  ManualRequiredTitle: decodeTextTokenId('appGame.childUx.manualRequired.title'),
  ManualRequiredBody: decodeTextTokenId('appGame.childUx.manualRequired.body'),
  UnavailableTitle: decodeTextTokenId('appGame.childUx.unavailable.title'),
  UnavailableBody: decodeTextTokenId('appGame.childUx.unavailable.body'),
  RequestSubmittedTitle: decodeTextTokenId('appGame.childUx.requestSubmitted.title'),
  RequestSubmittedBody: decodeTextTokenId('appGame.childUx.requestSubmitted.body'),
  RequestApprovedTitle: decodeTextTokenId('appGame.childUx.requestApproved.title'),
  RequestApprovedBody: decodeTextTokenId('appGame.childUx.requestApproved.body'),
  RequestDeniedTitle: decodeTextTokenId('appGame.childUx.requestDenied.title'),
  RequestDeniedBody: decodeTextTokenId('appGame.childUx.requestDenied.body'),
  AskParentAction: decodeTextTokenId('appGame.childUx.action.askParent'),
  RequestMoreTimeAction: decodeTextTokenId('appGame.childUx.action.requestMoreTime'),
  DismissAction: decodeTextTokenId('appGame.childUx.action.dismiss'),
  TryLaterAction: decodeTextTokenId('appGame.childUx.action.tryLater'),
  ContinueApprovedAction: decodeTextTokenId('appGame.childUx.action.continueApproved'),
} as const;

export type AppGameChildUxTextTokenValue = (typeof AppGameChildUxTextToken)[keyof typeof AppGameChildUxTextToken];

export const AppGameChildUxText: Record<AppGameChildUxTextTokenValue, DisplayText> = {
  [AppGameChildUxTextToken.FamilyRuleTitle]: decodeDisplayText('App time is limited'),
  [AppGameChildUxTextToken.FamilyRuleBody]: decodeDisplayText('This app is limited by your family rules.'),
  [AppGameChildUxTextToken.NewAppTitle]: decodeDisplayText('Parent approval needed'),
  [AppGameChildUxTextToken.NewAppBody]: decodeDisplayText('This new app needs parent approval before you can use it.'),
  [AppGameChildUxTextToken.AlmostFinishedTitle]: decodeDisplayText('Time is almost finished'),
  [AppGameChildUxTextToken.AlmostFinishedBody]: decodeDisplayText('Your app time is almost finished.'),
  [AppGameChildUxTextToken.LimitReachedTitle]: decodeDisplayText('Time limit reached'),
  [AppGameChildUxTextToken.LimitReachedBody]: decodeDisplayText(
    'This app is blocked right now. You can ask your parent for more time.'
  ),
  [AppGameChildUxTextToken.ManualRequiredTitle]: decodeDisplayText('Ask your parent for help'),
  [AppGameChildUxTextToken.ManualRequiredBody]: decodeDisplayText(
    'This device needs parent setup before Ocentra can change this app.'
  ),
  [AppGameChildUxTextToken.UnavailableTitle]: decodeDisplayText('Not available right now'),
  [AppGameChildUxTextToken.UnavailableBody]: decodeDisplayText('This app is not available on this device right now.'),
  [AppGameChildUxTextToken.RequestSubmittedTitle]: decodeDisplayText('Request sent'),
  [AppGameChildUxTextToken.RequestSubmittedBody]: decodeDisplayText('Your parent can review this request.'),
  [AppGameChildUxTextToken.RequestApprovedTitle]: decodeDisplayText('Approved'),
  [AppGameChildUxTextToken.RequestApprovedBody]: decodeDisplayText('Your parent approved this request.'),
  [AppGameChildUxTextToken.RequestDeniedTitle]: decodeDisplayText('Not approved'),
  [AppGameChildUxTextToken.RequestDeniedBody]: decodeDisplayText('Your parent did not approve this request.'),
  [AppGameChildUxTextToken.AskParentAction]: decodeDisplayText('Ask parent'),
  [AppGameChildUxTextToken.RequestMoreTimeAction]: decodeDisplayText('Request more time'),
  [AppGameChildUxTextToken.DismissAction]: decodeDisplayText('OK'),
  [AppGameChildUxTextToken.TryLaterAction]: decodeDisplayText('Try later'),
  [AppGameChildUxTextToken.ContinueApprovedAction]: decodeDisplayText('Continue'),
};
