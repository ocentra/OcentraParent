import { Schema } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';
import {
  ProductionReleasePublicRuntimeHandoffProofSchema,
  type ProductionReleasePublicRuntimeAdapter,
  type ProductionReleasePublicRuntimeAdapterRow,
  type ProductionReleasePublicRuntimeHandoffRow,
  type ProductionReleasePublicRuntimeSurface,
} from './production-release-public-runtime-handoff';
import {
  ForbiddenPublicRuntimeDataClasses,
  RequiredPublicRuntimeNonClaims,
} from './production-release-public-runtime-handoff-values';

export const ProductionReleasePublicRuntimeHandoffReadModel = ProductionReleasePublicRuntimeHandoffProofSchema.parse({
  schemaVersion: 'production-release-public-runtime-handoff-proof',
  handoffRows: [
    handoff('public-download', 'download-manifest-route', 'route-contract-only', 'backend-required', [
      'release-version',
      'platform',
      'package-artifact',
      'download-status',
    ]),
    handoff('release-status', 'release-status-route', 'manual-required', 'production-promotion-required', [
      'release-version',
      'commit',
      'platform',
    ]),
    handoff('update-status', 'update-status-route', 'manual-required', 'manual-required', [
      'update-status',
      'platform',
    ]),
    handoff('account-status', 'account-status-route', 'route-contract-only', 'backend-required', [
      'account-status',
      'entitlement-summary',
    ]),
    handoff('subscription-status', 'subscription-status-route', 'route-contract-only', 'backend-required', [
      'subscription-status',
    ]),
    handoff('support-status', 'support-status-route', 'manual-required', 'manual-required', [
      'support-runbook-status',
      'incident-status',
    ]),
  ],
  adapterRows: [
    adapter('public-website-runtime', 'not-implemented', 'not-executed'),
    adapter('download-status-backend', 'backend-required', 'manual-required'),
    adapter('release-publishing-pipeline', 'production-promotion-required', 'promotion-required'),
    adapter('updater-status-runtime', 'manual-required', 'manual-required'),
    adapter('account-backend', 'backend-required', 'manual-required'),
    adapter('billing-provider-runtime', 'not-implemented', 'not-executed'),
    adapter('support-backend-upload', 'manual-required', 'manual-required'),
  ],
  nonClaims: RequiredPublicRuntimeNonClaims,
  publicWebsiteRuntimeClaim: 'not-implemented',
  accountBackendRuntimeClaim: 'backend-required',
  billingProviderRuntimeClaim: 'not-implemented',
  supportBackendUploadClaim: 'manual-required',
  productionPublishingState: 'production-promotion-required',
  signingStoreProofState: 'manual-required',
  updaterExecutionState: 'manual-required',
  childActivityCustodyClaim: 'not-implemented',
  updatedAt: Schema.decodeUnknownSync(ParentTimestampSchema)('2026-06-05T05:12:00.000Z'),
});

export const ProductionReleasePublicRuntimeHandoffKnownGaps = [
  'family.ocentra.ca public runtime is not implemented.',
  'Download, release, update, account, subscription, and support runtime adapters require backend or manual proof.',
  'Production publishing, signing, notarization, store upload, and updater execution remain manual-required.',
  'Account backend, billing provider runtime, support backend upload, and child-activity custody remain unimplemented or unclaimed.',
] as const;

function handoff(
  surface: ProductionReleasePublicRuntimeSurface,
  handoffTarget: ProductionReleasePublicRuntimeHandoffRow['handoffTarget'],
  routeState: ProductionReleasePublicRuntimeHandoffRow['routeState'],
  backendAdapterState: ProductionReleasePublicRuntimeHandoffRow['backendAdapterState'],
  supportSafeDataClasses: ProductionReleasePublicRuntimeHandoffRow['supportSafeDataClasses']
) {
  return {
    schemaVersion: 'production-release-public-runtime-handoff-proof',
    surface,
    handoffTarget,
    routeState,
    runtimeAdapterState: routeState === 'route-contract-only' ? 'adapter-boundary-ready' : routeState,
    backendAdapterState,
    parentVisibleState: backendAdapterState,
    sourceProof: sourceProofFor(surface),
    supportSafeDataClasses,
    forbiddenDataClasses: ForbiddenPublicRuntimeDataClasses,
    handoffReference: `public-runtime-handoff-${surface}`,
    evidenceReference: `public-runtime-evidence-${surface}`,
    manualRequirement: `${surface}-requires-public-runtime-backend-and-manual-platform-proof`,
  } as const;
}

function adapter(
  adapterName: ProductionReleasePublicRuntimeAdapter,
  adapterState: ProductionReleasePublicRuntimeAdapterRow['adapterState'],
  executionClaim: ProductionReleasePublicRuntimeAdapterRow['executionClaim']
) {
  return {
    schemaVersion: 'production-release-public-runtime-handoff-proof',
    adapter: adapterName,
    adapterState,
    executionClaim,
    providerSecretCustody: 'not-present',
    childActivityCustody: 'not-included',
    evidenceReference: `public-runtime-adapter-${adapterName}`,
    requiredProof: `${adapterName}-requires-runtime-proof-before-implementation-claim`,
  } as const;
}

function sourceProofFor(
  surface: ProductionReleasePublicRuntimeSurface
): ProductionReleasePublicRuntimeHandoffRow['sourceProof'] {
  if (surface === 'account-status' || surface === 'subscription-status') {
    return 'billing-account-endpoint-contract-proof';
  }
  if (surface === 'update-status') {
    return 'v8-updater-rollback-runbook-proof';
  }
  if (surface === 'support-status') {
    return 'support-incident-workflow-proof';
  }
  return 'production-release-public-status-proof';
}
