import { describe, expect, it } from 'vitest';
import {
  TrackingIosPrivacyDisclosureProofReadModelSchema,
  TrackingIosPrivacyDisclosureProofRowSchema,
  buildTrackingIosPrivacyDisclosureProofReadModel,
  type TrackingIosPrivacyDisclosureInputRow,
} from '../src/tracking-ios-privacy-disclosure-release-proof';

const Timestamp = '2026-06-07T17:30:00.000Z';

const ProofOptions = {
  generatedAt: Timestamp,
  proofId: 'tracking-ios-privacy-disclosure-release-proof',
  familyId: 'family-tracking-ios-privacy-release',
  childProfileId: 'child-profile-maya',
  deviceId: 'device-maya-ios',
  deviceLabel: 'Maya iOS release gate',
  sourceProofRefs: [
    'docs/plans/tracking-plan/workpacks/12-ios-background-region-significant-change-adapter.md',
    'docs/expectations/platforms.md',
    'docs/plans/tracking-plan/v0-5-location-platform-deep-dive.md',
    'test-results/tracking-ios-location-manual-required-proof/proof.json',
  ],
} as const;

describe('tracking iOS privacy disclosure release proof', () => {
  it('builds release-blocked and manual-review rows for iOS privacy disclosures', () => {
    const readModel = buildTrackingIosPrivacyDisclosureProofReadModel(ProofOptions, disclosureRows());

    expect(readModel.releaseGateRows.map((row) => row.releaseGate)).toEqual([
      'location-purpose-disclosure',
      'background-location-disclosure',
      'region-monitoring-disclosure',
      'notification-disclosure',
      'data-custody-disclosure',
      'app-store-review-evidence',
    ]);
    expect(readModel.releaseBlockedCount).toBe(3);
    expect(readModel.manualReviewRequiredCount).toBe(3);
    expect(readModel.releaseGateRows.map((row) => row.parentVisibleStatusToken)).toEqual([
      'tracking-ios-location-purpose-disclosure-required',
      'tracking-ios-background-location-disclosure-required',
      'tracking-ios-region-monitoring-disclosure-required',
      'tracking-ios-notification-disclosure-required',
      'tracking-ios-data-custody-disclosure-required',
      'tracking-ios-app-store-review-evidence-required',
    ]);
    expect(readModel.runtimeEvidenceRefs).toEqual([
      {
        evidenceReferenceId: 'wp12-ios-location-purpose-copy-draft',
        kind: 'policy-decision',
        observedAt: Timestamp,
      },
      {
        evidenceReferenceId: 'wp12-ios-data-custody-copy-draft',
        kind: 'policy-decision',
        observedAt: Timestamp,
      },
      {
        evidenceReferenceId: 'wp12-ios-app-store-review-checklist-draft',
        kind: 'policy-decision',
        observedAt: Timestamp,
      },
    ]);
  });

  it('keeps every release claim false until Apple review and runtime evidence exist', () => {
    const readModel = buildTrackingIosPrivacyDisclosureProofReadModel(ProofOptions, disclosureRows());
    const backgroundRow = readModel.releaseGateRows[1];

    expect(
      TrackingIosPrivacyDisclosureProofRowSchema.safeParse({
        ...backgroundRow,
        backgroundLocationDeliveryClaimed: true,
      }).success
    ).toBe(false);
    expect(
      TrackingIosPrivacyDisclosureProofRowSchema.safeParse({
        ...backgroundRow,
        releaseClaimAllowed: true,
      }).success
    ).toBe(false);
    expect(
      TrackingIosPrivacyDisclosureProofReadModelSchema.safeParse({
        ...readModel,
        productReadyIosTrackingClaimed: true,
      }).success
    ).toBe(false);
  });
});

function disclosureRows(): readonly TrackingIosPrivacyDisclosureInputRow[] {
  return [
    disclosureRow({
      rowId: 'tracking-ios-location-purpose-disclosure',
      releaseGate: 'location-purpose-disclosure',
      disclosureEvidenceRefs: ['tracking-ios-location-purpose-copy-draft'],
      runtimeEvidenceRefs: ['wp12-ios-location-purpose-copy-draft'],
      requiredBeforeReleaseClaimRefs: ['parent-facing-location-purpose-copy', 'apple-privacy-label-location-data'],
    }),
    disclosureRow({
      rowId: 'tracking-ios-background-location-disclosure',
      releaseGate: 'background-location-disclosure',
      requiredBeforeReleaseClaimRefs: [
        'background-location-purpose-copy',
        'apple-background-mode-review-artifact',
        'physical-device-background-delivery-proof',
      ],
    }),
    disclosureRow({
      rowId: 'tracking-ios-region-monitoring-disclosure',
      releaseGate: 'region-monitoring-disclosure',
      requiredBeforeReleaseClaimRefs: [
        'region-monitoring-purpose-copy',
        'apple-region-monitoring-review-artifact',
        'physical-device-region-transition-proof',
      ],
    }),
    disclosureRow({
      rowId: 'tracking-ios-notification-disclosure',
      releaseGate: 'notification-disclosure',
      requiredBeforeReleaseClaimRefs: ['notification-purpose-copy', 'ios-local-notification-delivery-proof'],
    }),
    disclosureRow({
      rowId: 'tracking-ios-data-custody-disclosure',
      releaseGate: 'data-custody-disclosure',
      disclosureEvidenceRefs: ['tracking-ios-data-custody-copy-draft'],
      runtimeEvidenceRefs: ['wp12-ios-data-custody-copy-draft'],
      requiredBeforeReleaseClaimRefs: [
        'parent-owned-location-custody-copy',
        'retention-window-disclosure',
        'apple-privacy-nutrition-label-artifact',
      ],
    }),
    disclosureRow({
      rowId: 'tracking-ios-app-store-review-evidence',
      releaseGate: 'app-store-review-evidence',
      disclosureEvidenceRefs: ['tracking-ios-app-store-review-checklist-draft'],
      appStoreReviewArtifactRefs: ['app-store-review-required-before-release'],
      privacyNutritionArtifactRefs: ['privacy-nutrition-label-required-before-release'],
      runtimeEvidenceRefs: ['wp12-ios-app-store-review-checklist-draft'],
      requiredBeforeReleaseClaimRefs: [
        'apple-app-store-review-approval-artifact',
        'apple-entitlement-approval-artifact',
        'testflight-device-install-proof',
      ],
    }),
  ];
}

function disclosureRow(
  input: Pick<TrackingIosPrivacyDisclosureInputRow, 'rowId' | 'releaseGate' | 'requiredBeforeReleaseClaimRefs'> &
    Partial<
      Pick<
        TrackingIosPrivacyDisclosureInputRow,
        | 'disclosureEvidenceRefs'
        | 'manualProofRefs'
        | 'appStoreReviewArtifactRefs'
        | 'privacyNutritionArtifactRefs'
        | 'runtimeEvidenceRefs'
      >
    >
): TrackingIosPrivacyDisclosureInputRow {
  return {
    rowId: input.rowId,
    releaseGate: input.releaseGate,
    observedAt: Timestamp,
    disclosureEvidenceRefs: input.disclosureEvidenceRefs ?? [],
    manualProofRefs: input.manualProofRefs ?? ['wp12-ios-release-disclosure-manual-proof-plan'],
    appStoreReviewArtifactRefs: input.appStoreReviewArtifactRefs ?? [],
    privacyNutritionArtifactRefs: input.privacyNutritionArtifactRefs ?? [],
    runtimeEvidenceRefs: input.runtimeEvidenceRefs ?? [],
    requiredBeforeReleaseClaimRefs: input.requiredBeforeReleaseClaimRefs,
  };
}
