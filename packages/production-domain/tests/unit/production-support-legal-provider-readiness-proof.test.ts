import { describe, expect, it } from 'vitest';
import {
  ProductionSupportLegalProviderReadinessProofSchema,
  ProductionSupportLegalProviderReadinessRowSchema,
  summarizeProductionSupportLegalProviderReadinessRows,
} from '../../src/production-support-legal-provider-readiness-proof';
import { ProductionSupportLegalProviderReadinessReadModel } from '../../src/production-support-legal-provider-readiness-read-model';

describe('production support legal provider readiness proof', () => {
  acceptsLegalProviderReadinessRows();
  rejectsLegalProviderExecutionOverclaims();
  rejectsProviderSecretsAndCustodyData();
  rejectsIncompleteLegalProviderCoverage();
});

function acceptsLegalProviderReadinessRows(): void {
  it('accepts legal, export/delete, provider, remote support, and SLA rows as non-executing proof', () => {
    const proof = ProductionSupportLegalProviderReadinessProofSchema.parse(
      ProductionSupportLegalProviderReadinessReadModel
    );

    expect(summarizeProductionSupportLegalProviderReadinessRows(proof.rows)).toEqual({
      'privacy-legal-review-readiness': 1,
      'data-export-delete-runtime-readiness': 1,
      'provider-secret-custody-boundary': 1,
      'billing-provider-contact-readiness': 1,
      'remote-support-legal-session-boundary': 1,
      'production-sla-legal-boundary': 1,
    });
    expect(proof.legalDisclosureExecutionState).toBe('manual-required');
    expect(proof.dataExportDeleteRuntimeState).toBe('manual-required');
    expect(proof.providerSecretCustodyState).toBe('not-implemented');
    expect(proof.billingProviderContactExecutionState).toBe('manual-required');
    expect(proof.accountLookupExecutionState).toBe('manual-required');
    expect(proof.remoteSupportSessionState).toBe('not-implemented');
    expect(proof.productionSlaState).toBe('not-implemented');
    expect(proof.supportBackendUploadExecutionState).toBe('manual-required');
    expect(proof.publicRuntimeExecutionState).toBe('not-implemented');
    expect(proof.childActivityCustodyState).toBe('not-implemented');
  });
}

function rejectsLegalProviderExecutionOverclaims(): void {
  it('rejects legal, export/delete, provider contact, remote session, and SLA execution claims', () => {
    const legal = requiredReadiness('privacy-legal-review-readiness');
    const exportDelete = requiredReadiness('data-export-delete-runtime-readiness');
    const providerContact = requiredReadiness('billing-provider-contact-readiness');
    const remoteSession = requiredReadiness('remote-support-legal-session-boundary');

    expect(
      ProductionSupportLegalProviderReadinessRowSchema.safeParse({ ...legal, legalDisclosureState: 'executed' }).success
    ).toBe(false);
    expect(
      ProductionSupportLegalProviderReadinessRowSchema.safeParse({ ...exportDelete, dataExportDeleteState: 'executed' })
        .success
    ).toBe(false);
    expect(
      ProductionSupportLegalProviderReadinessRowSchema.safeParse({
        ...providerContact,
        billingProviderContactState: 'executed',
      }).success
    ).toBe(false);
    expect(
      ProductionSupportLegalProviderReadinessRowSchema.safeParse({
        ...remoteSession,
        remoteSupportSessionState: 'executed',
      }).success
    ).toBe(false);
    expect(
      ProductionSupportLegalProviderReadinessProofSchema.safeParse({
        ...ProductionSupportLegalProviderReadinessReadModel,
        productionSlaState: 'implemented',
      }).success
    ).toBe(false);
  });
}

function rejectsProviderSecretsAndCustodyData(): void {
  it('rejects provider secrets, billing records, remote transcripts, SLA commitments, and child custody', () => {
    const providerBoundary = requiredReadiness('provider-secret-custody-boundary');

    expect(
      ProductionSupportLegalProviderReadinessRowSchema.safeParse({
        ...providerBoundary,
        providerSecretCustodyState: 'implemented',
      }).success
    ).toBe(false);
    expect(
      ProductionSupportLegalProviderReadinessRowSchema.safeParse({
        ...providerBoundary,
        supportSafeDataClasses: [...providerBoundary.supportSafeDataClasses, 'provider-secret'],
      }).success
    ).toBe(false);
    expect(
      ProductionSupportLegalProviderReadinessRowSchema.safeParse({
        ...providerBoundary,
        forbiddenDataClasses: providerBoundary.forbiddenDataClasses.filter(
          (dataClass) => dataClass !== 'billing-provider-contact-record'
        ),
      }).success
    ).toBe(false);
  });
}

function rejectsIncompleteLegalProviderCoverage(): void {
  it('rejects missing legal/provider rows or provider-secret non-claims', () => {
    expect(
      ProductionSupportLegalProviderReadinessProofSchema.safeParse({
        ...ProductionSupportLegalProviderReadinessReadModel,
        rows: ProductionSupportLegalProviderReadinessReadModel.rows.filter(
          (row) => row.surface !== 'provider-secret-custody-boundary'
        ),
      }).success
    ).toBe(false);
    expect(
      ProductionSupportLegalProviderReadinessProofSchema.safeParse({
        ...ProductionSupportLegalProviderReadinessReadModel,
        nonClaims: ProductionSupportLegalProviderReadinessReadModel.nonClaims.filter(
          (nonClaim) => nonClaim !== 'no-provider-secret-custody'
        ),
      }).success
    ).toBe(false);
  });
}

function requiredReadiness(
  surface:
    | 'privacy-legal-review-readiness'
    | 'data-export-delete-runtime-readiness'
    | 'provider-secret-custody-boundary'
    | 'billing-provider-contact-readiness'
    | 'remote-support-legal-session-boundary'
) {
  const row = ProductionSupportLegalProviderReadinessReadModel.rows.find((entry) => entry.surface === surface);
  if (row === undefined) {
    throw new Error(`missing production support legal/provider readiness row: ${surface}`);
  }
  return row;
}
