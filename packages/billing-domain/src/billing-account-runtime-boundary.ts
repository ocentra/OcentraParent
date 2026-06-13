import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { FamilyReferenceSchema, ParentAccountReferenceSchema } from '@ocentra-parent/family-domain/references';
import { ParentTimestampSchema } from '@ocentra-parent/family-domain/reference-primitives';
import {
  BillingChildActivityCustodySchema,
  BillingEvidenceExportAccessSchema,
  BillingFailureKindSchema,
  BillingLocalSafetyBehaviorSchema,
  BillingParentResolutionSchema,
  BillingParentVisibleStateSchema,
  BillingSubscriptionStatusSchema,
} from './billing-entitlement-values';
import {
  BillingAccountBackendRuntimeStateSchema,
  BillingAccountRuntimeAuditReferenceSchema,
  BillingAccountRuntimeBackendClaimSchema,
  BillingAccountRuntimeBoundaryIdSchema,
  BillingAccountRuntimeChildActivityCustodyClaimSchema,
  BillingAccountRuntimeChildDeviceConsumptionClaimSchema,
  BillingAccountRuntimeChildDeviceConsumptionSchema,
  BillingAccountRuntimeEntitlementSigningStateSchema,
  BillingAccountRuntimeNonClaimSchema,
  BillingAccountRuntimeOperationSchema,
  BillingAccountRuntimeParentVisibleStateSchema,
  BillingAccountRuntimePortalUiClaimSchema,
  BillingAccountRuntimeProviderBoundarySchema,
  BillingAccountRuntimeProviderSecretClaimSchema,
  BillingAccountRuntimeProviderSecretCustodySchema,
  BillingAccountRuntimeSchemaVersionSchema,
  BillingAccountRuntimeSourceSchema,
  BillingAccountRuntimeStatusSchema,
  BillingAccountRuntimeStripeSdkClaimSchema,
  type BillingAccountRuntimeNonClaim,
  type BillingAccountRuntimeOperation,
} from './billing-account-runtime-boundary-values';

export * from './billing-account-runtime-boundary-values';

export const BillingAccountRuntimeFailureStateSchema = withParser(
  Schema.Struct({
    failureKind: BillingFailureKindSchema,
    parentVisibleState: BillingParentVisibleStateSchema,
    localSafetyBehavior: BillingLocalSafetyBehaviorSchema,
    retainEvidenceExportAccess: Schema.Boolean,
    existingLocalSafetyContinues: Schema.Boolean,
    parentResolution: BillingParentResolutionSchema,
    retryAllowed: Schema.Boolean,
    retryAfter: Schema.Union(ParentTimestampSchema, Schema.Null),
  }).pipe(
    Schema.filter(
      (failure) =>
        failure.retainEvidenceExportAccess ||
        'Expected billing account runtime failures to retain evidence export and audit access'
    ),
    Schema.filter(
      (failure) =>
        failure.existingLocalSafetyContinues ||
        'Expected billing account runtime failures to keep existing local safety behavior explicit'
    )
  )
);

export const BillingAccountRuntimeStatusRowSchema = withParser(
  Schema.Struct({
    schemaVersion: BillingAccountRuntimeSchemaVersionSchema,
    boundaryId: BillingAccountRuntimeBoundaryIdSchema,
    parentAccount: ParentAccountReferenceSchema,
    family: FamilyReferenceSchema,
    accountStatus: BillingAccountRuntimeStatusSchema,
    subscriptionStatus: BillingSubscriptionStatusSchema,
    source: BillingAccountRuntimeSourceSchema,
    backendRuntimeState: BillingAccountBackendRuntimeStateSchema,
    parentVisibleState: BillingAccountRuntimeParentVisibleStateSchema,
    localSafetyBehavior: BillingLocalSafetyBehaviorSchema,
    evidenceExportAccess: BillingEvidenceExportAccessSchema,
    childActivityCustody: BillingChildActivityCustodySchema,
    providerSecretCustody: BillingAccountRuntimeProviderSecretCustodySchema,
    failureState: Schema.Union(BillingAccountRuntimeFailureStateSchema, Schema.Null),
    auditReference: BillingAccountRuntimeAuditReferenceSchema,
  }).pipe(
    Schema.filter(
      (row) =>
        !['backend-unavailable', 'provider-unavailable', 'manual-review'].includes(row.accountStatus) ||
        row.failureState !== null ||
        'Expected unavailable or manual billing account rows to carry a parent-visible failure state'
    ),
    Schema.filter(
      (row) =>
        row.backendRuntimeState !== 'available' ||
        row.source === 'account-backend' ||
        'Expected available billing account runtime rows to come from an account backend source'
    )
  )
);

export const BillingAccountRuntimeOperationRowSchema = withParser(
  Schema.Struct({
    schemaVersion: BillingAccountRuntimeSchemaVersionSchema,
    operation: BillingAccountRuntimeOperationSchema,
    backendRuntimeState: BillingAccountBackendRuntimeStateSchema,
    providerBoundary: BillingAccountRuntimeProviderBoundarySchema,
    providerSecretCustody: BillingAccountRuntimeProviderSecretCustodySchema,
    childDeviceConsumption: BillingAccountRuntimeChildDeviceConsumptionSchema,
    manualRequired: Schema.Boolean,
    failureState: Schema.Union(BillingAccountRuntimeFailureStateSchema, Schema.Null),
  }).pipe(
    Schema.filter(
      (row) =>
        row.backendRuntimeState === 'available' ||
        row.backendRuntimeState === 'not-implemented' ||
        row.manualRequired ||
        row.failureState !== null ||
        'Expected unavailable billing runtime operations to be failure-backed or manual-required'
    ),
    Schema.filter(
      (row) =>
        row.operation !== 'provider-webhook-sync' ||
        row.providerBoundary === 'backend-reference-only' ||
        row.backendRuntimeState === 'not-implemented' ||
        'Expected provider webhook sync to stay behind the backend-reference boundary'
    ),
    Schema.filter(
      (row) =>
        row.childDeviceConsumption !== 'signed-snapshot-consumed' ||
        row.operation === 'entitlement-snapshot-read' ||
        'Expected child-device billing consumption to be limited to signed entitlement snapshot reads'
    ),
    Schema.filter(
      (row) =>
        row.operation !== 'entitlement-snapshot-read' ||
        row.childDeviceConsumption === 'signed-snapshot-consumed' ||
        'Expected entitlement snapshot reads to expose the child signed-snapshot consumption boundary'
    )
  )
);

export const BillingAccountRuntimeEntitlementSigningBoundarySchema = withParser(
  Schema.Struct({
    schemaVersion: BillingAccountRuntimeSchemaVersionSchema,
    signingState: BillingAccountRuntimeEntitlementSigningStateSchema,
    signedSnapshotAccepted: Schema.Boolean,
    manualRequired: Schema.Boolean,
    signingRuntimeClaim: BillingAccountRuntimeBackendClaimSchema,
    failureState: Schema.Union(BillingAccountRuntimeFailureStateSchema, Schema.Null),
  }).pipe(
    Schema.filter(
      (boundary) =>
        boundary.signingState !== 'manual-required' ||
        (boundary.manualRequired && boundary.failureState !== null) ||
        'Expected manual-required entitlement signing to carry manual state and failure context'
    ),
    Schema.filter(
      (boundary) =>
        boundary.signingState !== 'signed-snapshot-accepted' ||
        boundary.signedSnapshotAccepted ||
        'Expected signed entitlement snapshots to be explicitly accepted by schema proof'
    )
  )
);

export const BillingAccountRuntimeBoundaryProofSchema = withParser(
  Schema.Struct({
    schemaVersion: BillingAccountRuntimeSchemaVersionSchema,
    accountStatusRows: Schema.Array(BillingAccountRuntimeStatusRowSchema),
    runtimeOperations: Schema.Array(BillingAccountRuntimeOperationRowSchema),
    entitlementSigningBoundary: BillingAccountRuntimeEntitlementSigningBoundarySchema,
    failureStates: Schema.Array(BillingAccountRuntimeFailureStateSchema),
    nonClaims: Schema.Array(BillingAccountRuntimeNonClaimSchema),
    stripeSdkClaim: BillingAccountRuntimeStripeSdkClaimSchema,
    providerSecretClaim: BillingAccountRuntimeProviderSecretClaimSchema,
    accountBackendClaim: BillingAccountRuntimeBackendClaimSchema,
    portalUiClaim: BillingAccountRuntimePortalUiClaimSchema,
    childDeviceConsumptionClaim: BillingAccountRuntimeChildDeviceConsumptionClaimSchema,
    childActivityCustodyClaim: BillingAccountRuntimeChildActivityCustodyClaimSchema,
    updatedAt: ParentTimestampSchema,
  }).pipe(
    Schema.filter(
      (proof) =>
        billingAccountRuntimeBoundaryProofIsHonest(proof) ||
        'Expected billing account runtime proof to keep backend provider secret UI custody non-claims and signed child consumption explicit'
    )
  )
);

export type BillingAccountRuntimeStatusRow = Infer<typeof BillingAccountRuntimeStatusRowSchema>;
export type BillingAccountRuntimeOperationRow = Infer<typeof BillingAccountRuntimeOperationRowSchema>;
export type BillingAccountRuntimeFailureState = Infer<typeof BillingAccountRuntimeFailureStateSchema>;
export type BillingAccountRuntimeEntitlementSigningBoundary = Infer<
  typeof BillingAccountRuntimeEntitlementSigningBoundarySchema
>;
export type BillingAccountRuntimeBoundaryProof = Infer<typeof BillingAccountRuntimeBoundaryProofSchema>;

export const decodeBillingAccountRuntimeBoundaryProof = Schema.decodeUnknownSync(
  BillingAccountRuntimeBoundaryProofSchema
);

function billingAccountRuntimeBoundaryProofIsHonest(proof: {
  readonly accountStatusRows: ReadonlyArray<{
    readonly accountStatus: string;
    readonly failureState: unknown | null;
    readonly providerSecretCustody: 'not-present';
  }>;
  readonly runtimeOperations: ReadonlyArray<{
    readonly operation: BillingAccountRuntimeOperation;
    readonly childDeviceConsumption: 'signed-snapshot-consumed' | 'manual-required' | 'not-implemented';
    readonly providerSecretCustody: 'not-present';
  }>;
  readonly entitlementSigningBoundary: {
    readonly signingState: string;
    readonly manualRequired: boolean;
  };
  readonly childDeviceConsumptionClaim: 'signed-snapshot-consumption-contract' | 'not-supported';
  readonly failureStates: ReadonlyArray<unknown>;
  readonly nonClaims: ReadonlyArray<BillingAccountRuntimeNonClaim>;
}): boolean {
  const requiredNonClaims: ReadonlyArray<BillingAccountRuntimeNonClaim> = [
    'no-stripe-sdk',
    'no-provider-secrets',
    'no-billing-provider-runtime',
    'no-account-backend',
    'no-entitlement-signing-runtime',
    'no-portal-ui',
    'no-child-activity-custody',
  ];
  const requiredOperations: ReadonlyArray<BillingAccountRuntimeOperation> = [
    'account-status-read',
    'subscription-status-read',
    'entitlement-snapshot-read',
    'device-limit-decision-read',
    'download-status-read',
    'provider-webhook-sync',
  ];
  return (
    requiredNonClaims.every((claim) => proof.nonClaims.includes(claim)) &&
    requiredOperations.every((operation) => proof.runtimeOperations.some((row) => row.operation === operation)) &&
    proof.accountStatusRows.some((row) => row.accountStatus === 'active') &&
    proof.accountStatusRows.some((row) => row.accountStatus === 'provider-unavailable' && row.failureState !== null) &&
    proof.accountStatusRows.every((row) => row.providerSecretCustody === 'not-present') &&
    proof.runtimeOperations.every(
      (row) =>
        row.providerSecretCustody === 'not-present' &&
        (row.operation === 'entitlement-snapshot-read'
          ? row.childDeviceConsumption === 'signed-snapshot-consumed'
          : row.childDeviceConsumption !== 'signed-snapshot-consumed')
    ) &&
    proof.childDeviceConsumptionClaim === 'signed-snapshot-consumption-contract' &&
    proof.entitlementSigningBoundary.signingState === 'manual-required' &&
    proof.entitlementSigningBoundary.manualRequired &&
    proof.failureStates.length >= 2
  );
}
