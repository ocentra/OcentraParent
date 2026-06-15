import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentEvidenceReferenceSchema } from '@ocentra-parent/family-domain/references';
import {
  AppRiskDetectionAskParentRouting,
  AppRiskDetectionCandidateState,
  AppRiskDetectionConfidenceBand,
  AppRiskDetectionNoContentClaimState,
  AppRiskDetectionPolicyCandidateAction,
  AppRiskDetectionPolicyTargetKind,
  AppRiskDetectionPublisherTrustState,
  AppRiskDetectionRiskSignal,
  AppRiskDetectionSourceKind,
  AppRiskDetectionSurfaceState,
  appRiskDetectionCandidateIsHonest,
} from './app-riskdetection-rules';
import { ParentContractSchemaVersionSchema, ParentPlatformSchema, ParentTimestampSchema } from '@ocentra-parent/family-domain/reference-primitives';

const NonEmptyAppRiskDetectionText = Schema.String.pipe(Schema.minLength(1));

export const AppRiskDetectionCandidateIdSchema = NonEmptyAppRiskDetectionText.pipe(
  Schema.brand('AppRiskDetectionCandidateId')
);
export const AppRiskDetectionMatrixIdSchema = NonEmptyAppRiskDetectionText.pipe(
  Schema.brand('AppRiskDetectionMatrixId')
);
export const AppRiskDetectionInventoryEntryRefSchema = NonEmptyAppRiskDetectionText.pipe(
  Schema.brand('AppRiskDetectionInventoryEntryRef')
);
export const AppRiskDetectionIdentityRefSchema = NonEmptyAppRiskDetectionText.pipe(
  Schema.brand('AppRiskDetectionIdentityRef')
);
export const AppRiskDetectionSourceRefSchema = NonEmptyAppRiskDetectionText.pipe(
  Schema.brand('AppRiskDetectionSourceRef')
);
export const AppRiskDetectionLocalAiDigestRefSchema = NonEmptyAppRiskDetectionText.pipe(
  Schema.brand('AppRiskDetectionLocalAiDigestRef')
);
export const AppRiskDetectionParentDisplayLabelSchema = NonEmptyAppRiskDetectionText.pipe(
  Schema.brand('AppRiskDetectionParentDisplayLabel')
);

export const AppRiskDetectionRiskSignalSchema = withParser(
  Schema.Literal(...Object.values(AppRiskDetectionRiskSignal))
);
export const AppRiskDetectionSourceKindSchema = withParser(
  Schema.Literal(...Object.values(AppRiskDetectionSourceKind))
);
export const AppRiskDetectionCandidateStateSchema = withParser(
  Schema.Literal(...Object.values(AppRiskDetectionCandidateState))
);
export const AppRiskDetectionPublisherTrustStateSchema = withParser(
  Schema.Literal(...Object.values(AppRiskDetectionPublisherTrustState))
);
export const AppRiskDetectionPolicyCandidateActionSchema = withParser(
  Schema.Literal(...Object.values(AppRiskDetectionPolicyCandidateAction))
);
export const AppRiskDetectionConfidenceBandSchema = withParser(
  Schema.Literal(...Object.values(AppRiskDetectionConfidenceBand))
);
export const AppRiskDetectionPolicyTargetKindSchema = withParser(
  Schema.Literal(...Object.values(AppRiskDetectionPolicyTargetKind))
);
export const AppRiskDetectionAskParentRoutingSchema = withParser(
  Schema.Literal(...Object.values(AppRiskDetectionAskParentRouting))
);
export const AppRiskDetectionSurfaceStateSchema = withParser(
  Schema.Literal(...Object.values(AppRiskDetectionSurfaceState))
);
export const AppRiskDetectionNoContentClaimStateSchema = withParser(
  Schema.Literal(...Object.values(AppRiskDetectionNoContentClaimState))
);

const AppRiskDetectionParentOverrideSchema = Schema.Struct({
  parentDisplayLabel: AppRiskDetectionParentDisplayLabelSchema,
  policyCandidateAction: AppRiskDetectionPolicyCandidateActionSchema,
  rawIdentityChanged: Schema.Literal(false),
});

const AppRiskDetectionSurfaceDisclosureSchema = Schema.Struct({
  surfaceState: AppRiskDetectionSurfaceStateSchema,
  confidencePercent: Schema.Number.pipe(Schema.between(0, 100)),
  sourceEvidenceCount: Schema.Number.pipe(Schema.between(1, 99)),
  noContentClaimState: AppRiskDetectionNoContentClaimStateSchema,
});

const AppRiskDetectionCandidateBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  candidateId: AppRiskDetectionCandidateIdSchema,
  platform: ParentPlatformSchema,
  inventoryEntryRef: Schema.Union(AppRiskDetectionInventoryEntryRefSchema, Schema.Null),
  identityRef: Schema.Union(AppRiskDetectionIdentityRefSchema, Schema.Null),
  riskSignal: AppRiskDetectionRiskSignalSchema,
  sourceKind: AppRiskDetectionSourceKindSchema,
  candidateState: AppRiskDetectionCandidateStateSchema,
  publisherTrustState: AppRiskDetectionPublisherTrustStateSchema,
  confidence: Schema.Number.pipe(Schema.between(0, 1)),
  confidenceBand: AppRiskDetectionConfidenceBandSchema,
  evidenceReferences: Schema.Array(ParentEvidenceReferenceSchema),
  sourceRefs: Schema.Array(AppRiskDetectionSourceRefSchema),
  localAiDigestRef: Schema.Union(AppRiskDetectionLocalAiDigestRefSchema, Schema.Null),
  parentOverride: Schema.Union(AppRiskDetectionParentOverrideSchema, Schema.Null),
  policyCandidateAction: AppRiskDetectionPolicyCandidateActionSchema,
  policyTargetKind: AppRiskDetectionPolicyTargetKindSchema,
  askParentRouting: AppRiskDetectionAskParentRoutingSchema,
  notDirectEnforcement: Schema.Boolean,
  noContentClaim: Schema.Boolean,
  surfaceDisclosure: AppRiskDetectionSurfaceDisclosureSchema,
  lastCheckedAt: ParentTimestampSchema,
});

export const AppRiskDetectionCandidateSchema = withParser(
  AppRiskDetectionCandidateBaseSchema.pipe(
    Schema.filter(
      (candidate) =>
        appRiskDetectionCandidateIsHonest(candidate) ||
        'Expected native app risk candidates to cite evidence, stay advisory, disclose confidence/source, and avoid direct enforcement'
    )
  )
);

export const AppRiskDetectionMatrixSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    matrixId: AppRiskDetectionMatrixIdSchema,
    generatedAt: ParentTimestampSchema,
    candidates: Schema.Array(AppRiskDetectionCandidateSchema),
  }).pipe(
    Schema.filter(
      (matrix) =>
        matrix.candidates.length > 0 &&
        new Set(matrix.candidates.map((candidate) => candidate.candidateId)).size === matrix.candidates.length
    )
  )
);

export type AppRiskDetectionRiskSignal = Infer<typeof AppRiskDetectionRiskSignalSchema>;
export type AppRiskDetectionSourceKind = Infer<typeof AppRiskDetectionSourceKindSchema>;
export type AppRiskDetectionCandidateState = Infer<typeof AppRiskDetectionCandidateStateSchema>;
export type AppRiskDetectionPublisherTrustState = Infer<typeof AppRiskDetectionPublisherTrustStateSchema>;
export type AppRiskDetectionPolicyCandidateAction = Infer<typeof AppRiskDetectionPolicyCandidateActionSchema>;
export type AppRiskDetectionConfidenceBand = Infer<typeof AppRiskDetectionConfidenceBandSchema>;
export type AppRiskDetectionCandidate = Infer<typeof AppRiskDetectionCandidateSchema>;
export type AppRiskDetectionMatrix = Infer<typeof AppRiskDetectionMatrixSchema>;

export const decodeAppRiskDetectionCandidate = Schema.decodeUnknownSync(AppRiskDetectionCandidateSchema);
export const decodeAppRiskDetectionMatrix = Schema.decodeUnknownSync(AppRiskDetectionMatrixSchema);
