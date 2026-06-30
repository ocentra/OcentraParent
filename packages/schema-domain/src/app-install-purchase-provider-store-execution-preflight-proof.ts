import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';
import { AppInstallPurchaseProviderStoreExecutionReadinessProofReadModel } from './app-install-purchase-provider-store-execution-readiness-proof';
import { AppInstallPurchaseRuntimeWriterExecutionDeliveryProofReadModel } from './app-install-purchase-runtime-writer-execution-delivery-proof';
import { ParentPlatformSchema, ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';
import {
  buildAppInstallPurchaseProviderStoreExecutionPreflightRowGenerated,
  providerStoreExecutionPreflightProofIsHonestGenerated,
  providerStoreExecutionPreflightRowIsHonestGenerated,
  summarizeAppInstallPurchaseProviderStoreExecutionPreflightProofGenerated,
} from './generated/app-install-purchase-platform-evidence-helpers';

const ProviderStoreExecutionPreflightProofVersion = 'app-install-purchase-provider-store-execution-preflight-proof';
const SourceProviderStoreExecutionReadinessProofVersion =
  'app-install-purchase-provider-store-execution-readiness-proof';
const SourceRuntimeWriterExecutionDeliveryProofVersion = 'app-install-purchase-runtime-writer-execution-delivery-proof';
const ProviderStoreExecutionPreflightTimestamp = '2026-06-06T05:45:00.000Z';
const ProviderStoreExecutionPreflightBoundary =
  'provider store execution preflight proof only; parent-owned preflight row records evidence readiness and runtime writer receipt refs no Google Play execution no Apple App Store execution no Microsoft Store execution no billing provider contact no provider API execution no store integration no platform interception no platform adapter implementation no runtime device delivery no child-device delivery no app blocking no child activity data no Ocentra-hosted family data custody';
const RequiredPlatformSources = [
  ['windows', 'microsoft-store'],
  ['macos', 'mac-app-store'],
  ['linux', 'linux-package-manager'],
  ['android', 'google-play'],
  ['ios', 'apple-app-store'],
] as const;
const ProviderStoreExecutionPreflightStates = [
  'preflight-ready',
  'manual-provider-proof-required',
  'provider-unavailable',
] as const;
const ProviderStoreExecutionPreflightNonClaims = [
  'no-google-play-execution',
  'no-apple-app-store-execution',
  'no-microsoft-store-execution',
  'no-billing-provider-contact',
  'no-provider-api-execution',
  'no-store-integration',
  'no-platform-interception',
  'no-platform-adapter-implementation',
  'no-runtime-device-delivery',
  'no-child-device-delivery',
  'no-app-blocking',
  'no-child-activity-data',
  'no-ocentra-hosted-family-data-custody',
] as const;
const ProviderStoreExecutionPreflightBoundaryFragments = [
  'parent-owned preflight row',
  'runtime writer receipt refs',
  'no Google Play execution',
  'no Apple App Store execution',
  'no Microsoft Store execution',
  'no billing provider contact',
  'no provider API execution',
  'no store integration',
  'no platform interception',
  'no platform adapter implementation',
  'no runtime device delivery',
  'no child-device delivery',
  'no app blocking',
  'no child activity data',
  'no Ocentra-hosted family data custody',
] as const;

export const AppInstallPurchaseProviderStoreExecutionPreflightProofSchemaVersionSchema = withParser(
  Schema.Literal(ProviderStoreExecutionPreflightProofVersion)
);
const ProviderStoreExecutionPreflightStoreSurfaceSchema = withParser(
  Schema.Literal('microsoft-store', 'mac-app-store', 'linux-package-manager', 'google-play', 'apple-app-store')
);
const ProviderStoreExecutionPreflightStateSchema = withParser(Schema.Literal(...ProviderStoreExecutionPreflightStates));
const ProviderStoreExecutionPreflightReadinessStateSchema = withParser(
  Schema.Literal('provider-store-execution-ready', 'manual-required', 'unavailable')
);
const ProviderStoreExecutionPreflightRuntimeWriterReceiptSchema = withParser(
  Schema.Literal('parent-owned-delivery-result-recorded', 'manual-required')
);
const ProviderStoreExecutionPreflightProviderClaimSchema = withParser(Schema.Literal('not-executed'));
const ProviderStoreExecutionPreflightIntegrationClaimSchema = withParser(Schema.Literal('not-claimed'));
const ProviderStoreExecutionPreflightAdapterClaimSchema = withParser(Schema.Literal('not-implemented'));
const ProviderStoreExecutionPreflightDeliveryClaimSchema = withParser(Schema.Literal('not-delivered'));
const ProviderStoreExecutionPreflightCustodyClaimSchema = withParser(Schema.Literal('no-child-activity-data'));
const ProviderStoreExecutionPreflightNonClaimSchema = withParser(
  Schema.Literal(...ProviderStoreExecutionPreflightNonClaims)
);

const ProviderStoreExecutionPreflightRowIdSchema = brandedNonEmptyStringSchema(
  'AppInstallPurchaseProviderStoreExecutionPreflightRowId'
);
const ProviderStoreExecutionPreflightRefSchema = brandedNonEmptyStringSchema(
  'AppInstallPurchaseProviderStoreExecutionPreflightRef'
);
const ProviderStoreExecutionPreflightBoundarySchema = brandedNonEmptyStringSchema(
  'AppInstallPurchaseProviderStoreExecutionPreflightBoundary'
);

const ProviderStoreExecutionPreflightRowBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseProviderStoreExecutionPreflightProofSchemaVersionSchema,
  providerStoreExecutionPreflightRowId: ProviderStoreExecutionPreflightRowIdSchema,
  sourceProviderStoreExecutionReadinessProofVersion: Schema.Literal(SourceProviderStoreExecutionReadinessProofVersion),
  sourceProviderStoreExecutionReadinessRowId: ProviderStoreExecutionPreflightRefSchema,
  sourceRuntimeWriterExecutionDeliveryProofVersion: Schema.Literal(SourceRuntimeWriterExecutionDeliveryProofVersion),
  sourceRuntimeWriterExecutionDeliveryRowIds: Schema.Array(ProviderStoreExecutionPreflightRefSchema),
  platform: ParentPlatformSchema,
  storeSurface: ProviderStoreExecutionPreflightStoreSurfaceSchema,
  sourceProviderStoreExecutionReadinessState: ProviderStoreExecutionPreflightReadinessStateSchema,
  sourceRuntimeWriterReceiptClaims: Schema.Array(ProviderStoreExecutionPreflightRuntimeWriterReceiptSchema),
  providerStoreExecutionPreflightState: ProviderStoreExecutionPreflightStateSchema,
  requiredProviderEvidenceRefs: Schema.Array(ProviderStoreExecutionPreflightRefSchema),
  runtimeWriterReceiptRefs: Schema.Array(ProviderStoreExecutionPreflightRefSchema),
  auditEventRefs: Schema.Array(ProviderStoreExecutionPreflightRefSchema),
  reportRuntimeRefs: Schema.Array(ProviderStoreExecutionPreflightRefSchema),
  googlePlayExecutionClaim: ProviderStoreExecutionPreflightProviderClaimSchema,
  appleAppStoreExecutionClaim: ProviderStoreExecutionPreflightProviderClaimSchema,
  microsoftStoreExecutionClaim: ProviderStoreExecutionPreflightProviderClaimSchema,
  billingProviderContactClaim: ProviderStoreExecutionPreflightProviderClaimSchema,
  providerApiExecutionClaim: ProviderStoreExecutionPreflightProviderClaimSchema,
  storeIntegrationClaim: ProviderStoreExecutionPreflightIntegrationClaimSchema,
  platformInterceptionClaim: ProviderStoreExecutionPreflightIntegrationClaimSchema,
  platformAdapterClaim: ProviderStoreExecutionPreflightAdapterClaimSchema,
  runtimeDeviceDeliveryClaim: ProviderStoreExecutionPreflightDeliveryClaimSchema,
  childDeviceDeliveryClaim: ProviderStoreExecutionPreflightDeliveryClaimSchema,
  appBlockingClaim: ProviderStoreExecutionPreflightIntegrationClaimSchema,
  childDataCustody: ProviderStoreExecutionPreflightCustodyClaimSchema,
  ocentraHostedFamilyDataCustodyClaim: ProviderStoreExecutionPreflightIntegrationClaimSchema,
  claimBoundary: ProviderStoreExecutionPreflightBoundarySchema,
  evaluatedAt: ParentTimestampSchema,
});

type ProviderStoreExecutionPreflightRowCandidate = Infer<typeof ProviderStoreExecutionPreflightRowBaseSchema>;

export const AppInstallPurchaseProviderStoreExecutionPreflightRowSchema = withParser(
  ProviderStoreExecutionPreflightRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        providerStoreExecutionPreflightRowIsHonest(row) ||
        'Expected provider/store execution preflight rows to link provider readiness and runtime writer receipts without provider, store, platform, runtime delivery, child delivery, custody, or blocking claims'
    )
  )
);

const ProviderStoreExecutionPreflightProofBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseProviderStoreExecutionPreflightProofSchemaVersionSchema,
  sourceProviderStoreExecutionReadinessProofVersion: Schema.Literal(SourceProviderStoreExecutionReadinessProofVersion),
  sourceRuntimeWriterExecutionDeliveryProofVersion: Schema.Literal(SourceRuntimeWriterExecutionDeliveryProofVersion),
  providerStoreExecutionPreflightRows: Schema.Array(AppInstallPurchaseProviderStoreExecutionPreflightRowSchema),
  nonClaims: Schema.Array(ProviderStoreExecutionPreflightNonClaimSchema),
  knownGaps: Schema.Array(ProviderStoreExecutionPreflightRefSchema),
  updatedAt: ParentTimestampSchema,
});

export type AppInstallPurchaseProviderStoreExecutionPreflightProof = Infer<
  typeof ProviderStoreExecutionPreflightProofBaseSchema
>;

export const AppInstallPurchaseProviderStoreExecutionPreflightProofSchema = withParser(
  ProviderStoreExecutionPreflightProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        providerStoreExecutionPreflightProofIsHonest(proof) ||
        'Expected app install/purchase provider/store execution preflight proof to cover platform sources and preserve execution non-claims'
    )
  )
);

export const AppInstallPurchaseProviderStoreExecutionPreflightKnownGaps = [
  'Provider/store execution preflight rows record parent-owned evidence readiness only; no Google Play Apple App Store Microsoft Store or billing provider execution is implemented.',
  'Runtime writer receipt refs are parent-owned proof rows only; runtime device delivery, child-device delivery, platform adapters, app blocking, child activity data, and hosted custody remain unimplemented.',
] as const;

export const AppInstallPurchaseProviderStoreExecutionPreflightProofReadModel =
  AppInstallPurchaseProviderStoreExecutionPreflightProofSchema.parse({
    schemaVersion: ProviderStoreExecutionPreflightProofVersion,
    sourceProviderStoreExecutionReadinessProofVersion: SourceProviderStoreExecutionReadinessProofVersion,
    sourceRuntimeWriterExecutionDeliveryProofVersion: SourceRuntimeWriterExecutionDeliveryProofVersion,
    providerStoreExecutionPreflightRows:
      AppInstallPurchaseProviderStoreExecutionReadinessProofReadModel.providerStoreExecutionReadinessRows.map(
        providerStoreExecutionPreflightRow
      ),
    nonClaims: ProviderStoreExecutionPreflightNonClaims,
    knownGaps: AppInstallPurchaseProviderStoreExecutionPreflightKnownGaps,
    updatedAt: ProviderStoreExecutionPreflightTimestamp,
  });

export function summarizeAppInstallPurchaseProviderStoreExecutionPreflightProof(
  proof: AppInstallPurchaseProviderStoreExecutionPreflightProof
) {
  return summarizeAppInstallPurchaseProviderStoreExecutionPreflightProofGenerated(proof);
}

function providerStoreExecutionPreflightRow(
  row: (typeof AppInstallPurchaseProviderStoreExecutionReadinessProofReadModel.providerStoreExecutionReadinessRows)[number]
) {
  return buildAppInstallPurchaseProviderStoreExecutionPreflightRowGenerated(
    row,
    AppInstallPurchaseRuntimeWriterExecutionDeliveryProofReadModel.runtimeWriterExecutionDeliveryRows,
    SourceProviderStoreExecutionReadinessProofVersion,
    SourceRuntimeWriterExecutionDeliveryProofVersion,
    ProviderStoreExecutionPreflightBoundary,
    ProviderStoreExecutionPreflightTimestamp
  );
}

function providerStoreExecutionPreflightRowIsHonest(row: ProviderStoreExecutionPreflightRowCandidate): boolean {
  return providerStoreExecutionPreflightRowIsHonestGenerated(
    row,
    AppInstallPurchaseRuntimeWriterExecutionDeliveryProofReadModel.runtimeWriterExecutionDeliveryRows.length,
    ProviderStoreExecutionPreflightBoundaryFragments
  );
}

function providerStoreExecutionPreflightProofIsHonest(
  proof: AppInstallPurchaseProviderStoreExecutionPreflightProof
): boolean {
  return (
    proof.sourceProviderStoreExecutionReadinessProofVersion === SourceProviderStoreExecutionReadinessProofVersion &&
    proof.sourceRuntimeWriterExecutionDeliveryProofVersion === SourceRuntimeWriterExecutionDeliveryProofVersion &&
    providerStoreExecutionPreflightProofIsHonestGenerated(
      proof,
      RequiredPlatformSources,
      ProviderStoreExecutionPreflightStates,
      ProviderStoreExecutionPreflightNonClaims
    ) &&
    proof.providerStoreExecutionPreflightRows.every(providerStoreExecutionPreflightRowIsHonest)
  );
}
