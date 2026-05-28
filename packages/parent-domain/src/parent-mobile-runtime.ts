import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
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
export const ParentMobileServiceAvailabilityStateSchema = withParser(
  Schema.Literal('available', 'degraded', 'unavailable', 'manual-required', 'not-implemented')
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

export const ParentMobileServiceAvailabilitySchema = withParser(
  Schema.Struct({
    localService: ParentMobileServiceAvailabilityStateSchema,
    lanService: ParentMobileServiceAvailabilityStateSchema,
    cloudRelay: ParentMobileServiceAvailabilityStateSchema,
    selectedRouteId: Schema.Union(ParentMobileRouteIdSchema, Schema.Null),
  })
);

export const ParentMobileControllerProofSchema = withParser(
  Schema.Struct({
    controllerState: ParentMobileControllerStateSchema,
    controllerLeaseId: Schema.Union(ParentMobileControllerLeaseIdSchema, Schema.Null),
    takeoverRequestAllowed: Schema.Boolean,
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
  localModelExecutionState: ParentMobileLocalModelExecutionStateSchema,
  localModelExecutionAllowed: Schema.Literal(false),
  childAgentBehaviorClaim: ParentMobileChildAgentBehaviorClaimSchema,
  updatedAt: ParentTimestampSchema,
});

type ParentMobileRuntimeReadModelCandidate = Infer<typeof ParentMobileRuntimeReadModelBaseSchema>;

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

  if (readModel.controllerProof.controllerState === 'active-controller') {
    return readModel.controllerProof.controllerLeaseId !== null;
  }

  if (readModel.assistantJobProof.route === 'lan-ai-provider') {
    return (
      readModel.assistantJobProof.requiredCapabilities.length > 0 &&
      readModel.assistantJobProof.unavailableReason !== null
    );
  }

  return readModel.assistantJobProof.providerId === null && readModel.assistantJobProof.unavailableReason !== null;
}

export type ParentMobilePlatform = Infer<typeof ParentMobilePlatformSchema>;
export type ParentMobilePackageState = Infer<typeof ParentMobilePackageStateSchema>;
export type ParentMobileSigningState = Infer<typeof ParentMobileSigningStateSchema>;
export type ParentMobileStoreDistributionState = Infer<typeof ParentMobileStoreDistributionStateSchema>;
export type ParentMobileControllerState = Infer<typeof ParentMobileControllerStateSchema>;
export type ParentMobileServiceAvailabilityState = Infer<typeof ParentMobileServiceAvailabilityStateSchema>;
export type ParentMobileAssistantJobRoute = Infer<typeof ParentMobileAssistantJobRouteSchema>;
export type ParentMobileAssistantJobState = Infer<typeof ParentMobileAssistantJobStateSchema>;
export type ParentMobileLocalModelExecutionState = Infer<typeof ParentMobileLocalModelExecutionStateSchema>;
export type ParentMobileChildAgentBehaviorClaim = Infer<typeof ParentMobileChildAgentBehaviorClaimSchema>;
export type ParentMobilePackageProof = Infer<typeof ParentMobilePackageProofSchema>;
export type ParentMobileServiceAvailability = Infer<typeof ParentMobileServiceAvailabilitySchema>;
export type ParentMobileControllerProof = Infer<typeof ParentMobileControllerProofSchema>;
export type ParentMobileAssistantJobProof = Infer<typeof ParentMobileAssistantJobProofSchema>;
export type ParentMobileRuntimeReadModel = Infer<typeof ParentMobileRuntimeReadModelSchema>;
