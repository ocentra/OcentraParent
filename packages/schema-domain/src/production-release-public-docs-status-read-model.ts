import { Schema } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';
import {
  ProductionReleasePublicDocsStatusProofSchema,
  type ProductionReleasePublicDocsStatusDocument,
  type ProductionReleasePublicDocsStatusRow,
} from './production-release-public-docs-status';
import {
  ForbiddenPublicDocsStatusDataClasses,
  RequiredPublicDocsStatusNonClaims,
} from './production-release-public-docs-status-values';

export const ProductionReleasePublicDocsStatusReadModel = ProductionReleasePublicDocsStatusProofSchema.parse({
  schemaVersion: 'production-release-public-docs-status-proof',
  rows: [
    docsStatus('privacy-policy', 'documentation-expectation', 'public-family', [
      'public-policy-text',
      'data-custody-summary',
      'manual-proof-reference',
    ]),
    docsStatus('retention-policy', 'data-custody-expectation', 'public-family', [
      'retention-window-summary',
      'data-custody-summary',
      'manual-proof-reference',
    ]),
    docsStatus('export-delete-process', 'data-custody-expectation', 'public-family', [
      'export-delete-process-summary',
      'data-custody-summary',
      'manual-proof-reference',
    ]),
    docsStatus('support-runbook', 'release-installer-expectation', 'support-operator', [
      'support-runbook-status',
      'redaction-policy-summary',
      'manual-proof-reference',
    ]),
    docsStatus('incident-status-disclosure', 'support-incident-workflow-proof', 'support-operator', [
      'incident-status',
      'redaction-policy-summary',
      'manual-proof-reference',
    ]),
    docsStatus('legal-disclosure', 'documentation-expectation', 'legal-review', [
      'legal-disclosure-status',
      'contact-channel-status',
      'manual-proof-reference',
    ]),
  ],
  nonClaims: RequiredPublicDocsStatusNonClaims,
  publicWebsitePublicationClaim: 'manual-required',
  supportBackendUploadClaim: 'manual-required',
  accountLookupExecutionClaim: 'manual-required',
  billingProviderContactClaim: 'manual-required',
  remoteSupportSessionClaim: 'not-implemented',
  productionSlaClaim: 'not-implemented',
  childActivityCustodyClaim: 'not-implemented',
  updatedAt: Schema.decodeUnknownSync(ParentTimestampSchema)('2026-06-05T07:55:25.684Z'),
});

export const ProductionReleasePublicDocsStatusKnownGaps = [
  'Public privacy, retention, export/delete, support, incident, and legal docs are source-contract status only.',
  'Public publication on family.ocentra.ca remains manual-required until real route publication and legal review evidence exists.',
  'Support backend upload, account lookup, billing provider contact, remote support sessions, production SLA, and child-activity custody remain unimplemented or unclaimed.',
] as const;

function docsStatus(
  documentName: ProductionReleasePublicDocsStatusDocument,
  sourceProof: ProductionReleasePublicDocsStatusRow['sourceProof'],
  disclosureAudience: ProductionReleasePublicDocsStatusRow['disclosureAudience'],
  supportSafeDataClasses: ProductionReleasePublicDocsStatusRow['supportSafeDataClasses']
) {
  return {
    schemaVersion: 'production-release-public-docs-status-proof',
    document: documentName,
    sourceDocumentState: 'source-contract-ready',
    publicPublicationState: 'manual-required',
    publicRouteState: 'not-implemented',
    sourceProof,
    disclosureAudience,
    supportSafeDataClasses,
    forbiddenDataClasses: ForbiddenPublicDocsStatusDataClasses,
    publicationReference: `public-docs-status-${documentName}`,
    proofRequirement: `${documentName}-requires-public-publication-legal-review-and-support-safe-data-proof`,
  } as const;
}
