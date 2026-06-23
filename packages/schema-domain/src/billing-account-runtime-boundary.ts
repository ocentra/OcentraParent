import { type Infer, Schema, withParser } from './effect';
import { FamilyReferenceSchema, ParentAccountReferenceSchema } from './family-references';
import { ParentTimestampSchema } from './family-reference-primitives';
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
import {
  BillingInvoiceProviderModeSchema,
  type BillingInvoiceProviderMode,
} from './billing-invoice-tax-refund-dispute-values';

const BillingAccountRuntimeFailureStateStruct = Schema.Struct({
  failureKind: BillingFailureKindSchema,
  parentVisibleState: BillingParentVisibleStateSchema,
  localSafetyBehavior: BillingLocalSafetyBehaviorSchema,
  retainEvidenceExportAccess: Schema.Boolean,
  existingLocalSafetyContinues: Schema.Boolean,
  parentResolution: BillingParentResolutionSchema,
  retryAllowed: Schema.Boolean,
  retryAfter: Schema.Union(ParentTimestampSchema, Schema.Null),
});
type BillingAccountRuntimeFailureStateShape = Infer<typeof BillingAccountRuntimeFailureStateStruct>;

export const BillingAccountRuntimeFailureStateSchema = withParser(
  BillingAccountRuntimeFailureStateStruct.pipe(
    Schema.filter(
      (failure: BillingAccountRuntimeFailureStateShape) =>
        failure.retainEvidenceExportAccess ||
        'Expected billing account runtime failures to retain evidence export and audit access'
    ),
    Schema.filter(
      (failure: BillingAccountRuntimeFailureStateShape) =>
        failure.existingLocalSafetyContinues ||
        'Expected billing account runtime failures to keep existing local safety behavior explicit'
    )
  )
);

const BillingAccountRuntimeManualInvoiceStateStruct = Schema.Struct({
  visible: Schema.Boolean,
  invoiceState: Schema.Union(Schema.Literal('manual-support-required'), Schema.Null),
});
type BillingAccountRuntimeManualInvoiceStateShape = Infer<typeof BillingAccountRuntimeManualInvoiceStateStruct>;

export const BillingAccountRuntimeManualInvoiceStateSchema = withParser(
  BillingAccountRuntimeManualInvoiceStateStruct.pipe(
    Schema.filter(
      (state: BillingAccountRuntimeManualInvoiceStateShape) =>
        state.visible === (state.invoiceState !== null) ||
        'Expected manual invoice visibility to match the manual invoice state marker'
    )
  )
);

const BillingAccountRuntimeStatusRowStruct = Schema.Struct({
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
  providerMode: BillingInvoiceProviderModeSchema,
  nextRenewalAt: Schema.Union(ParentTimestampSchema, Schema.Null),
  manualInvoiceState: BillingAccountRuntimeManualInvoiceStateSchema,
  failureState: Schema.Union(BillingAccountRuntimeFailureStateSchema, Schema.Null),
  auditReference: BillingAccountRuntimeAuditReferenceSchema,
});
type BillingAccountRuntimeStatusRowShape = Infer<typeof BillingAccountRuntimeStatusRowStruct>;

export const BillingAccountRuntimeStatusRowSchema = withParser(
  BillingAccountRuntimeStatusRowStruct.pipe(
    Schema.filter(
      (row: BillingAccountRuntimeStatusRowShape) =>
        !['backend-unavailable', 'provider-unavailable', 'manual-review'].includes(row.accountStatus) ||
        row.failureState !== null ||
        'Expected unavailable or manual billing account rows to carry a parent-visible failure state'
    ),
    Schema.filter(
      (row: BillingAccountRuntimeStatusRowShape) =>
        row.backendRuntimeState !== 'available' ||
        row.source === 'account-backend' ||
        'Expected available billing account runtime rows to come from an account backend source'
    ),
    Schema.filter(
      (row: BillingAccountRuntimeStatusRowShape) =>
        row.providerMode !== 'manual-invoice' ||
        (row.nextRenewalAt === null &&
          row.manualInvoiceState.visible &&
          row.manualInvoiceState.invoiceState === 'manual-support-required') ||
        'Expected manual invoice account runtime rows to stay non-renewing and explicitly manual-support owned'
    ),
    Schema.filter(
      (row: BillingAccountRuntimeStatusRowShape) =>
        row.providerMode !== 'stripe-hosted' ||
        (!row.manualInvoiceState.visible && row.manualInvoiceState.invoiceState === null) ||
        'Expected Stripe-hosted runtime rows not to expose manual invoice state'
    ),
    Schema.filter(
      (row: BillingAccountRuntimeStatusRowShape) =>
        !['active', 'past-due'].includes(row.accountStatus) ||
        row.providerMode !== 'stripe-hosted' ||
        row.nextRenewalAt !== null ||
        'Expected active and past-due Stripe-hosted rows to surface the next renewal timestamp'
    )
  )
);

const BillingAccountRuntimeOperationRowStruct = Schema.Struct({
  schemaVersion: BillingAccountRuntimeSchemaVersionSchema,
  operation: BillingAccountRuntimeOperationSchema,
  backendRuntimeState: BillingAccountBackendRuntimeStateSchema,
  providerBoundary: BillingAccountRuntimeProviderBoundarySchema,
  providerSecretCustody: BillingAccountRuntimeProviderSecretCustodySchema,
  childDeviceConsumption: BillingAccountRuntimeChildDeviceConsumptionSchema,
  manualRequired: Schema.Boolean,
  failureState: Schema.Union(BillingAccountRuntimeFailureStateSchema, Schema.Null),
});
type BillingAccountRuntimeOperationRowShape = Infer<typeof BillingAccountRuntimeOperationRowStruct>;

export const BillingAccountRuntimeOperationRowSchema = withParser(
  BillingAccountRuntimeOperationRowStruct.pipe(
    Schema.filter(
      (row: BillingAccountRuntimeOperationRowShape) =>
        row.backendRuntimeState === 'available' ||
        row.backendRuntimeState === 'not-implemented' ||
        row.manualRequired ||
        row.failureState !== null ||
        'Expected unavailable billing runtime operations to be failure-backed or manual-required'
    ),
    Schema.filter(
      (row: BillingAccountRuntimeOperationRowShape) =>
        row.operation !== 'provider-webhook-sync' ||
        row.providerBoundary === 'backend-reference-only' ||
        row.backendRuntimeState === 'not-implemented' ||
        'Expected provider webhook sync to stay behind the backend-reference boundary'
    ),
    Schema.filter(
      (row: BillingAccountRuntimeOperationRowShape) =>
        row.childDeviceConsumption !== 'signed-snapshot-consumed' ||
        row.operation === 'entitlement-snapshot-read' ||
        'Expected child-device billing consumption to be limited to signed entitlement snapshot reads'
    ),
    Schema.filter(
      (row: BillingAccountRuntimeOperationRowShape) =>
        row.operation !== 'entitlement-snapshot-read' ||
        row.childDeviceConsumption === 'signed-snapshot-consumed' ||
        'Expected entitlement snapshot reads to expose the child signed-snapshot consumption boundary'
    )
  )
);

const BillingAccountRuntimeEntitlementSigningBoundaryStruct = Schema.Struct({
  schemaVersion: BillingAccountRuntimeSchemaVersionSchema,
  signingState: BillingAccountRuntimeEntitlementSigningStateSchema,
  signedSnapshotAccepted: Schema.Boolean,
  manualRequired: Schema.Boolean,
  signingRuntimeClaim: BillingAccountRuntimeBackendClaimSchema,
  failureState: Schema.Union(BillingAccountRuntimeFailureStateSchema, Schema.Null),
});
type BillingAccountRuntimeEntitlementSigningBoundaryShape = Infer<
  typeof BillingAccountRuntimeEntitlementSigningBoundaryStruct
>;

export const BillingAccountRuntimeEntitlementSigningBoundarySchema = withParser(
  BillingAccountRuntimeEntitlementSigningBoundaryStruct.pipe(
    Schema.filter(
      (boundary: BillingAccountRuntimeEntitlementSigningBoundaryShape) =>
        boundary.signingState !== 'manual-required' ||
        (boundary.manualRequired && boundary.failureState !== null) ||
        'Expected manual-required entitlement signing to carry manual state and failure context'
    ),
    Schema.filter(
      (boundary: BillingAccountRuntimeEntitlementSigningBoundaryShape) =>
        boundary.signingState !== 'signed-snapshot-accepted' ||
        boundary.signedSnapshotAccepted ||
        'Expected signed entitlement snapshots to be explicitly accepted by schema proof'
    )
  )
);

const BillingAccountRuntimeBoundaryProofStruct = Schema.Struct({
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
});
type BillingAccountRuntimeBoundaryProofShape = Infer<typeof BillingAccountRuntimeBoundaryProofStruct>;

export const BillingAccountRuntimeBoundaryProofSchema = withParser(
  BillingAccountRuntimeBoundaryProofStruct.pipe(
    Schema.filter(
      (proof: BillingAccountRuntimeBoundaryProofShape) =>
        billingAccountRuntimeBoundaryProofIsHonest(proof) ||
        'Expected billing account runtime proof to keep backend provider secret UI custody non-claims and signed child consumption explicit'
    )
  )
);

export type BillingAccountRuntimeStatusRow = Infer<typeof BillingAccountRuntimeStatusRowSchema>;
export type BillingAccountRuntimeOperationRow = Infer<typeof BillingAccountRuntimeOperationRowSchema>;
export type BillingAccountRuntimeFailureState = Infer<typeof BillingAccountRuntimeFailureStateSchema>;
export type BillingAccountRuntimeManualInvoiceState = Infer<typeof BillingAccountRuntimeManualInvoiceStateSchema>;
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
    readonly providerMode: BillingInvoiceProviderMode;
    readonly nextRenewalAt: string | null;
    readonly manualInvoiceState: {
      readonly visible: boolean;
      readonly invoiceState: 'manual-support-required' | null;
    };
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
    proof.accountStatusRows.some(
      (row) => row.providerMode === 'stripe-hosted' && row.nextRenewalAt !== null && !row.manualInvoiceState.visible
    ) &&
    proof.accountStatusRows.some(
      (row) =>
        row.providerMode === 'manual-invoice' &&
        row.nextRenewalAt === null &&
        row.manualInvoiceState.invoiceState === 'manual-support-required'
    ) &&
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
