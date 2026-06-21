import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import { AppInstallPurchaseProviderStoreExecutionReadinessProofReadModel } from './app-install-purchase-provider-store-execution-readiness-proof';
import { AppInstallPurchaseRuntimeWriterExecutionDeliveryProofReadModel } from './app-install-purchase-runtime-writer-execution-delivery-proof';
import { ParentPlatformSchema, ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';
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

const ProviderStoreExecutionPreflightRowIdSchema = brandedNonEmptyStringSchema('AppInstallPurchaseProviderStoreExecutionPreflightRowId');
const ProviderStoreExecutionPreflightRefSchema = brandedNonEmptyStringSchema('AppInstallPurchaseProviderStoreExecutionPreflightRef');
const ProviderStoreExecutionPreflightBoundarySchema = brandedNonEmptyStringSchema('AppInstallPurchaseProviderStoreExecutionPreflightBoundary');

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
  return {
    providerStoreExecutionPreflightRows: proof.providerStoreExecutionPreflightRows.length,
    preflightReadyRows: proof.providerStoreExecutionPreflightRows.filter(
      (row) => row.providerStoreExecutionPreflightState === 'preflight-ready'
    ).length,
    manualProviderProofRequiredRows: proof.providerStoreExecutionPreflightRows.filter(
      (row) => row.providerStoreExecutionPreflightState === 'manual-provider-proof-required'
    ).length,
    providerUnavailableRows: proof.providerStoreExecutionPreflightRows.filter(
      (row) => row.providerStoreExecutionPreflightState === 'provider-unavailable'
    ).length,
    providerExecutedRows: proof.providerStoreExecutionPreflightRows.filter(
      (row) => row.providerApiExecutionClaim !== 'not-executed'
    ).length,
    runtimeDeviceDeliveredRows: proof.providerStoreExecutionPreflightRows.filter(
      (row) => row.runtimeDeviceDeliveryClaim !== 'not-delivered'
    ).length,
  } as const;
}

function providerStoreExecutionPreflightRow(
  row: (typeof AppInstallPurchaseProviderStoreExecutionReadinessProofReadModel.providerStoreExecutionReadinessRows)[number]
) {
  const runtimeWriterRows =
    AppInstallPurchaseRuntimeWriterExecutionDeliveryProofReadModel.runtimeWriterExecutionDeliveryRows;
  return {
    schemaVersion: ProviderStoreExecutionPreflightProofVersion,
    providerStoreExecutionPreflightRowId: `provider-store-execution-preflight-${row.platform}-${row.storeSurface}`,
    sourceProviderStoreExecutionReadinessProofVersion: SourceProviderStoreExecutionReadinessProofVersion,
    sourceProviderStoreExecutionReadinessRowId: row.providerStoreExecutionReadinessRowId,
    sourceRuntimeWriterExecutionDeliveryProofVersion: SourceRuntimeWriterExecutionDeliveryProofVersion,
    sourceRuntimeWriterExecutionDeliveryRowIds: runtimeWriterRows.map(
      (runtimeRow) => runtimeRow.runtimeWriterExecutionDeliveryRowId
    ),
    platform: row.platform,
    storeSurface: row.storeSurface,
    sourceProviderStoreExecutionReadinessState: row.providerStoreExecutionReadinessState,
    sourceRuntimeWriterReceiptClaims: runtimeWriterRows.map((runtimeRow) => runtimeRow.runtimeWriterDeliveryClaim),
    providerStoreExecutionPreflightState: providerStoreExecutionPreflightState(row),
    requiredProviderEvidenceRefs: row.requiredProofRefs,
    runtimeWriterReceiptRefs: runtimeWriterRows.map((runtimeRow) => runtimeRow.deliveryResultReceiptRef),
    auditEventRefs: uniqueRefs([
      ...row.parentActionAuditEventRefs,
      ...runtimeWriterRows.flatMap((r) => r.deliveryResultAuditEventRefs),
    ]),
    reportRuntimeRefs: uniqueRefs([...row.reportRuntimeRefs, ...runtimeWriterRows.flatMap((r) => r.reportRuntimeRefs)]),
    googlePlayExecutionClaim: 'not-executed',
    appleAppStoreExecutionClaim: 'not-executed',
    microsoftStoreExecutionClaim: 'not-executed',
    billingProviderContactClaim: 'not-executed',
    providerApiExecutionClaim: 'not-executed',
    storeIntegrationClaim: 'not-claimed',
    platformInterceptionClaim: 'not-claimed',
    platformAdapterClaim: 'not-implemented',
    runtimeDeviceDeliveryClaim: 'not-delivered',
    childDeviceDeliveryClaim: 'not-delivered',
    appBlockingClaim: 'not-claimed',
    childDataCustody: 'no-child-activity-data',
    ocentraHostedFamilyDataCustodyClaim: 'not-claimed',
    claimBoundary: ProviderStoreExecutionPreflightBoundary,
    evaluatedAt: ProviderStoreExecutionPreflightTimestamp,
  } as const;
}

function providerStoreExecutionPreflightState(
  row: (typeof AppInstallPurchaseProviderStoreExecutionReadinessProofReadModel.providerStoreExecutionReadinessRows)[number]
) {
  if (row.providerStoreExecutionReadinessState === 'provider-store-execution-ready') {
    return 'preflight-ready';
  }
  if (row.providerStoreExecutionReadinessState === 'unavailable') {
    return 'provider-unavailable';
  }
  return 'manual-provider-proof-required';
}

function uniqueRefs(refs: readonly string[]) {
  return Array.from(new Set(refs));
}

function providerStoreExecutionPreflightRowIsHonest(row: ProviderStoreExecutionPreflightRowCandidate): boolean {
  return (
    providerStoreExecutionPreflightStateMatchesSource(row) &&
    providerStoreExecutionPreflightRefsAreComplete(row) &&
    providerStoreExecutionPreflightClaimsStayUnimplemented(row) &&
    providerStoreExecutionPreflightBoundaryIsExplicit(row.claimBoundary)
  );
}

function providerStoreExecutionPreflightStateMatchesSource(row: ProviderStoreExecutionPreflightRowCandidate): boolean {
  if (row.sourceProviderStoreExecutionReadinessState === 'provider-store-execution-ready') {
    return row.providerStoreExecutionPreflightState === 'preflight-ready';
  }
  if (row.sourceProviderStoreExecutionReadinessState === 'unavailable') {
    return row.providerStoreExecutionPreflightState === 'provider-unavailable';
  }
  return row.providerStoreExecutionPreflightState === 'manual-provider-proof-required';
}

function providerStoreExecutionPreflightRefsAreComplete(row: ProviderStoreExecutionPreflightRowCandidate): boolean {
  return (
    row.sourceProviderStoreExecutionReadinessProofVersion === SourceProviderStoreExecutionReadinessProofVersion &&
    row.sourceProviderStoreExecutionReadinessRowId.length > 0 &&
    row.sourceRuntimeWriterExecutionDeliveryProofVersion === SourceRuntimeWriterExecutionDeliveryProofVersion &&
    row.sourceRuntimeWriterExecutionDeliveryRowIds.length ===
      AppInstallPurchaseRuntimeWriterExecutionDeliveryProofReadModel.runtimeWriterExecutionDeliveryRows.length &&
    row.sourceRuntimeWriterReceiptClaims.includes('parent-owned-delivery-result-recorded') &&
    row.sourceRuntimeWriterReceiptClaims.includes('manual-required') &&
    row.requiredProviderEvidenceRefs.length > 0 &&
    row.runtimeWriterReceiptRefs.length > 0 &&
    row.auditEventRefs.length > 0 &&
    row.reportRuntimeRefs.length > 0
  );
}

function providerStoreExecutionPreflightClaimsStayUnimplemented(
  row: ProviderStoreExecutionPreflightRowCandidate
): boolean {
  return providerStoreExecutionClaimsStayUnimplemented(row) && deliveryAndCustodyClaimsStayUnimplemented(row);
}

function providerStoreExecutionClaimsStayUnimplemented(row: ProviderStoreExecutionPreflightRowCandidate): boolean {
  return (
    row.googlePlayExecutionClaim === 'not-executed' &&
    row.appleAppStoreExecutionClaim === 'not-executed' &&
    row.microsoftStoreExecutionClaim === 'not-executed' &&
    row.billingProviderContactClaim === 'not-executed' &&
    row.providerApiExecutionClaim === 'not-executed' &&
    row.storeIntegrationClaim === 'not-claimed' &&
    row.platformInterceptionClaim === 'not-claimed' &&
    row.platformAdapterClaim === 'not-implemented'
  );
}

function deliveryAndCustodyClaimsStayUnimplemented(row: ProviderStoreExecutionPreflightRowCandidate): boolean {
  return (
    row.runtimeDeviceDeliveryClaim === 'not-delivered' &&
    row.childDeviceDeliveryClaim === 'not-delivered' &&
    row.appBlockingClaim === 'not-claimed' &&
    row.childDataCustody === 'no-child-activity-data' &&
    row.ocentraHostedFamilyDataCustodyClaim === 'not-claimed'
  );
}

function providerStoreExecutionPreflightProofIsHonest(
  proof: AppInstallPurchaseProviderStoreExecutionPreflightProof
): boolean {
  const keys = new Set(proof.providerStoreExecutionPreflightRows.map((row) => `${row.platform}:${row.storeSurface}`));
  const states = new Set(
    proof.providerStoreExecutionPreflightRows.map((row) => row.providerStoreExecutionPreflightState)
  );
  const nonClaims = new Set(proof.nonClaims);
  return (
    proof.sourceProviderStoreExecutionReadinessProofVersion === SourceProviderStoreExecutionReadinessProofVersion &&
    proof.sourceRuntimeWriterExecutionDeliveryProofVersion === SourceRuntimeWriterExecutionDeliveryProofVersion &&
    proof.providerStoreExecutionPreflightRows.length === RequiredPlatformSources.length &&
    keys.size === proof.providerStoreExecutionPreflightRows.length &&
    RequiredPlatformSources.every(([platform, storeSurface]) => keys.has(`${platform}:${storeSurface}`)) &&
    ProviderStoreExecutionPreflightStates.every((state) => states.has(state)) &&
    ProviderStoreExecutionPreflightNonClaims.every((claim) => nonClaims.has(claim)) &&
    proof.providerStoreExecutionPreflightRows.every(providerStoreExecutionPreflightRowIsHonest) &&
    proof.knownGaps.length > 0
  );
}

function providerStoreExecutionPreflightBoundaryIsExplicit(
  boundary: typeof ProviderStoreExecutionPreflightBoundarySchema.Type
): boolean {
  return ProviderStoreExecutionPreflightBoundaryFragments.every((fragment) => boundary.includes(fragment));
}

