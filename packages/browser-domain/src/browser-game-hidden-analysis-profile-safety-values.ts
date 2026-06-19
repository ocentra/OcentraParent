import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import { ParentEvidenceReferenceIdSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';

export const BrowserGameHiddenAnalysisProfileSafetySchemaVersionSchema = withParser(
  Schema.Literal('browser-game-hidden-analysis-profile-safety-contract')
);

export const BrowserGameHiddenAnalysisProfileDesignIdSchema = withParser(
  brandedNonEmptyStringSchema('BrowserGameHiddenAnalysisProfileDesignId')
);

export const BrowserGameHiddenAnalysisProfileFingerprintSchema = withParser(
  brandedNonEmptyStringSchema('BrowserGameHiddenAnalysisProfileFingerprint')
);

export const BrowserGameHiddenAnalysisLoaderRequestIdSchema = withParser(
  brandedNonEmptyStringSchema('BrowserGameHiddenAnalysisLoaderRequestId')
);

export const BrowserGameHiddenAnalysisLoaderResultIdSchema = withParser(
  brandedNonEmptyStringSchema('BrowserGameHiddenAnalysisLoaderResultId')
);

export const BrowserGameHiddenAnalysisLoaderProofRefSchema = withParser(
  brandedNonEmptyStringSchema('BrowserGameHiddenAnalysisLoaderProofRef')
);

export const BrowserGameHiddenAnalysisSummaryRefSchema = withParser(
  brandedNonEmptyStringSchema('BrowserGameHiddenAnalysisSummaryRef')
);

export const BrowserGameHiddenAnalysisProfileKindSchema = withParser(
  Schema.Literal(
    'isolated-managed-profile',
    'metadata-only-profile',
    'cloud-session-review-profile',
    'educational-game-review-profile',
    'manual-required',
    'unavailable'
  )
);

export const BrowserGameHiddenAnalysisCapabilityStateSchema = withParser(
  Schema.Literal('available', 'disabled-by-policy', 'profile-proof-missing', 'manual-required', 'unavailable')
);

export const BrowserGameHiddenAnalysisStateSchema = withParser(
  Schema.Literal(
    'queued',
    'loading',
    'metadata-only',
    'analysis-ready',
    'manual-required',
    'disabled-by-policy',
    'profile-proof-missing',
    'timeout',
    'platform-restricted',
    'unsupported-content',
    'unavailable'
  )
);

export const BrowserGameHiddenAnalysisConfidenceSchema = withParser(Schema.Literal('high', 'medium', 'low', 'unknown'));

export const BrowserGameHiddenAnalysisReasonCodeSchema = withParser(
  Schema.Literal(
    'ocentra-owned-profile',
    'separate-from-child-profile',
    'bounded-retention',
    'loader-proof-required',
    'policy-disabled',
    'profile-proof-missing',
    'manual-required',
    'timeout',
    'platform-restricted',
    'unsupported-content',
    'unavailable'
  )
);

export const BrowserGameHiddenAnalysisEvidenceRefsSchema = Schema.Array(ParentEvidenceReferenceIdSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected browser-game hidden analysis evidence refs')
);

export const BrowserGameHiddenAnalysisReasonCodesSchema = Schema.Array(BrowserGameHiddenAnalysisReasonCodeSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected browser-game hidden analysis reason codes')
);

export type BrowserGameHiddenAnalysisCapabilityState = Infer<typeof BrowserGameHiddenAnalysisCapabilityStateSchema>;
export type BrowserGameHiddenAnalysisConfidence = Infer<typeof BrowserGameHiddenAnalysisConfidenceSchema>;
export type BrowserGameHiddenAnalysisProfileKind = Infer<typeof BrowserGameHiddenAnalysisProfileKindSchema>;
export type BrowserGameHiddenAnalysisReasonCode = Infer<typeof BrowserGameHiddenAnalysisReasonCodeSchema>;
export type BrowserGameHiddenAnalysisState = Infer<typeof BrowserGameHiddenAnalysisStateSchema>;

