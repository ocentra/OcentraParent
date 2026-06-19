import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import { ParentEvidenceReferenceIdSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';

export const BrowserGameEducationalClassifierSchemaVersionSchema = withParser(
  Schema.Literal('browser-game-educational-classifier-contract')
);

export const BrowserGameEducationalClassifierResultIdSchema = withParser(
  brandedNonEmptyStringSchema('BrowserGameEducationalClassifierResultId')
);

export const BrowserGameEducationalEvidenceRowIdSchema = withParser(
  brandedNonEmptyStringSchema('BrowserGameEducationalEvidenceRowId')
);

export const BrowserGameEducationalEvidenceKindSchema = withParser(
  Schema.Literal(
    'domain-reputation',
    'school-provided-url',
    'teacher-allowlist',
    'parent-allowlist',
    'page-metadata',
    'subject-metadata',
    'ai-classification',
    'past-parent-approval',
    'homework-context',
    'school-platform',
    'platform-self-label',
    'manual-required'
  )
);

export const BrowserGameEducationalCategorySchema = withParser(
  Schema.Literal(
    'math',
    'science',
    'coding',
    'typing',
    'language',
    'history',
    'art',
    'chess-logic',
    'problem-solving',
    'school-platform',
    'unknown-educational-category'
  )
);

export const BrowserGameEducationalClassificationOutcomeSchema = withParser(
  Schema.Literal(
    'educational-candidate',
    'entertainment-candidate',
    'misleading-educational-claim',
    'unknown-candidate',
    'manual-required',
    'unavailable'
  )
);

export const BrowserGameEducationalClassifierConfidenceSchema = withParser(
  Schema.Literal('high', 'medium', 'low', 'unknown')
);

export const BrowserGameEducationalClassifierDegradedStateSchema = withParser(
  Schema.Literal('none', 'degraded', 'manual-required', 'unavailable')
);

export const BrowserGameEducationalRecommendedGateSchema = withParser(
  Schema.Literal(
    'allow-during-homework-candidate',
    'allow-with-time-limit-candidate',
    'parent-review-candidate',
    'block-portal-candidate',
    'manual-review-candidate',
    'unknown-candidate'
  )
);

export const BrowserGameEducationalUncertaintyReasonSchema = withParser(
  Schema.Literal(
    'missing-school-source',
    'missing-metadata',
    'low-confidence',
    'conflicting-evidence',
    'platform-label-only',
    'manual-required'
  )
);

export const BrowserGameEducationalEvidenceRefsSchema = Schema.Array(ParentEvidenceReferenceIdSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected browser-game educational classifier evidence refs')
);

export type BrowserGameEducationalCategory = Infer<typeof BrowserGameEducationalCategorySchema>;
export type BrowserGameEducationalClassificationOutcome = Infer<
  typeof BrowserGameEducationalClassificationOutcomeSchema
>;
export type BrowserGameEducationalClassifierConfidence = Infer<typeof BrowserGameEducationalClassifierConfidenceSchema>;
export type BrowserGameEducationalEvidenceKind = Infer<typeof BrowserGameEducationalEvidenceKindSchema>;
export type BrowserGameEducationalRecommendedGate = Infer<typeof BrowserGameEducationalRecommendedGateSchema>;

