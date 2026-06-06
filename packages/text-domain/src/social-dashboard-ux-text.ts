import { decodeDisplayText, decodeTextTokenId, type DisplayText } from './contracts';

export const SocialDashboardUxTextToken = {
  Title: decodeTextTokenId('browser.social.dashboard.title'),
  AccountApprovals: decodeTextTokenId('browser.social.dashboard.section.accountApprovals'),
  FeedVideoGates: decodeTextTokenId('browser.social.dashboard.section.feedVideoGates'),
  NativeAppCapability: decodeTextTokenId('browser.social.dashboard.section.nativeAppCapability'),
  ConnectorBoundaries: decodeTextTokenId('browser.social.dashboard.section.connectorBoundaries'),
  DecisionMemory: decodeTextTokenId('browser.social.dashboard.section.decisionMemory'),
  SettingsCustody: decodeTextTokenId('browser.social.dashboard.section.settingsCustody'),
  ManualRequiredGaps: decodeTextTokenId('browser.social.dashboard.section.manualRequiredGaps'),
  ReadyForReviewStatus: decodeTextTokenId('browser.social.dashboard.status.readyForReview'),
  ManualRequiredStatus: decodeTextTokenId('browser.social.dashboard.status.manualRequired'),
  ContractOnlyStatus: decodeTextTokenId('browser.social.dashboard.status.contractOnly'),
  UnavailableStatus: decodeTextTokenId('browser.social.dashboard.status.unavailable'),
  OpenParentApprovalAction: decodeTextTokenId('browser.social.dashboard.action.openParentApproval'),
  ReviewFeedGateAction: decodeTextTokenId('browser.social.dashboard.action.reviewFeedGate'),
  ReviewNativeCapabilityAction: decodeTextTokenId('browser.social.dashboard.action.reviewNativeCapability'),
  ReviewConnectorBoundaryAction: decodeTextTokenId('browser.social.dashboard.action.reviewConnectorBoundary'),
  ReviewDecisionMemoryAction: decodeTextTokenId('browser.social.dashboard.action.reviewDecisionMemory'),
  ReviewSettingsCustodyAction: decodeTextTokenId('browser.social.dashboard.action.reviewSettingsCustody'),
  ManualReviewAction: decodeTextTokenId('browser.social.dashboard.action.manualReview'),
} as const;

export type SocialDashboardUxTextTokenValue =
  (typeof SocialDashboardUxTextToken)[keyof typeof SocialDashboardUxTextToken];

export const SocialDashboardUxText: Record<SocialDashboardUxTextTokenValue, DisplayText> = {
  [SocialDashboardUxTextToken.Title]: decodeDisplayText('Social review'),
  [SocialDashboardUxTextToken.AccountApprovals]: decodeDisplayText('Account approvals'),
  [SocialDashboardUxTextToken.FeedVideoGates]: decodeDisplayText('Feed and video route gates'),
  [SocialDashboardUxTextToken.NativeAppCapability]: decodeDisplayText('Native app capability'),
  [SocialDashboardUxTextToken.ConnectorBoundaries]: decodeDisplayText('Connected account boundaries'),
  [SocialDashboardUxTextToken.DecisionMemory]: decodeDisplayText('Remembered decisions'),
  [SocialDashboardUxTextToken.SettingsCustody]: decodeDisplayText('Settings and custody'),
  [SocialDashboardUxTextToken.ManualRequiredGaps]: decodeDisplayText('Needs manual proof'),
  [SocialDashboardUxTextToken.ReadyForReviewStatus]: decodeDisplayText('Ready for parent review'),
  [SocialDashboardUxTextToken.ManualRequiredStatus]: decodeDisplayText('Manual proof required'),
  [SocialDashboardUxTextToken.ContractOnlyStatus]: decodeDisplayText('Contract proof only'),
  [SocialDashboardUxTextToken.UnavailableStatus]: decodeDisplayText('Unavailable on this surface'),
  [SocialDashboardUxTextToken.OpenParentApprovalAction]: decodeDisplayText('Review approval'),
  [SocialDashboardUxTextToken.ReviewFeedGateAction]: decodeDisplayText('Review route gate'),
  [SocialDashboardUxTextToken.ReviewNativeCapabilityAction]: decodeDisplayText('Review device capability'),
  [SocialDashboardUxTextToken.ReviewConnectorBoundaryAction]: decodeDisplayText('Review connected account boundary'),
  [SocialDashboardUxTextToken.ReviewDecisionMemoryAction]: decodeDisplayText('Review remembered decision'),
  [SocialDashboardUxTextToken.ReviewSettingsCustodyAction]: decodeDisplayText('Review settings custody'),
  [SocialDashboardUxTextToken.ManualReviewAction]: decodeDisplayText('Manual review'),
};

const MissingSocialDashboardUxTextTokenMessage = decodeDisplayText('Missing social dashboard UX text token.');

export function resolveSocialDashboardUxText(token: SocialDashboardUxTextTokenValue): DisplayText {
  const text = SocialDashboardUxText[token];
  if (text === undefined) {
    throw new Error(MissingSocialDashboardUxTextTokenMessage);
  }
  return text;
}
