import { type Infer, Schema, brandedNonEmptyStringSchema, withParser } from '@ocentra-parent/schema-domain/effect';

export const FamilyWebRouteMapSchemaVersionSchema = withParser(Schema.Literal('family-web-route-map-proof'));

export const FamilyWebPageSchema = withParser(
  Schema.Literal('home', 'download', 'register-login', 'privacy', 'support', 'status', 'install-help')
);

export const FamilyWebPagePurposeSchema = withParser(
  Schema.Literal(
    'public-entry',
    'download-entry',
    'account-handoff',
    'privacy-disclosure',
    'support-entry',
    'status-entry',
    'install-help'
  )
);

export const FamilyWebPageStateSchema = withParser(
  Schema.Literal('route-contract-only', 'manual-required', 'not-implemented', 'implemented', 'executed')
);

export const FamilyWebLinkTargetSchema = FamilyWebPageSchema;

export const FamilyWebCollectionModeSchema = withParser(
  Schema.Literal('none', 'anonymous-operational-telemetry', 'explicit-account-data', 'forbidden-child-data')
);

export const FamilyWebCollectionStateSchema = withParser(
  Schema.Literal('default-public-page', 'disabled-by-default', 'account-handoff-only', 'forbidden')
);

export const FamilyWebCopyConstraintSchema = withParser(
  Schema.Literal(
    'no-unproven-enforcement-claim',
    'no-store-nothing-overclaim',
    'no-vague-privacy-promise',
    'no-child-activity-collection-overclaim',
    'telemetry-disclosed-before-enable',
    'account-data-only-via-registration-handoff'
  )
);

export const FamilyWebNonClaimSchema = withParser(
  Schema.Literal(
    'no-public-auth-backend',
    'no-public-installer-mechanics',
    'no-public-status-runtime',
    'no-public-child-activity-custody',
    'no-public-enforcement-runtime'
  )
);

export const FamilyWebSourceProofSchema = withParser(
  Schema.Literal(
    'family-setup-expectation',
    'data-custody-expectation',
    'release-installer-expectation',
    'production-distribution-support-feature',
    'account-identity-family-plan-handoff'
  )
);

export const FamilyWebDeploymentSurfaceSchema = withParser(Schema.Literal('separate-vite-app'));
export const FamilyWebDeploymentTargetSchema = withParser(Schema.Literal('cloudflare-pages-or-workers'));
export const FamilyWebDeploymentPreviewStateSchema = withParser(Schema.Literal('preview-url-required'));
export const FamilyWebDeploymentRuntimeStateSchema = withParser(
  Schema.Literal('not-implemented', 'manual-required', 'implemented')
);
export const FamilyWebProductionHostSchema = withParser(Schema.Literal('family.ocentra.ca'));

export const FamilyWebRegistrationHandoffPlanSchema = withParser(Schema.Literal('account-identity-family-plan'));

export const FamilyWebRegistrationHandoffStateSchema = withParser(
  Schema.Literal('account-handoff-required', 'manual-required', 'not-implemented', 'implemented')
);

export const FamilyWebReferenceSchema = brandedNonEmptyStringSchema('FamilyWebReference');
export const FamilyWebRequirementSchema = brandedNonEmptyStringSchema('FamilyWebRequirement');
export const FamilyWebRoutePathSchema = brandedNonEmptyStringSchema('FamilyWebRoutePath');

export type FamilyWebPage = Infer<typeof FamilyWebPageSchema>;
export type FamilyWebCollectionMode = Infer<typeof FamilyWebCollectionModeSchema>;
export type FamilyWebCopyConstraint = Infer<typeof FamilyWebCopyConstraintSchema>;
export type FamilyWebNonClaim = Infer<typeof FamilyWebNonClaimSchema>;

export const RequiredFamilyWebPages = [
  'home',
  'download',
  'register-login',
  'privacy',
  'support',
  'status',
  'install-help',
] as const satisfies ReadonlyArray<FamilyWebPage>;

export const RequiredFamilyWebCollectionModes = [
  'none',
  'anonymous-operational-telemetry',
  'explicit-account-data',
  'forbidden-child-data',
] as const satisfies ReadonlyArray<FamilyWebCollectionMode>;

export const RequiredFamilyWebCopyConstraints = [
  'no-unproven-enforcement-claim',
  'no-store-nothing-overclaim',
  'no-vague-privacy-promise',
  'no-child-activity-collection-overclaim',
  'telemetry-disclosed-before-enable',
  'account-data-only-via-registration-handoff',
] as const satisfies ReadonlyArray<FamilyWebCopyConstraint>;

export const RequiredFamilyWebNonClaims = [
  'no-public-auth-backend',
  'no-public-installer-mechanics',
  'no-public-status-runtime',
  'no-public-child-activity-custody',
  'no-public-enforcement-runtime',
] as const satisfies ReadonlyArray<FamilyWebNonClaim>;

export const FamilyWebRoutePathByPage: Record<FamilyWebPage, string> = {
  home: '/',
  download: '/download',
  'register-login': '/register-login',
  privacy: '/privacy',
  support: '/support',
  status: '/status',
  'install-help': '/install-help',
};

export const FamilyWebPurposeByPage: Record<FamilyWebPage, Infer<typeof FamilyWebPagePurposeSchema>> = {
  home: 'public-entry',
  download: 'download-entry',
  'register-login': 'account-handoff',
  privacy: 'privacy-disclosure',
  support: 'support-entry',
  status: 'status-entry',
  'install-help': 'install-help',
};

export const FamilyWebLinksByPage: Record<FamilyWebPage, ReadonlyArray<FamilyWebPage>> = {
  home: ['download', 'register-login', 'privacy', 'support', 'status', 'install-help'],
  download: ['home', 'register-login', 'privacy', 'support', 'status', 'install-help'],
  'register-login': ['home', 'download', 'privacy', 'support', 'status'],
  privacy: ['home', 'download', 'register-login', 'support', 'status'],
  support: ['home', 'download', 'privacy', 'status', 'install-help'],
  status: ['home', 'download', 'privacy', 'support', 'install-help'],
  'install-help': ['home', 'download', 'register-login', 'privacy', 'support', 'status'],
};

export const FamilyWebCollectionCoverage: Record<FamilyWebCollectionMode, ReadonlyArray<FamilyWebPage>> = {
  none: ['home', 'download', 'privacy', 'support', 'status', 'install-help'],
  'anonymous-operational-telemetry': RequiredFamilyWebPages,
  'explicit-account-data': ['register-login'],
  'forbidden-child-data': RequiredFamilyWebPages,
};

export const FamilyWebCollectionStateByMode: Record<
  FamilyWebCollectionMode,
  Infer<typeof FamilyWebCollectionStateSchema>
> = {
  none: 'default-public-page',
  'anonymous-operational-telemetry': 'disabled-by-default',
  'explicit-account-data': 'account-handoff-only',
  'forbidden-child-data': 'forbidden',
};
