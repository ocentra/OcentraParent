import { describe, expect, it } from 'vitest';
import {
  buildSocialVideoSourceCustodySettings,
  SocialVideoSourceCustodySettingsSchema,
} from '../../src/social-video-source-custody-settings';

const RequiredNoClaimLabels = [
  'raw-message-content-not-allowed',
  'raw-video-content-not-allowed',
  'screenshot-custody-not-allowed',
  'connector-token-not-stored',
  'connector-api-not-called',
  'runtime-settings-ui-not-claimed',
  'runtime-custody-mutation-not-claimed',
  'final-policy-decision-not-claimed',
  'enforcement-not-claimed',
] as const;

describe('social video source custody settings contract', () => {
  it('accepts enabled source custody settings over source privacy refs', acceptsEnabledSourceCustody);
  it('accepts parent-review connector and manual-required custody boundaries', acceptsReviewAndManualBoundaries);
  it('rejects disabled manual and unavailable settings that pretend to feed policy input', rejectsUnsafePolicyUse);
  it('rejects connector screen manual and retention mode mismatches', rejectsCustodyModeMismatches);
  it('rejects raw custody connector runtime final-policy and enforcement claims', rejectsForbiddenClaims);
});

function acceptsEnabledSourceCustody() {
  const settings = buildSocialVideoSourceCustodySettings(custodyInput());

  expect(settings.schemaVersion).toBe(1);
  expect(settings.permissionState).toBe('enabled');
  expect(settings.custodyMode).toBe('local-redacted-refs-only');
  expect(settings.sourcePrivacyEvidenceIds).toEqual(['source-privacy-youtube-homework-video']);
  expect(settings.permittedDownstreamUses).toEqual([
    'ai-candidate-input',
    'policy-candidate-input',
    'parent-explanation',
    'audit-summary',
  ]);
  expect(settings.rawMessageContentAllowed).toBe(false);
  expect(settings.connectorApiCalled).toBe(false);
  expect(settings.runtimeCustodyMutationClaimed).toBe(false);
}

function acceptsReviewAndManualBoundaries() {
  const parentReview = buildSocialVideoSourceCustodySettings(
    custodyInput({
      settingsId: 'source-custody-connector-parent-review',
      settingScope: 'connector-authorization-ref',
      permissionState: 'parent-review-required',
      custodyMode: 'connector-authorization-ref-only',
      permittedDownstreamUses: ['parent-explanation', 'manual-review', 'audit-summary'],
      disabledUseReasons: ['connector-scope-needs-parent-review'],
      parentReviewRefs: ['parent-review-social-connector-scope'],
      connectorAuthorizationRefs: ['connector-authorization-ref-youtube-supervision'],
    })
  );
  const manualRequired = buildSocialVideoSourceCustodySettings(
    custodyInput({
      settingsId: 'source-custody-native-manual-required',
      settingScope: 'native-platform-manual-required',
      permissionState: 'manual-required',
      custodyMode: 'manual-required',
      retentionMode: 'manual-required',
      permittedDownstreamUses: ['manual-review', 'audit-summary'],
      disabledUseReasons: ['native-platform-source-proof-missing'],
      manualProofRequirements: ['android-or-ios-native-source-proof-required'],
    })
  );

  expect(parentReview.parentReviewRefs).toEqual(['parent-review-social-connector-scope']);
  expect(manualRequired.permissionState).toBe('manual-required');
  expect(manualRequired.permittedDownstreamUses).toEqual(['manual-review', 'audit-summary']);
}

function rejectsUnsafePolicyUse() {
  const invalidRows = [
    custodyCandidate({
      settingsId: 'source-custody-disabled-policy-input',
      permissionState: 'disabled',
      disabledUseReasons: ['parent-disabled-social-video-source-use'],
      manualProofRequirements: ['parent-disabled-proof-ref'],
    }),
    custodyCandidate({
      settingsId: 'source-custody-manual-policy-input',
      permissionState: 'manual-required',
      custodyMode: 'manual-required',
      retentionMode: 'manual-required',
      disabledUseReasons: ['manual-native-proof-required'],
      manualProofRequirements: ['native-source-proof-required'],
    }),
    custodyCandidate({
      settingsId: 'source-custody-unavailable-policy-input',
      permissionState: 'unavailable',
      disabledUseReasons: ['source-adapter-unavailable'],
      manualProofRequirements: ['source-adapter-proof-required'],
    }),
  ];

  for (const invalid of invalidRows) {
    expect(SocialVideoSourceCustodySettingsSchema.safeParse(invalid).success).toBe(false);
  }
}

function rejectsCustodyModeMismatches() {
  const invalidRows = [
    custodyCandidate({
      settingsId: 'source-custody-connector-missing-ref',
      settingScope: 'connector-authorization-ref',
      custodyMode: 'connector-authorization-ref-only',
    }),
    custodyCandidate({
      settingsId: 'source-custody-screen-wrong-scope',
      settingScope: 'bounded-video-metadata',
      custodyMode: 'screen-summary-ref-only',
    }),
    custodyCandidate({
      settingsId: 'source-custody-parent-provided-wrong-scope',
      settingScope: 'managed-browser-social-route',
      custodyMode: 'parent-provided-refs-only',
    }),
    custodyCandidate({
      settingsId: 'source-custody-manual-retention-enabled',
      custodyMode: 'local-redacted-refs-only',
      retentionMode: 'manual-required',
      manualProofRequirements: ['manual-retention-proof-required'],
    }),
  ];

  for (const invalid of invalidRows) {
    expect(SocialVideoSourceCustodySettingsSchema.safeParse(invalid).success).toBe(false);
  }
}

function rejectsForbiddenClaims() {
  const valid = buildSocialVideoSourceCustodySettings(custodyInput());
  const invalidRows = [
    { ...valid, rawMessageContentAllowed: true },
    { ...valid, rawVideoContentAllowed: true },
    { ...valid, screenshotCustodyAllowed: true },
    { ...valid, connectorTokenStored: true },
    { ...valid, connectorApiCalled: true },
    { ...valid, runtimeSettingsUiClaimed: true },
    { ...valid, runtimeCustodyMutationClaimed: true },
    { ...valid, finalPolicyDecisionClaimed: true },
    { ...valid, enforcementClaimed: true },
    { ...valid, noClaimLabels: ['raw-message-content-not-allowed'] },
  ];

  for (const invalid of invalidRows) {
    expect(SocialVideoSourceCustodySettingsSchema.safeParse(invalid).success).toBe(false);
  }
}

function custodyInput(overrides = {}) {
  return {
    settingsId: 'source-custody-managed-browser-video',
    generatedAt: '2026-06-06T08:44:00.000Z',
    childProfileRef: 'child-profile-middle-school',
    deviceId: 'device-managed-laptop',
    sourcePrivacyEvidenceIds: ['source-privacy-youtube-homework-video'],
    evidenceRefs: ['source-privacy-youtube-homework-video', 'parent-settings-proof-ref'],
    settingScope: 'managed-browser-social-route',
    permissionState: 'enabled',
    custodyMode: 'local-redacted-refs-only',
    retentionMode: 'redacted-ref-journal-only',
    permittedDownstreamUses: ['ai-candidate-input', 'policy-candidate-input', 'parent-explanation', 'audit-summary'],
    disabledUseReasons: [],
    parentReviewRefs: [],
    connectorAuthorizationRefs: [],
    manualProofRequirements: [],
    noClaimLabels: RequiredNoClaimLabels,
    ...overrides,
  };
}

function custodyCandidate(overrides = {}) {
  return {
    ...custodyInput(overrides),
    schemaVersion: 1,
    rawMessageContentAllowed: false,
    rawVideoContentAllowed: false,
    screenshotCustodyAllowed: false,
    connectorTokenStored: false,
    connectorApiCalled: false,
    runtimeSettingsUiClaimed: false,
    runtimeCustodyMutationClaimed: false,
    finalPolicyDecisionClaimed: false,
    enforcementClaimed: false,
  };
}
