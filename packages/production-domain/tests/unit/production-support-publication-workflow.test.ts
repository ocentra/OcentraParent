import { describe, expect, it } from 'vitest';
import {
  ProductionSupportPublicationWorkflowProofSchema,
  ProductionSupportPublicationWorkflowRowSchema,
  summarizeProductionSupportPublicationWorkflowRows,
} from '@ocentra-parent/schema-domain/production-support-publication-workflow';
import { ProductionSupportPublicationWorkflowReadModel } from '@ocentra-parent/schema-domain/production-support-publication-workflow-read-model';

describe('production support publication workflow', () => {
  acceptsPublicationWorkflowRows();
  rejectsPublicationSupportAndLegalOverclaims();
  rejectsSensitivePublicationWorkflowData();
  rejectsIncompletePublicationWorkflowCoverage();
});

function acceptsPublicationWorkflowRows(): void {
  it('accepts public privacy legal support and backend-upload publication workflow rows as manual proof', () => {
    const proof = ProductionSupportPublicationWorkflowProofSchema.parse(ProductionSupportPublicationWorkflowReadModel);

    expect(summarizeProductionSupportPublicationWorkflowRows(proof.rows)).toEqual({
      'public-privacy-policy-publication': 1,
      'privacy-legal-disclosure-execution': 1,
      'support-runbook-publication': 1,
      'support-incident-status-publication': 1,
      'support-backend-upload-publication-handoff': 1,
      'public-support-contact-publication': 1,
    });
    expect(proof.publicRuntimeClaim).toBe('not-implemented');
    expect(proof.legalExecutionClaim).toBe('manual-required');
    expect(proof.supportBackendUploadExecutionClaim).toBe('manual-required');
    expect(proof.accountLookupExecutionClaim).toBe('manual-required');
    expect(proof.billingProviderContactClaim).toBe('manual-required');
    expect(proof.productionSlaClaim).toBe('not-implemented');
    expect(proof.childActivityCustodyClaim).toBe('not-implemented');
    expect(proof.nonClaims).toEqual([
      'no-real-public-runtime',
      'no-support-backend-upload-execution',
      'no-account-lookup-execution',
      'no-billing-provider-contact',
      'no-production-sla',
      'no-child-activity-custody',
      'no-legal-disclosure-execution',
      'no-remote-support-session',
    ]);
  });
}

function rejectsPublicationSupportAndLegalOverclaims(): void {
  it('rejects implemented publication runtime, executed legal disclosure, and support upload execution', () => {
    const privacyRow = requiredWorkflow('public-privacy-policy-publication');
    const legalRow = requiredWorkflow('privacy-legal-disclosure-execution');
    const uploadRow = requiredWorkflow('support-backend-upload-publication-handoff');

    expect(
      ProductionSupportPublicationWorkflowRowSchema.safeParse({
        ...privacyRow,
        publicPublicationState: 'implemented',
      }).success
    ).toBe(false);
    expect(
      ProductionSupportPublicationWorkflowRowSchema.safeParse({
        ...legalRow,
        legalExecutionState: 'executed',
      }).success
    ).toBe(false);
    expect(
      ProductionSupportPublicationWorkflowRowSchema.safeParse({
        ...uploadRow,
        supportBackendUploadState: 'executed',
      }).success
    ).toBe(false);
  });
}

function rejectsSensitivePublicationWorkflowData(): void {
  it('rejects workflow rows that allow raw support bundles or omit provider secrets from exclusions', () => {
    const uploadRow = requiredWorkflow('support-backend-upload-publication-handoff');

    expect(
      ProductionSupportPublicationWorkflowRowSchema.safeParse({
        ...uploadRow,
        supportSafeDataClasses: [...uploadRow.supportSafeDataClasses, 'raw-support-bundle'],
      }).success
    ).toBe(false);
    expect(
      ProductionSupportPublicationWorkflowRowSchema.safeParse({
        ...uploadRow,
        forbiddenDataClasses: uploadRow.forbiddenDataClasses.filter((dataClass) => dataClass !== 'provider-secrets'),
      }).success
    ).toBe(false);
  });
}

function rejectsIncompletePublicationWorkflowCoverage(): void {
  it('rejects proof that omits support publication handoff or remote support non-claims', () => {
    expect(
      ProductionSupportPublicationWorkflowProofSchema.safeParse({
        ...ProductionSupportPublicationWorkflowReadModel,
        rows: ProductionSupportPublicationWorkflowReadModel.rows.filter(
          (row) => row.item !== 'support-backend-upload-publication-handoff'
        ),
      }).success
    ).toBe(false);
    expect(
      ProductionSupportPublicationWorkflowProofSchema.safeParse({
        ...ProductionSupportPublicationWorkflowReadModel,
        nonClaims: ProductionSupportPublicationWorkflowReadModel.nonClaims.filter(
          (nonClaim) => nonClaim !== 'no-remote-support-session'
        ),
      }).success
    ).toBe(false);
  });
}

function requiredWorkflow(
  item:
    | 'public-privacy-policy-publication'
    | 'privacy-legal-disclosure-execution'
    | 'support-backend-upload-publication-handoff'
): (typeof ProductionSupportPublicationWorkflowReadModel.rows)[number] {
  const row = ProductionSupportPublicationWorkflowReadModel.rows.find((entry) => entry.item === item);
  if (row === undefined) {
    throw new Error(`missing publication workflow row: ${item}`);
  }
  return row;
}
