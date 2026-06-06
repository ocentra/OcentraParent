import { Schema } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from './reference-primitives';
import {
  ProductionIncidentSupportStatusProofSchema,
  type ProductionIncidentSupportStatusRow,
  type ProductionIncidentSupportStatusSurface,
} from './production-incident-support-status-proof';
import {
  ForbiddenProductionIncidentSupportStatusDataClasses,
  RequiredProductionIncidentSupportStatusNonClaims,
} from './production-incident-support-status-values';

export const ProductionIncidentSupportStatusReadModel = ProductionIncidentSupportStatusProofSchema.parse({
  schemaVersion: 'production-incident-support-status-proof',
  rows: [
    incidentStatus('support-incident-intake', 'support-incident-workflow-proof', [
      'incident-status-metadata',
      'parent-consent-reference',
      'manual-proof-reference',
    ]),
    incidentStatus('parent-consent-status', 'support-incident-workflow-proof', [
      'parent-consent-reference',
      'redaction-summary-reference',
      'manual-proof-reference',
    ]),
    incidentStatus('privacy-legal-disclosure-status', 'production-release-public-docs-status-proof', [
      'privacy-legal-disclosure-status',
      'support-runbook-reference',
      'manual-proof-reference',
    ]),
    incidentStatus('data-export-request-status', 'production-support-backend-upload-custody-audit-proof', [
      'data-export-delete-status',
      'custody-audit-reference',
      'redaction-summary-reference',
    ]),
    incidentStatus('delete-request-status', 'production-support-backend-upload-custody-audit-proof', [
      'data-export-delete-status',
      'custody-audit-reference',
      'manual-proof-reference',
    ]),
    incidentStatus('incident-publication-status', 'production-support-publication-workflow-proof', [
      'incident-status-metadata',
      'public-status-reference',
      'support-runbook-reference',
    ]),
    incidentStatus('case-resolution-handoff-status', 'production-support-case-resolution-status-proof', [
      'case-resolution-status',
      'incident-status-metadata',
      'manual-proof-reference',
    ]),
  ],
  nonClaims: RequiredProductionIncidentSupportStatusNonClaims,
  publicPublicationState: 'publication-required',
  legalExecutionState: 'manual-required',
  supportBackendUploadExecutionState: 'manual-required',
  accountLookupExecutionState: 'manual-required',
  billingProviderContactState: 'manual-required',
  remoteSupportSessionState: 'not-implemented',
  productionSlaState: 'not-implemented',
  childActivityCustodyState: 'not-implemented',
  updatedAt: Schema.decodeUnknownSync(ParentTimestampSchema)('2026-06-05T23:52:00.000Z'),
});

export const ProductionIncidentSupportStatusKnownGaps = [
  'Production incident support status rows are source-contract proof only; family.ocentra.ca publication remains unimplemented.',
  'Legal execution, support backend upload execution, account lookup, billing provider contact, remote support sessions, and production SLA remain manual-required or not implemented.',
  'No child activity evidence, provider secrets, raw support bundles, account lookup results, billing contact records, remote transcripts, or parent rules are included.',
] as const;

function incidentStatus(
  surface: ProductionIncidentSupportStatusSurface,
  sourceProof: ProductionIncidentSupportStatusRow['sourceProof'],
  supportSafeDataClasses: ProductionIncidentSupportStatusRow['supportSafeDataClasses']
) {
  return {
    schemaVersion: 'production-incident-support-status-proof',
    surface,
    sourceProof,
    sourceContractState: 'source-contract-ready',
    parentConsentState: surface === 'support-incident-intake' ? 'parent-consent-required' : 'source-contract-ready',
    privacyLegalState: surface === 'privacy-legal-disclosure-status' ? 'disclosure-required' : 'source-contract-ready',
    exportDeleteState:
      surface === 'data-export-request-status' || surface === 'delete-request-status'
        ? 'export-delete-ready'
        : 'manual-required',
    publicPublicationState: surface === 'incident-publication-status' ? 'publication-required' : 'manual-required',
    backendUploadState: 'manual-required',
    supportSafeDataClasses,
    forbiddenDataClasses: ForbiddenProductionIncidentSupportStatusDataClasses,
    incidentReference: `production-incident-support-status-${surface}`,
    custodyReference: `production-incident-support-custody-${surface}`,
    manualRequirement: `${surface}-requires-parent-consent-publication-and-support-execution-proof`,
  } as const;
}
