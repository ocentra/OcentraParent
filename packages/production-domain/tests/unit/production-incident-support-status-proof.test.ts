import { describe, expect, it } from 'vitest';
import {
  ProductionIncidentSupportStatusProofSchema,
  ProductionIncidentSupportStatusRowSchema,
  summarizeProductionIncidentSupportStatusRows,
} from '@ocentra-parent/schema-domain/production-incident-support-status-proof';
import { ProductionIncidentSupportStatusReadModel } from '@ocentra-parent/schema-domain/production-incident-support-status-read-model';

describe('production incident support status proof', () => {
  acceptsIncidentSupportStatusRows();
  rejectsProductionSupportExecutionOverclaims();
  rejectsSensitiveIncidentSupportData();
  rejectsIncompleteIncidentSupportStatusCoverage();
});

function acceptsIncidentSupportStatusRows(): void {
  it('accepts incident support status rows without publication, upload, or custody claims', () => {
    const proof = ProductionIncidentSupportStatusProofSchema.parse(ProductionIncidentSupportStatusReadModel);

    expect(summarizeProductionIncidentSupportStatusRows(proof.rows)).toEqual({
      'support-incident-intake': 1,
      'parent-consent-status': 1,
      'privacy-legal-disclosure-status': 1,
      'data-export-request-status': 1,
      'delete-request-status': 1,
      'incident-publication-status': 1,
      'case-resolution-handoff-status': 1,
    });
    expect(proof.publicPublicationState).toBe('publication-required');
    expect(proof.legalExecutionState).toBe('manual-required');
    expect(proof.supportBackendUploadExecutionState).toBe('manual-required');
    expect(proof.accountLookupExecutionState).toBe('manual-required');
    expect(proof.billingProviderContactState).toBe('manual-required');
    expect(proof.remoteSupportSessionState).toBe('not-implemented');
    expect(proof.productionSlaState).toBe('not-implemented');
    expect(proof.childActivityCustodyState).toBe('not-implemented');
  });
}

function rejectsProductionSupportExecutionOverclaims(): void {
  it('rejects executed publication or backend upload status rows and proof-level claims', () => {
    const publication = requiredStatus('incident-publication-status');
    const exportRequest = requiredStatus('data-export-request-status');

    expect(
      ProductionIncidentSupportStatusRowSchema.safeParse({
        ...publication,
        publicPublicationState: 'executed',
      }).success
    ).toBe(false);
    expect(
      ProductionIncidentSupportStatusRowSchema.safeParse({
        ...exportRequest,
        backendUploadState: 'executed',
      }).success
    ).toBe(false);
    expect(
      ProductionIncidentSupportStatusProofSchema.safeParse({
        ...ProductionIncidentSupportStatusReadModel,
        productionSlaState: 'implemented',
      }).success
    ).toBe(false);
  });
}

function rejectsSensitiveIncidentSupportData(): void {
  it('rejects support rows that allow raw support bundles or omit provider secret exclusions', () => {
    const intake = requiredStatus('support-incident-intake');

    expect(
      ProductionIncidentSupportStatusRowSchema.safeParse({
        ...intake,
        supportSafeDataClasses: [...intake.supportSafeDataClasses, 'raw-support-bundle'],
      }).success
    ).toBe(false);
    expect(
      ProductionIncidentSupportStatusRowSchema.safeParse({
        ...intake,
        forbiddenDataClasses: intake.forbiddenDataClasses.filter((dataClass) => dataClass !== 'provider-secrets'),
      }).success
    ).toBe(false);
  });
}

function rejectsIncompleteIncidentSupportStatusCoverage(): void {
  it('rejects proof that omits delete request status or provider-secret non-claims', () => {
    expect(
      ProductionIncidentSupportStatusProofSchema.safeParse({
        ...ProductionIncidentSupportStatusReadModel,
        rows: ProductionIncidentSupportStatusReadModel.rows.filter((row) => row.surface !== 'delete-request-status'),
      }).success
    ).toBe(false);
    expect(
      ProductionIncidentSupportStatusProofSchema.safeParse({
        ...ProductionIncidentSupportStatusReadModel,
        nonClaims: ProductionIncidentSupportStatusReadModel.nonClaims.filter(
          (nonClaim) => nonClaim !== 'no-provider-secrets'
        ),
      }).success
    ).toBe(false);
  });
}

function requiredStatus(
  surface: 'support-incident-intake' | 'data-export-request-status' | 'incident-publication-status'
) {
  const row = ProductionIncidentSupportStatusReadModel.rows.find((entry) => entry.surface === surface);
  if (row === undefined) {
    throw new Error(`missing production incident support status row: ${surface}`);
  }
  return row;
}
