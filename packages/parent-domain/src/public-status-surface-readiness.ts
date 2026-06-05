import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from './reference-primitives';

const NonEmptyPublicStatusSurfaceText = Schema.String.pipe(Schema.minLength(1));

export const PublicStatusSurfaceReadinessSchemaVersionSchema = withParser(
  Schema.Literal('public-status-surface-readiness-proof')
);

export const PublicStatusSurfaceSchema = withParser(
  Schema.Literal(
    'family-public-site',
    'public-download',
    'release-status',
    'update-status',
    'account-status',
    'subscription-status',
    'support-status'
  )
);

export const PublicStatusSurfaceReadinessStateSchema = withParser(
  Schema.Literal(
    'source-contract-ready',
    'adapter-boundary-ready',
    'backend-required',
    'manual-required',
    'not-implemented',
    'production-promotion-required'
  )
);

export const PublicStatusSurfaceSourceProofSchema = withParser(
  Schema.Literal(
    'production-release-public-status-proof',
    'production-release-public-runtime-handoff-proof',
    'production-release-public-docs-status-proof',
    'billing-account-endpoint-contract-proof',
    'billing-entitlement-runtime-proof',
    'production-support-case-resolution-status-proof'
  )
);

export const PublicStatusSurfaceDataClassSchema = withParser(
  Schema.Literal(
    'release-version',
    'platform',
    'download-status',
    'update-status',
    'account-status',
    'subscription-status',
    'entitlement-summary',
    'support-runbook-status',
    'incident-status',
    'public-doc-status',
    'child-activity-evidence',
    'browser-url-history',
    'screenshots',
    'journals',
    'sqlite-snapshots',
    'private-paths',
    'provider-secrets',
    'raw-support-bundle',
    'billing-provider-contact',
    'remote-support-transcript'
  )
);

export const PublicStatusSurfaceNonClaimSchema = withParser(
  Schema.Literal(
    'no-family-ocentra-ca-runtime',
    'no-account-backend-runtime',
    'no-billing-provider-runtime',
    'no-production-publishing',
    'no-signing-store-proof',
    'no-updater-execution',
    'no-support-backend-upload',
    'no-production-sla',
    'no-legal-execution',
    'no-remote-support-session',
    'no-child-activity-custody'
  )
);

export const PublicStatusSurfaceReferenceSchema = NonEmptyPublicStatusSurfaceText.pipe(
  Schema.brand('PublicStatusSurfaceReference')
);
export const PublicStatusSurfaceRequirementSchema = NonEmptyPublicStatusSurfaceText.pipe(
  Schema.brand('PublicStatusSurfaceRequirement')
);

export const ForbiddenPublicStatusSurfaceDataClasses = [
  'child-activity-evidence',
  'browser-url-history',
  'screenshots',
  'journals',
  'sqlite-snapshots',
  'private-paths',
  'provider-secrets',
  'raw-support-bundle',
  'billing-provider-contact',
  'remote-support-transcript',
] as const;

export const RequiredPublicStatusSurfaces = [
  'family-public-site',
  'public-download',
  'release-status',
  'update-status',
  'account-status',
  'subscription-status',
  'support-status',
] as const;

export const RequiredPublicStatusSurfaceNonClaims = [
  'no-family-ocentra-ca-runtime',
  'no-account-backend-runtime',
  'no-billing-provider-runtime',
  'no-production-publishing',
  'no-signing-store-proof',
  'no-updater-execution',
  'no-support-backend-upload',
  'no-production-sla',
  'no-legal-execution',
  'no-remote-support-session',
  'no-child-activity-custody',
] as const;

type PublicStatusSurfaceReadinessProofCandidate = {
  readonly rows: ReadonlyArray<{ readonly surface: string }>;
  readonly nonClaims: ReadonlyArray<string>;
  readonly publicWebsiteRuntimeClaim: string;
  readonly accountBackendRuntimeClaim: string;
  readonly billingProviderRuntimeClaim: string;
  readonly supportBackendUploadClaim: string;
  readonly productionSlaClaim: string;
  readonly legalExecutionClaim: string;
  readonly remoteSupportSessionClaim: string;
  readonly childActivityCustodyClaim: string;
};

export const PublicStatusSurfaceReadinessRowSchema = withParser(
  Schema.Struct({
    schemaVersion: PublicStatusSurfaceReadinessSchemaVersionSchema,
    surface: PublicStatusSurfaceSchema,
    publicRouteState: PublicStatusSurfaceReadinessStateSchema,
    runtimeAdapterState: PublicStatusSurfaceReadinessStateSchema,
    backendDependencyState: PublicStatusSurfaceReadinessStateSchema,
    parentVisibleState: PublicStatusSurfaceReadinessStateSchema,
    sourceProofs: Schema.Array(PublicStatusSurfaceSourceProofSchema),
    supportSafeDataClasses: Schema.Array(PublicStatusSurfaceDataClassSchema),
    forbiddenDataClasses: Schema.Array(PublicStatusSurfaceDataClassSchema),
    readinessReference: PublicStatusSurfaceReferenceSchema,
    manualRequirement: PublicStatusSurfaceRequirementSchema,
  }).pipe(
    Schema.filter(
      (row) =>
        row.supportSafeDataClasses.every(
          (dataClass) => !ForbiddenPublicStatusSurfaceDataClasses.includes(dataClass as never)
        ) || 'Expected public status surfaces to exclude child activity, provider secrets, and support payloads'
    ),
    Schema.filter(
      (row) =>
        ForbiddenPublicStatusSurfaceDataClasses.every((dataClass) => row.forbiddenDataClasses.includes(dataClass)) ||
        'Expected public status surfaces to enumerate every forbidden custody/support data class'
    )
  )
);

export const PublicStatusSurfaceReadinessProofSchema = withParser(
  Schema.Struct({
    schemaVersion: PublicStatusSurfaceReadinessSchemaVersionSchema,
    publicHost: Schema.Literal('family.ocentra.ca'),
    rows: Schema.Array(PublicStatusSurfaceReadinessRowSchema),
    nonClaims: Schema.Array(PublicStatusSurfaceNonClaimSchema),
    publicWebsiteRuntimeClaim: PublicStatusSurfaceReadinessStateSchema,
    accountBackendRuntimeClaim: PublicStatusSurfaceReadinessStateSchema,
    billingProviderRuntimeClaim: PublicStatusSurfaceReadinessStateSchema,
    supportBackendUploadClaim: PublicStatusSurfaceReadinessStateSchema,
    productionSlaClaim: PublicStatusSurfaceReadinessStateSchema,
    legalExecutionClaim: PublicStatusSurfaceReadinessStateSchema,
    remoteSupportSessionClaim: PublicStatusSurfaceReadinessStateSchema,
    childActivityCustodyClaim: PublicStatusSurfaceReadinessStateSchema,
    updatedAt: ParentTimestampSchema,
  }).pipe(
    Schema.filter(
      (proof) =>
        publicStatusSurfaceReadinessProofIsHonest(proof) ||
        'Expected public status surface readiness proof to cover every surface and preserve public/runtime non-claims'
    )
  )
);

export type PublicStatusSurfaceReadinessRow = Infer<typeof PublicStatusSurfaceReadinessRowSchema>;
export type PublicStatusSurfaceReadinessProof = Infer<typeof PublicStatusSurfaceReadinessProofSchema>;
export type PublicStatusSurface = Infer<typeof PublicStatusSurfaceSchema>;

export const decodePublicStatusSurfaceReadinessProof = Schema.decodeUnknownSync(
  PublicStatusSurfaceReadinessProofSchema
);

export const PublicStatusSurfaceReadinessReadModel = PublicStatusSurfaceReadinessProofSchema.parse({
  schemaVersion: 'public-status-surface-readiness-proof',
  publicHost: 'family.ocentra.ca',
  rows: [
    row('family-public-site', 'not-implemented', 'not-implemented', 'not-implemented', ['public-doc-status']),
    row('public-download', 'source-contract-ready', 'adapter-boundary-ready', 'backend-required', [
      'release-version',
      'platform',
      'download-status',
    ]),
    row('release-status', 'source-contract-ready', 'manual-required', 'production-promotion-required', [
      'release-version',
      'platform',
    ]),
    row('update-status', 'source-contract-ready', 'manual-required', 'manual-required', ['update-status', 'platform']),
    row('account-status', 'source-contract-ready', 'adapter-boundary-ready', 'backend-required', [
      'account-status',
      'entitlement-summary',
    ]),
    row('subscription-status', 'source-contract-ready', 'adapter-boundary-ready', 'backend-required', [
      'subscription-status',
      'entitlement-summary',
    ]),
    row('support-status', 'source-contract-ready', 'manual-required', 'manual-required', [
      'support-runbook-status',
      'incident-status',
    ]),
  ],
  nonClaims: RequiredPublicStatusSurfaceNonClaims,
  publicWebsiteRuntimeClaim: 'not-implemented',
  accountBackendRuntimeClaim: 'backend-required',
  billingProviderRuntimeClaim: 'not-implemented',
  supportBackendUploadClaim: 'manual-required',
  productionSlaClaim: 'not-implemented',
  legalExecutionClaim: 'manual-required',
  remoteSupportSessionClaim: 'not-implemented',
  childActivityCustodyClaim: 'not-implemented',
  updatedAt: Schema.decodeUnknownSync(ParentTimestampSchema)('2026-06-05T20:18:48.536Z'),
});

export const PublicStatusSurfaceReadinessKnownGaps = [
  'family.ocentra.ca public runtime is not implemented.',
  'Public download, account, subscription, release, update, and support status remain source-contract or adapter-boundary readiness only.',
  'Account backend, billing provider runtime, support backend upload, signing/store proof, updater execution, production SLA, legal execution, and remote support sessions remain manual-required or unimplemented.',
  'No child activity, raw support bundle payloads, provider secrets, billing-provider contact records, remote transcripts, or parent rules are hosted by this proof.',
] as const;

export function summarizePublicStatusSurfaceReadinessRows(
  rows: ReadonlyArray<PublicStatusSurfaceReadinessRow>
): Record<PublicStatusSurface, number> {
  return RequiredPublicStatusSurfaces.reduce(
    (summary, surface) => ({
      ...summary,
      [surface]: rows.filter((rowEntry) => rowEntry.surface === surface).length,
    }),
    {} as Record<PublicStatusSurface, number>
  );
}

function publicStatusSurfaceReadinessProofIsHonest(proof: PublicStatusSurfaceReadinessProofCandidate): boolean {
  return (
    RequiredPublicStatusSurfaces.every((surface) => proof.rows.some((rowEntry) => rowEntry.surface === surface)) &&
    RequiredPublicStatusSurfaceNonClaims.every((nonClaim) => proof.nonClaims.includes(nonClaim)) &&
    proof.publicWebsiteRuntimeClaim === 'not-implemented' &&
    proof.accountBackendRuntimeClaim === 'backend-required' &&
    proof.billingProviderRuntimeClaim === 'not-implemented' &&
    proof.supportBackendUploadClaim === 'manual-required' &&
    proof.productionSlaClaim === 'not-implemented' &&
    proof.legalExecutionClaim === 'manual-required' &&
    proof.remoteSupportSessionClaim === 'not-implemented' &&
    proof.childActivityCustodyClaim === 'not-implemented'
  );
}

function row(
  surface: PublicStatusSurface,
  publicRouteState: PublicStatusSurfaceReadinessRow['publicRouteState'],
  runtimeAdapterState: PublicStatusSurfaceReadinessRow['runtimeAdapterState'],
  backendDependencyState: PublicStatusSurfaceReadinessRow['backendDependencyState'],
  supportSafeDataClasses: PublicStatusSurfaceReadinessRow['supportSafeDataClasses']
) {
  return {
    schemaVersion: 'public-status-surface-readiness-proof',
    surface,
    publicRouteState,
    runtimeAdapterState,
    backendDependencyState,
    parentVisibleState: backendDependencyState,
    sourceProofs: sourceProofsFor(surface),
    supportSafeDataClasses,
    forbiddenDataClasses: ForbiddenPublicStatusSurfaceDataClasses,
    readinessReference: `public-status-surface-readiness-${surface}`,
    manualRequirement: `${surface}-requires-public-runtime-backend-and-manual-proof-before-production-claim`,
  } as const;
}

function sourceProofsFor(surface: PublicStatusSurface): PublicStatusSurfaceReadinessRow['sourceProofs'] {
  if (surface === 'account-status' || surface === 'subscription-status') {
    return [
      'production-release-public-status-proof',
      'production-release-public-runtime-handoff-proof',
      'billing-account-endpoint-contract-proof',
      'billing-entitlement-runtime-proof',
    ];
  }
  if (surface === 'support-status') {
    return [
      'production-release-public-runtime-handoff-proof',
      'production-release-public-docs-status-proof',
      'production-support-case-resolution-status-proof',
    ];
  }
  return [
    'production-release-public-status-proof',
    'production-release-public-runtime-handoff-proof',
    'production-release-public-docs-status-proof',
  ];
}
