import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  AppGameCategoryRiskPolicyAdapterDispatchState,
  AppGameCategoryRiskPolicyCandidateAction,
  AppGameCategoryRiskPolicyRouteFamily,
  AppGameCategoryRiskPolicyRouteSourceKind,
  AppGameCategoryRiskPolicyRoutingState,
  appGameCategoryRiskPolicyRouteActionMatchesCandidate,
  appGameCategoryRiskPolicyRouteKeepsSoftBoundary,
  appGameCategoryRiskPolicyRouteLocalAiRequiresDigest,
  appGameCategoryRiskPolicyRouteManualReviewRequiresManualState,
  appGameCategoryRiskPolicyRouteTargetMatchesFamily,
  appGameCategoryRiskPolicyRouteUsesCategoryProof,
} from './app-game-category-risk-policy-routing-rules';
import { AppGamePolicyCompilerEvidenceSchema, AppGamePolicyTargetRefSchema } from './app-game-policy-target-compiler';
import {
  AppGamePolicyCompilerRequestedActionSchema,
  AppGamePolicyTargetKindSchema,
} from './app-game-policy-target-compiler';
import { PolicyActionSchema } from './policy';
import { ParentEvidenceReferenceSchema } from './references';
import { ParentContractSchemaVersionSchema } from './reference-primitives';

const NonEmptyCategoryRiskPolicyRoutingText = Schema.String.pipe(Schema.minLength(1));

export const AppGameCategoryRiskPolicyRouteIdSchema = NonEmptyCategoryRiskPolicyRoutingText.pipe(
  Schema.brand('AppGameCategoryRiskPolicyRouteId')
);
export const AppGameCategoryCandidateRefSchema = NonEmptyCategoryRiskPolicyRoutingText.pipe(
  Schema.brand('AppGameCategoryCandidateRef')
);
export const AppGameCategoryRiskPolicySourceRefSchema = NonEmptyCategoryRiskPolicyRoutingText.pipe(
  Schema.brand('AppGameCategoryRiskPolicySourceRef')
);
export const AppGameCategoryRiskPolicyAiDigestRefSchema = NonEmptyCategoryRiskPolicyRoutingText.pipe(
  Schema.brand('AppGameCategoryRiskPolicyAiDigestRef')
);
export const AppGameCategoryRiskPolicyConfidenceSchema = Schema.Number.pipe(Schema.between(0, 1));

export const AppGameCategoryRiskPolicyRouteFamilySchema = withParser(
  Schema.Literal(...Object.values(AppGameCategoryRiskPolicyRouteFamily))
);
export const AppGameCategoryRiskPolicyRouteSourceKindSchema = withParser(
  Schema.Literal(...Object.values(AppGameCategoryRiskPolicyRouteSourceKind))
);
export const AppGameCategoryRiskPolicyCandidateActionSchema = withParser(
  Schema.Literal(...Object.values(AppGameCategoryRiskPolicyCandidateAction))
);
export const AppGameCategoryRiskPolicyRoutingStateSchema = withParser(
  Schema.Literal(...Object.values(AppGameCategoryRiskPolicyRoutingState))
);
export const AppGameCategoryRiskPolicyAdapterDispatchStateSchema = withParser(
  Schema.Literal(AppGameCategoryRiskPolicyAdapterDispatchState.NotDispatched)
);

export const AppGameCategoryRiskPolicyRouteSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    routeId: AppGameCategoryRiskPolicyRouteIdSchema,
    categoryCandidateRef: AppGameCategoryCandidateRefSchema,
    routeFamily: AppGameCategoryRiskPolicyRouteFamilySchema,
    sourceKind: AppGameCategoryRiskPolicyRouteSourceKindSchema,
    sourceRef: AppGameCategoryRiskPolicySourceRefSchema,
    targetKind: AppGamePolicyTargetKindSchema,
    targetRef: AppGamePolicyTargetRefSchema,
    confidence: AppGameCategoryRiskPolicyConfidenceSchema,
    candidateAction: AppGameCategoryRiskPolicyCandidateActionSchema,
    requestedAction: AppGamePolicyCompilerRequestedActionSchema,
    policyAction: PolicyActionSchema,
    routingState: AppGameCategoryRiskPolicyRoutingStateSchema,
    categoryProof: AppGamePolicyCompilerEvidenceSchema,
    supportingEvidence: Schema.Array(ParentEvidenceReferenceSchema),
    aiDigestRef: Schema.Union(AppGameCategoryRiskPolicyAiDigestRefSchema, Schema.Null),
    adapterDispatchState: AppGameCategoryRiskPolicyAdapterDispatchStateSchema,
  })
    .pipe(
      Schema.filter(
        (route) =>
          appGameCategoryRiskPolicyRouteTargetMatchesFamily(route) ||
          'Expected app/game category policy routes to target matching category families'
      )
    )
    .pipe(
      Schema.filter(
        (route) =>
          appGameCategoryRiskPolicyRouteUsesCategoryProof(route) ||
          'Expected app/game category policy routes to carry active category proof and supporting evidence'
      )
    )
    .pipe(
      Schema.filter(
        (route) =>
          appGameCategoryRiskPolicyRouteActionMatchesCandidate(route) ||
          'Expected app/game category policy route actions to match candidate actions'
      )
    )
    .pipe(
      Schema.filter(
        (route) =>
          appGameCategoryRiskPolicyRouteKeepsSoftBoundary(route) ||
          'Expected app/game category policy routes to avoid hard adapter actions'
      )
    )
    .pipe(
      Schema.filter(
        (route) =>
          appGameCategoryRiskPolicyRouteManualReviewRequiresManualState(route) ||
          'Expected manual-review category routes to stay manual-required'
      )
    )
    .pipe(
      Schema.filter(
        (route) =>
          appGameCategoryRiskPolicyRouteLocalAiRequiresDigest(route) ||
          'Expected local-AI category policy routes to cite an AI digest ref'
      )
    )
);

export type AppGameCategoryRiskPolicyRoute = Infer<typeof AppGameCategoryRiskPolicyRouteSchema>;

export {
  AppGameCategoryRiskPolicyAdapterDispatchState,
  AppGameCategoryRiskPolicyCandidateAction,
  AppGameCategoryRiskPolicyRouteFamily,
  AppGameCategoryRiskPolicyRouteSourceKind,
  AppGameCategoryRiskPolicyRoutingState,
};
