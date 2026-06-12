import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  LanPairingDeviceReachabilitySchema,
  LanPairingProductionDiscoveryStateSchema,
  LanPairingRejectionReasonSchema,
  LanPairingRouteIdSchema,
  LanPairingTrustStateSchema,
} from '@ocentra-parent/lan-domain/lan-pairing-values';
import { ParentMobileCommandAuthorityStateSchema } from './parent-mobile-runtime';
import { ParentTimestampSchema } from '@ocentra-parent/family-domain/reference-primitives';
import { V09RuntimeProofStateSchema } from './v0-9-mobile-controller-discovery-runtime';

const NonEmptyV09HouseholdPhysicalProofText = Schema.String.pipe(Schema.minLength(1));

export const V09HouseholdPhysicalProofArtifactGateIdSchema = withParser(
  Schema.Literal('v0-9-household-physical-proof-artifact-gate')
);

export const V09HouseholdPhysicalProofSourceSchema = withParser(
  Schema.Literal(
    'v0-9-household-discovery-mobile-controller-product-proof',
    'v0-9-production-discovery-household-proof',
    'v0-9-production-lan-mobile-controller-proof'
  )
);

export const V09HouseholdPhysicalProofReadinessDecisionSchema = withParser(
  Schema.Literal('manual-evidence-required-before-physical-household-lan-readiness')
);

export const V09HouseholdPhysicalProofArtifactRequirementSchema = withParser(
  Schema.Literal(
    'two-physical-household-hosts',
    'same-router-or-subnet-evidence',
    'child-service-router-reachability',
    'os-firewall-or-local-network-permission',
    'controller-origin-allowlist-artifact',
    'selected-device-route-recovery',
    'controller-observer-route-health',
    'revoked-route-rejection',
    'stale-offline-device-rejection',
    'real-mobile-controller-package',
    'manual-evidence-custody-record'
  )
);

export const V09HouseholdPhysicalProofManualEvidenceStatusSchema = withParser(
  Schema.Literal('manual-required', 'missing', 'collected', 'rejected', 'not-implemented')
);

export const V09HouseholdPhysicalProofDeviceReadinessCheckSchema = withParser(
  Schema.Literal('discovered-child-agent', 'selected-child-route', 'parent-controller-origin', 'parent-observer-route')
);

export const V09HouseholdPhysicalProofRouteHealthCheckSchema = withParser(
  Schema.Literal(
    'selected-route-accepted',
    'observer-read-only',
    'controller-takeover-manual-required',
    'revoked-route-rejected',
    'stale-offline-route-rejected'
  )
);

export const V09HouseholdPhysicalProofEvidenceCustodyStateSchema = withParser(
  Schema.Literal('not-collected', 'partial-artifacts-collected', 'ready-for-human-review', 'rejected-overclaim')
);

export const V09HouseholdPhysicalProofPathSchema = NonEmptyV09HouseholdPhysicalProofText.pipe(
  Schema.brand('V09HouseholdPhysicalProofPath')
);
export const V09HouseholdPhysicalProofCommandSchema = NonEmptyV09HouseholdPhysicalProofText.pipe(
  Schema.brand('V09HouseholdPhysicalProofCommand')
);
export const V09HouseholdPhysicalProofLabelSchema = NonEmptyV09HouseholdPhysicalProofText.pipe(
  Schema.brand('V09HouseholdPhysicalProofLabel')
);
export const V09HouseholdPhysicalProofBoundarySchema = NonEmptyV09HouseholdPhysicalProofText.pipe(
  Schema.brand('V09HouseholdPhysicalProofBoundary')
);

export const V09HouseholdPhysicalProofSourceInputSchema = withParser(
  Schema.Struct({
    source: V09HouseholdPhysicalProofSourceSchema,
    path: V09HouseholdPhysicalProofPathSchema,
    command: V09HouseholdPhysicalProofCommandSchema,
  })
);

export const V09HouseholdPhysicalProofArtifactRequirementEvidenceSchema = withParser(
  Schema.Struct({
    requirement: V09HouseholdPhysicalProofArtifactRequirementSchema,
    status: V09HouseholdPhysicalProofManualEvidenceStatusSchema,
    requiredArtifactSummary: V09HouseholdPhysicalProofBoundarySchema,
    evidencePath: Schema.Union(V09HouseholdPhysicalProofPathSchema, Schema.Null),
    evidenceCapturedAt: Schema.Union(ParentTimestampSchema, Schema.Null),
  })
);

export const V09HouseholdPhysicalProofDeviceReadinessEvidenceSchema = withParser(
  Schema.Struct({
    check: V09HouseholdPhysicalProofDeviceReadinessCheckSchema,
    routeId: Schema.Union(LanPairingRouteIdSchema, Schema.Null),
    discoveryState: LanPairingProductionDiscoveryStateSchema,
    trustState: LanPairingTrustStateSchema,
    reachability: LanPairingDeviceReachabilitySchema,
    runtimeProofState: V09RuntimeProofStateSchema,
    physicalArtifactStatus: V09HouseholdPhysicalProofManualEvidenceStatusSchema,
    evidenceLabel: V09HouseholdPhysicalProofLabelSchema,
  })
);

export const V09HouseholdPhysicalProofRouteHealthEvidenceSchema = withParser(
  Schema.Struct({
    check: V09HouseholdPhysicalProofRouteHealthCheckSchema,
    routeId: Schema.Union(LanPairingRouteIdSchema, Schema.Null),
    commandAuthorityState: ParentMobileCommandAuthorityStateSchema,
    rejectionReason: Schema.Union(LanPairingRejectionReasonSchema, Schema.Null),
    runtimeProofState: V09RuntimeProofStateSchema,
    physicalArtifactStatus: V09HouseholdPhysicalProofManualEvidenceStatusSchema,
    evidenceLabel: V09HouseholdPhysicalProofLabelSchema,
  })
);

export const V09HouseholdPhysicalProofManualEvidenceCustodySchema = withParser(
  Schema.Struct({
    custodyState: V09HouseholdPhysicalProofEvidenceCustodyStateSchema,
    requiredArtifactCount: Schema.Number,
    collectedArtifactCount: Schema.Number,
    missingArtifactCount: Schema.Number,
    reviewerSummary: V09HouseholdPhysicalProofBoundarySchema,
  })
);

const V09HouseholdPhysicalProofArtifactGateReadModelBaseSchema = Schema.Struct({
  schemaVersion: V09HouseholdPhysicalProofArtifactGateIdSchema,
  checkedAt: ParentTimestampSchema,
  readinessDecision: V09HouseholdPhysicalProofReadinessDecisionSchema,
  physicalHouseholdLanClaimState: V09RuntimeProofStateSchema,
  cloudRelayState: V09RuntimeProofStateSchema,
  sourceProofs: Schema.Array(V09HouseholdPhysicalProofSourceInputSchema),
  artifactRequirements: Schema.Array(V09HouseholdPhysicalProofArtifactRequirementEvidenceSchema),
  deviceReadiness: Schema.Array(V09HouseholdPhysicalProofDeviceReadinessEvidenceSchema),
  routeHealth: Schema.Array(V09HouseholdPhysicalProofRouteHealthEvidenceSchema),
  manualEvidenceStatus: V09HouseholdPhysicalProofManualEvidenceCustodySchema,
  claimsProved: Schema.Array(V09HouseholdPhysicalProofLabelSchema),
  claimsNotProved: Schema.Array(V09HouseholdPhysicalProofBoundarySchema),
});

type V09HouseholdPhysicalProofArtifactGateReadModelCandidate = Infer<
  typeof V09HouseholdPhysicalProofArtifactGateReadModelBaseSchema
>;

export const V09HouseholdPhysicalProofArtifactGateReadModelSchema = withParser(
  V09HouseholdPhysicalProofArtifactGateReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        householdPhysicalProofArtifactGateIsHonest(readModel) ||
        'Expected V0.9 physical household proof artifact gate to keep physical LAN readiness manual-required until real two-device evidence artifacts are collected'
    )
  )
);

const RequiredSourceProofs = [
  'v0-9-household-discovery-mobile-controller-product-proof',
  'v0-9-production-discovery-household-proof',
  'v0-9-production-lan-mobile-controller-proof',
] as const satisfies ReadonlyArray<V09HouseholdPhysicalProofSource>;

const RequiredArtifactRequirements = [
  'two-physical-household-hosts',
  'same-router-or-subnet-evidence',
  'child-service-router-reachability',
  'os-firewall-or-local-network-permission',
  'controller-origin-allowlist-artifact',
  'selected-device-route-recovery',
  'controller-observer-route-health',
  'revoked-route-rejection',
  'stale-offline-device-rejection',
  'real-mobile-controller-package',
  'manual-evidence-custody-record',
] as const satisfies ReadonlyArray<V09HouseholdPhysicalProofArtifactRequirement>;

const RequiredDeviceReadinessChecks = [
  'discovered-child-agent',
  'selected-child-route',
  'parent-controller-origin',
  'parent-observer-route',
] as const satisfies ReadonlyArray<V09HouseholdPhysicalProofDeviceReadinessCheck>;

const RequiredRouteHealthChecks = [
  'selected-route-accepted',
  'observer-read-only',
  'controller-takeover-manual-required',
  'revoked-route-rejected',
  'stale-offline-route-rejected',
] as const satisfies ReadonlyArray<V09HouseholdPhysicalProofRouteHealthCheck>;

function householdPhysicalProofArtifactGateIsHonest(
  readModel: V09HouseholdPhysicalProofArtifactGateReadModelCandidate
): boolean {
  return (
    readModel.physicalHouseholdLanClaimState === 'manual-required' &&
    readModel.cloudRelayState === 'not-implemented' &&
    sourceProofsAreComplete(readModel.sourceProofs) &&
    artifactRequirementsAreComplete(readModel.artifactRequirements) &&
    deviceReadinessIsComplete(readModel.deviceReadiness) &&
    routeHealthIsComplete(readModel.routeHealth) &&
    manualEvidenceCustodyIsHonest(readModel.manualEvidenceStatus, readModel.artifactRequirements) &&
    readModel.claimsNotProved.some((claim) => claim.includes('physical household LAN readiness'))
  );
}

function sourceProofsAreComplete(proofs: ReadonlyArray<V09HouseholdPhysicalProofSourceInput>): boolean {
  const sources = new Set(proofs.map((proof) => proof.source));
  return RequiredSourceProofs.every((source) => sources.has(source));
}

function artifactRequirementsAreComplete(
  requirements: ReadonlyArray<V09HouseholdPhysicalProofArtifactRequirementEvidence>
): boolean {
  const byRequirement = new Map(requirements.map((requirement) => [requirement.requirement, requirement] as const));
  return (
    RequiredArtifactRequirements.every((requirement) => byRequirement.has(requirement)) &&
    requirements.every((requirement) => artifactEvidenceReferenceIsHonest(requirement))
  );
}

function artifactEvidenceReferenceIsHonest(requirement: V09HouseholdPhysicalProofArtifactRequirementEvidence): boolean {
  if (requirement.status === 'collected') {
    return requirement.evidencePath !== null && requirement.evidenceCapturedAt !== null;
  }
  return requirement.evidencePath === null && requirement.evidenceCapturedAt === null;
}

function deviceReadinessIsComplete(
  deviceReadiness: ReadonlyArray<V09HouseholdPhysicalProofDeviceReadinessEvidence>
): boolean {
  const byCheck = new Map(deviceReadiness.map((entry) => [entry.check, entry] as const));
  return (
    RequiredDeviceReadinessChecks.every((check) => byCheck.has(check)) &&
    byCheck.get('selected-child-route')?.runtimeProofState === 'ci-mechanical-proof' &&
    byCheck.get('selected-child-route')?.physicalArtifactStatus === 'manual-required' &&
    byCheck.get('parent-observer-route')?.physicalArtifactStatus === 'manual-required'
  );
}

function routeHealthIsComplete(routeHealth: ReadonlyArray<V09HouseholdPhysicalProofRouteHealthEvidence>): boolean {
  const byCheck = new Map(routeHealth.map((entry) => [entry.check, entry] as const));
  return (
    RequiredRouteHealthChecks.every((check) => byCheck.has(check)) &&
    byCheck.get('selected-route-accepted')?.rejectionReason === null &&
    byCheck.get('observer-read-only')?.commandAuthorityState === 'observer-read-only' &&
    byCheck.get('controller-takeover-manual-required')?.runtimeProofState === 'manual-required' &&
    byCheck.get('revoked-route-rejected')?.rejectionReason === 'revoked'
  );
}

function manualEvidenceCustodyIsHonest(
  status: V09HouseholdPhysicalProofManualEvidenceCustody,
  requirements: ReadonlyArray<V09HouseholdPhysicalProofArtifactRequirementEvidence>
): boolean {
  const collectedCount = requirements.filter((requirement) => requirement.status === 'collected').length;
  return (
    status.requiredArtifactCount === RequiredArtifactRequirements.length &&
    status.collectedArtifactCount === collectedCount &&
    status.missingArtifactCount === requirements.length - collectedCount &&
    status.custodyState !== 'ready-for-human-review'
  );
}

export type V09HouseholdPhysicalProofArtifactGateId = Infer<typeof V09HouseholdPhysicalProofArtifactGateIdSchema>;
export type V09HouseholdPhysicalProofSource = Infer<typeof V09HouseholdPhysicalProofSourceSchema>;
export type V09HouseholdPhysicalProofArtifactRequirement = Infer<
  typeof V09HouseholdPhysicalProofArtifactRequirementSchema
>;
export type V09HouseholdPhysicalProofManualEvidenceStatus = Infer<
  typeof V09HouseholdPhysicalProofManualEvidenceStatusSchema
>;
export type V09HouseholdPhysicalProofDeviceReadinessCheck = Infer<
  typeof V09HouseholdPhysicalProofDeviceReadinessCheckSchema
>;
export type V09HouseholdPhysicalProofRouteHealthCheck = Infer<typeof V09HouseholdPhysicalProofRouteHealthCheckSchema>;
export type V09HouseholdPhysicalProofSourceInput = Infer<typeof V09HouseholdPhysicalProofSourceInputSchema>;
export type V09HouseholdPhysicalProofArtifactRequirementEvidence = Infer<
  typeof V09HouseholdPhysicalProofArtifactRequirementEvidenceSchema
>;
export type V09HouseholdPhysicalProofDeviceReadinessEvidence = Infer<
  typeof V09HouseholdPhysicalProofDeviceReadinessEvidenceSchema
>;
export type V09HouseholdPhysicalProofRouteHealthEvidence = Infer<
  typeof V09HouseholdPhysicalProofRouteHealthEvidenceSchema
>;
export type V09HouseholdPhysicalProofManualEvidenceCustody = Infer<
  typeof V09HouseholdPhysicalProofManualEvidenceCustodySchema
>;
export type V09HouseholdPhysicalProofArtifactGateReadModel = Infer<
  typeof V09HouseholdPhysicalProofArtifactGateReadModelSchema
>;
