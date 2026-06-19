import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ActivityEvidenceRefSchema } from '@ocentra-parent/evidence-domain/contracts';
import {
  AppGameIdentityIdSchema,
  AppGameParentLabelSchema,
  AppGameProductKindSchema,
} from './app-game-identity-primitives';
import {
  type AppGameCategorySourceKind,
  AppGameCategoryCandidateIdSchema,
  AppGameCategoryCandidateStateSchema,
  AppGameCategoryEnforcementStateSchema,
  AppGameCategoryFamilySchema,
  AppGameCategoryPolicyCandidateActionSchema,
  AppGameCategoryReasonCodeSchema,
  AppGameCategorySourceKindSchema,
  AppGameCategorySourceRefSchema,
  AppGameContextSignalKindSchema,
  AppGameNativeAppCategorySchema,
  AppGameNativeGameCategorySchema,
  AppGameRiskSignalKindSchema,
} from './app-game-category-risk-primitives';
import {
  AppGameAiDigestRefSchema,
  AppGameCatalogRefSchema,
  AppGameConfidenceSchema,
  AppGameInventoryEntryIdSchema,
  AppGameSchemaVersion,
} from './app-game-primitives';

export const AppGameCategoryParentOverrideSchema = withParser(
  Schema.Struct({
    parentLabel: AppGameParentLabelSchema,
    displayFamily: AppGameCategoryFamilySchema,
    displayNativeAppCategory: Schema.Union(AppGameNativeAppCategorySchema, Schema.Null),
    displayNativeGameCategory: Schema.Union(AppGameNativeGameCategorySchema, Schema.Null),
    displayRiskSignal: Schema.Union(AppGameRiskSignalKindSchema, Schema.Null),
    displayContextSignal: Schema.Union(AppGameContextSignalKindSchema, Schema.Null),
    policyCandidateAction: AppGameCategoryPolicyCandidateActionSchema,
    rawIdentityChanged: Schema.Literal(false),
  })
);

const AppGameCategoryCandidateBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(AppGameSchemaVersion),
  candidateId: AppGameCategoryCandidateIdSchema,
  inventoryEntryId: Schema.Union(AppGameInventoryEntryIdSchema, Schema.Null),
  identityId: Schema.Union(AppGameIdentityIdSchema, Schema.Null),
  productKind: AppGameProductKindSchema,
  categoryFamily: AppGameCategoryFamilySchema,
  nativeAppCategory: Schema.Union(AppGameNativeAppCategorySchema, Schema.Null),
  nativeGameCategory: Schema.Union(AppGameNativeGameCategorySchema, Schema.Null),
  riskSignal: Schema.Union(AppGameRiskSignalKindSchema, Schema.Null),
  gameContextSignal: Schema.Union(AppGameContextSignalKindSchema, Schema.Null),
  sourceKind: AppGameCategorySourceKindSchema,
  sourceRef: AppGameCategorySourceRefSchema,
  candidateState: AppGameCategoryCandidateStateSchema,
  confidence: AppGameConfidenceSchema,
  reasonCode: AppGameCategoryReasonCodeSchema,
  evidence: Schema.Array(ActivityEvidenceRefSchema),
  catalogRef: Schema.Union(AppGameCatalogRefSchema, Schema.Null),
  aiDigestRef: Schema.Union(AppGameAiDigestRefSchema, Schema.Null),
  parentOverride: Schema.Union(AppGameCategoryParentOverrideSchema, Schema.Null),
  policyCandidateAction: AppGameCategoryPolicyCandidateActionSchema,
  enforcementState: AppGameCategoryEnforcementStateSchema,
});

type AppGameCategoryCandidateBase = Infer<typeof AppGameCategoryCandidateBaseSchema>;

export const AppGameCategoryCandidateSchema = withParser(
  AppGameCategoryCandidateBaseSchema.pipe(
    Schema.filter(
      (candidate) =>
        appGameCategoryCandidateHasEvidence(candidate) || 'Expected category candidates to cite source evidence'
    )
  )
    .pipe(
      Schema.filter(
        (candidate) =>
          appGameCategoryFamilyHasOneMatchingValue(candidate) ||
          'Expected category family to match exactly one taxonomy value'
      )
    )
    .pipe(
      Schema.filter(
        (candidate) =>
          appGameCategoryCandidateStateMatchesSource(candidate) ||
          'Expected category candidate state to match the source strength'
      )
    )
    .pipe(
      Schema.filter(
        (candidate) =>
          appGameCategoryAiCandidateIsReviewOnly(candidate) ||
          'Expected AI category candidates to cite a digest and stay review-only'
      )
    )
    .pipe(
      Schema.filter(
        (candidate) =>
          appGameCategoryRiskCandidateIsNotEnforcement(candidate) ||
          'Expected risk labels to stay policy inputs, not enforcement decisions'
      )
    )
);

export const AppGameCategoryCandidateSetSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(AppGameSchemaVersion),
    candidates: Schema.Array(AppGameCategoryCandidateSchema),
  }).pipe(
    Schema.filter(
      (candidateSet) =>
        appGameCategoryCandidateIdsAreUnique(candidateSet.candidates) || 'Expected category candidate ids to be unique'
    )
  )
);

export type AppGameCategoryParentOverride = Infer<typeof AppGameCategoryParentOverrideSchema>;
export type AppGameCategoryCandidate = Infer<typeof AppGameCategoryCandidateSchema>;
export type AppGameCategoryCandidateSet = Infer<typeof AppGameCategoryCandidateSetSchema>;

const AppGameCategoryCandidateSourceStateValidators = {
  catalog: (candidate: AppGameCategoryCandidateBase) =>
    candidate.candidateState === 'catalogCandidate' && candidate.catalogRef !== null,
  storeMetadata: (candidate: AppGameCategoryCandidateBase) => appGameCategoryCandidateStateIsObserved(candidate),
  launcherManifest: (candidate: AppGameCategoryCandidateBase) => appGameCategoryCandidateStateIsObserved(candidate),
  parentLabel: (candidate: AppGameCategoryCandidateBase) =>
    candidate.candidateState === 'parentDisplayOverride' && candidate.parentOverride !== null,
  localAi: (candidate: AppGameCategoryCandidateBase) => candidate.candidateState === 'aiCandidate',
  processMetadata: (candidate: AppGameCategoryCandidateBase) => appGameCategoryCandidateStateIsObserved(candidate),
  executableName: (candidate: AppGameCategoryCandidateBase) =>
    candidate.candidateState === 'nameHeuristicCandidate' || candidate.candidateState === 'unknownCandidate',
  managedDevice: (candidate: AppGameCategoryCandidateBase) => appGameCategoryCandidateStateIsObserved(candidate),
  manualReview: (candidate: AppGameCategoryCandidateBase) => candidate.candidateState === 'manualReviewCandidate',
} satisfies Record<AppGameCategorySourceKind, (candidate: AppGameCategoryCandidateBase) => boolean>;

function appGameCategoryCandidateHasEvidence(candidate: AppGameCategoryCandidateBase): boolean {
  return candidate.evidence.length > 0;
}

function appGameCategoryFamilyHasOneMatchingValue(candidate: AppGameCategoryCandidateBase): boolean {
  const slotCount = [
    candidate.nativeAppCategory,
    candidate.nativeGameCategory,
    candidate.riskSignal,
    candidate.gameContextSignal,
  ].filter((slot) => slot !== null).length;

  if (slotCount !== 1) {
    return false;
  }

  switch (candidate.categoryFamily) {
    case 'nativeApp':
      return candidate.nativeAppCategory !== null;
    case 'nativeGame':
      return candidate.nativeGameCategory !== null;
    case 'riskCandidate':
      return candidate.riskSignal !== null;
    case 'gameContext':
      return candidate.gameContextSignal !== null;
  }
}

function appGameCategoryCandidateStateMatchesSource(candidate: AppGameCategoryCandidateBase): boolean {
  return AppGameCategoryCandidateSourceStateValidators[candidate.sourceKind](candidate);
}

function appGameCategoryAiCandidateIsReviewOnly(candidate: AppGameCategoryCandidateBase): boolean {
  if (candidate.sourceKind !== 'localAi') {
    return true;
  }

  return (
    candidate.aiDigestRef !== null &&
    candidate.enforcementState === 'notEnforcement' &&
    (candidate.policyCandidateAction === 'none' ||
      candidate.policyCandidateAction === 'observe' ||
      candidate.policyCandidateAction === 'warn' ||
      candidate.policyCandidateAction === 'askParent' ||
      candidate.policyCandidateAction === 'manualReview')
  );
}

function appGameCategoryRiskCandidateIsNotEnforcement(candidate: AppGameCategoryCandidateBase): boolean {
  return candidate.categoryFamily !== 'riskCandidate' || candidate.enforcementState === 'notEnforcement';
}

function appGameCategoryCandidateStateIsObserved(candidate: AppGameCategoryCandidateBase): boolean {
  return candidate.candidateState !== 'parentDisplayOverride' && candidate.candidateState !== 'aiCandidate';
}

function appGameCategoryCandidateIdsAreUnique(candidates: ReadonlyArray<AppGameCategoryCandidate>): boolean {
  const ids = candidates.map((candidate) => candidate.candidateId);

  return ids.length === new Set(ids).size;
}
