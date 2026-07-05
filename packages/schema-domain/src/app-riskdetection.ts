/* thin adapter over Rust-generated app risk detection contracts */

import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from './effect';
import { ParentEvidenceReferenceSchema } from './family-references';
import {
  ParentContractSchemaVersionSchema,
  ParentPlatformSchema,
  ParentTimestampSchema,
} from './family-reference-primitives';
import {
  AppRiskDetectionContractRuntime,
  GeneratedAppRiskDetectionAskParentRouting,
  GeneratedAppRiskDetectionAskParentRoutingValues,
  GeneratedAppRiskDetectionCandidateState,
  GeneratedAppRiskDetectionCandidateStateValues,
  GeneratedAppRiskDetectionConfidenceBand,
  GeneratedAppRiskDetectionConfidenceBandValues,
  GeneratedAppRiskDetectionMatrix as GeneratedAppRiskDetectionMatrixReadModel,
  GeneratedAppRiskDetectionNoContentClaimState,
  GeneratedAppRiskDetectionNoContentClaimStateValues,
  GeneratedAppRiskDetectionPolicyCandidateAction,
  GeneratedAppRiskDetectionPolicyCandidateActionValues,
  GeneratedAppRiskDetectionPolicyTargetKind,
  GeneratedAppRiskDetectionPolicyTargetKindValues,
  GeneratedAppRiskDetectionPublisherTrustState,
  GeneratedAppRiskDetectionPublisherTrustStateValues,
  GeneratedAppRiskDetectionRiskSignal,
  GeneratedAppRiskDetectionRiskSignalValues,
  GeneratedAppRiskDetectionSourceKind,
  GeneratedAppRiskDetectionSourceKindValues,
  GeneratedAppRiskDetectionSurfaceState,
  GeneratedAppRiskDetectionSurfaceStateValues,
  type GeneratedAppRiskDetectionCandidate,
  type GeneratedAppRiskDetectionMatrix,
} from './generated-app-riskdetection-contracts';
import { appRiskDetectionCandidateIsHonestGenerated } from './generated-app-riskdetection-contract-rules';

export const AppRiskDetectionContractSchemaVersionSchema = withParser(
  Schema.Literal(AppRiskDetectionContractRuntime.SchemaVersion)
);

export const AppRiskDetectionCandidateIdSchema = brandedNonEmptyStringSchema('AppRiskDetectionCandidateId');
export const AppRiskDetectionMatrixIdSchema = brandedNonEmptyStringSchema('AppRiskDetectionMatrixId');
export const AppRiskDetectionInventoryEntryRefSchema = brandedNonEmptyStringSchema('AppRiskDetectionInventoryEntryRef');
export const AppRiskDetectionIdentityRefSchema = brandedNonEmptyStringSchema('AppRiskDetectionIdentityRef');
export const AppRiskDetectionSourceRefSchema = brandedNonEmptyStringSchema('AppRiskDetectionSourceRef');
export const AppRiskDetectionLocalAiDigestRefSchema = brandedNonEmptyStringSchema('AppRiskDetectionLocalAiDigestRef');
export const AppRiskDetectionParentDisplayLabelSchema = brandedNonEmptyStringSchema(
  'AppRiskDetectionParentDisplayLabel'
);

export const AppRiskDetectionRiskSignalSchema = withParser(
  Schema.Literal(...GeneratedAppRiskDetectionRiskSignalValues)
);
export const AppRiskDetectionSourceKindSchema = withParser(
  Schema.Literal(...GeneratedAppRiskDetectionSourceKindValues)
);
export const AppRiskDetectionCandidateStateSchema = withParser(
  Schema.Literal(...GeneratedAppRiskDetectionCandidateStateValues)
);
export const AppRiskDetectionPublisherTrustStateSchema = withParser(
  Schema.Literal(...GeneratedAppRiskDetectionPublisherTrustStateValues)
);
export const AppRiskDetectionPolicyCandidateActionSchema = withParser(
  Schema.Literal(...GeneratedAppRiskDetectionPolicyCandidateActionValues)
);
export const AppRiskDetectionConfidenceBandSchema = withParser(
  Schema.Literal(...GeneratedAppRiskDetectionConfidenceBandValues)
);
export const AppRiskDetectionPolicyTargetKindSchema = withParser(
  Schema.Literal(...GeneratedAppRiskDetectionPolicyTargetKindValues)
);
export const AppRiskDetectionAskParentRoutingSchema = withParser(
  Schema.Literal(...GeneratedAppRiskDetectionAskParentRoutingValues)
);
export const AppRiskDetectionSurfaceStateSchema = withParser(
  Schema.Literal(...GeneratedAppRiskDetectionSurfaceStateValues)
);
export const AppRiskDetectionNoContentClaimStateSchema = withParser(
  Schema.Literal(...GeneratedAppRiskDetectionNoContentClaimStateValues)
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
        appRiskDetectionCandidateIsHonestGenerated(candidate as GeneratedAppRiskDetectionCandidate) ||
        'Expected native app risk candidates to cite evidence, stay advisory, disclose confidence/source, and avoid direct enforcement'
    )
  )
);

export const AppRiskDetectionMatrixSchema = withParser(
  Schema.Struct({
    schemaVersion: AppRiskDetectionContractSchemaVersionSchema,
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

export type AppRiskDetectionRiskSignal = GeneratedAppRiskDetectionRiskSignal;
export type AppRiskDetectionSourceKind = GeneratedAppRiskDetectionSourceKind;
export type AppRiskDetectionCandidateState = GeneratedAppRiskDetectionCandidateState;
export type AppRiskDetectionPublisherTrustState = GeneratedAppRiskDetectionPublisherTrustState;
export type AppRiskDetectionPolicyCandidateAction = GeneratedAppRiskDetectionPolicyCandidateAction;
export type AppRiskDetectionConfidenceBand = GeneratedAppRiskDetectionConfidenceBand;
export type AppRiskDetectionCandidate = Infer<typeof AppRiskDetectionCandidateSchema> &
  GeneratedAppRiskDetectionCandidate;
export type AppRiskDetectionMatrix = Infer<typeof AppRiskDetectionMatrixSchema> & GeneratedAppRiskDetectionMatrix;

export const decodeAppRiskDetectionCandidate = Schema.decodeUnknownSync(AppRiskDetectionCandidateSchema);
export const decodeAppRiskDetectionMatrix = Schema.decodeUnknownSync(AppRiskDetectionMatrixSchema);
export const AppRiskDetectionMatrixReadModel = AppRiskDetectionMatrixSchema.parse(
  GeneratedAppRiskDetectionMatrixReadModel
);

export const AppRiskDetectionRiskSignal = GeneratedAppRiskDetectionRiskSignal;
export const AppRiskDetectionSourceKind = GeneratedAppRiskDetectionSourceKind;
export const AppRiskDetectionCandidateState = GeneratedAppRiskDetectionCandidateState;
export const AppRiskDetectionPublisherTrustState = GeneratedAppRiskDetectionPublisherTrustState;
export const AppRiskDetectionPolicyCandidateAction = GeneratedAppRiskDetectionPolicyCandidateAction;
export const AppRiskDetectionConfidenceBand = GeneratedAppRiskDetectionConfidenceBand;
export const AppRiskDetectionPolicyTargetKind = GeneratedAppRiskDetectionPolicyTargetKind;
export const AppRiskDetectionAskParentRouting = GeneratedAppRiskDetectionAskParentRouting;
export const AppRiskDetectionSurfaceState = GeneratedAppRiskDetectionSurfaceState;
export const AppRiskDetectionNoContentClaimState = GeneratedAppRiskDetectionNoContentClaimState;
