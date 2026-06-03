import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from './reference-primitives';
import {
  BrowserGameMetadataConfidenceSchema,
  BrowserGameMetadataEvidenceRefsSchema,
  BrowserGameMetadataExtractionIdSchema,
  BrowserGameMetadataExtractorSchemaVersionSchema,
  BrowserGameMetadataFieldIdSchema,
  BrowserGameMetadataFieldKindSchema,
  BrowserGameMetadataFingerprintSchema,
  BrowserGameMetadataReasonCodesSchema,
  BrowserGameMetadataSourceKindSchema,
  BrowserGameMetadataStatusSchema,
} from './browser-game-metadata-extractor-values';

const BrowserGameMetadataFieldShapeBaseSchema = Schema.Struct({
  fieldId: BrowserGameMetadataFieldIdSchema,
  fieldKind: BrowserGameMetadataFieldKindSchema,
  metadataFingerprint: BrowserGameMetadataFingerprintSchema,
  sourceKind: BrowserGameMetadataSourceKindSchema,
  sourceEvidenceRefs: BrowserGameMetadataEvidenceRefsSchema,
  confidence: BrowserGameMetadataConfidenceSchema,
  status: BrowserGameMetadataStatusSchema,
  reasonCodes: BrowserGameMetadataReasonCodesSchema,
  educationalCandidate: Schema.Boolean,
  ageRatingCandidate: Schema.Boolean,
  cloudTitleCandidate: Schema.Boolean,
  rawTitleStored: Schema.Boolean,
  rawDescriptionStored: Schema.Boolean,
  rawPageBodyStored: Schema.Boolean,
  rawImageStored: Schema.Boolean,
  rawStructuredDataStored: Schema.Boolean,
  runtimeDomExtractionClaimed: Schema.Boolean,
  platformApiCalledClaimed: Schema.Boolean,
  aiClassificationClaimed: Schema.Boolean,
  policyDecisionClaimed: Schema.Boolean,
  cloudFrameAnalysisClaimed: Schema.Boolean,
  nativeGameControlClaimed: Schema.Boolean,
  enforcementClaimed: Schema.Boolean,
});

type BrowserGameMetadataFieldShapeCandidate = Infer<typeof BrowserGameMetadataFieldShapeBaseSchema>;

export const BrowserGameMetadataFieldShapeSchema = withParser(
  BrowserGameMetadataFieldShapeBaseSchema.pipe(
    Schema.filter(
      (field) =>
        browserGameMetadataFieldShapeIsHonest(field) ||
        'Expected browser-game metadata field shape to stay redacted and evidence-backed'
    )
  )
);

const BrowserGameMetadataFieldShapesSchema = Schema.Array(BrowserGameMetadataFieldShapeSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected browser-game metadata field shapes')
);

const BrowserGameMetadataExtractionBaseSchema = Schema.Struct({
  schemaVersion: BrowserGameMetadataExtractorSchemaVersionSchema,
  extractionId: BrowserGameMetadataExtractionIdSchema,
  extractedAt: ParentTimestampSchema,
  sourceEvidenceRefs: BrowserGameMetadataEvidenceRefsSchema,
  fields: BrowserGameMetadataFieldShapesSchema,
  confidence: BrowserGameMetadataConfidenceSchema,
  status: BrowserGameMetadataStatusSchema,
  rawTitleStored: Schema.Boolean,
  rawDescriptionStored: Schema.Boolean,
  rawPageBodyStored: Schema.Boolean,
  rawImageStored: Schema.Boolean,
  rawStructuredDataStored: Schema.Boolean,
  runtimeDomExtractionClaimed: Schema.Boolean,
  platformApiCalledClaimed: Schema.Boolean,
  aiClassificationClaimed: Schema.Boolean,
  policyDecisionClaimed: Schema.Boolean,
  cloudFrameAnalysisClaimed: Schema.Boolean,
  nativeGameControlClaimed: Schema.Boolean,
  enforcementClaimed: Schema.Boolean,
});

type BrowserGameMetadataExtractionCandidate = Infer<typeof BrowserGameMetadataExtractionBaseSchema>;

export const BrowserGameMetadataExtractionSchema = withParser(
  BrowserGameMetadataExtractionBaseSchema.pipe(
    Schema.filter(
      (extraction) =>
        browserGameMetadataExtractionIsHonest(extraction) ||
        'Expected browser-game metadata extraction to remain contract-only'
    )
  )
);

export const decodeBrowserGameMetadataExtraction = Schema.decodeUnknownSync(BrowserGameMetadataExtractionSchema);

export type BrowserGameMetadataExtraction = Infer<typeof BrowserGameMetadataExtractionSchema>;
export type BrowserGameMetadataFieldShape = Infer<typeof BrowserGameMetadataFieldShapeSchema>;

function browserGameMetadataFieldShapeIsHonest(field: BrowserGameMetadataFieldShapeCandidate): boolean {
  if (browserGameMetadataFieldShapeClaimsAuthority(field) || browserGameMetadataFieldPurposeIsInconsistent(field)) {
    return false;
  }
  if (field.status === 'extracted-shape') {
    return (
      field.confidence !== 'unknown' &&
      field.fieldKind !== 'unknown' &&
      field.sourceKind !== 'manual-review-ref' &&
      field.sourceKind !== 'unavailable'
    );
  }
  if (field.status === 'candidate-shape') {
    return field.confidence !== 'high' && field.fieldKind !== 'unknown' && field.sourceKind !== 'unavailable';
  }
  return (
    field.confidence !== 'high' &&
    (field.fieldKind === 'unknown' ||
      field.sourceKind === 'manual-review-ref' ||
      field.sourceKind === 'unavailable' ||
      field.reasonCodes.includes('manual-required') ||
      field.reasonCodes.includes('unavailable'))
  );
}

function browserGameMetadataExtractionIsHonest(extraction: BrowserGameMetadataExtractionCandidate): boolean {
  if (browserGameMetadataExtractionClaimsAuthority(extraction)) {
    return false;
  }
  if (extraction.status === 'extracted-shape') {
    return (
      extraction.confidence !== 'unknown' && extraction.fields.every((field) => field.status === 'extracted-shape')
    );
  }
  return extraction.confidence !== 'high' && extraction.fields.some((field) => field.status !== 'extracted-shape');
}

function browserGameMetadataFieldPurposeIsInconsistent(field: BrowserGameMetadataFieldShapeCandidate): boolean {
  if (field.educationalCandidate && field.fieldKind !== 'educational-subject-shape') {
    return true;
  }
  if (field.ageRatingCandidate && field.fieldKind !== 'age-rating-shape') {
    return true;
  }
  return field.cloudTitleCandidate && field.fieldKind !== 'cloud-platform-title-shape';
}

function browserGameMetadataFieldShapeClaimsAuthority(field: BrowserGameMetadataFieldShapeCandidate): boolean {
  return (
    field.rawTitleStored ||
    field.rawDescriptionStored ||
    field.rawPageBodyStored ||
    field.rawImageStored ||
    field.rawStructuredDataStored ||
    field.runtimeDomExtractionClaimed ||
    field.platformApiCalledClaimed ||
    field.aiClassificationClaimed ||
    field.policyDecisionClaimed ||
    field.cloudFrameAnalysisClaimed ||
    field.nativeGameControlClaimed ||
    field.enforcementClaimed
  );
}

function browserGameMetadataExtractionClaimsAuthority(extraction: BrowserGameMetadataExtractionCandidate): boolean {
  return (
    extraction.rawTitleStored ||
    extraction.rawDescriptionStored ||
    extraction.rawPageBodyStored ||
    extraction.rawImageStored ||
    extraction.rawStructuredDataStored ||
    extraction.runtimeDomExtractionClaimed ||
    extraction.platformApiCalledClaimed ||
    extraction.aiClassificationClaimed ||
    extraction.policyDecisionClaimed ||
    extraction.cloudFrameAnalysisClaimed ||
    extraction.nativeGameControlClaimed ||
    extraction.enforcementClaimed
  );
}
