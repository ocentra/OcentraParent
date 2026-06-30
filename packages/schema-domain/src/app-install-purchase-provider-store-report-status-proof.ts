import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';
import { AppInstallPurchaseApprovalReportDomainProofReadModel } from './app-install-purchase-approval-report-domain-proof';
import { AppInstallPurchaseProviderStoreExecutionReadinessProofReadModel } from './app-install-purchase-provider-store-execution-readiness-proof';
import { ParentPlatformSchema, ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';
import {
  buildAppInstallPurchaseProviderStoreReportStatusRowGenerated,
  providerStoreReportStatusProofIsHonestGenerated,
  providerStoreReportStatusRowIsHonestGenerated,
  summarizeAppInstallPurchaseProviderStoreReportStatusProofGenerated,
} from './generated/app-install-purchase-report-status-helpers';
const ProviderStoreReportStatusProofVersion = 'app-install-purchase-provider-store-report-status-proof';
const SourceProviderStoreExecutionReadinessProofVersion =
  AppInstallPurchaseProviderStoreExecutionReadinessProofReadModel.schemaVersion;
const SourceApprovalReportDomainProofVersion = AppInstallPurchaseApprovalReportDomainProofReadModel.schemaVersion;
const ProviderStoreReportStatusTimestamp = '2026-06-06T01:24:00.000Z';
const ProviderStoreReportStatusClaimBoundary =
  'provider store report status proof only; no provider API execution no store integration no billing provider contact no portal approval UI no portal report UI no runtime report delivery no platform adapter implementation no child-device delivery no app blocking no child activity data no Ocentra-hosted family data custody';
const RequiredProviderStoreReportStatusStates = [
  'provider-store-report-status-ready',
  'manual-required',
  'unavailable',
] as const;
const ProviderStoreReportStatusNonClaims = [
  'no-provider-api-execution',
  'no-store-integration',
  'no-billing-provider-contact',
  'no-portal-approval-ui',
  'no-portal-report-ui',
  'no-runtime-report-delivery',
  'no-platform-adapter-implementation',
  'no-child-device-delivery',
  'no-app-blocking',
  'no-child-activity-data',
  'no-ocentra-hosted-family-data-custody',
] as const;
const ProviderStoreReportStatusBoundaryFragments = [
  'no provider API execution',
  'no store integration',
  'no billing provider contact',
  'no portal approval UI',
  'no portal report UI',
  'no runtime report delivery',
  'no platform adapter implementation',
  'no child-device delivery',
  'no app blocking',
  'no child activity data',
  'no Ocentra-hosted family data custody',
] as const;

export const AppInstallPurchaseProviderStoreReportStatusProofSchemaVersionSchema = withParser(
  Schema.Literal(ProviderStoreReportStatusProofVersion)
);
const AppInstallPurchaseProviderStoreReportStatusStoreSurfaceSchema = withParser(
  Schema.Literal('microsoft-store', 'mac-app-store', 'linux-package-manager', 'google-play', 'apple-app-store')
);
const AppInstallPurchaseProviderStoreReportStatusStateSchema = withParser(
  Schema.Literal(...RequiredProviderStoreReportStatusStates)
);
const AppInstallPurchaseProviderStoreReportStatusReadinessStateSchema = withParser(
  Schema.Literal('provider-store-execution-ready', 'manual-required', 'unavailable')
);
const AppInstallPurchaseProviderStoreReportStatusApprovalReportStateSchema = withParser(
  Schema.Literal('approval-report-ready', 'approval-report-manual-review', 'approval-report-unavailable')
);
const AppInstallPurchaseProviderStoreReportStatusNotExecutedSchema = withParser(Schema.Literal('not-executed'));
const AppInstallPurchaseProviderStoreReportStatusNotClaimedSchema = withParser(Schema.Literal('not-claimed'));
const AppInstallPurchaseProviderStoreReportStatusNotImplementedSchema = withParser(Schema.Literal('not-implemented'));
const AppInstallPurchaseProviderStoreReportStatusNotDeliveredSchema = withParser(Schema.Literal('not-delivered'));
const AppInstallPurchaseProviderStoreReportStatusCustodySchema = withParser(Schema.Literal('no-child-activity-data'));
const AppInstallPurchaseProviderStoreReportStatusNonClaimSchema = withParser(
  Schema.Literal(...ProviderStoreReportStatusNonClaims)
);

const ProviderStoreReportStatusRowIdSchema = brandedNonEmptyStringSchema(
  'AppInstallPurchaseProviderStoreReportStatusRowId'
);
const ProviderStoreReportStatusRefSchema = brandedNonEmptyStringSchema(
  'AppInstallPurchaseProviderStoreReportStatusRef'
);
const ProviderStoreReportStatusBoundarySchema = brandedNonEmptyStringSchema(
  'AppInstallPurchaseProviderStoreReportStatusBoundary'
);

const ProviderStoreReportStatusRowBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseProviderStoreReportStatusProofSchemaVersionSchema,
  providerStoreReportStatusRowId: ProviderStoreReportStatusRowIdSchema,
  sourceProviderStoreExecutionReadinessProofVersion: Schema.Literal(SourceProviderStoreExecutionReadinessProofVersion),
  sourceProviderStoreExecutionReadinessRowId: ProviderStoreReportStatusRefSchema,
  sourceProviderStoreExecutionReadinessState: AppInstallPurchaseProviderStoreReportStatusReadinessStateSchema,
  sourceApprovalReportDomainProofVersion: Schema.Literal(SourceApprovalReportDomainProofVersion),
  sourceApprovalReportDomainRowIds: Schema.Array(ProviderStoreReportStatusRefSchema),
  sourceApprovalReportDomainStates: Schema.Array(AppInstallPurchaseProviderStoreReportStatusApprovalReportStateSchema),
  sourceReportRuntimeRefs: Schema.Array(ProviderStoreReportStatusRefSchema),
  sourceAuditEventRefs: Schema.Array(ProviderStoreReportStatusRefSchema),
  platform: ParentPlatformSchema,
  storeSurface: AppInstallPurchaseProviderStoreReportStatusStoreSurfaceSchema,
  providerStoreReportStatusState: AppInstallPurchaseProviderStoreReportStatusStateSchema,
  providerApiExecutionClaim: AppInstallPurchaseProviderStoreReportStatusNotExecutedSchema,
  storeIntegrationClaim: AppInstallPurchaseProviderStoreReportStatusNotClaimedSchema,
  billingProviderContactClaim: AppInstallPurchaseProviderStoreReportStatusNotExecutedSchema,
  portalApprovalUiClaim: AppInstallPurchaseProviderStoreReportStatusNotImplementedSchema,
  portalReportUiClaim: AppInstallPurchaseProviderStoreReportStatusNotImplementedSchema,
  runtimeReportDeliveryClaim: AppInstallPurchaseProviderStoreReportStatusNotDeliveredSchema,
  platformAdapterClaim: AppInstallPurchaseProviderStoreReportStatusNotImplementedSchema,
  childDeviceDeliveryClaim: AppInstallPurchaseProviderStoreReportStatusNotDeliveredSchema,
  appBlockingClaim: AppInstallPurchaseProviderStoreReportStatusNotClaimedSchema,
  childDataCustody: AppInstallPurchaseProviderStoreReportStatusCustodySchema,
  ocentraHostedFamilyDataCustodyClaim: AppInstallPurchaseProviderStoreReportStatusNotClaimedSchema,
  claimBoundary: ProviderStoreReportStatusBoundarySchema,
  evaluatedAt: ParentTimestampSchema,
});

type ProviderStoreReportStatusRowCandidate = Infer<typeof ProviderStoreReportStatusRowBaseSchema>;

export const AppInstallPurchaseProviderStoreReportStatusRowSchema = withParser(
  ProviderStoreReportStatusRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        providerStoreReportStatusRowIsHonest(row) ||
        'Expected provider/store report status rows to link provider readiness to approval/report domain refs without provider, portal, delivery, adapter, custody, or blocking claims'
    )
  )
);

const ProviderStoreReportStatusProofBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseProviderStoreReportStatusProofSchemaVersionSchema,
  sourceProviderStoreExecutionReadinessProofVersion: Schema.Literal(SourceProviderStoreExecutionReadinessProofVersion),
  sourceApprovalReportDomainProofVersion: Schema.Literal(SourceApprovalReportDomainProofVersion),
  providerStoreReportStatusRows: Schema.Array(AppInstallPurchaseProviderStoreReportStatusRowSchema),
  nonClaims: Schema.Array(AppInstallPurchaseProviderStoreReportStatusNonClaimSchema),
  knownGaps: Schema.Array(ProviderStoreReportStatusRefSchema),
  updatedAt: ParentTimestampSchema,
});

export type AppInstallPurchaseProviderStoreReportStatusProof = Infer<typeof ProviderStoreReportStatusProofBaseSchema>;

export const AppInstallPurchaseProviderStoreReportStatusProofSchema = withParser(
  ProviderStoreReportStatusProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        providerStoreReportStatusProofIsHonest(proof) ||
        'Expected app install/purchase provider/store report status proof to cover readiness status rows and preserve non-claims'
    )
  )
);

export const AppInstallPurchaseProviderStoreReportStatusKnownGaps = [
  'Provider/store report status rows are rust-parent-runtime proof rows only; no provider API execution store integration or billing provider contact is implemented.',
  'Approval/report domain refs remain read-model proof rows only; portal approval UI portal report UI and runtime report delivery remain unimplemented.',
  'Package export checklist and README updates are sequenced behind current package-json checklist and rust-parent-runtime README locks; feature and expectation docs already record provider/store report status proof coverage.',
] as const;

export const AppInstallPurchaseProviderStoreReportStatusProofReadModel =
  AppInstallPurchaseProviderStoreReportStatusProofSchema.parse({
    schemaVersion: ProviderStoreReportStatusProofVersion,
    sourceProviderStoreExecutionReadinessProofVersion: SourceProviderStoreExecutionReadinessProofVersion,
    sourceApprovalReportDomainProofVersion: SourceApprovalReportDomainProofVersion,
    providerStoreReportStatusRows:
      AppInstallPurchaseProviderStoreExecutionReadinessProofReadModel.providerStoreExecutionReadinessRows.map(
        providerStoreReportStatusRow
      ),
    nonClaims: ProviderStoreReportStatusNonClaims,
    knownGaps: AppInstallPurchaseProviderStoreReportStatusKnownGaps,
    updatedAt: ProviderStoreReportStatusTimestamp,
  });

export function summarizeAppInstallPurchaseProviderStoreReportStatusProof(
  proof: AppInstallPurchaseProviderStoreReportStatusProof
) {
  return summarizeAppInstallPurchaseProviderStoreReportStatusProofGenerated(proof);
}

function providerStoreReportStatusRow(
  row: (typeof AppInstallPurchaseProviderStoreExecutionReadinessProofReadModel.providerStoreExecutionReadinessRows)[number]
) {
  return buildAppInstallPurchaseProviderStoreReportStatusRowGenerated(
    row,
    AppInstallPurchaseApprovalReportDomainProofReadModel.approvalReportDomainRows,
    SourceProviderStoreExecutionReadinessProofVersion,
    SourceApprovalReportDomainProofVersion,
    ProviderStoreReportStatusClaimBoundary,
    ProviderStoreReportStatusTimestamp
  );
}

function providerStoreReportStatusRowIsHonest(row: ProviderStoreReportStatusRowCandidate): boolean {
  return providerStoreReportStatusRowIsHonestGenerated(
    row,
    AppInstallPurchaseApprovalReportDomainProofReadModel.approvalReportDomainRows.length,
    ProviderStoreReportStatusBoundaryFragments
  );
}

function providerStoreReportStatusProofIsHonest(proof: AppInstallPurchaseProviderStoreReportStatusProof): boolean {
  return (
    providerStoreReportStatusProofIsHonestGenerated(
      proof,
      AppInstallPurchaseProviderStoreExecutionReadinessProofReadModel.providerStoreExecutionReadinessRows.length,
      RequiredProviderStoreReportStatusStates,
      ProviderStoreReportStatusNonClaims
    ) && proof.providerStoreReportStatusRows.every(providerStoreReportStatusRowIsHonest)
  );
}
