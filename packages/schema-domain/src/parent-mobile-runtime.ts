import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';
import { ParentControlCapabilityNameSchema, ParentControlCapabilityStatusSchema } from './capabilities';
import {
  AndroidParentMobileCapabilityStatuses,
  IosParentMobileCapabilityStatuses,
} from './parent-mobile-runtime-capability-statuses';
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

type ParentMobileRuntimeReadModelCandidate = Infer<typeof ParentMobileRuntimeReadModelBaseSchema>;

const RequiredParentMobileRouteKinds = [
  'local-service',
  'lan-service',
  'cloud-relay',
  'parent-cache',
  'parent-owned-storage',
] as const satisfies ReadonlyArray<ParentMobileServiceRouteKind>;

const ParentMobileStaticRouteExpectations = {
  'cloud-relay': {
    state: 'not-implemented',
    custody: 'unavailable',
  },
  'parent-cache': {
    state: 'stale',
    custody: 'parent-cache',
  },
  'parent-owned-storage': {
    state: 'offline',
    custody: 'parent-owned-storage',
  },
} as const;

type ParentMobileStaticRouteKind = keyof typeof ParentMobileStaticRouteExpectations;

const ParentMobileControllerStateExpectations = {
  'active-controller': {
    controllerLease: 'required',
    takeoverRequestAllowed: true,
    commandAuthorityState: 'active-controller-backend-proof',
    requestBoundary: 'backend-controller-owned',
  },
  observer: {
    controllerLease: 'absent',
    takeoverRequestAllowed: false,
    commandAuthorityState: 'observer-read-only',
    requestBoundary: 'observer-read-only',
  },
  'manual-required': {
    controllerLease: 'absent',
    takeoverRequestAllowed: true,
    commandAuthorityState: 'controller-takeover-manual-required',
    requestBoundary: 'request-first-manual-required',
  },
  unavailable: {
    controllerLease: 'absent',
    takeoverRequestAllowed: false,
    commandAuthorityState: 'unavailable',
    requestBoundary: 'unavailable',
  },
} as const;

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

  if (!parentMobilePackageProofIsConsistent(readModel.packageProof)) {
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

function parentMobilePackageProofIsConsistent(
  packageProof: ParentMobileRuntimeReadModelCandidate['packageProof']
): boolean {
  if (packageProof.packageState === 'unavailable') {
    return packageProof.packageLifecycleState === 'unavailable';
  }

  return packageProof.packageLifecycleState === 'manual-required';
}

function parentMobileRouteStatusMatchesAvailability(
  serviceAvailability: ParentMobileRuntimeReadModelCandidate['serviceAvailability'],
  routeStatus: ParentMobileServiceRouteStatus | undefined
): boolean {
  if (routeStatus === undefined) {
    return false;
  }

  if (routeStatus.state !== expectedParentMobileRouteState(serviceAvailability, routeStatus.routeKind)) {
    return false;
  }

  if (routeStatus.statusReason !== expectedParentMobileRouteStatusReason(routeStatus)) {
    return false;
  }

  return parentMobileRouteStatusCustodyMatches(routeStatus);
}

function parentMobileRouteStatusCustodyMatches(routeStatus: ParentMobileServiceRouteStatus): boolean {
  if (routeStatus.state === 'unavailable') {
    return routeStatus.custody === 'unavailable' && routeStatus.selectedRouteId === null;
  }

  const staticExpectation = parentMobileStaticRouteExpectation(routeStatus.routeKind);
  return staticExpectation === null
    ? routeStatus.custody === routeStatus.routeKind
    : routeStatus.state === staticExpectation.state &&
        routeStatus.custody === staticExpectation.custody &&
        routeStatus.selectedRouteId === null;
}

function parentMobileStaticRouteExpectation(routeKind: ParentMobileServiceRouteKind) {
  return routeKind in ParentMobileStaticRouteExpectations
    ? ParentMobileStaticRouteExpectations[routeKind as ParentMobileStaticRouteKind]
    : null;
}

function expectedParentMobileRouteStatusReason(routeStatus: ParentMobileServiceRouteStatus): string {
  if (routeStatus.routeKind === 'local-service') {
    return parentMobileLiveRouteStatusReason('local-service', routeStatus.state);
  }

  if (routeStatus.routeKind === 'lan-service') {
    return parentMobileLiveRouteStatusReason('lan-service', routeStatus.state);
  }

  if (routeStatus.routeKind === 'cloud-relay') {
    return 'cloud-relay-not-implemented';
  }

  if (routeStatus.routeKind === 'parent-cache') {
    return routeStatus.state === 'unavailable' ? 'parent-cache-unavailable' : 'parent-cache-stale';
  }

  return routeStatus.state === 'unavailable' ? 'parent-owned-storage-unavailable' : 'parent-owned-storage-offline';
}

function parentMobileLiveRouteStatusReason(
  routeKind: 'local-service' | 'lan-service',
  state: ParentMobileServiceAvailabilityState
): string {
  if (state === 'available') {
    return `${routeKind}-available`;
  }

  if (state === 'degraded') {
    return `${routeKind}-degraded`;
  }

  if (state === 'unavailable') {
    return `${routeKind}-unavailable`;
  }

  return `${routeKind}-proof-required`;
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
  const expected = ParentMobileControllerStateExpectations[controllerProof.controllerState];
  const leaseMatches =
    expected.controllerLease === 'required'
      ? controllerProof.controllerLeaseId !== null
      : controllerProof.controllerLeaseId === null;

  return (
    leaseMatches &&
    controllerProof.takeoverRequestAllowed === expected.takeoverRequestAllowed &&
    controllerProof.commandAuthorityState === expected.commandAuthorityState &&
    controllerProof.requestBoundary === expected.requestBoundary
  );
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
