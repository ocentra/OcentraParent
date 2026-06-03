import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  AppInstallPurchaseApprovalPlatformSupportStateSchema,
  AppInstallPurchaseApprovalStoreSurfaceSchema,
} from './app-install-purchase-approval';
import { appGameInstallStoreHandoffRowIsHonest } from './app-game-install-store-handoff-rules';
import { EnforcementCapabilityStateSchema } from './enforcement';
import { ParentEvidenceReferenceSchema } from './references';
import { ParentContractSchemaVersionSchema, ParentPlatformSchema, ParentTimestampSchema } from './reference-primitives';

const NonEmptyHandoffText = Schema.String.pipe(Schema.minLength(1));

export const AppGameInstallStoreHandoffSchemaVersionSchema = withParser(
  Schema.Literal('app-game-install-store-handoff-proof')
);

export const AppGameInstallStoreHandoffSignalKindSchema = withParser(
  Schema.Literal(
    'new-inventory-detected',
    'installer-updater-process',
    'store-package-install',
    'game-purchase-signal',
    'uninstall-detected',
    'tamper-uninstall-candidate'
  )
);

export const AppGameInstallStoreHandoffProductSliceSchema = withParser(
  Schema.Literal('native-app', 'native-game', 'shared-app-game')
);

export const AppGameInstallStoreHandoffDestinationFeatureDocSchema = withParser(
  Schema.Literal(
    'docs/features/app-game-control.md',
    'docs/features/app-install-purchase-approval.md',
    'docs/features/enforcement-integrity-tamper.md'
  )
);

export const AppGameInstallStoreHandoffExpectationDocSchema = withParser(
  Schema.Literal(
    'docs/expectations/app-game-evidence.md',
    'docs/expectations/app-install-purchase-approval.md',
    'docs/expectations/enforcement.md',
    'docs/expectations/tamper-uninstall-protection.md'
  )
);

export const AppGameInstallStoreHandoffDecisionAuthoritySchema = withParser(
  Schema.Literal(
    'evidence-context-only',
    'approval-feature-handoff',
    'tamper-uninstall-feature-handoff',
    'manual-review-required'
  )
);

export const AppGameInstallStoreHandoffStoreSignalUseSchema = withParser(
  Schema.Literal('not-store-signal', 'context-only-not-decision')
);

export const AppGameInstallStoreHandoffNoClaimBoundarySchema = withParser(
  Schema.Literal(
    'inventory-is-not-use',
    'store-signal-not-safety-decision',
    'not-generic-app-blocking',
    'no-store-interception',
    'no-platform-adapter-execution',
    'no-tamper-blocking',
    'no-billing-entitlement-logic'
  )
);

const AppGameInstallStoreHandoffIdSchema = NonEmptyHandoffText.pipe(Schema.brand('AppGameInstallStoreHandoffId'));
const AppGameInstallStoreHandoffApprovalRequestRefSchema = NonEmptyHandoffText.pipe(
  Schema.brand('AppGameInstallStoreHandoffApprovalRequestRef')
);
const AppGameInstallStoreHandoffManualRequirementSchema = NonEmptyHandoffText.pipe(
  Schema.brand('AppGameInstallStoreHandoffManualRequirement')
);
const AppGameInstallStoreHandoffParentVisibleStateSchema = NonEmptyHandoffText.pipe(
  Schema.brand('AppGameInstallStoreHandoffParentVisibleState')
);
const AppGameInstallStoreHandoffProofPackRefSchema = NonEmptyHandoffText.pipe(
  Schema.brand('AppGameInstallStoreHandoffProofPackRef')
);
const AppGameInstallStoreHandoffMatrixIdSchema = NonEmptyHandoffText.pipe(
  Schema.brand('AppGameInstallStoreHandoffMatrixId')
);

const AppGameInstallStoreHandoffRowBaseSchema = Schema.Struct({
  schemaVersion: AppGameInstallStoreHandoffSchemaVersionSchema,
  handoffId: AppGameInstallStoreHandoffIdSchema,
  productSlice: AppGameInstallStoreHandoffProductSliceSchema,
  signalKind: AppGameInstallStoreHandoffSignalKindSchema,
  platform: ParentPlatformSchema,
  storeSurface: AppInstallPurchaseApprovalStoreSurfaceSchema,
  decisionAuthority: AppGameInstallStoreHandoffDecisionAuthoritySchema,
  storeSignalUse: AppGameInstallStoreHandoffStoreSignalUseSchema,
  approvalSupportState: AppInstallPurchaseApprovalPlatformSupportStateSchema,
  capabilityState: EnforcementCapabilityStateSchema,
  approvalRequestRef: Schema.Union(AppGameInstallStoreHandoffApprovalRequestRefSchema, Schema.Null),
  evidenceReferences: Schema.Array(ParentEvidenceReferenceSchema),
  destinationFeatureDocs: Schema.Array(AppGameInstallStoreHandoffDestinationFeatureDocSchema),
  expectationDocRefs: Schema.Array(AppGameInstallStoreHandoffExpectationDocSchema),
  manualRequirement: Schema.Union(AppGameInstallStoreHandoffManualRequirementSchema, Schema.Null),
  parentVisibleManualState: Schema.Union(AppGameInstallStoreHandoffParentVisibleStateSchema, Schema.Null),
  noClaimBoundaries: Schema.Array(AppGameInstallStoreHandoffNoClaimBoundarySchema),
  proofPackRefs: Schema.Array(AppGameInstallStoreHandoffProofPackRefSchema),
  adapterExecutionClaim: Schema.Literal('not-claimed'),
  policyDecisionClaim: Schema.Literal('not-claimed'),
  recordedAt: ParentTimestampSchema,
});

type AppGameInstallStoreHandoffRowCandidate = Infer<typeof AppGameInstallStoreHandoffRowBaseSchema>;

export const AppGameInstallStoreHandoffRowSchema = withParser(
  AppGameInstallStoreHandoffRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        appGameInstallStoreHandoffRowIsHonest(row) ||
        'Expected app/game install-store handoff rows to keep store signals contextual, approval refs evidence-backed, and uninstall/tamper routed without adapter claims'
    )
  )
);

export const AppGameInstallStoreHandoffMatrixSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    matrixId: AppGameInstallStoreHandoffMatrixIdSchema,
    generatedAt: ParentTimestampSchema,
    rows: Schema.Array(AppGameInstallStoreHandoffRowSchema),
  }).pipe(
    Schema.filter(
      (matrix) =>
        matrix.rows.length > 0 && matrixRowsAreUnique(matrix.rows) && matrixHasRequiredSignalCoverage(matrix.rows)
    )
  )
);

function matrixRowsAreUnique(rows: readonly AppGameInstallStoreHandoffRowCandidate[]): boolean {
  const seen = new Set<string>();

  for (const row of rows) {
    if (seen.has(row.handoffId)) {
      return false;
    }

    seen.add(row.handoffId);
  }

  return true;
}

const requiredSignalKinds: readonly AppGameInstallStoreHandoffSignalKind[] = [
  'new-inventory-detected',
  'installer-updater-process',
  'store-package-install',
  'game-purchase-signal',
  'uninstall-detected',
  'tamper-uninstall-candidate',
];

function matrixHasRequiredSignalCoverage(rows: readonly AppGameInstallStoreHandoffRowCandidate[]): boolean {
  const signalKinds = new Set(rows.map((row) => row.signalKind));
  return requiredSignalKinds.every((signalKind) => signalKinds.has(signalKind));
}

export type AppGameInstallStoreHandoffSignalKind = Infer<typeof AppGameInstallStoreHandoffSignalKindSchema>;
export type AppGameInstallStoreHandoffProductSlice = Infer<typeof AppGameInstallStoreHandoffProductSliceSchema>;
export type AppGameInstallStoreHandoffDecisionAuthority = Infer<
  typeof AppGameInstallStoreHandoffDecisionAuthoritySchema
>;
export type AppGameInstallStoreHandoffNoClaimBoundary = Infer<typeof AppGameInstallStoreHandoffNoClaimBoundarySchema>;
export type AppGameInstallStoreHandoffRow = Infer<typeof AppGameInstallStoreHandoffRowSchema>;
export type AppGameInstallStoreHandoffMatrix = Infer<typeof AppGameInstallStoreHandoffMatrixSchema>;

export const decodeAppGameInstallStoreHandoffRow = Schema.decodeUnknownSync(AppGameInstallStoreHandoffRowSchema);
export const decodeAppGameInstallStoreHandoffMatrix = Schema.decodeUnknownSync(AppGameInstallStoreHandoffMatrixSchema);
