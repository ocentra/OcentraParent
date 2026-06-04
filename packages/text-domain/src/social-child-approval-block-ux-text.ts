import { decodeDisplayText, decodeTextTokenId, type DisplayText } from './contracts';

export const SocialChildApprovalBlockUxTextToken = {
  ApprovalPendingTitle: decodeTextTokenId('browser.social.child.approvalPending.title'),
  ApprovalPendingBody: decodeTextTokenId('browser.social.child.approvalPending.body'),
  BlockedRouteTitle: decodeTextTokenId('browser.social.child.blockedRoute.title'),
  BlockedRouteBody: decodeTextTokenId('browser.social.child.blockedRoute.body'),
  WarningRouteTitle: decodeTextTokenId('browser.social.child.warningRoute.title'),
  WarningRouteBody: decodeTextTokenId('browser.social.child.warningRoute.body'),
  ManualReviewTitle: decodeTextTokenId('browser.social.child.manualReview.title'),
  ManualReviewBody: decodeTextTokenId('browser.social.child.manualReview.body'),
  TimeLimitTitle: decodeTextTokenId('browser.social.child.timeLimit.title'),
  TimeLimitBody: decodeTextTokenId('browser.social.child.timeLimit.body'),
  NativeUnavailableTitle: decodeTextTokenId('browser.social.child.nativeUnavailable.title'),
  NativeUnavailableBody: decodeTextTokenId('browser.social.child.nativeUnavailable.body'),
  WaitForParentAction: decodeTextTokenId('browser.social.child.action.waitForParent'),
  AcknowledgeWarningAction: decodeTextTokenId('browser.social.child.action.acknowledgeWarning'),
  SafeBackAction: decodeTextTokenId('browser.social.child.action.safeBack'),
  ManualReviewAction: decodeTextTokenId('browser.social.child.action.manualReview'),
  NoAction: decodeTextTokenId('browser.social.child.action.noAction'),
} as const;

export type SocialChildApprovalBlockUxTextTokenValue =
  (typeof SocialChildApprovalBlockUxTextToken)[keyof typeof SocialChildApprovalBlockUxTextToken];

export const SocialChildApprovalBlockUxText: Record<SocialChildApprovalBlockUxTextTokenValue, DisplayText> = {
  [SocialChildApprovalBlockUxTextToken.ApprovalPendingTitle]: decodeDisplayText('Parent approval needed'),
  [SocialChildApprovalBlockUxTextToken.ApprovalPendingBody]: decodeDisplayText(
    'A parent needs to review this social account step before you continue.'
  ),
  [SocialChildApprovalBlockUxTextToken.BlockedRouteTitle]: decodeDisplayText('This social route is blocked'),
  [SocialChildApprovalBlockUxTextToken.BlockedRouteBody]: decodeDisplayText(
    'This route is limited by your family rules right now.'
  ),
  [SocialChildApprovalBlockUxTextToken.WarningRouteTitle]: decodeDisplayText('Check this choice first'),
  [SocialChildApprovalBlockUxTextToken.WarningRouteBody]: decodeDisplayText(
    'This social route may need a different choice right now.'
  ),
  [SocialChildApprovalBlockUxTextToken.ManualReviewTitle]: decodeDisplayText('Ask your parent for help'),
  [SocialChildApprovalBlockUxTextToken.ManualReviewBody]: decodeDisplayText(
    'A parent needs to review this social route before Ocentra can choose the next step.'
  ),
  [SocialChildApprovalBlockUxTextToken.TimeLimitTitle]: decodeDisplayText('Social time may be limited'),
  [SocialChildApprovalBlockUxTextToken.TimeLimitBody]: decodeDisplayText(
    'This social route may be limited by your family schedule.'
  ),
  [SocialChildApprovalBlockUxTextToken.NativeUnavailableTitle]: decodeDisplayText('Not available in this app'),
  [SocialChildApprovalBlockUxTextToken.NativeUnavailableBody]: decodeDisplayText(
    'Ocentra needs more device proof before it can change this native social app.'
  ),
  [SocialChildApprovalBlockUxTextToken.WaitForParentAction]: decodeDisplayText('Wait for parent'),
  [SocialChildApprovalBlockUxTextToken.AcknowledgeWarningAction]: decodeDisplayText('OK'),
  [SocialChildApprovalBlockUxTextToken.SafeBackAction]: decodeDisplayText('Go back'),
  [SocialChildApprovalBlockUxTextToken.ManualReviewAction]: decodeDisplayText('Ask parent'),
  [SocialChildApprovalBlockUxTextToken.NoAction]: decodeDisplayText('No action available'),
};

const MissingSocialChildApprovalBlockUxTextTokenMessage = decodeDisplayText(
  'Missing social child approval block UX text token.'
);

export function resolveSocialChildApprovalBlockUxText(token: SocialChildApprovalBlockUxTextTokenValue): DisplayText {
  const text = SocialChildApprovalBlockUxText[token];
  if (text === undefined) {
    throw new Error(MissingSocialChildApprovalBlockUxTextTokenMessage);
  }
  return text;
}
