import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema,
  NonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import { ActivityEvidenceIdSchema, ActivityTimestampSchema } from '@ocentra-parent/schema-domain/evidence-primitives';
import { BrowserUrlShapeClassificationIdSchema } from './browser-url-intelligence-schemas';

export const BrowserHiddenAnalysisSchemaVersion = 1;
const OptionalHiddenAnalysisTextSchema = Schema.Union(NonEmptyStringSchema, Schema.Null);
const PositiveHiddenAnalysisIntegerSchema = Schema.Number.pipe(
  Schema.int(),
  Schema.filter((value) => value > 0 || 'Expected a positive hidden analysis integer')
);

export const BrowserHiddenAnalysisProfileDesignIdSchema = withParser(
  brandedNonEmptyStringSchema('BrowserHiddenAnalysisProfileDesignId')
);

export const BrowserHiddenAnalysisProfileIdSchema = withParser(
  brandedNonEmptyStringSchema('BrowserHiddenAnalysisProfileId')
);

export const BrowserHiddenAnalysisStateSchema = withParser(
  Schema.Literal(
    'not-needed',
    'queued',
    'loading',
    'metadata-only',
    'analysis-ready',
    'blocked-by-robots-or-login',
    'timeout',
    'platform-restricted',
    'network-error',
    'unsupported-content',
    'manual-required'
  )
);

export const BrowserHiddenAnalysisDegradedReasonSchema = withParser(
  Schema.Literal(
    'not-needed',
    'manual-required',
    'robots-or-login',
    'timeout',
    'platform-restricted',
    'network-error',
    'unsupported-content',
    'profile-not-isolated',
    'forbidden-capability-requested',
    'loader-proof-missing',
    'loader-unavailable',
    'disabled-by-policy'
  )
);

export const BrowserHiddenAnalysisProfileSafetySchema = Schema.Struct({
  ocentraOwnedProfile: Schema.Boolean,
  separateFromChildVisibleProfile: Schema.Boolean,
  usesChildCookies: Schema.Boolean,
  usesChildSessionTokens: Schema.Boolean,
  allowsAutoplayAudio: Schema.Boolean,
  allowsDownloads: Schema.Boolean,
  allowsFormSubmit: Schema.Boolean,
  claimsCaptchaAutomation: Schema.Boolean,
  claimsLoginBypass: Schema.Boolean,
  retainsRawPageBody: Schema.Boolean,
  boundedRetention: Schema.Boolean,
});

const BrowserHiddenAnalysisProfileDesignBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(BrowserHiddenAnalysisSchemaVersion),
  designId: BrowserHiddenAnalysisProfileDesignIdSchema,
  createdAt: ActivityTimestampSchema,
  sourceEvidenceIds: Schema.Array(ActivityEvidenceIdSchema),
  urlShapeClassificationId: BrowserUrlShapeClassificationIdSchema,
  hiddenProfileId: BrowserHiddenAnalysisProfileIdSchema,
  childVisibleProfileRef: OptionalHiddenAnalysisTextSchema,
  loaderProofRef: OptionalHiddenAnalysisTextSchema,
  state: BrowserHiddenAnalysisStateSchema,
  degradedReasons: Schema.Array(BrowserHiddenAnalysisDegradedReasonSchema),
  timeoutMs: PositiveHiddenAnalysisIntegerSchema,
  retentionTtlSeconds: PositiveHiddenAnalysisIntegerSchema,
  maxStructuredSummaryBytes: PositiveHiddenAnalysisIntegerSchema,
  safety: BrowserHiddenAnalysisProfileSafetySchema,
});

export const BrowserHiddenAnalysisProfileDesignSchema = withParser(
  BrowserHiddenAnalysisProfileDesignBaseSchema.pipe(
    Schema.filter(
      (value) =>
        browserHiddenAnalysisProfileDesignIsConsistent(value) ||
        'Expected hidden analysis profile design to preserve isolation and no-loader-proof boundaries'
    )
  )
);

export const decodeBrowserHiddenAnalysisProfileDesign = Schema.decodeUnknownSync(
  BrowserHiddenAnalysisProfileDesignSchema
);

export type BrowserHiddenAnalysisProfileDesignId = Infer<typeof BrowserHiddenAnalysisProfileDesignIdSchema>;
export type BrowserHiddenAnalysisProfileId = Infer<typeof BrowserHiddenAnalysisProfileIdSchema>;
export type BrowserHiddenAnalysisState = Infer<typeof BrowserHiddenAnalysisStateSchema>;
export type BrowserHiddenAnalysisDegradedReason = Infer<typeof BrowserHiddenAnalysisDegradedReasonSchema>;
export type BrowserHiddenAnalysisProfileSafety = Infer<typeof BrowserHiddenAnalysisProfileSafetySchema>;
export type BrowserHiddenAnalysisProfileDesign = Infer<typeof BrowserHiddenAnalysisProfileDesignSchema>;

function browserHiddenAnalysisProfileDesignIsConsistent(
  value: Infer<typeof BrowserHiddenAnalysisProfileDesignBaseSchema>
) {
  if (value.sourceEvidenceIds.length === 0 || hiddenAnalysisProfileSafetyIsInvalid(value.safety)) {
    return false;
  }
  if (value.childVisibleProfileRef !== null) {
    return false;
  }
  if (value.state === 'metadata-only' || value.state === 'analysis-ready') {
    return value.loaderProofRef !== null && value.degradedReasons.length === 0;
  }
  if (value.state === 'queued' || value.state === 'loading') {
    return value.loaderProofRef === null && value.degradedReasons.length === 0;
  }
  return value.degradedReasons.length > 0;
}

function hiddenAnalysisProfileSafetyIsInvalid(value: Infer<typeof BrowserHiddenAnalysisProfileSafetySchema>) {
  return (
    !value.ocentraOwnedProfile ||
    !value.separateFromChildVisibleProfile ||
    value.usesChildCookies ||
    value.usesChildSessionTokens ||
    value.allowsAutoplayAudio ||
    value.allowsDownloads ||
    value.allowsFormSubmit ||
    value.claimsCaptchaAutomation ||
    value.claimsLoginBypass ||
    value.retainsRawPageBody ||
    !value.boundedRetention
  );
}
