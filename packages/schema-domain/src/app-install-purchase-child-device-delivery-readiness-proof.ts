import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';
import { AppInstallPurchaseChildDeviceDeliveryRuntimeWriterProofReadModel } from './app-install-purchase-child-device-delivery-runtime-writer-proof';
import { AppInstallPurchasePackageSourceAdapterExecutionProofReadModel } from './app-install-purchase-package-source-adapter-execution-proof';
import { AppInstallPurchasePlatformLimitationActionProofReadModel } from './app-install-purchase-platform-limitation-action-proof';
import { ParentPlatformSchema, ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';
import {
  buildAppInstallPurchaseChildDeviceDeliveryReadinessRowGenerated,
  childDeviceDeliveryReadinessProofIsHonestGenerated,
  childDeviceDeliveryReadinessRowIsHonestGenerated,
  summarizeAppInstallPurchaseChildDeviceDeliveryReadinessProofGenerated,
} from './generated/app-install-purchase-delivery-runtime-helpers';
const ChildDeviceDeliveryReadinessProofVersion = 'app-install-purchase-child-device-delivery-readiness-proof';
const SourceChildDeviceDeliveryRuntimeWriterProofVersion =
  'app-install-purchase-child-device-delivery-runtime-writer-proof';
const SourcePackageSourceAdapterExecutionProofVersion = 'app-install-purchase-package-source-adapter-execution-proof';
const SourcePlatformLimitationActionProofVersion = 'app-install-purchase-platform-limitation-action-proof';
const ChildDeviceDeliveryReadinessTimestamp = '2026-06-06T05:17:00.000Z';
const ChildDeviceDeliveryReadinessBoundary =
  'child-device delivery readiness proof only; links child delivery runtime-writer envelopes package-source adapter execution rows and platform limitation action rows no child-device delivery no runtime writer execution no runtime writer delivery no provider API execution no store integration no platform adapter implementation no app blocking no child activity data no Ocentra-hosted family data custody';
const ChildDeviceDeliveryReadinessStates = [
  'delivery-evidence-ready',
  'manual-proof-required',
  'platform-unavailable',
  'policy-blocked',
] as const;
const ChildDeviceDeliveryReadinessNonClaims = [
  'no-child-device-delivery',
  'no-runtime-writer-execution',
  'no-runtime-writer-delivery',
  'no-provider-api-execution',
  'no-store-integration',
  'no-platform-adapter-implementation',
  'no-app-blocking',
  'no-child-activity-data',
  'no-ocentra-hosted-family-data-custody',
] as const;
const ChildDeviceDeliveryReadinessBoundaryFragments = [
  'child delivery runtime-writer envelopes',
  'package-source adapter execution rows',
  'platform limitation action rows',
  'no child-device delivery',
  'no runtime writer execution',
  'no runtime writer delivery',
  'no provider API execution',
  'no store integration',
  'no platform adapter implementation',
  'no app blocking',
  'no child activity data',
  'no Ocentra-hosted family data custody',
] as const;

export const AppInstallPurchaseChildDeviceDeliveryReadinessProofSchemaVersionSchema = withParser(
  Schema.Literal(ChildDeviceDeliveryReadinessProofVersion)
);
const ChildDeviceDeliveryReadinessStateSchema = withParser(Schema.Literal(...ChildDeviceDeliveryReadinessStates));
const ChildDeviceDeliveryReadinessNotDeliveredSchema = withParser(Schema.Literal('not-delivered'));
const ChildDeviceDeliveryReadinessNotExecutedSchema = withParser(Schema.Literal('not-executed'));
const ChildDeviceDeliveryReadinessNotClaimedSchema = withParser(Schema.Literal('not-claimed'));
const ChildDeviceDeliveryReadinessNotImplementedSchema = withParser(Schema.Literal('not-implemented'));
const ChildDeviceDeliveryReadinessCustodySchema = withParser(Schema.Literal('no-child-activity-data'));
const ChildDeviceDeliveryReadinessNonClaimSchema = withParser(Schema.Literal(...ChildDeviceDeliveryReadinessNonClaims));

const ChildDeviceDeliveryReadinessRowIdSchema = brandedNonEmptyStringSchema(
  'AppInstallPurchaseChildDeviceDeliveryReadinessRowId'
);
const ChildDeviceDeliveryReadinessRefSchema = brandedNonEmptyStringSchema(
  'AppInstallPurchaseChildDeviceDeliveryReadinessRef'
);
const ChildDeviceDeliveryReadinessBoundarySchema = brandedNonEmptyStringSchema(
  'AppInstallPurchaseChildDeviceDeliveryReadinessBoundary'
);

const ChildDeviceDeliveryReadinessRowBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseChildDeviceDeliveryReadinessProofSchemaVersionSchema,
  childDeviceDeliveryReadinessRowId: ChildDeviceDeliveryReadinessRowIdSchema,
  platform: ParentPlatformSchema,
  childDeviceDeliveryReadinessState: ChildDeviceDeliveryReadinessStateSchema,
  sourceChildDeviceDeliveryRuntimeWriterProofVersion: Schema.Literal(
    SourceChildDeviceDeliveryRuntimeWriterProofVersion
  ),
  sourceChildDeliveryRuntimeWriterRowIds: Schema.Array(ChildDeviceDeliveryReadinessRefSchema),
  sourcePackageSourceAdapterExecutionProofVersion: Schema.Literal(SourcePackageSourceAdapterExecutionProofVersion),
  sourcePackageSourceAdapterExecutionRowId: ChildDeviceDeliveryReadinessRefSchema,
  sourcePlatformLimitationActionProofVersion: Schema.Literal(SourcePlatformLimitationActionProofVersion),
  sourcePlatformLimitationActionRowId: ChildDeviceDeliveryReadinessRefSchema,
  requiredDeliveryProofRefs: Schema.Array(ChildDeviceDeliveryReadinessRefSchema),
  parentVisibleStatusRefs: Schema.Array(ChildDeviceDeliveryReadinessRefSchema),
  childDeviceDeliveryClaim: ChildDeviceDeliveryReadinessNotDeliveredSchema,
  runtimeWriterExecutionClaim: ChildDeviceDeliveryReadinessNotExecutedSchema,
  runtimeWriterDeliveryClaim: ChildDeviceDeliveryReadinessNotDeliveredSchema,
  providerApiExecutionClaim: ChildDeviceDeliveryReadinessNotExecutedSchema,
  storeIntegrationClaim: ChildDeviceDeliveryReadinessNotClaimedSchema,
  platformAdapterClaim: ChildDeviceDeliveryReadinessNotImplementedSchema,
  appBlockingClaim: ChildDeviceDeliveryReadinessNotClaimedSchema,
  childDataCustody: ChildDeviceDeliveryReadinessCustodySchema,
  ocentraHostedFamilyDataCustodyClaim: ChildDeviceDeliveryReadinessNotClaimedSchema,
  claimBoundary: ChildDeviceDeliveryReadinessBoundarySchema,
  recordedAt: ParentTimestampSchema,
});

type ChildDeviceDeliveryReadinessRowCandidate = Infer<typeof ChildDeviceDeliveryReadinessRowBaseSchema>;

export const AppInstallPurchaseChildDeviceDeliveryReadinessRowSchema = withParser(
  ChildDeviceDeliveryReadinessRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        childDeviceDeliveryReadinessRowIsHonest(row) ||
        'Expected child-device delivery readiness rows to link child delivery, adapter, and limitation refs without delivery, provider, adapter, custody, or blocking claims'
    )
  )
);

const ChildDeviceDeliveryReadinessProofBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseChildDeviceDeliveryReadinessProofSchemaVersionSchema,
  sourceChildDeviceDeliveryRuntimeWriterProofVersion: Schema.Literal(
    SourceChildDeviceDeliveryRuntimeWriterProofVersion
  ),
  sourcePackageSourceAdapterExecutionProofVersion: Schema.Literal(SourcePackageSourceAdapterExecutionProofVersion),
  sourcePlatformLimitationActionProofVersion: Schema.Literal(SourcePlatformLimitationActionProofVersion),
  childDeviceDeliveryReadinessRows: Schema.Array(AppInstallPurchaseChildDeviceDeliveryReadinessRowSchema),
  nonClaims: Schema.Array(ChildDeviceDeliveryReadinessNonClaimSchema),
  knownGaps: Schema.Array(ChildDeviceDeliveryReadinessRefSchema),
  updatedAt: ParentTimestampSchema,
});

export type AppInstallPurchaseChildDeviceDeliveryReadinessProof = Infer<
  typeof ChildDeviceDeliveryReadinessProofBaseSchema
>;

export const AppInstallPurchaseChildDeviceDeliveryReadinessProofSchema = withParser(
  ChildDeviceDeliveryReadinessProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        childDeviceDeliveryReadinessProofIsHonest(proof) ||
        'Expected child-device delivery readiness proof to cover five platforms and preserve non-claims'
    )
  )
);

export const AppInstallPurchaseChildDeviceDeliveryReadinessKnownGaps = [
  'Child-device delivery readiness rows are evidence handoff rows only; no child-device delivery path is implemented.',
  'Runtime writer execution/delivery, provider/store execution, platform adapters, app blocking, child activity data, and hosted custody remain unimplemented.',
  'Android and iOS stay policy-blocked until device-owner or Family Controls entitlement proof exists.',
] as const;

export const AppInstallPurchaseChildDeviceDeliveryReadinessProofReadModel =
  AppInstallPurchaseChildDeviceDeliveryReadinessProofSchema.parse({
    schemaVersion: ChildDeviceDeliveryReadinessProofVersion,
    sourceChildDeviceDeliveryRuntimeWriterProofVersion: SourceChildDeviceDeliveryRuntimeWriterProofVersion,
    sourcePackageSourceAdapterExecutionProofVersion: SourcePackageSourceAdapterExecutionProofVersion,
    sourcePlatformLimitationActionProofVersion: SourcePlatformLimitationActionProofVersion,
    childDeviceDeliveryReadinessRows:
      AppInstallPurchasePlatformLimitationActionProofReadModel.platformLimitationActionRows.map(
        childDeviceDeliveryReadinessRow
      ),
    nonClaims: ChildDeviceDeliveryReadinessNonClaims,
    knownGaps: AppInstallPurchaseChildDeviceDeliveryReadinessKnownGaps,
    updatedAt: ChildDeviceDeliveryReadinessTimestamp,
  });

export function summarizeAppInstallPurchaseChildDeviceDeliveryReadinessProof(
  proof: AppInstallPurchaseChildDeviceDeliveryReadinessProof
) {
  return summarizeAppInstallPurchaseChildDeviceDeliveryReadinessProofGenerated(proof);
}

function childDeviceDeliveryReadinessRow(
  row: (typeof AppInstallPurchasePlatformLimitationActionProofReadModel.platformLimitationActionRows)[number]
) {
  const adapterRow =
    AppInstallPurchasePackageSourceAdapterExecutionProofReadModel.packageSourceAdapterExecutionRows.find(
      (candidate) => candidate.platform === row.platform
    );
  if (!adapterRow) {
    throw new Error(`Missing package-source adapter execution row for ${row.platform}`);
  }
  return buildAppInstallPurchaseChildDeviceDeliveryReadinessRowGenerated(
    row,
    adapterRow,
    SourceChildDeviceDeliveryRuntimeWriterProofVersion,
    AppInstallPurchaseChildDeviceDeliveryRuntimeWriterProofReadModel.childDeviceDeliveryRuntimeWriterRows.map(
      (childRow) => childRow.childDeviceDeliveryRuntimeWriterRowId
    ),
    SourcePackageSourceAdapterExecutionProofVersion,
    SourcePlatformLimitationActionProofVersion,
    ChildDeviceDeliveryReadinessBoundary,
    ChildDeviceDeliveryReadinessTimestamp
  );
}

function childDeviceDeliveryReadinessRowIsHonest(row: ChildDeviceDeliveryReadinessRowCandidate) {
  return childDeviceDeliveryReadinessRowIsHonestGenerated(row, ChildDeviceDeliveryReadinessBoundaryFragments);
}

function childDeviceDeliveryReadinessProofIsHonest(proof: AppInstallPurchaseChildDeviceDeliveryReadinessProof) {
  return childDeviceDeliveryReadinessProofIsHonestGenerated(
    proof,
    ChildDeviceDeliveryReadinessStates,
    ChildDeviceDeliveryReadinessNonClaims
  );
}
