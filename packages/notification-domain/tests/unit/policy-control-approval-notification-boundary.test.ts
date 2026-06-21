import { describe, expect, it } from 'vitest';
import {
  PolicyControlApprovalNotificationBoundaryEntrySchema,
  PolicyControlApprovalNotificationBoundarySample,
  PolicyControlApprovalNotificationBoundaryReadModelSchema,
} from '@ocentra-parent/schema-domain/policy-control-approval-notification-boundary';

describe('policy control approval notification boundary', () => {
  it('covers preview pending approved denied modified expired and replay-rejected approval queue states', () => {
    const readModel = PolicyControlApprovalNotificationBoundaryReadModelSchema.parse(
      PolicyControlApprovalNotificationBoundarySample
    );

    expect(readModel.readModelId).toBe('policy-control-approval-notification-boundary');
    expect(readModel.returned).toBe(7);
    expect(readModel.previewOnlyCount).toBe(1);
    expect(readModel.pendingParentReviewCount).toBe(1);
    expect(readModel.approvedCount).toBe(1);
    expect(readModel.deniedCount).toBe(1);
    expect(readModel.modifiedCount).toBe(1);
    expect(readModel.expiredRequestCount).toBe(1);
    expect(readModel.replayRejectedCount).toBe(1);
    expect(readModel.providerDeliveryClaimed).toBe(false);
    expect(readModel.policyMutationClaimed).toBe(false);
    expect(readModel.enforcementMutationClaimed).toBe(false);
    expect(readModel.assistantAutoApprovalClaimed).toBe(false);
  });

  it('keeps preview-only assistant drafts confirmation-gated and approvals override-backed', () => {
    const previewOnly = entryFor('policy-control-preview-only-assistant-draft');
    const approved = entryFor('policy-control-approved-bonus-time');
    const denied = entryFor('policy-control-denied-temporary-override');
    const replayRejected = entryFor('policy-control-replay-rejected');

    expect(previewOnly.origin).toBe('assistant-draft');
    expect(previewOnly.parentConfirmationRequired).toBe(true);
    expect(previewOnly.approvalRef).toBe(null);
    expect(previewOnly.overrideRef).toBe(null);

    expect(approved.parentReviewed).toBe(true);
    expect(approved.approvalRef).toBe('policy-approval-approved-ref');
    expect(approved.overrideRef).toBe('policy-override-approved-ref');
    expect(approved.overrideKind).toBe('bonus-time');

    expect(denied.parentReviewed).toBe(true);
    expect(denied.approvalRef).toBe('policy-approval-denied-ref');
    expect(denied.overrideRef).toBe(null);

    expect(replayRejected.portalQueueVisible).toBe(false);
    expect(replayRejected.approvalRef).toBe(null);
    expect(replayRejected.overrideRef).toBe(null);
  });

  it('rejects rows that fake parent review provider delivery or override creation', () => {
    const previewOnly = entryFor('policy-control-preview-only-assistant-draft');
    const approved = entryFor('policy-control-approved-bonus-time');
    const denied = entryFor('policy-control-denied-temporary-override');

    expect(
      PolicyControlApprovalNotificationBoundaryEntrySchema.safeParse({
        ...previewOnly,
        parentConfirmationRequired: false,
      }).success
    ).toBe(false);

    expect(
      PolicyControlApprovalNotificationBoundaryEntrySchema.safeParse({
        ...previewOnly,
        origin: 'child-request',
      }).success
    ).toBe(false);

    expect(
      PolicyControlApprovalNotificationBoundaryEntrySchema.safeParse({
        ...approved,
        overrideRef: null,
      }).success
    ).toBe(false);

    expect(
      PolicyControlApprovalNotificationBoundaryEntrySchema.safeParse({
        ...approved,
        providerDeliveryClaimed: true,
      }).success
    ).toBe(false);

    expect(
      PolicyControlApprovalNotificationBoundaryEntrySchema.safeParse({
        ...denied,
        overrideRef: 'unexpected-override-ref',
        overrideKind: 'temporary-block',
      }).success
    ).toBe(false);
  });
});

function entryFor(entryId: string) {
    const entry = PolicyControlApprovalNotificationBoundarySample.entries.find(
      (candidate: { entryId: string }) => candidate.entryId === entryId
    );
  if (entry === undefined) {
    throw new Error(`Missing policy control approval notification entry: ${entryId}`);
  }
  return entry;
}
