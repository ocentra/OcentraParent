import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema,
  NonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import { ActivityEvidenceIdSchema, ActivityTimestampSchema } from '@ocentra-parent/schema-domain/evidence-primitives';
import {
  BrowserHiddenAnalysisDegradedReasonSchema,
  BrowserHiddenAnalysisProfileDesignIdSchema,
  BrowserHiddenAnalysisProfileDesignSchema,
  BrowserHiddenAnalysisStateSchema,
} from '@ocentra-parent/schema-domain/browser-hidden-analysis-schemas';

export const BrowserHiddenAnalysisLoaderSchemaVersion = 1;
const OptionalHiddenAnalysisLoaderTextSchema = Schema.Union(NonEmptyStringSchema, Schema.Null);

export const BrowserHiddenAnalysisLoaderRequestIdSchema = withParser(
  brandedNonEmptyStringSchema('BrowserHiddenAnalysisLoaderRequestId')
);

export const BrowserHiddenAnalysisLoaderResultIdSchema = withParser(
  brandedNonEmptyStringSchema('BrowserHiddenAnalysisLoaderResultId')
);

export const BrowserHiddenAnalysisLoaderCapabilityStateSchema = withParser(
  Schema.Literal('available', 'disabled-by-policy', 'unavailable', 'manual-required')
);

const BrowserHiddenAnalysisLoaderRequestBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(BrowserHiddenAnalysisLoaderSchemaVersion),
  loaderRequestId: BrowserHiddenAnalysisLoaderRequestIdSchema,
  requestedAt: ActivityTimestampSchema,
  profileDesign: BrowserHiddenAnalysisProfileDesignSchema,
  capabilityState: BrowserHiddenAnalysisLoaderCapabilityStateSchema,
  policyAllowsHiddenAnalysis: Schema.Boolean,
});

export const BrowserHiddenAnalysisLoaderRequestSchema = withParser(
  BrowserHiddenAnalysisLoaderRequestBaseSchema.pipe(
    Schema.filter(
      (value) =>
        browserHiddenAnalysisLoaderRequestIsConsistent(value) ||
        'Expected hidden analysis loader request to preserve policy and profile-design boundaries'
    )
  )
);

const BrowserHiddenAnalysisLoaderResultBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(BrowserHiddenAnalysisLoaderSchemaVersion),
  loaderResultId: BrowserHiddenAnalysisLoaderResultIdSchema,
  loaderRequestId: BrowserHiddenAnalysisLoaderRequestIdSchema,
  producedAt: ActivityTimestampSchema,
  profileDesignId: BrowserHiddenAnalysisProfileDesignIdSchema,
  sourceEvidenceIds: Schema.Array(ActivityEvidenceIdSchema),
  state: BrowserHiddenAnalysisStateSchema,
  loaderProofRef: OptionalHiddenAnalysisLoaderTextSchema,
  degradedReasons: Schema.Array(BrowserHiddenAnalysisDegradedReasonSchema),
  loadedByHiddenAdapter: Schema.Boolean,
  pageBodyCaptured: Schema.Boolean,
  transcriptTextCaptured: Schema.Boolean,
});

export const BrowserHiddenAnalysisLoaderResultSchema = withParser(
  BrowserHiddenAnalysisLoaderResultBaseSchema.pipe(
    Schema.filter(
      (value) =>
        browserHiddenAnalysisLoaderResultIsConsistent(value) ||
        'Expected hidden analysis loader result to preserve proof and no-capture boundaries'
    )
  )
);

export const decodeBrowserHiddenAnalysisLoaderRequest = Schema.decodeUnknownSync(
  BrowserHiddenAnalysisLoaderRequestSchema
);
export const decodeBrowserHiddenAnalysisLoaderResult = Schema.decodeUnknownSync(
  BrowserHiddenAnalysisLoaderResultSchema
);

export type BrowserHiddenAnalysisLoaderRequestId = Infer<typeof BrowserHiddenAnalysisLoaderRequestIdSchema>;
export type BrowserHiddenAnalysisLoaderResultId = Infer<typeof BrowserHiddenAnalysisLoaderResultIdSchema>;
export type BrowserHiddenAnalysisLoaderCapabilityState = Infer<typeof BrowserHiddenAnalysisLoaderCapabilityStateSchema>;
export type BrowserHiddenAnalysisLoaderRequest = Infer<typeof BrowserHiddenAnalysisLoaderRequestSchema>;
export type BrowserHiddenAnalysisLoaderResult = Infer<typeof BrowserHiddenAnalysisLoaderResultSchema>;

export function planBrowserHiddenAnalysisLoader(input: unknown): BrowserHiddenAnalysisLoaderResult {
  const request = decodeBrowserHiddenAnalysisLoaderRequest(input);
  const result = hiddenAnalysisLoaderResultForRequest(request);

  return decodeBrowserHiddenAnalysisLoaderResult(result);
}

function browserHiddenAnalysisLoaderRequestIsConsistent(
  value: Infer<typeof BrowserHiddenAnalysisLoaderRequestBaseSchema>
) {
  if (!value.policyAllowsHiddenAnalysis) {
    return value.capabilityState === 'disabled-by-policy';
  }
  if (value.capabilityState === 'available') {
    return value.profileDesign.state === 'queued' || value.profileDesign.state === 'loading';
  }
  return true;
}

function browserHiddenAnalysisLoaderResultIsConsistent(
  value: Infer<typeof BrowserHiddenAnalysisLoaderResultBaseSchema>
) {
  if (value.pageBodyCaptured || value.transcriptTextCaptured || value.sourceEvidenceIds.length === 0) {
    return false;
  }
  if (BrowserHiddenAnalysisLoadedStates.includes(value.state)) {
    return loadedHiddenAnalysisResultIsConsistent(value);
  }
  if (BrowserHiddenAnalysisPendingStates.includes(value.state)) {
    return pendingHiddenAnalysisResultIsConsistent(value);
  }
  return degradedHiddenAnalysisResultIsConsistent(value);
}

type BrowserHiddenAnalysisLoaderResultCandidate = Infer<typeof BrowserHiddenAnalysisLoaderResultBaseSchema>;

const BrowserHiddenAnalysisLoadedStates: ReadonlyArray<BrowserHiddenAnalysisLoaderResultCandidate['state']> = [
  'metadata-only',
  'analysis-ready',
] as const;

const BrowserHiddenAnalysisPendingStates: ReadonlyArray<BrowserHiddenAnalysisLoaderResultCandidate['state']> = [
  'queued',
  'loading',
] as const;

function loadedHiddenAnalysisResultIsConsistent(value: BrowserHiddenAnalysisLoaderResultCandidate): boolean {
  return value.loadedByHiddenAdapter && value.loaderProofRef !== null && value.degradedReasons.length === 0;
}

function pendingHiddenAnalysisResultIsConsistent(value: BrowserHiddenAnalysisLoaderResultCandidate): boolean {
  return !value.loadedByHiddenAdapter && value.loaderProofRef === null && value.degradedReasons.length === 0;
}

function degradedHiddenAnalysisResultIsConsistent(value: BrowserHiddenAnalysisLoaderResultCandidate): boolean {
  return !value.loadedByHiddenAdapter && value.loaderProofRef === null && value.degradedReasons.length > 0;
}

function hiddenAnalysisLoaderResultForRequest(request: BrowserHiddenAnalysisLoaderRequest) {
  if (!request.policyAllowsHiddenAnalysis) {
    return manualRequiredLoaderResult(request, ['disabled-by-policy']);
  }
  if (request.capabilityState === 'unavailable') {
    return manualRequiredLoaderResult(request, ['loader-unavailable']);
  }
  if (request.capabilityState === 'manual-required') {
    return manualRequiredLoaderResult(request, ['manual-required']);
  }

  return {
    schemaVersion: BrowserHiddenAnalysisLoaderSchemaVersion,
    loaderResultId: `${request.loaderRequestId}-result`,
    loaderRequestId: request.loaderRequestId,
    producedAt: request.requestedAt,
    profileDesignId: request.profileDesign.designId,
    sourceEvidenceIds: request.profileDesign.sourceEvidenceIds,
    state: 'loading',
    loaderProofRef: null,
    degradedReasons: [],
    loadedByHiddenAdapter: false,
    pageBodyCaptured: false,
    transcriptTextCaptured: false,
  };
}

function manualRequiredLoaderResult(
  request: BrowserHiddenAnalysisLoaderRequest,
  degradedReasons: Array<Infer<typeof BrowserHiddenAnalysisDegradedReasonSchema>>
) {
  return {
    schemaVersion: BrowserHiddenAnalysisLoaderSchemaVersion,
    loaderResultId: `${request.loaderRequestId}-result`,
    loaderRequestId: request.loaderRequestId,
    producedAt: request.requestedAt,
    profileDesignId: request.profileDesign.designId,
    sourceEvidenceIds: request.profileDesign.sourceEvidenceIds,
    state: 'manual-required',
    loaderProofRef: null,
    degradedReasons,
    loadedByHiddenAdapter: false,
    pageBodyCaptured: false,
    transcriptTextCaptured: false,
  };
}

