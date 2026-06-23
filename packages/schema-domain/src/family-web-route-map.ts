import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';
import {
  FamilyWebCollectionCoverage,
  FamilyWebCollectionModeSchema,
  FamilyWebCollectionStateByMode,
  FamilyWebCollectionStateSchema,
  FamilyWebCopyConstraintSchema,
  FamilyWebDeploymentPreviewStateSchema,
  FamilyWebDeploymentRuntimeStateSchema,
  FamilyWebDeploymentSurfaceSchema,
  FamilyWebDeploymentTargetSchema,
  FamilyWebLinkTargetSchema,
  FamilyWebLinksByPage,
  FamilyWebNonClaimSchema,
  FamilyWebPagePurposeSchema,
  FamilyWebPageSchema,
  FamilyWebPageStateSchema,
  FamilyWebProductionHostSchema,
  FamilyWebPurposeByPage,
  FamilyWebReferenceSchema,
  FamilyWebRegistrationHandoffPlanSchema,
  FamilyWebRegistrationHandoffStateSchema,
  FamilyWebRequirementSchema,
  FamilyWebRouteMapSchemaVersionSchema,
  FamilyWebRoutePathByPage,
  FamilyWebRoutePathSchema,
  FamilyWebSourceProofSchema,
  RequiredFamilyWebCollectionModes,
  RequiredFamilyWebCopyConstraints,
  RequiredFamilyWebNonClaims,
  RequiredFamilyWebPages,
} from './family-web-route-map-values';

type FamilyWebRouteMapCandidate = {
  readonly pages: ReadonlyArray<{
    readonly page: string;
    readonly routePath: string;
    readonly pagePurpose: string;
    readonly routeState: string;
    readonly linkTargets: ReadonlyArray<string>;
  }>;
  readonly collectionPolicies: ReadonlyArray<{
    readonly collectionMode: string;
    readonly pageCoverage: ReadonlyArray<string>;
    readonly collectionState: string;
  }>;
  readonly copyConstraints: ReadonlyArray<string>;
  readonly nonClaims: ReadonlyArray<string>;
  readonly deployment: {
    readonly publicHost: string;
    readonly surfaceShape: string;
    readonly deploymentTarget: string;
    readonly previewUrlState: string;
    readonly publicRuntimeState: string;
  };
  readonly registrationHandoff: {
    readonly entryPage: string;
    readonly owningPlan: string;
    readonly handoffState: string;
    readonly localCaptureState: string;
    readonly allowedCollectionModes: ReadonlyArray<string>;
    readonly forbiddenCollectionModes: ReadonlyArray<string>;
  };
};

export const FamilyWebPageRouteSchema = withParser(
  Schema.Struct({
    schemaVersion: FamilyWebRouteMapSchemaVersionSchema,
    page: FamilyWebPageSchema,
    routePath: FamilyWebRoutePathSchema,
    pagePurpose: FamilyWebPagePurposeSchema,
    routeState: FamilyWebPageStateSchema,
    linkTargets: Schema.Array(FamilyWebLinkTargetSchema),
    sourceProof: FamilyWebSourceProofSchema,
    statusReference: FamilyWebReferenceSchema,
    manualRequirement: FamilyWebRequirementSchema,
  }).pipe(
    Schema.filter((row) =>
      row.routeState !== 'implemented' && row.routeState !== 'executed'
        ? true
        : 'Expected family web routes to stay source-contract only in this slice'
    ),
    Schema.filter((row) =>
      row.linkTargets.length > 0 && row.linkTargets.every((target) => target !== row.page)
        ? true
        : 'Expected family web routes to link to other public pages without self links'
    )
  )
);

export const FamilyWebDataCollectionPolicySchema = withParser(
  Schema.Struct({
    schemaVersion: FamilyWebRouteMapSchemaVersionSchema,
    collectionMode: FamilyWebCollectionModeSchema,
    pageCoverage: Schema.Array(FamilyWebPageSchema),
    collectionState: FamilyWebCollectionStateSchema,
    disclosureBoundary: FamilyWebReferenceSchema,
    sourceProof: FamilyWebSourceProofSchema,
    manualRequirement: FamilyWebRequirementSchema,
  }).pipe(
    Schema.filter((row) =>
      row.collectionMode !== 'forbidden-child-data' || row.collectionState === 'forbidden'
        ? true
        : 'Expected child data collection rows to stay forbidden on the public family surface'
    )
  )
);

export const FamilyWebDeploymentShapeSchema = withParser(
  Schema.Struct({
    schemaVersion: FamilyWebRouteMapSchemaVersionSchema,
    publicHost: FamilyWebProductionHostSchema,
    surfaceShape: FamilyWebDeploymentSurfaceSchema,
    deploymentTarget: FamilyWebDeploymentTargetSchema,
    previewUrlState: FamilyWebDeploymentPreviewStateSchema,
    publicRuntimeState: FamilyWebDeploymentRuntimeStateSchema,
    sourceProof: FamilyWebSourceProofSchema,
    manualRequirement: FamilyWebRequirementSchema,
  }).pipe(
    Schema.filter((deployment) =>
      deployment.publicRuntimeState !== 'implemented'
        ? true
        : 'Expected family.ocentra.ca deployment shape to avoid runtime implementation claims'
    )
  )
);

export const FamilyWebRegistrationHandoffSchema = withParser(
  Schema.Struct({
    schemaVersion: FamilyWebRouteMapSchemaVersionSchema,
    entryPage: FamilyWebPageSchema,
    owningPlan: FamilyWebRegistrationHandoffPlanSchema,
    handoffState: FamilyWebRegistrationHandoffStateSchema,
    localCaptureState: FamilyWebRegistrationHandoffStateSchema,
    allowedCollectionModes: Schema.Array(FamilyWebCollectionModeSchema),
    forbiddenCollectionModes: Schema.Array(FamilyWebCollectionModeSchema),
    handoffReference: FamilyWebReferenceSchema,
    manualRequirement: FamilyWebRequirementSchema,
  }).pipe(
    Schema.filter((handoff) =>
      handoff.localCaptureState !== 'implemented'
        ? true
        : 'Expected register/login to hand off to account identity instead of claiming local auth implementation'
    )
  )
);

export const FamilyWebRouteMapSchema = withParser(
  Schema.Struct({
    schemaVersion: FamilyWebRouteMapSchemaVersionSchema,
    pages: Schema.Array(FamilyWebPageRouteSchema),
    collectionPolicies: Schema.Array(FamilyWebDataCollectionPolicySchema),
    deployment: FamilyWebDeploymentShapeSchema,
    registrationHandoff: FamilyWebRegistrationHandoffSchema,
    copyConstraints: Schema.Array(FamilyWebCopyConstraintSchema),
    nonClaims: Schema.Array(FamilyWebNonClaimSchema),
    updatedAt: ParentTimestampSchema,
  }).pipe(
    Schema.filter((proof) =>
      familyWebRouteMapIsHonest(proof)
        ? true
        : 'Expected family web route map proof to cover required pages, privacy boundaries, deployment shape, and registration handoff'
    )
  )
);

export type FamilyWebPageRoute = Infer<typeof FamilyWebPageRouteSchema>;
export type FamilyWebDataCollectionPolicy = Infer<typeof FamilyWebDataCollectionPolicySchema>;
export type FamilyWebDeploymentShape = Infer<typeof FamilyWebDeploymentShapeSchema>;
export type FamilyWebRegistrationHandoff = Infer<typeof FamilyWebRegistrationHandoffSchema>;
export type FamilyWebRouteMap = Infer<typeof FamilyWebRouteMapSchema>;
export type FamilyWebPage = Infer<typeof FamilyWebPageSchema>;
export type FamilyWebCollectionMode = Infer<typeof FamilyWebCollectionModeSchema>;

export const decodeFamilyWebRouteMap = Schema.decodeUnknownSync(FamilyWebRouteMapSchema);

export function mapFamilyWebRoutes(
  rows: ReadonlyArray<FamilyWebPageRoute>
): Record<FamilyWebPage, FamilyWebPageRoute['routePath']> {
  return RequiredFamilyWebPages.reduce(
    (summary, page) => ({
      ...summary,
      [page]: requiredPage(rows, page).routePath,
    }),
    {} as Record<FamilyWebPage, FamilyWebPageRoute['routePath']>
  );
}

export function mapFamilyWebLinks(
  rows: ReadonlyArray<FamilyWebPageRoute>
): Record<FamilyWebPage, ReadonlyArray<FamilyWebPage>> {
  return RequiredFamilyWebPages.reduce(
    (summary, page) => ({
      ...summary,
      [page]: requiredPage(rows, page).linkTargets,
    }),
    {} as Record<FamilyWebPage, ReadonlyArray<FamilyWebPage>>
  );
}

export function summarizeFamilyWebCollectionPolicies(
  rows: ReadonlyArray<FamilyWebDataCollectionPolicy>
): Record<FamilyWebCollectionMode, FamilyWebDataCollectionPolicy['collectionState']> {
  return RequiredFamilyWebCollectionModes.reduce(
    (summary, collectionMode) => ({
      ...summary,
      [collectionMode]: requiredCollectionPolicy(rows, collectionMode).collectionState,
    }),
    {} as Record<FamilyWebCollectionMode, FamilyWebDataCollectionPolicy['collectionState']>
  );
}

function familyWebRouteMapIsHonest(proof: FamilyWebRouteMapCandidate): boolean {
  return (
    familyWebPagesMatch(proof) &&
    familyWebCollectionPoliciesMatch(proof) &&
    familyWebConstraintsAndNonClaimsMatch(proof) &&
    familyWebDeploymentIsHonest(proof) &&
    familyWebRegistrationHandoffIsHonest(proof)
  );
}

function familyWebPagesMatch(proof: FamilyWebRouteMapCandidate): boolean {
  return (
    proof.pages.length === RequiredFamilyWebPages.length &&
    RequiredFamilyWebPages.every((page) => {
      const row = proof.pages.find((entry) => entry.page === page);
      return (
        row !== undefined &&
        row.routeState === 'route-contract-only' &&
        row.routePath === FamilyWebRoutePathByPage[page] &&
        row.pagePurpose === FamilyWebPurposeByPage[page] &&
        orderedEqual(row.linkTargets, FamilyWebLinksByPage[page])
      );
    })
  );
}

function familyWebCollectionPoliciesMatch(proof: FamilyWebRouteMapCandidate): boolean {
  return (
    proof.collectionPolicies.length === RequiredFamilyWebCollectionModes.length &&
    RequiredFamilyWebCollectionModes.every((collectionMode) => {
      const row = proof.collectionPolicies.find((entry) => entry.collectionMode === collectionMode);
      return (
        row !== undefined &&
        orderedEqual(row.pageCoverage, FamilyWebCollectionCoverage[collectionMode]) &&
        row.collectionState === FamilyWebCollectionStateByMode[collectionMode]
      );
    })
  );
}

function familyWebConstraintsAndNonClaimsMatch(proof: FamilyWebRouteMapCandidate): boolean {
  return (
    proof.copyConstraints.length === RequiredFamilyWebCopyConstraints.length &&
    includesAll(proof.copyConstraints, RequiredFamilyWebCopyConstraints) &&
    proof.nonClaims.length === RequiredFamilyWebNonClaims.length &&
    includesAll(proof.nonClaims, RequiredFamilyWebNonClaims)
  );
}

function familyWebDeploymentIsHonest(proof: FamilyWebRouteMapCandidate): boolean {
  return (
    proof.deployment.publicHost === 'family.ocentra.ca' &&
    proof.deployment.surfaceShape === 'separate-vite-app' &&
    proof.deployment.deploymentTarget === 'cloudflare-pages-or-workers' &&
    proof.deployment.previewUrlState === 'preview-url-required' &&
    proof.deployment.publicRuntimeState === 'not-implemented'
  );
}

function familyWebRegistrationHandoffIsHonest(proof: FamilyWebRouteMapCandidate): boolean {
  return (
    proof.registrationHandoff.entryPage === 'register-login' &&
    proof.registrationHandoff.owningPlan === 'account-identity-family-plan' &&
    proof.registrationHandoff.handoffState === 'account-handoff-required' &&
    proof.registrationHandoff.localCaptureState === 'not-implemented' &&
    orderedEqual(proof.registrationHandoff.allowedCollectionModes, ['explicit-account-data']) &&
    orderedEqual(proof.registrationHandoff.forbiddenCollectionModes, ['forbidden-child-data'])
  );
}

function requiredPage(rows: ReadonlyArray<FamilyWebPageRoute>, page: FamilyWebPage): FamilyWebPageRoute {
  const row = rows.find((entry) => entry.page === page);
  if (row === undefined) {
    throw new Error(`missing family web route row: ${page}`);
  }
  return row;
}

function requiredCollectionPolicy(
  rows: ReadonlyArray<FamilyWebDataCollectionPolicy>,
  collectionMode: FamilyWebCollectionMode
): FamilyWebDataCollectionPolicy {
  const row = rows.find((entry) => entry.collectionMode === collectionMode);
  if (row === undefined) {
    throw new Error(`missing family web data collection row: ${collectionMode}`);
  }
  return row;
}

function orderedEqual(left: ReadonlyArray<string>, right: ReadonlyArray<string>): boolean {
  return JSON.stringify(left) === JSON.stringify(right);
}

function includesAll(left: ReadonlyArray<string>, right: ReadonlyArray<string>): boolean {
  return right.every((entry) => left.includes(entry));
}
