import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentEvidenceReferenceIdSchema } from '@ocentra-parent/family-domain/reference-primitives';

const NonEmptyBrowserGameMetadataText = Schema.String.pipe(Schema.minLength(1));

export const BrowserGameMetadataExtractorSchemaVersionSchema = withParser(
  Schema.Literal('browser-game-metadata-extractor-contract')
);

export const BrowserGameMetadataExtractionIdSchema = withParser(
  NonEmptyBrowserGameMetadataText.pipe(Schema.brand('BrowserGameMetadataExtractionId'))
);

export const BrowserGameMetadataFieldIdSchema = withParser(
  NonEmptyBrowserGameMetadataText.pipe(Schema.brand('BrowserGameMetadataFieldId'))
);

export const BrowserGameMetadataFingerprintSchema = withParser(
  NonEmptyBrowserGameMetadataText.pipe(Schema.brand('BrowserGameMetadataFingerprint'))
);

export const BrowserGameMetadataSourceKindSchema = withParser(
  Schema.Literal(
    'html-meta-ref',
    'structured-data-ref',
    'platform-api-ref',
    'parent-curated-ref',
    'school-curated-ref',
    'manual-review-ref',
    'unavailable'
  )
);

export const BrowserGameMetadataFieldKindSchema = withParser(
  Schema.Literal(
    'title-shape',
    'description-shape',
    'genre-shape',
    'age-rating-shape',
    'publisher-shape',
    'thumbnail-shape',
    'educational-subject-shape',
    'cloud-platform-title-shape',
    'unknown'
  )
);

export const BrowserGameMetadataStatusSchema = withParser(
  Schema.Literal('extracted-shape', 'candidate-shape', 'manual-required', 'unavailable')
);

export const BrowserGameMetadataConfidenceSchema = withParser(Schema.Literal('high', 'medium', 'low', 'unknown'));

export const BrowserGameMetadataReasonCodeSchema = withParser(
  Schema.Literal(
    'metadata-shape-present',
    'title-shape-present',
    'description-shape-present',
    'rating-shape-present',
    'educational-subject-shape-present',
    'cloud-title-shape-present',
    'metadata-missing',
    'manual-required',
    'unavailable'
  )
);

export const BrowserGameMetadataEvidenceRefsSchema = Schema.Array(ParentEvidenceReferenceIdSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected browser-game metadata evidence refs')
);

export const BrowserGameMetadataReasonCodesSchema = Schema.Array(BrowserGameMetadataReasonCodeSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected browser-game metadata reason codes')
);

export type BrowserGameMetadataConfidence = Infer<typeof BrowserGameMetadataConfidenceSchema>;
export type BrowserGameMetadataFieldKind = Infer<typeof BrowserGameMetadataFieldKindSchema>;
export type BrowserGameMetadataReasonCode = Infer<typeof BrowserGameMetadataReasonCodeSchema>;
export type BrowserGameMetadataSourceKind = Infer<typeof BrowserGameMetadataSourceKindSchema>;
export type BrowserGameMetadataStatus = Infer<typeof BrowserGameMetadataStatusSchema>;
