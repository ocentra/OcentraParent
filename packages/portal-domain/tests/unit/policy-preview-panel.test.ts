import { describe, expect, it } from 'vitest';
import { createPolicyPreviewPanelIntent, type PolicyPreviewPanelReadModel } from '../../src/policy-preview-panel';

describe('policy preview panel intent', () => {
  registerPreviewOnlyPanelTests();
  registerBlockedTargetPanelTests();
  registerReadOnlyAccessPanelTests();
  registerReplayRejectionPanelTests();
});

function registerPreviewOnlyPanelTests(): void {
  it('keeps assistant drafts preview-only until parent confirmation', () => {
    const intent = createPolicyPreviewPanelIntent(
      { payload: {} },
      readModel({
        policyPreviewSaveState: 'preview-required',
        policyPreviewManualReviewState: 'not-required',
        policyPreviewTargetState: 'supported',
        policySourceStatus: 'preview',
        policySourceSurface: 'ai-preview',
        policyRequestOrigin: 'assistant-draft',
        policyAssistantConfirmationState: 'parent-confirmation-required',
        policyRequestStatus: 'preview-only',
      }),
      'active-controller'
    );

    const summaryDetails = detailMap(intent.summaryDetails);
    const previewDetails = detailMap(intent.cards[0]?.details ?? []);
    const sourceDetails = detailMap(intent.cards[1]?.details ?? []);
    const accessDetails = detailMap(intent.cards[2]?.details ?? []);

    expect(String(intent.summary)).toContain('preview-only');
    expect(summaryDetails.get('Parent access')).toBe('Active controller');
    expect(summaryDetails.get('Privacy mode')).toBe('Local only');
    expect(summaryDetails.get('Adapter boundary')).toBe('Local adapter not connected');
    expect(summaryDetails.get('Execution state')).toBe('Off');
    expect(summaryDetails.get('Provider source')).toBe('Unavailable');
    expect(previewDetails.get('Request origin')).toBe('Assistant draft');
    expect(previewDetails.get('Assistant confirmation')).toBe('Parent confirmation required');
    expect(previewDetails.get('Request status')).toBe('Preview only');
    expect(sourceDetails.get('Source status')).toBe('Preview');
    expect(String(intent.cards[2]?.summary)).toContain('parent confirmation');
    expect(accessDetails.get('Write authority')).toBe('Parent confirmation is required before any write.');
  });
}

function registerBlockedTargetPanelTests(): void {
  it('keeps blocked and manual preview states visibly not ready', () => {
    const intent = createPolicyPreviewPanelIntent(
      { payload: {} },
      readModel({
        policyPreviewSaveState: 'blocked',
        policyPreviewManualReviewState: 'required',
        policyPreviewTargetState: 'offline',
        policyPreviewTargetExplanationCode: 'conflict-offline-target',
        policyPreviewFindingKinds: 'conflict,manual-review',
        policySourceStatus: null,
        policyRequestStatus: 'pending-parent-review',
      }),
      'proof-missing'
    );

    const previewDetails = detailMap(intent.cards[0]?.details ?? []);

    expect(String(intent.summary)).toContain('blocked');
    expect(previewDetails.get('Save state')).toBe('Blocked');
    expect(previewDetails.get('Manual review')).toBe('Required');
    expect(previewDetails.get('Target state')).toBe('Offline');
    expect(previewDetails.get('Finding kinds')).toBe('conflict,manual-review');
    expect(String(intent.cards[1]?.summary)).toBe('No source lifecycle has been reported.');
  });

  it('keeps unsupported targets visibly not ready instead of implying success', () => {
    const intent = createPolicyPreviewPanelIntent(
      { payload: {} },
      readModel({
        policyPreviewSaveState: 'preview-required',
        policyPreviewManualReviewState: 'not-required',
        policyPreviewTargetState: 'unsupported',
        policyPreviewTargetExplanationCode: 'unsupported-browser',
        policyPreviewFindingKinds: 'unsupported-target',
        policyRequestStatus: 'preview-only',
      }),
      'proof-missing'
    );

    const previewDetails = detailMap(intent.cards[0]?.details ?? []);

    expect(String(intent.summary)).toContain('not ready to save');
    expect(previewDetails.get('Target state')).toBe('Unsupported');
    expect(previewDetails.get('Finding kinds')).toBe('unsupported-target');
  });
}

function registerReadOnlyAccessPanelTests(): void {
  it('keeps observer-only roles visibly read-only even when preview data exists', () => {
    const intent = createPolicyPreviewPanelIntent(
      { payload: {} },
      readModel({
        policyPreviewSaveState: 'ready-to-save',
        policyPreviewManualReviewState: 'not-required',
        policyPreviewTargetState: 'supported',
        policySourceStatus: 'delivered',
        policySourceSurface: 'parent-portal',
        policyAssistantConfirmationState: 'parent-confirmed',
        policyRequestStatus: 'approved',
        policyApprovalId: 'approval-2',
        policyOverrideId: 'override-2',
        policyReviewedByActorId: 'parent-2',
        policyReviewedByActorRole: 'parent',
        policyReviewedAt: '2026-06-18T10:00:00.000Z',
        policyAuditReferenceId: 'audit-2',
      }),
      'observer-only'
    );

    const accessDetails = detailMap(intent.cards[2]?.details ?? []);

    expect(String(intent.cards[1]?.summary)).toContain('Delivered is reported');
    expect(String(intent.cards[2]?.summary)).toContain('cannot confirm or save writes');
    expect(accessDetails.get('Parent access')).toBe('Observer only');
    expect(accessDetails.get('Approval ID')).toBe('approval-2');
    expect(accessDetails.get('Override ID')).toBe('override-2');
    expect(accessDetails.get('Reviewed by')).toBe('parent-2 (parent)');
    expect(accessDetails.get('Audit reference')).toBe('audit-2');
    expect(accessDetails.get('Write authority')).toBe(
      'Observer scope is read-only and cannot confirm or save policy writes.'
    );
  });
}

function registerReplayRejectionPanelTests(): void {
  it('surfaces replay-rejected approval attempts without inventing a new override', () => {
    const intent = createPolicyPreviewPanelIntent(
      { payload: {} },
      readModel({
        policyPreviewSaveState: 'blocked',
        policyPreviewManualReviewState: 'required',
        policyPreviewTargetState: 'supported',
        policySourceStatus: 'rejected',
        policySourceSurface: 'parent-portal',
        policyRequestOrigin: 'child',
        policyAssistantConfirmationState: 'not-required',
        policyRequestStatus: 'replay-rejected',
        policyApprovalId: 'approval-3',
        policyReplayOfApprovalId: 'approval-1',
      }),
      'active-controller'
    );

    const accessDetails = detailMap(intent.cards[2]?.details ?? []);

    expect(String(intent.cards[2]?.summary)).toContain('rejected as a replay');
    expect(accessDetails.get('Request status')).toBe('Replay rejected');
    expect(accessDetails.get('Approval ID')).toBe('approval-3');
    expect(accessDetails.get('Replay of approval')).toBe('approval-1');
    expect(accessDetails.get('Override ID')).toBe('Not reported');
  });
}

function readModel(overrides: Partial<PolicyPreviewPanelReadModel>): PolicyPreviewPanelReadModel {
  return {
    returned: 1,
    previewId: 'policy-preview-1',
    targetType: 'url',
    targetValue: 'https://example.test/learn',
    decisionAction: 'allow',
    parentRuleContextReferenceCount: 1,
    parentRuleContextRefIds: 'parent-rule-context-1',
    dryRun: true,
    policyApprovalId: null,
    policyOverrideId: null,
    policyReplayOfApprovalId: null,
    policyReviewedByActorId: null,
    policyReviewedByActorRole: null,
    policyReviewedAt: null,
    policyAuditReferenceId: null,
    ...overrides,
  };
}

function detailMap(details: readonly { readonly label: unknown; readonly value: unknown }[]): Map<string, string> {
  return new Map(details.map((detail) => [String(detail.label), String(detail.value)]));
}
