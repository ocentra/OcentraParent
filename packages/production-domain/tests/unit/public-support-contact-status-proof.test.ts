import { describe, expect, it } from 'vitest';
import {
  PublicSupportContactStatusProofSchema,
  PublicSupportContactStatusRowSchema,
  summarizePublicSupportContactStatusRows,
} from '../../src/public-support-contact-status-proof';
import { PublicSupportContactStatusReadModel } from '../../src/public-support-contact-status-read-model';

describe('public support contact status proof', () => {
  acceptsPublicSupportContactStatusRows();
  rejectsPublicRuntimeAndSupportExecutionOverclaims();
  rejectsSensitiveSupportContactData();
  rejectsIncompletePublicSupportContactCoverage();
});

function acceptsPublicSupportContactStatusRows(): void {
  it('accepts public support contact status rows without runtime or provider contact claims', () => {
    const proof = PublicSupportContactStatusProofSchema.parse(PublicSupportContactStatusReadModel);

    expect(summarizePublicSupportContactStatusRows(proof.rows)).toEqual({
      'public-support-contact': 1,
      'support-status-page-contact': 1,
      'support-runbook-contact': 1,
      'incident-status-contact': 1,
      'backend-upload-support-contact': 1,
      'billing-support-contact': 1,
    });
    expect(proof.publicRuntimeExecutionClaim).toBe('not-implemented');
    expect(proof.supportBackendUploadExecutionClaim).toBe('manual-required');
    expect(proof.accountLookupExecutionClaim).toBe('manual-required');
    expect(proof.billingProviderContactClaim).toBe('manual-required');
    expect(proof.remoteSupportSessionClaim).toBe('not-implemented');
    expect(proof.productionSlaClaim).toBe('not-implemented');
    expect(proof.legalDisclosureExecutionClaim).toBe('manual-required');
    expect(proof.childActivityCustodyClaim).toBe('not-implemented');
    expect(proof.rows.map((row) => [row.surface, row.contactStatusBoundaryState])).toEqual([
      ['public-support-contact', 'backend-required'],
      ['support-status-page-contact', 'manual-required'],
      ['support-runbook-contact', 'backend-required'],
      ['incident-status-contact', 'backend-required'],
      ['backend-upload-support-contact', 'backend-required'],
      ['billing-support-contact', 'backend-required'],
    ]);
    expect(proof.rows.map((row) => row.statusBoundaryReference)).toEqual([
      'public-support-contact-status-boundary-public-support-contact',
      'public-support-contact-status-boundary-support-status-page-contact',
      'public-support-contact-status-boundary-support-runbook-contact',
      'public-support-contact-status-boundary-incident-status-contact',
      'public-support-contact-status-boundary-backend-upload-support-contact',
      'public-support-contact-status-boundary-billing-support-contact',
    ]);
    expect(proof.nonClaims).toEqual([
      'no-public-runtime-execution',
      'no-support-backend-upload-execution',
      'no-account-lookup-execution',
      'no-billing-provider-contact',
      'no-remote-support-session',
      'no-production-sla',
      'no-child-activity-custody',
      'no-provider-secrets',
      'no-legal-disclosure-execution',
    ]);
  });
}

function rejectsPublicRuntimeAndSupportExecutionOverclaims(): void {
  it('rejects implemented public routes, executed contact, and support backend upload execution', () => {
    const publicContact = requiredContact('public-support-contact');
    const uploadContact = requiredContact('backend-upload-support-contact');

    expect(
      PublicSupportContactStatusRowSchema.safeParse({
        ...publicContact,
        publicRouteState: 'implemented',
      }).success
    ).toBe(false);
    expect(
      PublicSupportContactStatusRowSchema.safeParse({
        ...publicContact,
        publicRuntimeState: 'implemented',
      }).success
    ).toBe(false);
    expect(
      PublicSupportContactStatusRowSchema.safeParse({
        ...publicContact,
        contactExecutionState: 'executed',
      }).success
    ).toBe(false);
    expect(
      PublicSupportContactStatusRowSchema.safeParse({
        ...publicContact,
        contactStatusBoundaryState: 'implemented',
      }).success
    ).toBe(false);
    expect(
      PublicSupportContactStatusRowSchema.safeParse({
        ...uploadContact,
        supportBackendUploadState: 'executed',
      }).success
    ).toBe(false);
  });
}

function rejectsSensitiveSupportContactData(): void {
  it('rejects contact rows that allow raw support bundles or omit provider secret exclusions', () => {
    const supportStatus = requiredContact('support-status-page-contact');

    expect(
      PublicSupportContactStatusRowSchema.safeParse({
        ...supportStatus,
        supportSafeDataClasses: [...supportStatus.supportSafeDataClasses, 'raw-support-bundle'],
      }).success
    ).toBe(false);
    expect(
      PublicSupportContactStatusRowSchema.safeParse({
        ...supportStatus,
        forbiddenDataClasses: supportStatus.forbiddenDataClasses.filter(
          (dataClass) => dataClass !== 'provider-secrets'
        ),
      }).success
    ).toBe(false);
  });
}

function rejectsIncompletePublicSupportContactCoverage(): void {
  it('rejects proof that omits billing support contact or provider-secret non-claims', () => {
    expect(
      PublicSupportContactStatusProofSchema.safeParse({
        ...PublicSupportContactStatusReadModel,
        rows: PublicSupportContactStatusReadModel.rows.filter((row) => row.surface !== 'billing-support-contact'),
      }).success
    ).toBe(false);
    expect(
      PublicSupportContactStatusProofSchema.safeParse({
        ...PublicSupportContactStatusReadModel,
        nonClaims: PublicSupportContactStatusReadModel.nonClaims.filter(
          (nonClaim) => nonClaim !== 'no-provider-secrets'
        ),
      }).success
    ).toBe(false);
  });
}

function requiredContact(
  surface: 'public-support-contact' | 'support-status-page-contact' | 'backend-upload-support-contact'
): (typeof PublicSupportContactStatusReadModel.rows)[number] {
  const row = PublicSupportContactStatusReadModel.rows.find((entry) => entry.surface === surface);
  if (row === undefined) {
    throw new Error(`missing public support contact status row: ${surface}`);
  }
  return row;
}
