import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';
import { AppInstallPurchasePackageSourceCaptureStatusProofReadModel } from './app-install-purchase-package-source-capture-status-proof';
import { AppInstallPurchaseRuntimeWriterDeliveryProofReadModel } from './app-install-purchase-runtime-writer-delivery-proof';
import { ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';
import {
  buildAppInstallPurchaseChildDeviceDeliveryRuntimeWriterRowGenerated,
  childDeviceDeliveryRuntimeWriterProofIsHonestGenerated,
  childDeviceDeliveryRuntimeWriterRowIsHonestGenerated,
  summarizeAppInstallPurchaseChildDeviceDeliveryRuntimeWriterProofGenerated,
} from './generated/app-install-purchase-delivery-runtime-helpers';
const ChildDeviceDeliveryRuntimeWriterProofVersion = 'app-install-purchase-child-device-delivery-runtime-writer-proof';
const SourceRuntimeWriterDeliveryProofVersion = 'app-install-purchase-runtime-writer-delivery-proof';
const SourcePackageSourceCaptureStatusProofVersion = 'app-install-purchase-package-source-capture-status-proof';
const ChildDeviceDeliveryRuntimeWriterTimestamp = '2026-06-05T14:12:00.000Z';
const ChildDeviceDeliveryRuntimeWriterClaimBoundary =
  'child-device delivery runtime writer proof only; no runtime writer execution no runtime writer delivery no parent action runtime delivery no provider API execution no store integration no platform adapter implementation no child-device delivery no runtime report delivery no real install or purchase interception no child activity data no app blocking no Ocentra-hosted family data custody';
const RequiredDecisionActions = ['approve', 'deny', 'time-box', 'review-needed'] as const;
const RequiredRuntimeWriterDeliveryStates = ['writer-envelope-ready', 'manual-review-required'] as const;
const RequiredChildDeliveryEnvelopeStates = ['child-delivery-envelope-ready', 'manual-review-required'] as const;
const RequiredPackageSourceCaptureStatuses = ['captured', 'blocked', 'manual-required', 'unavailable'] as const;
const ChildDeviceDeliveryRuntimeWriterNonClaims = [
  'no-runtime-writer-execution',
  'no-runtime-writer-delivery',
  'no-parent-action-runtime-delivery',
  'no-provider-api-execution',
  'no-store-integration',
  'no-platform-adapter-implementation',
  'no-child-device-delivery',
  'no-runtime-report-delivery',
  'no-real-install-or-purchase-interception',
  'no-child-activity-data',
  'no-app-blocking',
  'no-ocentra-hosted-family-data-custody',
] as const;
const ChildDeviceDeliveryRuntimeWriterBoundaryFragments = [
  'no runtime writer execution',
  'no runtime writer delivery',
  'no parent action runtime delivery',
  'no provider API execution',
  'no store integration',
  'no platform adapter implementation',
  'no child-device delivery',
  'no runtime report delivery',
  'no real install or purchase interception',
  'no child activity data',
  'no app blocking',
  'no Ocentra-hosted family data custody',
] as const;

export const AppInstallPurchaseChildDeviceDeliveryRuntimeWriterProofSchemaVersionSchema = withParser(
  Schema.Literal(ChildDeviceDeliveryRuntimeWriterProofVersion)
);
const AppInstallPurchaseChildDeviceDeliveryRuntimeWriterActionSchema = withParser(
  Schema.Literal(...RequiredDecisionActions)
);
const AppInstallPurchaseChildDeviceDeliveryRuntimeWriterDeliveryStateSchema = withParser(
  Schema.Literal(...RequiredRuntimeWriterDeliveryStates)
);
const AppInstallPurchaseChildDeliveryEnvelopeStateSchema = withParser(
  Schema.Literal(...RequiredChildDeliveryEnvelopeStates)
);
const AppInstallPurchaseChildDeviceDeliveryRuntimeWriterPackageCaptureStatusSchema = withParser(
  Schema.Literal(...RequiredPackageSourceCaptureStatuses)
);
const AppInstallPurchaseChildDeviceDeliveryRuntimeWriterExecutionClaimSchema = withParser(
  Schema.Literal('not-executed')
);
const AppInstallPurchaseChildDeviceDeliveryRuntimeWriterDeliveryClaimSchema = withParser(
  Schema.Literal('not-delivered')
);
const AppInstallPurchaseChildDeviceDeliveryRuntimeWriterIntegrationClaimSchema = withParser(
  Schema.Literal('not-claimed')
);
const AppInstallPurchaseChildDeviceDeliveryRuntimeWriterAdapterClaimSchema = withParser(
  Schema.Literal('not-implemented')
);
const AppInstallPurchaseChildDeviceDeliveryRuntimeWriterInterceptionClaimSchema = withParser(
  Schema.Literal('not-claimed')
);
const AppInstallPurchaseChildDeviceDeliveryRuntimeWriterCustodyClaimSchema = withParser(
  Schema.Literal('no-child-activity-data')
);
const AppInstallPurchaseChildDeviceDeliveryRuntimeWriterNonClaimSchema = withParser(
  Schema.Literal(...ChildDeviceDeliveryRuntimeWriterNonClaims)
);

const ChildDeviceDeliveryRuntimeWriterRowIdSchema = brandedNonEmptyStringSchema(
  'AppInstallPurchaseChildDeviceDeliveryRuntimeWriterRowId'
);
const ChildDeviceDeliveryRuntimeWriterRefSchema = brandedNonEmptyStringSchema(
  'AppInstallPurchaseChildDeviceDeliveryRuntimeWriterRef'
);
const ChildDeviceDeliveryRuntimeWriterAuditRefSchema = brandedNonEmptyStringSchema(
  'AppInstallPurchaseChildDeviceDeliveryRuntimeWriterAuditRef'
);
const ChildDeviceDeliveryRuntimeWriterClaimBoundarySchema = brandedNonEmptyStringSchema(
  'AppInstallPurchaseChildDeviceDeliveryRuntimeWriterClaimBoundary'
);

const ChildDeviceDeliveryRuntimeWriterRowBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseChildDeviceDeliveryRuntimeWriterProofSchemaVersionSchema,
  childDeviceDeliveryRuntimeWriterRowId: ChildDeviceDeliveryRuntimeWriterRowIdSchema,
  sourceRuntimeWriterDeliveryProofVersion: Schema.Literal(SourceRuntimeWriterDeliveryProofVersion),
  sourceRuntimeWriterDeliveryRowId: ChildDeviceDeliveryRuntimeWriterRefSchema,
  sourceDecisionAction: AppInstallPurchaseChildDeviceDeliveryRuntimeWriterActionSchema,
  sourceRuntimeWriterDeliveryState: AppInstallPurchaseChildDeviceDeliveryRuntimeWriterDeliveryStateSchema,
  sourcePackageSourceCaptureStatusProofVersion: Schema.Literal(SourcePackageSourceCaptureStatusProofVersion),
  sourcePackageSourceCaptureRefs: Schema.Array(ChildDeviceDeliveryRuntimeWriterRefSchema),
  sourcePackageSourceCaptureStatuses: Schema.Array(
    AppInstallPurchaseChildDeviceDeliveryRuntimeWriterPackageCaptureStatusSchema
  ),
  childDeliveryEnvelopeState: AppInstallPurchaseChildDeliveryEnvelopeStateSchema,
  childDeliveryTargetRefs: Schema.Array(ChildDeviceDeliveryRuntimeWriterRefSchema),
  runtimeWriterAuditEventRefs: Schema.Array(ChildDeviceDeliveryRuntimeWriterAuditRefSchema),
  packageSourceAuditEventRefs: Schema.Array(ChildDeviceDeliveryRuntimeWriterAuditRefSchema),
  reportRuntimeRefs: Schema.Array(ChildDeviceDeliveryRuntimeWriterRefSchema),
  runtimeWriterExecutionClaim: AppInstallPurchaseChildDeviceDeliveryRuntimeWriterExecutionClaimSchema,
  runtimeWriterDeliveryClaim: AppInstallPurchaseChildDeviceDeliveryRuntimeWriterDeliveryClaimSchema,
  parentActionRuntimeDeliveryClaim: AppInstallPurchaseChildDeviceDeliveryRuntimeWriterDeliveryClaimSchema,
  providerApiExecutionClaim: AppInstallPurchaseChildDeviceDeliveryRuntimeWriterExecutionClaimSchema,
  storeIntegrationClaim: AppInstallPurchaseChildDeviceDeliveryRuntimeWriterIntegrationClaimSchema,
  platformAdapterClaim: AppInstallPurchaseChildDeviceDeliveryRuntimeWriterAdapterClaimSchema,
  childDeviceDeliveryClaim: AppInstallPurchaseChildDeviceDeliveryRuntimeWriterDeliveryClaimSchema,
  runtimeReportDeliveryClaim: AppInstallPurchaseChildDeviceDeliveryRuntimeWriterDeliveryClaimSchema,
  interceptionClaim: AppInstallPurchaseChildDeviceDeliveryRuntimeWriterInterceptionClaimSchema,
  appBlockingClaim: AppInstallPurchaseChildDeviceDeliveryRuntimeWriterIntegrationClaimSchema,
  childDataCustody: AppInstallPurchaseChildDeviceDeliveryRuntimeWriterCustodyClaimSchema,
  ocentraHostedFamilyDataCustodyClaim: AppInstallPurchaseChildDeviceDeliveryRuntimeWriterIntegrationClaimSchema,
  claimBoundary: ChildDeviceDeliveryRuntimeWriterClaimBoundarySchema,
  linkedAt: ParentTimestampSchema,
});

type ChildDeviceDeliveryRuntimeWriterRowCandidate = Infer<typeof ChildDeviceDeliveryRuntimeWriterRowBaseSchema>;

export const AppInstallPurchaseChildDeviceDeliveryRuntimeWriterRowSchema = withParser(
  ChildDeviceDeliveryRuntimeWriterRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        childDeviceDeliveryRuntimeWriterRowIsHonest(row) ||
        'Expected child-device delivery runtime writer rows to link writer and package-source status refs without execution, delivery, provider, adapter, custody, interception, or blocking claims'
    )
  )
);

const ChildDeviceDeliveryRuntimeWriterProofBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseChildDeviceDeliveryRuntimeWriterProofSchemaVersionSchema,
  sourceRuntimeWriterDeliveryProofVersion: Schema.Literal(SourceRuntimeWriterDeliveryProofVersion),
  sourcePackageSourceCaptureStatusProofVersion: Schema.Literal(SourcePackageSourceCaptureStatusProofVersion),
  childDeviceDeliveryRuntimeWriterRows: Schema.Array(AppInstallPurchaseChildDeviceDeliveryRuntimeWriterRowSchema),
  nonClaims: Schema.Array(AppInstallPurchaseChildDeviceDeliveryRuntimeWriterNonClaimSchema),
  knownGaps: Schema.Array(ChildDeviceDeliveryRuntimeWriterRefSchema),
  updatedAt: ParentTimestampSchema,
});

export type AppInstallPurchaseChildDeviceDeliveryRuntimeWriterProof = Infer<
  typeof ChildDeviceDeliveryRuntimeWriterProofBaseSchema
>;

export const AppInstallPurchaseChildDeviceDeliveryRuntimeWriterProofSchema = withParser(
  ChildDeviceDeliveryRuntimeWriterProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        childDeviceDeliveryRuntimeWriterProofIsHonest(proof) ||
        'Expected app install/purchase child-device delivery runtime writer proof to cover review actions and package-source statuses while preserving non-claims'
    )
  )
);

export const AppInstallPurchaseChildDeviceDeliveryRuntimeWriterKnownGaps = [
  'Child-device delivery runtime writer rows are contract/proof rows only; no writer process or delivery transport is implemented.',
  'Provider/store execution, store integration, platform adapters, child-device delivery, runtime report delivery, interception, app blocking, and Ocentra-hosted family custody remain unimplemented.',
  'Review-needed remains manual-required until portal approval UI and a real runtime writer/delivery path exist.',
] as const;

export const AppInstallPurchaseChildDeviceDeliveryRuntimeWriterProofReadModel =
  AppInstallPurchaseChildDeviceDeliveryRuntimeWriterProofSchema.parse({
    schemaVersion: ChildDeviceDeliveryRuntimeWriterProofVersion,
    sourceRuntimeWriterDeliveryProofVersion: SourceRuntimeWriterDeliveryProofVersion,
    sourcePackageSourceCaptureStatusProofVersion: SourcePackageSourceCaptureStatusProofVersion,
    childDeviceDeliveryRuntimeWriterRows:
      AppInstallPurchaseRuntimeWriterDeliveryProofReadModel.runtimeWriterDeliveryRows.map(
        childDeviceDeliveryRuntimeWriterRow
      ),
    nonClaims: ChildDeviceDeliveryRuntimeWriterNonClaims,
    knownGaps: AppInstallPurchaseChildDeviceDeliveryRuntimeWriterKnownGaps,
    updatedAt: ChildDeviceDeliveryRuntimeWriterTimestamp,
  });

export function summarizeAppInstallPurchaseChildDeviceDeliveryRuntimeWriterProof(
  proof: AppInstallPurchaseChildDeviceDeliveryRuntimeWriterProof
) {
  return summarizeAppInstallPurchaseChildDeviceDeliveryRuntimeWriterProofGenerated(proof);
}

function childDeviceDeliveryRuntimeWriterRow(
  row: (typeof AppInstallPurchaseRuntimeWriterDeliveryProofReadModel.runtimeWriterDeliveryRows)[number]
) {
  return buildAppInstallPurchaseChildDeviceDeliveryRuntimeWriterRowGenerated(
    row,
    SourceRuntimeWriterDeliveryProofVersion,
    SourcePackageSourceCaptureStatusProofVersion,
    packageSourceCaptureRefs(),
    packageSourceCaptureStatuses(),
    childDeliveryTargetRefs(row.reportRuntimeRefs),
    packageSourceAuditEventRefs(),
    ChildDeviceDeliveryRuntimeWriterClaimBoundary,
    ChildDeviceDeliveryRuntimeWriterTimestamp
  );
}

function packageSourceCaptureRefs() {
  return AppInstallPurchasePackageSourceCaptureStatusProofReadModel.packageSourceCaptureRows.map(
    (row) => row.packageSourceCaptureRowId
  );
}

function packageSourceCaptureStatuses() {
  return AppInstallPurchasePackageSourceCaptureStatusProofReadModel.packageSourceCaptureRows.map(
    (row) => row.packageSourceCaptureStatus
  );
}

function packageSourceAuditEventRefs() {
  return Array.from(
    new Set(
      AppInstallPurchasePackageSourceCaptureStatusProofReadModel.packageSourceCaptureRows.flatMap(
        (row) => row.auditEventRefs
      )
    )
  );
}

function childDeliveryTargetRefs(runtimeReportRefs: readonly string[]) {
  return Array.from(
    new Set([
      ...runtimeReportRefs,
      ...AppInstallPurchasePackageSourceCaptureStatusProofReadModel.packageSourceCaptureRows.flatMap(
        (row) => row.packageSourceCaptureArtifactRefs
      ),
    ])
  );
}

function childDeviceDeliveryRuntimeWriterRowIsHonest(row: ChildDeviceDeliveryRuntimeWriterRowCandidate): boolean {
  return childDeviceDeliveryRuntimeWriterRowIsHonestGenerated(
    row,
    SourcePackageSourceCaptureStatusProofVersion,
    RequiredPackageSourceCaptureStatuses,
    ChildDeviceDeliveryRuntimeWriterBoundaryFragments
  );
}

function childDeviceDeliveryRuntimeWriterProofIsHonest(
  proof: AppInstallPurchaseChildDeviceDeliveryRuntimeWriterProof
): boolean {
  return (
    childDeviceDeliveryRuntimeWriterProofIsHonestGenerated(
      proof,
      SourceRuntimeWriterDeliveryProofVersion,
      SourcePackageSourceCaptureStatusProofVersion,
      RequiredDecisionActions,
      RequiredChildDeliveryEnvelopeStates,
      ChildDeviceDeliveryRuntimeWriterNonClaims
    ) &&
    proof.childDeviceDeliveryRuntimeWriterRows.every((row) => childDeviceDeliveryRuntimeWriterRowIsHonest(row)) &&
    proof.knownGaps.length > 0
  );
}
