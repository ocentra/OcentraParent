import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentControlCapabilityNameSchema, ParentControlCapabilityStatusSchema } from './capabilities';
import {
  AndroidParentMobileCapabilityStatuses,
  IosParentMobileCapabilityStatuses,
} from './parent-mobile-runtime-capability-statuses';
import { ParentDeviceIdSchema, ParentEvidenceReferenceIdSchema, ParentTimestampSchema } from './reference-primitives';

export const ParentMobilePlatformSchema = withParser(Schema.Literal('android', 'ios'));
export const ParentMobilePackageStateSchema = withParser(
  Schema.Literal('ci-mechanical-proof', 'manual-required', 'scaffold-only', 'unavailable')
);
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

const NonEmptyParentMobileText = Schema.String.pipe(Schema.minLength(1));
const ParentMobileSchemaVersionSchema = withParser(Schema.Literal('v0.9-parent-mobile-shell'));
const ParentMobilePackageLaunchTargetSchema = NonEmptyParentMobileText.pipe(
  Schema.brand('ParentMobilePackageLaunchTarget')
);
const ParentMobilePackageProofCommandSchema = NonEmptyParentMobileText.pipe(
  Schema.brand('ParentMobilePackageProofCommand')
);
const ParentMobileControllerLeaseIdSchema = NonEmptyParentMobileText.pipe(
  Schema.brand('ParentMobileControllerLeaseId')
);
const ParentMobileRouteIdSchema = NonEmptyParentMobileText.pipe(Schema.brand('ParentMobileRouteId'));
const ParentMobileProviderIdSchema = NonEmptyParentMobileText.pipe(Schema.brand('ParentMobileProviderId'));
const ParentMobileCapabilityNameSchema = NonEmptyParentMobileText.pipe(Schema.brand('ParentMobileCapabilityName'));
const ParentMobileUnavailableReasonSchema = NonEmptyParentMobileText.pipe(
  Schema.brand('ParentMobileUnavailableReason')
);
const ParentMobileRouteProofRequirementSchema = NonEmptyParentMobileText.pipe(
  Schema.brand('ParentMobileRouteProofRequirement')
);
const ParentMobileCapabilityProofRequirementSchema = NonEmptyParentMobileText.pipe(
  Schema.brand('ParentMobileCapabilityProofRequirement')
);
const ParentMobileClaimBoundarySchema = NonEmptyParentMobileText.pipe(Schema.brand('ParentMobileClaimBoundary'));

export const ParentMobilePackageProofSchema = withParser(
  Schema.Struct({
    platform: ParentMobilePlatformSchema,
    packageState: ParentMobilePackageStateSchema,
    launchTarget: ParentMobilePackageLaunchTargetSchema,
    proofCommand: ParentMobilePackageProofCommandSchema,
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

type ParentMobileRuntimeReadModelCandidate = Infer<typeof ParentMobileRuntimeReadModelBaseSchema>;

const RequiredParentMobileRouteKinds = [
  'local-service',
  'lan-service',
  'cloud-relay',
  'parent-cache',
  'parent-owned-storage',
] as const satisfies ReadonlyArray<ParentMobileServiceRouteKind>;

export const ParentMobileRuntimeReadModelSchema = withParser(
  ParentMobileRuntimeReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        parentMobileRuntimeReadModelIsConsistent(readModel) ||
        'Expected parent mobile runtime read model to stay parent-only, avoid local model execution, and keep service/controller claims honest'
    )
  )
);

function parentMobileRuntimeReadModelIsConsistent(readModel: ParentMobileRuntimeReadModelCandidate): boolean {
  if (readModel.packageProof.platform !== readModel.platform) {
    return false;
  }

  if (readModel.localModelExecutionAllowed !== false) {
    return false;
  }

  if (readModel.childAgentBehaviorClaim !== 'not-claimed') {
    return false;
  }

  if (readModel.serviceAvailability.cloudRelay !== 'not-implemented') {
    return false;
  }

  if (!parentMobileRouteStatusesAreConsistent(readModel.serviceAvailability)) {
    return false;
  }

  if (!parentMobileControllerProofIsConsistent(readModel.controllerProof)) {
    return false;
  }

  if (!parentMobileCapabilityProofsAreConsistent(readModel)) {
    return false;
  }

  if (readModel.assistantJobProof.route === 'lan-ai-provider') {
    return parentMobileLanAiProviderJobIsConsistent(readModel.assistantJobProof);
  }

  return readModel.assistantJobProof.providerId === null && readModel.assistantJobProof.unavailableReason !== null;
}

function parentMobileRouteStatusesAreConsistent(
  serviceAvailability: ParentMobileRuntimeReadModelCandidate['serviceAvailability']
): boolean {
  const byKind = new Map(serviceAvailability.routeStatuses.map((route) => [route.routeKind, route] as const));
  const selectedRoutes = serviceAvailability.routeStatuses.filter((route) => route.selectedRouteId !== null);

  if (
    byKind.size !== serviceAvailability.routeStatuses.length ||
    !RequiredParentMobileRouteKinds.every((kind) => byKind.has(kind))
  ) {
    return false;
  }

  if (serviceAvailability.selectedRouteId === null) {
    if (selectedRoutes.length !== 0) {
      return false;
    }
  } else if (
    selectedRoutes.length !== 1 ||
    selectedRoutes[0]?.selectedRouteId !== serviceAvailability.selectedRouteId
  ) {
    return false;
  }

  return RequiredParentMobileRouteKinds.every((kind) =>
    parentMobileRouteStatusMatchesAvailability(serviceAvailability, byKind.get(kind))
  );
}

function parentMobileRouteStatusMatchesAvailability(
  serviceAvailability: ParentMobileRuntimeReadModelCandidate['serviceAvailability'],
  routeStatus: ParentMobileServiceRouteStatus | undefined
): boolean {
  if (
    routeStatus === undefined ||
    routeStatus.state !== expectedParentMobileRouteState(serviceAvailability, routeStatus.routeKind)
  ) {
    return false;
  }

  if (routeStatus.routeKind === 'cloud-relay') {
    return (
      routeStatus.state === 'not-implemented' &&
      routeStatus.custody === 'unavailable' &&
      routeStatus.selectedRouteId === null
    );
  }

  if (routeStatus.routeKind === 'parent-cache') {
    return (
      routeStatus.state === 'stale' && routeStatus.custody === 'parent-cache' && routeStatus.selectedRouteId === null
    );
  }

  if (routeStatus.routeKind === 'parent-owned-storage') {
    return (
      routeStatus.state === 'offline' &&
      routeStatus.custody === 'parent-owned-storage' &&
      routeStatus.selectedRouteId === null
    );
  }

  return routeStatus.custody === routeStatus.routeKind;
}

function expectedParentMobileRouteState(
  serviceAvailability: ParentMobileRuntimeReadModelCandidate['serviceAvailability'],
  routeKind: ParentMobileServiceRouteKind
): ParentMobileServiceAvailabilityState {
  if (routeKind === 'local-service') {
    return serviceAvailability.localService;
  }

  if (routeKind === 'lan-service') {
    return serviceAvailability.lanService;
  }

  if (routeKind === 'cloud-relay') {
    return serviceAvailability.cloudRelay;
  }

  if (routeKind === 'parent-cache') {
    return serviceAvailability.parentCache;
  }

  return serviceAvailability.parentOwnedStorage;
}

function parentMobileControllerProofIsConsistent(
  controllerProof: ParentMobileRuntimeReadModelCandidate['controllerProof']
): boolean {
  if (controllerProof.controllerState === 'active-controller') {
    return (
      controllerProof.controllerLeaseId !== null &&
      controllerProof.commandAuthorityState === 'active-controller-backend-proof'
    );
  }

  if (controllerProof.controllerState === 'observer') {
    return (
      controllerProof.controllerLeaseId === null &&
      controllerProof.takeoverRequestAllowed === false &&
      controllerProof.commandAuthorityState === 'observer-read-only'
    );
  }

  if (controllerProof.controllerState === 'manual-required') {
    return (
      controllerProof.controllerLeaseId === null &&
      controllerProof.commandAuthorityState === 'controller-takeover-manual-required'
    );
  }

  return controllerProof.controllerLeaseId === null && controllerProof.commandAuthorityState === 'unavailable';
}

function parentMobileLanAiProviderJobIsConsistent(
  assistantJobProof: ParentMobileRuntimeReadModelCandidate['assistantJobProof']
): boolean {
  if (assistantJobProof.requiredCapabilities.length === 0) {
    return false;
  }

  if (assistantJobProof.jobState === 'submitted') {
    return assistantJobProof.providerId !== null && assistantJobProof.unavailableReason === null;
  }

  return assistantJobProof.providerId === null && assistantJobProof.unavailableReason !== null;
}

function parentMobileCapabilityProofsAreConsistent(readModel: ParentMobileRuntimeReadModelCandidate): boolean {
  const expected =
    readModel.platform === 'android' ? AndroidParentMobileCapabilityStatuses : IosParentMobileCapabilityStatuses;
  const capabilityStatuses = new Map(
    readModel.platformCapabilities.map((entry) => [entry.capability, entry.status] as const)
  );

  if (
    capabilityStatuses.size !== readModel.platformCapabilities.length ||
    capabilityStatuses.size !== expected.length
  ) {
    return false;
  }

  if (readModel.platformCapabilities.some((entry) => entry.status === 'supported' || entry.status === 'implemented')) {
    return false;
  }

  return expected.every(([capability, status]) => capabilityStatuses.get(capability) === status);
}

export type ParentMobilePlatform = Infer<typeof ParentMobilePlatformSchema>;
export type ParentMobilePackageState = Infer<typeof ParentMobilePackageStateSchema>;
export type ParentMobileSigningState = Infer<typeof ParentMobileSigningStateSchema>;
export type ParentMobileStoreDistributionState = Infer<typeof ParentMobileStoreDistributionStateSchema>;
export type ParentMobileControllerState = Infer<typeof ParentMobileControllerStateSchema>;
export type ParentMobileCommandAuthorityState = Infer<typeof ParentMobileCommandAuthorityStateSchema>;
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
