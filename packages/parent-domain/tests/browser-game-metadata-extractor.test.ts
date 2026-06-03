import { describe, expect, it } from 'vitest';
import {
  BrowserGameMetadataExtractionSchema,
  BrowserGameMetadataFieldShapeSchema,
  type BrowserGameMetadataExtraction,
  type BrowserGameMetadataFieldShape,
} from '../src/browser-game-metadata-extractor';

describe('browser-game metadata extractor contracts', () => {
  it('accepts redacted metadata field shapes for title, rating, subject, and cloud-title rows', acceptsFieldShapes);
  it('accepts candidate, manual-required, and unavailable metadata rows', acceptsFallbackRows);
  it('accepts extraction bundles without runtime or authority claims', acceptsExtractions);
  it('rejects raw metadata, DOM/API, AI, policy, cloud-frame, native, and enforcement claims', rejectsClaims);
  it('rejects inconsistent field purposes and dishonest extraction upgrades', rejectsInconsistentRows);
});

function acceptsFieldShapes() {
  expect(BrowserGameMetadataFieldShapeSchema.safeParse(metadataField()).success).toBe(true);
  expect(
    BrowserGameMetadataFieldShapeSchema.safeParse(
      metadataField({
        fieldId: 'metadata-field-rating',
        fieldKind: 'age-rating-shape',
        metadataFingerprint: 'metadata-fingerprint-rating',
        reasonCodes: ['rating-shape-present'],
        ageRatingCandidate: true,
      })
    ).success
  ).toBe(true);
  expect(
    BrowserGameMetadataFieldShapeSchema.safeParse(
      metadataField({
        fieldId: 'metadata-field-subject',
        fieldKind: 'educational-subject-shape',
        metadataFingerprint: 'metadata-fingerprint-subject',
        reasonCodes: ['educational-subject-shape-present'],
        educationalCandidate: true,
      })
    ).success
  ).toBe(true);
  expect(
    BrowserGameMetadataFieldShapeSchema.safeParse(
      metadataField({
        fieldId: 'metadata-field-cloud-title',
        fieldKind: 'cloud-platform-title-shape',
        metadataFingerprint: 'metadata-fingerprint-cloud-title',
        reasonCodes: ['cloud-title-shape-present'],
        cloudTitleCandidate: true,
      })
    ).success
  ).toBe(true);
}

function acceptsFallbackRows() {
  expect(
    BrowserGameMetadataFieldShapeSchema.safeParse(
      metadataField({
        fieldId: 'metadata-field-candidate',
        status: 'candidate-shape',
        confidence: 'medium',
      })
    ).success
  ).toBe(true);
  expect(BrowserGameMetadataFieldShapeSchema.safeParse(manualMetadataField()).success).toBe(true);
  expect(
    BrowserGameMetadataFieldShapeSchema.safeParse(
      manualMetadataField({
        fieldId: 'metadata-field-unavailable',
        status: 'unavailable',
        sourceKind: 'unavailable',
        reasonCodes: ['unavailable'],
      })
    ).success
  ).toBe(true);
}

function acceptsExtractions() {
  expect(BrowserGameMetadataExtractionSchema.safeParse(metadataExtraction()).success).toBe(true);
  expect(
    BrowserGameMetadataExtractionSchema.safeParse(
      metadataExtraction({
        status: 'manual-required',
        confidence: 'low',
        fields: [manualMetadataField()],
      })
    ).success
  ).toBe(true);
}

function rejectsClaims() {
  const invalidClaims = [
    { rawTitleStored: true },
    { rawDescriptionStored: true },
    { rawPageBodyStored: true },
    { rawImageStored: true },
    { rawStructuredDataStored: true },
    { runtimeDomExtractionClaimed: true },
    { platformApiCalledClaimed: true },
    { aiClassificationClaimed: true },
    { policyDecisionClaimed: true },
    { cloudFrameAnalysisClaimed: true },
    { nativeGameControlClaimed: true },
    { enforcementClaimed: true },
  ];

  for (const invalid of invalidClaims) {
    expect(BrowserGameMetadataFieldShapeSchema.safeParse(metadataField(invalid)).success).toBe(false);
    expect(BrowserGameMetadataExtractionSchema.safeParse(metadataExtraction(invalid)).success).toBe(false);
  }
}

function rejectsInconsistentRows() {
  const invalidFields = [
    { fieldKind: 'unknown' },
    { sourceKind: 'unavailable' },
    { educationalCandidate: true, fieldKind: 'title-shape' },
    { ageRatingCandidate: true, fieldKind: 'title-shape' },
    { cloudTitleCandidate: true, fieldKind: 'title-shape' },
    { status: 'manual-required', confidence: 'high', fieldKind: 'unknown' },
  ];

  for (const invalid of invalidFields) {
    expect(BrowserGameMetadataFieldShapeSchema.safeParse(metadataField(invalid)).success).toBe(false);
  }

  expect(
    BrowserGameMetadataExtractionSchema.safeParse(
      metadataExtraction({
        fields: [manualMetadataField()],
      })
    ).success
  ).toBe(false);
  expect(
    BrowserGameMetadataExtractionSchema.safeParse(
      metadataExtraction({
        status: 'manual-required',
        confidence: 'high',
        fields: [manualMetadataField()],
      })
    ).success
  ).toBe(false);
}

function metadataField(overrides = {}): BrowserGameMetadataFieldShape {
  return {
    fieldId: 'metadata-field-title',
    fieldKind: 'title-shape',
    metadataFingerprint: 'metadata-fingerprint-title',
    sourceKind: 'html-meta-ref',
    sourceEvidenceRefs: ['metadata-evidence-title'],
    confidence: 'high',
    status: 'extracted-shape',
    reasonCodes: ['metadata-shape-present', 'title-shape-present'],
    educationalCandidate: false,
    ageRatingCandidate: false,
    cloudTitleCandidate: false,
    rawTitleStored: false,
    rawDescriptionStored: false,
    rawPageBodyStored: false,
    rawImageStored: false,
    rawStructuredDataStored: false,
    runtimeDomExtractionClaimed: false,
    platformApiCalledClaimed: false,
    aiClassificationClaimed: false,
    policyDecisionClaimed: false,
    cloudFrameAnalysisClaimed: false,
    nativeGameControlClaimed: false,
    enforcementClaimed: false,
    ...overrides,
  };
}

function manualMetadataField(overrides = {}): BrowserGameMetadataFieldShape {
  return metadataField({
    fieldId: 'metadata-field-manual',
    fieldKind: 'unknown',
    metadataFingerprint: 'metadata-fingerprint-manual-required',
    sourceKind: 'manual-review-ref',
    confidence: 'low',
    status: 'manual-required',
    reasonCodes: ['metadata-missing', 'manual-required'],
    ...overrides,
  });
}

function metadataExtraction(overrides = {}): BrowserGameMetadataExtraction {
  return {
    schemaVersion: 'browser-game-metadata-extractor-contract',
    extractionId: 'metadata-extraction-reviewed',
    extractedAt: '2026-06-03T11:50:00.000Z',
    sourceEvidenceRefs: ['metadata-extraction-evidence'],
    fields: [metadataField()],
    confidence: 'high',
    status: 'extracted-shape',
    rawTitleStored: false,
    rawDescriptionStored: false,
    rawPageBodyStored: false,
    rawImageStored: false,
    rawStructuredDataStored: false,
    runtimeDomExtractionClaimed: false,
    platformApiCalledClaimed: false,
    aiClassificationClaimed: false,
    policyDecisionClaimed: false,
    cloudFrameAnalysisClaimed: false,
    nativeGameControlClaimed: false,
    enforcementClaimed: false,
    ...overrides,
  };
}
