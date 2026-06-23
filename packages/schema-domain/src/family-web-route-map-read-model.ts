import { Schema } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';
import {
  FamilyWebRouteMapSchema,
  type FamilyWebCollectionMode,
  type FamilyWebDataCollectionPolicy,
  type FamilyWebPage,
  type FamilyWebPageRoute,
} from './family-web-route-map';
import {
  FamilyWebReferenceSchema,
  FamilyWebCollectionCoverage,
  FamilyWebCollectionStateByMode,
  FamilyWebLinksByPage,
  FamilyWebPurposeByPage,
  FamilyWebRequirementSchema,
  FamilyWebRoutePathSchema,
  FamilyWebRoutePathByPage,
  RequiredFamilyWebCopyConstraints,
  RequiredFamilyWebNonClaims,
} from './family-web-route-map-values';

const decodeFamilyWebReference = Schema.decodeUnknownSync(FamilyWebReferenceSchema);
const decodeFamilyWebRequirement = Schema.decodeUnknownSync(FamilyWebRequirementSchema);
const decodeFamilyWebRoutePath = Schema.decodeUnknownSync(FamilyWebRoutePathSchema);

export const FamilyWebRouteMapReadModel = FamilyWebRouteMapSchema.parse({
  schemaVersion: 'family-web-route-map-proof',
  pages: [
    page('home', 'family-setup-expectation'),
    page('download', 'release-installer-expectation'),
    page('register-login', 'account-identity-family-plan-handoff'),
    page('privacy', 'data-custody-expectation'),
    page('support', 'production-distribution-support-feature'),
    page('status', 'production-distribution-support-feature'),
    page('install-help', 'release-installer-expectation'),
  ],
  collectionPolicies: [
    collectionPolicy('none', 'family-web-no-data-collection-default', 'family-setup-expectation'),
    collectionPolicy(
      'anonymous-operational-telemetry',
      'family-web-telemetry-disabled-unless-disclosed',
      'data-custody-expectation'
    ),
    collectionPolicy(
      'explicit-account-data',
      'family-web-register-login-handoff-only',
      'account-identity-family-plan-handoff'
    ),
    collectionPolicy('forbidden-child-data', 'family-web-child-data-never-collected', 'data-custody-expectation'),
  ],
  deployment: {
    schemaVersion: 'family-web-route-map-proof',
    publicHost: 'family.ocentra.ca',
    surfaceShape: 'separate-vite-app',
    deploymentTarget: 'cloudflare-pages-or-workers',
    previewUrlState: 'preview-url-required',
    publicRuntimeState: 'not-implemented',
    sourceProof: 'production-distribution-support-feature',
    manualRequirement: 'family-web-preview-and-production-host-proof-required',
  },
  registrationHandoff: {
    schemaVersion: 'family-web-route-map-proof',
    entryPage: 'register-login',
    owningPlan: 'account-identity-family-plan',
    handoffState: 'account-handoff-required',
    localCaptureState: 'not-implemented',
    allowedCollectionModes: ['explicit-account-data'],
    forbiddenCollectionModes: ['forbidden-child-data'],
    handoffReference: 'family-web-register-login-account-plan-handoff',
    manualRequirement: 'register-login-requires-account-identity-contract-before-form-or-session-claim',
  },
  copyConstraints: RequiredFamilyWebCopyConstraints,
  nonClaims: RequiredFamilyWebNonClaims,
  updatedAt: Schema.decodeUnknownSync(ParentTimestampSchema)('2026-06-13T22:12:00.000Z'),
});

export const FamilyWebRouteMapKnownGaps = [
  'family.ocentra.ca remains a source-contract and preview-shape boundary only in this slice.',
  'Registration and login stay owned by account-identity-family-plan; this module defines only the handoff contract.',
  'Installer delivery, status runtime, and support runtime remain manual-required or unimplemented public-surface boundaries.',
  'Anonymous operational telemetry stays disabled by default until a separate disclosed data-custody slice proves it.',
] as const;

function page(pageName: FamilyWebPage, sourceProof: FamilyWebPageRoute['sourceProof']): FamilyWebPageRoute {
  return {
    schemaVersion: 'family-web-route-map-proof',
    page: pageName,
    routePath: decodeFamilyWebRoutePath(FamilyWebRoutePathByPage[pageName]),
    pagePurpose: FamilyWebPurposeByPage[pageName],
    routeState: 'route-contract-only',
    linkTargets: [...FamilyWebLinksByPage[pageName]],
    sourceProof,
    statusReference: decodeFamilyWebReference(`family-web-route-${pageName}`),
    manualRequirement: decodeFamilyWebRequirement(`${pageName}-requires-public-page-build-and-link-proof`),
  };
}

function collectionPolicy(
  collectionMode: FamilyWebCollectionMode,
  disclosureBoundary: string,
  sourceProof: FamilyWebDataCollectionPolicy['sourceProof']
): FamilyWebDataCollectionPolicy {
  return {
    schemaVersion: 'family-web-route-map-proof',
    collectionMode,
    pageCoverage: [...FamilyWebCollectionCoverage[collectionMode]],
    collectionState: FamilyWebCollectionStateByMode[collectionMode],
    disclosureBoundary: decodeFamilyWebReference(disclosureBoundary),
    sourceProof,
    manualRequirement: decodeFamilyWebRequirement(`${collectionMode}-requires-privacy-boundary-proof`),
  };
}
