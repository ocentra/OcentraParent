import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';
import { ParentControlCapabilityNameSchema, ParentControlCapabilityStatusSchema } from './capabilities';
import { parentMobileRuntimeReadModelIsConsistent } from './parent-mobile-runtime-guards';
import {
  ParentDeviceIdSchema,
  ParentEvidenceReferenceIdSchema,
  ParentTimestampSchema,
} from '@ocentra-parent/schema-domain/family-reference-primitives';

export const ParentMobilePlatformSchema = withParser(Schema.Literal('android', 'ios'));
export const ParentMobilePackageStateSchema = withParser(
  Schema.Literal('ci-mechanical-proof', 'manual-required', 'scaffold-only', 'unavailable')
);
export const ParentMobilePackageLifecycleStateSchema = withParser(Schema.Literal('manual-required', 'unavailable'));
export const ParentMobileSigningStateSchema = withParser(Schema.Literal('manual-required', 'unavailable'));
export const ParentMobileStoreDistributionStateSchema = withParser(Schema.Literal('manual-required', 'planned'));
export const ParentMobileControllerStateSchema = withParser(
  Schema.Literal('observer', 'active-controller', 'manual-required', 'unavailable')
);
export const ParentMobileCommandAuthorityStateSchema = withParser(
  Schema.Literal(
    'observer-read-only',
    'active-controller-backend-proof',
    'controller-takeover-manual-required',
    'unavailable'
  )
);
export const ParentMobileControllerRequestBoundarySchema = withParser(
  Schema.Literal('observer-read-only', 'backend-controller-owned', 'request-first-manual-required', 'unavailable')
);
export const ParentMobileServiceAvailabilityStateSchema = withParser(
  Schema.Literal('available', 'degraded', 'unavailable', 'manual-required', 'not-implemented', 'stale', 'offline')
);
export const ParentMobileServiceRouteKindSchema = withParser(
  Schema.Literal('local-service', 'lan-service', 'cloud-relay', 'parent-cache', 'parent-owned-storage')
);
export const ParentMobileServiceRouteCustodySchema = withParser(
  Schema.Literal('local-service', 'lan-service', 'cloud-relay', 'parent-cache', 'parent-owned-storage', 'unavailable')
);
export const ParentMobileAssistantJobRouteSchema = withParser(Schema.Literal('lan-ai-provider', 'unavailable'));
export const ParentMobileAssistantJobStateSchema = withParser(Schema.Literal('submitted', 'degraded', 'unavailable'));
export const ParentMobileLocalModelExecutionStateSchema = withParser(Schema.Literal('disabled-by-default'));
export const ParentMobileChildAgentBehaviorClaimSchema = withParser(Schema.Literal('not-claimed'));
const ParentMobileSchemaVersionSchema = withParser(Schema.Literal('v0.9-parent-mobile-shell'));
const ParentMobilePackageLaunchTargetSchema = brandedNonEmptyStringSchema('ParentMobilePackageLaunchTarget');
const ParentMobilePackageProofCommandSchema = brandedNonEmptyStringSchema('ParentMobilePackageProofCommand');
const ParentMobilePackageLifecycleProofRequirementSchema = brandedNonEmptyStringSchema(
  'ParentMobilePackageLifecycleProofRequirement'
);
const ParentMobileControllerLeaseIdSchema = brandedNonEmptyStringSchema('ParentMobileControllerLeaseId');
const ParentMobileRouteIdSchema = brandedNonEmptyStringSchema('ParentMobileRouteId');
const ParentMobileRouteStatusReasonSchema = brandedNonEmptyStringSchema('ParentMobileRouteStatusReason');
const ParentMobileProviderIdSchema = brandedNonEmptyStringSchema('ParentMobileProviderId');
const ParentMobileCapabilityNameSchema = brandedNonEmptyStringSchema('ParentMobileCapabilityName');
const ParentMobileUnavailableReasonSchema = brandedNonEmptyStringSchema('ParentMobileUnavailableReason');
const ParentMobileRouteProofRequirementSchema = brandedNonEmptyStringSchema('ParentMobileRouteProofRequirement');
const ParentMobileCapabilityProofRequirementSchema = brandedNonEmptyStringSchema(
  'ParentMobileCapabilityProofRequirement'
);
const ParentMobileClaimBoundarySchema = brandedNonEmptyStringSchema('ParentMobileClaimBoundary');

export const ParentMobilePackageProofSchema = withParser(
  Schema.Struct({
    platform: ParentMobilePlatformSchema,
    packageState: ParentMobilePackageStateSchema,
    launchTarget: ParentMobilePackageLaunchTargetSchema,
    proofCommand: ParentMobilePackageProofCommandSchema,
    packageLifecycleState: ParentMobilePackageLifecycleStateSchema,
    packageLifecycleProofRequirement: ParentMobilePackageLifecycleProofRequirementSchema,
    signingState: ParentMobileSigningStateSchema,
    storeDistributionState: ParentMobileStoreDistributionStateSchema,
  })
);

export const ParentMobileCapabilityProofSchema = withParser(
  Schema.Struct({
    capability: ParentControlCapabilityNameSchema,
    status: ParentControlCapabilityStatusSchema,
    proofRequirement: ParentMobileCapabilityProofRequirementSchema,
    claimBoundary: ParentMobileClaimBoundarySchema,
  })
);

export const ParentMobileServiceRouteStatusSchema = withParser(
  Schema.Struct({
    routeKind: ParentMobileServiceRouteKindSchema,
    state: ParentMobileServiceAvailabilityStateSchema,
    custody: ParentMobileServiceRouteCustodySchema,
    selectedRouteId: Schema.Union(ParentMobileRouteIdSchema, Schema.Null),
    statusReason: ParentMobileRouteStatusReasonSchema,
    proofRequirement: ParentMobileRouteProofRequirementSchema,
  })
);

export const ParentMobileServiceAvailabilitySchema = withParser(
  Schema.Struct({
    localService: ParentMobileServiceAvailabilityStateSchema,
    lanService: ParentMobileServiceAvailabilityStateSchema,
    cloudRelay: ParentMobileServiceAvailabilityStateSchema,
    parentCache: ParentMobileServiceAvailabilityStateSchema,
    parentOwnedStorage: ParentMobileServiceAvailabilityStateSchema,
    selectedRouteId: Schema.Union(ParentMobileRouteIdSchema, Schema.Null),
    routeStatuses: Schema.Array(ParentMobileServiceRouteStatusSchema),
  })
);

export const ParentMobileControllerProofSchema = withParser(
  Schema.Struct({
    controllerState: ParentMobileControllerStateSchema,
    controllerLeaseId: Schema.Union(ParentMobileControllerLeaseIdSchema, Schema.Null),
    takeoverRequestAllowed: Schema.Boolean,
    commandAuthorityState: ParentMobileCommandAuthorityStateSchema,
    requestBoundary: ParentMobileControllerRequestBoundarySchema,
  })
);

export const ParentMobileAssistantJobProofSchema = withParser(
  Schema.Struct({
    route: ParentMobileAssistantJobRouteSchema,
    jobState: ParentMobileAssistantJobStateSchema,
    providerId: Schema.Union(ParentMobileProviderIdSchema, Schema.Null),
    requiredCapabilities: Schema.Array(ParentMobileCapabilityNameSchema),
    evidenceReferenceIds: Schema.Array(ParentEvidenceReferenceIdSchema),
    unavailableReason: Schema.Union(ParentMobileUnavailableReasonSchema, Schema.Null),
  })
);

const ParentMobileRuntimeReadModelBaseSchema = Schema.Struct({
  schemaVersion: ParentMobileSchemaVersionSchema,
  parentDeviceId: ParentDeviceIdSchema,
  platform: ParentMobilePlatformSchema,
  packageProof: ParentMobilePackageProofSchema,
  serviceAvailability: ParentMobileServiceAvailabilitySchema,
  controllerProof: ParentMobileControllerProofSchema,
  assistantJobProof: ParentMobileAssistantJobProofSchema,
  platformCapabilities: Schema.Array(ParentMobileCapabilityProofSchema),
  localModelExecutionState: ParentMobileLocalModelExecutionStateSchema,
  localModelExecutionAllowed: Schema.Literal(false),
  childAgentBehaviorClaim: ParentMobileChildAgentBehaviorClaimSchema,
  updatedAt: ParentTimestampSchema,
});

export type ParentMobileRuntimeReadModelCandidate = Infer<typeof ParentMobileRuntimeReadModelBaseSchema>;

export const ParentMobileRuntimeReadModelSchema = withParser(
  ParentMobileRuntimeReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        parentMobileRuntimeReadModelIsConsistent(readModel) ||
        'Expected parent mobile runtime read model to stay parent-only, avoid local model execution, and keep service/controller claims honest'
    )
  )
);

export type ParentMobilePlatform = Infer<typeof ParentMobilePlatformSchema>;
export type ParentMobilePackageState = Infer<typeof ParentMobilePackageStateSchema>;
export type ParentMobilePackageLifecycleState = Infer<typeof ParentMobilePackageLifecycleStateSchema>;
export type ParentMobileSigningState = Infer<typeof ParentMobileSigningStateSchema>;
export type ParentMobileStoreDistributionState = Infer<typeof ParentMobileStoreDistributionStateSchema>;
export type ParentMobileControllerState = Infer<typeof ParentMobileControllerStateSchema>;
export type ParentMobileCommandAuthorityState = Infer<typeof ParentMobileCommandAuthorityStateSchema>;
export type ParentMobileControllerRequestBoundary = Infer<typeof ParentMobileControllerRequestBoundarySchema>;
export type ParentMobileServiceAvailabilityState = Infer<typeof ParentMobileServiceAvailabilityStateSchema>;
export type ParentMobileServiceRouteKind = Infer<typeof ParentMobileServiceRouteKindSchema>;
export type ParentMobileServiceRouteCustody = Infer<typeof ParentMobileServiceRouteCustodySchema>;
export type ParentMobileAssistantJobRoute = Infer<typeof ParentMobileAssistantJobRouteSchema>;
export type ParentMobileAssistantJobState = Infer<typeof ParentMobileAssistantJobStateSchema>;
export type ParentMobileLocalModelExecutionState = Infer<typeof ParentMobileLocalModelExecutionStateSchema>;
export type ParentMobileChildAgentBehaviorClaim = Infer<typeof ParentMobileChildAgentBehaviorClaimSchema>;
export type ParentMobilePackageProof = Infer<typeof ParentMobilePackageProofSchema>;
export type ParentMobileServiceRouteStatus = Infer<typeof ParentMobileServiceRouteStatusSchema>;
export type ParentMobileServiceAvailability = Infer<typeof ParentMobileServiceAvailabilitySchema>;
export type ParentMobileControllerProof = Infer<typeof ParentMobileControllerProofSchema>;
export type ParentMobileAssistantJobProof = Infer<typeof ParentMobileAssistantJobProofSchema>;
export type ParentMobileCapabilityProof = Infer<typeof ParentMobileCapabilityProofSchema>;
export type ParentMobileRuntimeReadModel = Infer<typeof ParentMobileRuntimeReadModelSchema>;
