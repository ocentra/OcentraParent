import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from '@ocentra-parent/family-domain/reference-primitives';
import {
  ForbiddenPublicSupportContactStatusDataClasses,
  PublicSupportContactStatusDataClassSchema,
  PublicSupportContactStatusNonClaimSchema,
  PublicSupportContactStatusReferenceSchema,
  PublicSupportContactStatusRequirementSchema,
  PublicSupportContactStatusSchemaVersionSchema,
  PublicSupportContactStatusSourceProofSchema,
  PublicSupportContactStatusStateSchema,
  PublicSupportContactStatusSurfaceSchema,
  RequiredPublicSupportContactStatusNonClaims,
  RequiredPublicSupportContactStatusSurfaces,
} from './public-support-contact-status-values';

export * from './public-support-contact-status-values';

type PublicSupportContactStatusProofCandidate = {
  readonly rows: ReadonlyArray<{ readonly surface: string }>;
  readonly nonClaims: ReadonlyArray<string>;
  readonly publicRuntimeExecutionClaim: string;
  readonly supportBackendUploadExecutionClaim: string;
  readonly accountLookupExecutionClaim: string;
  readonly billingProviderContactClaim: string;
  readonly remoteSupportSessionClaim: string;
  readonly productionSlaClaim: string;
  readonly legalDisclosureExecutionClaim: string;
  readonly childActivityCustodyClaim: string;
};

export const PublicSupportContactStatusRowSchema = withParser(
  Schema.Struct({
    schemaVersion: PublicSupportContactStatusSchemaVersionSchema,
    surface: PublicSupportContactStatusSurfaceSchema,
    sourceProof: PublicSupportContactStatusSourceProofSchema,
    sourceContractState: PublicSupportContactStatusStateSchema,
    publicRouteState: PublicSupportContactStatusStateSchema,
    publicRuntimeState: PublicSupportContactStatusStateSchema,
    contactExecutionState: PublicSupportContactStatusStateSchema,
    contactStatusBoundaryState: PublicSupportContactStatusStateSchema,
    supportBackendUploadState: PublicSupportContactStatusStateSchema,
    supportSafeDataClasses: Schema.Array(PublicSupportContactStatusDataClassSchema),
    forbiddenDataClasses: Schema.Array(PublicSupportContactStatusDataClassSchema),
    publicationReference: PublicSupportContactStatusReferenceSchema,
    runtimeReference: PublicSupportContactStatusReferenceSchema,
    statusBoundaryReference: PublicSupportContactStatusReferenceSchema,
    manualRequirement: PublicSupportContactStatusRequirementSchema,
  }).pipe(
    Schema.filter(
      (row) =>
        (row.publicRouteState !== 'implemented' && row.publicRouteState !== 'executed') ||
        'Expected public support contact status rows to avoid public route execution claims'
    ),
    Schema.filter(
      (row) =>
        (row.publicRuntimeState !== 'implemented' && row.publicRuntimeState !== 'executed') ||
        'Expected public support contact status rows to avoid public runtime execution claims'
    ),
    Schema.filter(
      (row) =>
        row.contactExecutionState !== 'executed' ||
        'Expected public support contact status rows to keep contact execution manual-required'
    ),
    Schema.filter(
      (row) =>
        (row.contactStatusBoundaryState !== 'implemented' && row.contactStatusBoundaryState !== 'executed') ||
        'Expected public support contact status rows to keep status boundary execution unclaimed'
    ),
    Schema.filter(
      (row) =>
        (row.supportBackendUploadState !== 'implemented' && row.supportBackendUploadState !== 'executed') ||
        'Expected public support contact status rows to avoid support backend upload execution claims'
    ),
    Schema.filter(
      (row) =>
        row.supportSafeDataClasses.every(
          (dataClass) => !ForbiddenPublicSupportContactStatusDataClasses.includes(dataClass as never)
        ) || 'Expected public support contact status rows to exclude sensitive support and custody data'
    ),
    Schema.filter(
      (row) =>
        ForbiddenPublicSupportContactStatusDataClasses.every((dataClass) =>
          row.forbiddenDataClasses.includes(dataClass)
        ) || 'Expected public support contact status rows to enumerate forbidden support data classes'
    )
  )
);

export const PublicSupportContactStatusProofSchema = withParser(
  Schema.Struct({
    schemaVersion: PublicSupportContactStatusSchemaVersionSchema,
    rows: Schema.Array(PublicSupportContactStatusRowSchema),
    nonClaims: Schema.Array(PublicSupportContactStatusNonClaimSchema),
    publicRuntimeExecutionClaim: PublicSupportContactStatusStateSchema,
    supportBackendUploadExecutionClaim: PublicSupportContactStatusStateSchema,
    accountLookupExecutionClaim: PublicSupportContactStatusStateSchema,
    billingProviderContactClaim: PublicSupportContactStatusStateSchema,
    remoteSupportSessionClaim: PublicSupportContactStatusStateSchema,
    productionSlaClaim: PublicSupportContactStatusStateSchema,
    legalDisclosureExecutionClaim: PublicSupportContactStatusStateSchema,
    childActivityCustodyClaim: PublicSupportContactStatusStateSchema,
    updatedAt: ParentTimestampSchema,
  }).pipe(
    Schema.filter(
      (proof) =>
        publicSupportContactStatusProofIsHonest(proof) ||
        'Expected public support contact status proof to cover support contact rows while preserving non-claims'
    )
  )
);

export type PublicSupportContactStatusRow = Infer<typeof PublicSupportContactStatusRowSchema>;
export type PublicSupportContactStatusProof = Infer<typeof PublicSupportContactStatusProofSchema>;
export type PublicSupportContactStatusSurface = Infer<typeof PublicSupportContactStatusSurfaceSchema>;

export const decodePublicSupportContactStatusProof = Schema.decodeUnknownSync(PublicSupportContactStatusProofSchema);

export function summarizePublicSupportContactStatusRows(
  rows: ReadonlyArray<PublicSupportContactStatusRow>
): Record<PublicSupportContactStatusSurface, number> {
  return RequiredPublicSupportContactStatusSurfaces.reduce(
    (summary, surfaceName) => ({
      ...summary,
      [surfaceName]: rows.filter((row) => row.surface === surfaceName).length,
    }),
    {} as Record<PublicSupportContactStatusSurface, number>
  );
}

function publicSupportContactStatusProofIsHonest(proof: PublicSupportContactStatusProofCandidate): boolean {
  return (
    RequiredPublicSupportContactStatusSurfaces.every((surfaceName) =>
      proof.rows.some((row) => row.surface === surfaceName)
    ) &&
    RequiredPublicSupportContactStatusNonClaims.every((nonClaim) => proof.nonClaims.includes(nonClaim)) &&
    proof.publicRuntimeExecutionClaim === 'not-implemented' &&
    proof.supportBackendUploadExecutionClaim === 'manual-required' &&
    proof.accountLookupExecutionClaim === 'manual-required' &&
    proof.billingProviderContactClaim === 'manual-required' &&
    proof.remoteSupportSessionClaim === 'not-implemented' &&
    proof.productionSlaClaim === 'not-implemented' &&
    proof.legalDisclosureExecutionClaim === 'manual-required' &&
    proof.childActivityCustodyClaim === 'not-implemented'
  );
}
