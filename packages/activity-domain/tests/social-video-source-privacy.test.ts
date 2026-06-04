import { describe, expect, it } from 'vitest';
import {
  buildSocialVideoSourcePrivacySummary,
  SocialVideoSourcePrivacySummarySchema,
} from '../src/social-video-source-privacy';

describe('social video source privacy summary contract', () => {
  it('accepts managed browser, parent-provided, connector, and screen-summary refs only', acceptsTypedRefs);
  it('accepts native and platform manual-required source states without policy use', acceptsManualRequiredStates);
  it('rejects source type and source ref mismatches', rejectsSourceRefMismatches);
  it('rejects arbitrary manual-required reason text', rejectsArbitraryManualReasonText);
  it(
    'rejects raw content, media, connector token, native control, policy, and enforcement claims',
    rejectsForbiddenClaims
  );
});

function acceptsTypedRefs() {
  const summary = buildSocialVideoSourcePrivacySummary(privacyInput());

  expect(summary.schemaVersion).toBe(1);
  expect(summary.platform).toBe('youtube');
  expect(summary.targetKind).toBe('video-url');
  expect(summary.sourceTypes).toEqual([
    'managed-browser-social-route-ref',
    'managed-browser-video-metadata-ref',
    'parent-provided-url-ref',
    'parent-provided-channel-ref',
    'screen-summary-ref',
    'connector-authorization-ref',
  ]);
  expect(summary.permittedDownstreamUses).toEqual([
    'ai-analysis-input',
    'policy-candidate-input',
    'parent-explanation',
    'audit-summary',
  ]);
  expect(summary.rawContentCaptured).toBe(false);
  expect(summary.rawMessageContentCaptured).toBe(false);
  expect(summary.rawVideoCaptured).toBe(false);
  expect(summary.screenshotCaptured).toBe(false);
  expect(summary.connectorTokenStored).toBe(false);
}

function acceptsManualRequiredStates() {
  const summary = buildSocialVideoSourcePrivacySummary(
    privacyInput({
      sourcePrivacyEvidenceId: 'source-privacy-native-manual-required',
      platform: 'tiktok',
      targetKind: 'native-social-app',
      sourceTypes: ['android-native-manual-required'],
      socialRouteEvidenceIds: [],
      socialVideoMetadataEvidenceIds: [],
      parentProvidedUrlRefs: [],
      parentProvidedChannelRefs: [],
      screenSummaryEvidenceRefs: [],
      connectorAuthorizationRefs: [],
      manualRequiredReason: 'native-app-source-unavailable',
      confidence: 'unknown',
      degradedState: 'manual-required',
      permittedDownstreamUses: ['manual-review', 'audit-summary'],
    })
  );

  expect(summary.degradedState).toBe('manual-required');
  expect(summary.confidence).toBe('unknown');
  expect(summary.permittedDownstreamUses).toEqual(['manual-review', 'audit-summary']);
}

function rejectsSourceRefMismatches() {
  const invalidRows = [
    privacyInput({ sourceTypes: ['managed-browser-social-route-ref'], socialRouteEvidenceIds: [] }),
    privacyInput({ sourceTypes: ['screen-summary-ref'], screenSummaryEvidenceRefs: [] }),
    privacyInput({ sourceTypes: ['connector-authorization-ref'], connectorAuthorizationRefs: [] }),
    privacyInput({
      sourceTypes: ['android-native-manual-required'],
      manualRequiredReason: null,
      confidence: 'unknown',
      degradedState: 'manual-required',
    }),
    privacyInput({ sourceTypes: ['android-native-manual-required'], confidence: 'medium', degradedState: 'none' }),
    privacyInput({
      degradedState: 'manual-required',
      confidence: 'unknown',
      manualRequiredReason: 'parent-review-required',
    }),
  ];

  for (const invalid of invalidRows) {
    expect(SocialVideoSourcePrivacySummarySchema.safeParse(summaryCandidate(invalid)).success).toBe(false);
  }
}

function rejectsArbitraryManualReasonText() {
  const invalidReasons = ['needs manual review because child typed it', 'raw platform reason text'];

  for (const manualRequiredReason of invalidReasons) {
    expect(
      SocialVideoSourcePrivacySummarySchema.safeParse(
        summaryCandidate({
          sourceTypes: ['android-native-manual-required'],
          socialRouteEvidenceIds: [],
          socialVideoMetadataEvidenceIds: [],
          parentProvidedUrlRefs: [],
          parentProvidedChannelRefs: [],
          screenSummaryEvidenceRefs: [],
          connectorAuthorizationRefs: [],
          manualRequiredReason,
          confidence: 'unknown',
          degradedState: 'manual-required',
          permittedDownstreamUses: ['manual-review', 'audit-summary'],
        })
      ).success
    ).toBe(false);
  }
}

function rejectsForbiddenClaims() {
  const valid = buildSocialVideoSourcePrivacySummary(privacyInput());
  const invalidRows = [
    { ...valid, rawContentCaptured: true },
    { ...valid, rawMessageContentCaptured: true },
    { ...valid, rawVideoCaptured: true },
    { ...valid, screenshotCaptured: true },
    { ...valid, connectorTokenStored: true },
    { ...valid, connectorApiCalled: true },
    { ...valid, nativeAppControlClaimed: true },
    { ...valid, finalPolicyDecisionClaimed: true },
    { ...valid, enforcementClaimed: true },
  ];

  for (const invalid of invalidRows) {
    expect(SocialVideoSourcePrivacySummarySchema.safeParse(invalid).success).toBe(false);
  }
}

function privacyInput(overrides = {}) {
  return {
    sourcePrivacyEvidenceId: 'source-privacy-youtube-homework-video',
    summarizedAt: '2026-06-04T01:44:00.000Z',
    childProfileRef: 'child-profile-middle-school',
    deviceId: 'device-managed-laptop',
    sourceEvidenceIds: [
      'social-route-evidence-youtube-video',
      'social-video-metadata-youtube-video',
      'screen-summary-youtube-video-ref',
    ],
    platform: 'youtube',
    targetKind: 'video-url',
    sourceTypes: [
      'managed-browser-social-route-ref',
      'managed-browser-video-metadata-ref',
      'parent-provided-url-ref',
      'parent-provided-channel-ref',
      'screen-summary-ref',
      'connector-authorization-ref',
    ],
    socialRouteEvidenceIds: ['social-route-evidence-youtube-video'],
    socialVideoMetadataEvidenceIds: ['social-video-metadata-youtube-video'],
    parentProvidedUrlRefs: ['parent-provided-url-ref-youtube-video'],
    parentProvidedChannelRefs: ['parent-provided-channel-ref-teacher'],
    screenSummaryEvidenceRefs: ['screen-summary-youtube-video-ref'],
    connectorAuthorizationRefs: ['connector-authorization-ref-youtube-supervision'],
    manualRequiredReason: null,
    custodyLabel: 'child-device-local',
    confidence: 'medium',
    degradedState: 'none',
    permittedDownstreamUses: ['ai-analysis-input', 'policy-candidate-input', 'parent-explanation', 'audit-summary'],
    ...overrides,
  };
}

function summaryCandidate(overrides = {}) {
  return {
    ...privacyInput(overrides),
    schemaVersion: 1,
    rawContentCaptured: false,
    rawMessageContentCaptured: false,
    rawVideoCaptured: false,
    screenshotCaptured: false,
    connectorTokenStored: false,
    connectorApiCalled: false,
    nativeAppControlClaimed: false,
    finalPolicyDecisionClaimed: false,
    enforcementClaimed: false,
  };
}
