import { describe, expect, it } from 'vitest';
import {
  ProductionSupportPublicationRuntimeReadinessProofSchema,
  ProductionSupportPublicationRuntimeReadinessRowSchema,
  summarizeProductionSupportPublicationRuntimeReadinessRows,
} from '../src/production-support-publication-runtime-readiness-proof';
import { ProductionSupportPublicationRuntimeReadinessReadModel } from '../src/production-support-publication-runtime-readiness-read-model';

describe('production support publication runtime readiness proof', () => {
  acceptsRuntimeReadinessRows();
  rejectsRuntimePublicationAndUploadOverclaims();
  rejectsSensitiveRuntimeReadinessData();
  rejectsIncompleteRuntimeReadinessCoverage();
});

function acceptsRuntimeReadinessRows(): void {
  it('accepts public runtime support publication and upload readiness rows as manual proof', () => {
    const proof = ProductionSupportPublicationRuntimeReadinessProofSchema.parse(
      ProductionSupportPublicationRuntimeReadinessReadModel
    );

    expect(summarizeProductionSupportPublicationRuntimeReadinessRows(proof.rows)).toEqual({
      'public-runtime-publication-adapter-readiness': 1,
      'support-runbook-publication-runner-readiness': 1,
      'incident-status-publication-runner-readiness': 1,
      'support-upload-publication-runtime-readiness': 1,
      'privacy-legal-publication-runtime-readiness': 1,
      'public-support-contact-runtime-readiness': 1,
    });
    expect(proof.publicRuntimeExecutionClaim).toBe('not-implemented');
    expect(proof.publicationRunnerExecutionClaim).toBe('manual-required');
    expect(proof.supportBackendUploadExecutionClaim).toBe('manual-required');
    expect(proof.accountLookupExecutionClaim).toBe('manual-required');
    expect(proof.billingProviderContactClaim).toBe('manual-required');
    expect(proof.productionSlaClaim).toBe('not-implemented');
    expect(proof.legalDisclosureExecutionClaim).toBe('manual-required');
    expect(proof.childActivityCustodyClaim).toBe('not-implemented');
  });
}

function rejectsRuntimePublicationAndUploadOverclaims(): void {
  it('rejects implemented public runtime, publication runner, and support upload execution states', () => {
    const publicRuntimeRow = requiredRuntimeReadiness('public-runtime-publication-adapter-readiness');
    const runnerRow = requiredRuntimeReadiness('support-runbook-publication-runner-readiness');
    const uploadRow = requiredRuntimeReadiness('support-upload-publication-runtime-readiness');

    expect(
      ProductionSupportPublicationRuntimeReadinessRowSchema.safeParse({
        ...publicRuntimeRow,
        publicRuntimeState: 'implemented',
      }).success
    ).toBe(false);
    expect(
      ProductionSupportPublicationRuntimeReadinessRowSchema.safeParse({
        ...runnerRow,
        publicationRunnerState: 'executed',
      }).success
    ).toBe(false);
    expect(
      ProductionSupportPublicationRuntimeReadinessRowSchema.safeParse({
        ...uploadRow,
        supportBackendUploadState: 'executed',
      }).success
    ).toBe(false);
  });
}

function rejectsSensitiveRuntimeReadinessData(): void {
  it('rejects runtime readiness rows that allow support payloads or omit provider secrets from exclusions', () => {
    const uploadRow = requiredRuntimeReadiness('support-upload-publication-runtime-readiness');

    expect(
      ProductionSupportPublicationRuntimeReadinessRowSchema.safeParse({
        ...uploadRow,
        supportSafeDataClasses: [...uploadRow.supportSafeDataClasses, 'raw-support-bundle'],
      }).success
    ).toBe(false);
    expect(
      ProductionSupportPublicationRuntimeReadinessRowSchema.safeParse({
        ...uploadRow,
        forbiddenDataClasses: uploadRow.forbiddenDataClasses.filter((dataClass) => dataClass !== 'provider-secrets'),
      }).success
    ).toBe(false);
  });
}

function rejectsIncompleteRuntimeReadinessCoverage(): void {
  it('rejects proof missing public support runtime readiness or provider-secret non-claims', () => {
    expect(
      ProductionSupportPublicationRuntimeReadinessProofSchema.safeParse({
        ...ProductionSupportPublicationRuntimeReadinessReadModel,
        rows: ProductionSupportPublicationRuntimeReadinessReadModel.rows.filter(
          (row) => row.item !== 'public-support-contact-runtime-readiness'
        ),
      }).success
    ).toBe(false);
    expect(
      ProductionSupportPublicationRuntimeReadinessProofSchema.safeParse({
        ...ProductionSupportPublicationRuntimeReadinessReadModel,
        nonClaims: ProductionSupportPublicationRuntimeReadinessReadModel.nonClaims.filter(
          (nonClaim) => nonClaim !== 'no-provider-secret-custody'
        ),
      }).success
    ).toBe(false);
  });
}

function requiredRuntimeReadiness(
  item:
    | 'public-runtime-publication-adapter-readiness'
    | 'support-runbook-publication-runner-readiness'
    | 'support-upload-publication-runtime-readiness'
): (typeof ProductionSupportPublicationRuntimeReadinessReadModel.rows)[number] {
  const row = ProductionSupportPublicationRuntimeReadinessReadModel.rows.find((entry) => entry.item === item);
  if (row === undefined) {
    throw new Error(`missing publication runtime readiness row: ${item}`);
  }
  return row;
}
