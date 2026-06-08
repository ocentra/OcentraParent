import { Schema } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from './reference-primitives';
import {
  PublicSupportContactStatusProofSchema,
  type PublicSupportContactStatusRow,
  type PublicSupportContactStatusSurface,
} from './public-support-contact-status-proof';
import {
  ForbiddenPublicSupportContactStatusDataClasses,
  RequiredPublicSupportContactStatusNonClaims,
} from './public-support-contact-status-values';

export const PublicSupportContactStatusReadModel = PublicSupportContactStatusProofSchema.parse({
  schemaVersion: 'public-support-contact-status-proof',
  rows: [
    supportContact('public-support-contact', 'production-support-publication-workflow-proof', [
      'contact-channel-status',
      'support-runbook-status',
      'manual-proof-reference',
      'publication-reference',
    ]),
    supportContact('support-status-page-contact', 'production-release-public-runtime-handoff-proof', [
      'contact-channel-status',
      'incident-status',
      'support-runbook-status',
    ]),
    supportContact('support-runbook-contact', 'production-release-public-docs-status-proof', [
      'support-runbook-status',
      'contact-channel-status',
      'manual-proof-reference',
    ]),
    supportContact('incident-status-contact', 'support-incident-workflow-proof', [
      'incident-status',
      'contact-channel-status',
      'legal-review-status',
    ]),
    supportContact('backend-upload-support-contact', 'production-support-backend-upload-status-proof', [
      'support-upload-status-summary',
      'contact-channel-status',
      'manual-proof-reference',
    ]),
    supportContact('billing-support-contact', 'billing-support-admin-boundary-proof', [
      'billing-support-status',
      'account-status',
      'contact-channel-status',
    ]),
  ],
  nonClaims: RequiredPublicSupportContactStatusNonClaims,
  publicRuntimeExecutionClaim: 'not-implemented',
  supportBackendUploadExecutionClaim: 'manual-required',
  accountLookupExecutionClaim: 'manual-required',
  billingProviderContactClaim: 'manual-required',
  remoteSupportSessionClaim: 'not-implemented',
  productionSlaClaim: 'not-implemented',
  legalDisclosureExecutionClaim: 'manual-required',
  childActivityCustodyClaim: 'not-implemented',
  updatedAt: Schema.decodeUnknownSync(ParentTimestampSchema)('2026-06-05T18:03:52.486Z'),
});

export const PublicSupportContactStatusKnownGaps = [
  'Public support contact/status rows are source-contract proof only; family.ocentra.ca runtime remains unimplemented.',
  'Support backend upload execution, account lookup, billing provider contact, legal disclosure execution, remote support sessions, and production SLA remain manual-required or not implemented.',
  'No child activity evidence, provider secrets, raw support bundles, account lookup results, billing contact records, remote support transcripts, or parent rule sources are included.',
] as const;

function supportContact(
  surface: PublicSupportContactStatusSurface,
  sourceProof: PublicSupportContactStatusRow['sourceProof'],
  supportSafeDataClasses: PublicSupportContactStatusRow['supportSafeDataClasses']
) {
  return {
    schemaVersion: 'public-support-contact-status-proof',
    surface,
    sourceProof,
    sourceContractState: 'source-contract-ready',
    publicRouteState: surface === 'public-support-contact' ? 'publication-required' : 'route-contract-only',
    publicRuntimeState: 'not-implemented',
    contactExecutionState: 'manual-required',
    contactStatusBoundaryState: surface === 'support-status-page-contact' ? 'manual-required' : 'backend-required',
    supportBackendUploadState: surface === 'backend-upload-support-contact' ? 'manual-required' : 'not-implemented',
    supportSafeDataClasses,
    forbiddenDataClasses: ForbiddenPublicSupportContactStatusDataClasses,
    publicationReference: `public-support-contact-status-${surface}`,
    runtimeReference: `public-support-contact-runtime-${surface}`,
    statusBoundaryReference: `public-support-contact-status-boundary-${surface}`,
    manualRequirement: `${surface}-requires-public-runtime-legal-review-and-support-execution-proof`,
  } as const;
}
