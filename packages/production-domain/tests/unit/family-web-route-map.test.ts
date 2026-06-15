import { describe, expect, it } from 'vitest';
import {
  FamilyWebRouteMapSchema,
  mapFamilyWebLinks,
  mapFamilyWebRoutes,
  summarizeFamilyWebCollectionPolicies,
} from '../../src/family-web-route-map';
import { FamilyWebRouteMapReadModel } from '../../src/family-web-route-map-read-model';

describe('family web route map', () => {
  acceptsRouteAndLinkMap();
  rejectsChildActivityCollection();
  rejectsPrivacyCopyOverclaims();
  rejectsRegistrationHandoffDrift();
});

function acceptsRouteAndLinkMap(): void {
  it('accepts the required public pages, deployment shape, and registration handoff boundary', () => {
    const proof = FamilyWebRouteMapSchema.parse(FamilyWebRouteMapReadModel);

    expect(mapFamilyWebRoutes(proof.pages)).toEqual({
      home: '/',
      download: '/download',
      'register-login': '/register-login',
      privacy: '/privacy',
      support: '/support',
      status: '/status',
      'install-help': '/install-help',
    });
    expect(mapFamilyWebLinks(proof.pages)).toEqual({
      home: ['download', 'register-login', 'privacy', 'support', 'status', 'install-help'],
      download: ['home', 'register-login', 'privacy', 'support', 'status', 'install-help'],
      'register-login': ['home', 'download', 'privacy', 'support', 'status'],
      privacy: ['home', 'download', 'register-login', 'support', 'status'],
      support: ['home', 'download', 'privacy', 'status', 'install-help'],
      status: ['home', 'download', 'privacy', 'support', 'install-help'],
      'install-help': ['home', 'download', 'register-login', 'privacy', 'support', 'status'],
    });
    expect(summarizeFamilyWebCollectionPolicies(proof.collectionPolicies)).toEqual({
      none: 'default-public-page',
      'anonymous-operational-telemetry': 'disabled-by-default',
      'explicit-account-data': 'account-handoff-only',
      'forbidden-child-data': 'forbidden',
    });
    expect(proof.deployment).toEqual({
      schemaVersion: 'family-web-route-map-proof',
      publicHost: 'family.ocentra.ca',
      surfaceShape: 'separate-vite-app',
      deploymentTarget: 'cloudflare-pages-or-workers',
      previewUrlState: 'preview-url-required',
      publicRuntimeState: 'not-implemented',
      sourceProof: 'production-distribution-support-feature',
      manualRequirement: 'family-web-preview-and-production-host-proof-required',
    });
    expect(proof.registrationHandoff).toEqual({
      schemaVersion: 'family-web-route-map-proof',
      entryPage: 'register-login',
      owningPlan: 'account-identity-family-plan',
      handoffState: 'account-handoff-required',
      localCaptureState: 'not-implemented',
      allowedCollectionModes: ['explicit-account-data'],
      forbiddenCollectionModes: ['forbidden-child-data'],
      handoffReference: 'family-web-register-login-account-plan-handoff',
      manualRequirement: 'register-login-requires-account-identity-contract-before-form-or-session-claim',
    });
  });
}

function rejectsChildActivityCollection(): void {
  it('rejects route maps that stop forbidding child data on the public family surface', () => {
    expect(
      FamilyWebRouteMapSchema.safeParse({
        ...FamilyWebRouteMapReadModel,
        collectionPolicies: FamilyWebRouteMapReadModel.collectionPolicies.map((policy) =>
          policy.collectionMode === 'forbidden-child-data'
            ? {
                ...policy,
                collectionState: 'account-handoff-only',
              }
            : policy
        ),
      }).success
    ).toBe(false);
  });
}

function rejectsPrivacyCopyOverclaims(): void {
  it('rejects privacy posture that drops the no-overclaim copy boundary', () => {
    expect(
      FamilyWebRouteMapSchema.safeParse({
        ...FamilyWebRouteMapReadModel,
        copyConstraints: FamilyWebRouteMapReadModel.copyConstraints.filter(
          (constraint) => constraint !== 'no-store-nothing-overclaim'
        ),
      }).success
    ).toBe(false);
  });
}

function rejectsRegistrationHandoffDrift(): void {
  it('rejects register/login rows that claim local auth implementation instead of an account-plan handoff', () => {
    expect(
      FamilyWebRouteMapSchema.safeParse({
        ...FamilyWebRouteMapReadModel,
        registrationHandoff: {
          ...FamilyWebRouteMapReadModel.registrationHandoff,
          localCaptureState: 'implemented',
        },
      }).success
    ).toBe(false);
  });
}
