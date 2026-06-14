import { describe, expect, it } from 'vitest';
import {
  createPolicyPreviewPanelIntent,
  type PolicyPreviewPanelReadModel,
} from '../../src/policy-preview-panel';

describe('policy preview panel intent', () => {
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
      })
    );

    const summaryDetails = detailMap(intent.summaryDetails);
    const previewDetails = detailMap(intent.cards[0]?.details ?? []);
    const sourceDetails = detailMap(intent.cards[1]?.details ?? []);

    expect(String(intent.summary)).toContain('preview-only');
    expect(summaryDetails.get('Privacy mode')).toBe('Local only');
    expect(summaryDetails.get('Adapter boundary')).toBe('Local adapter not connected');
    expect(summaryDetails.get('Execution state')).toBe('Off');
    expect(summaryDetails.get('Provider source')).toBe('Unavailable');
    expect(previewDetails.get('Request origin')).toBe('Assistant draft');
    expect(previewDetails.get('Assistant confirmation')).toBe('Parent confirmation required');
    expect(previewDetails.get('Request status')).toBe('Preview only');
    expect(sourceDetails.get('Source status')).toBe('Preview');
  });

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
      })
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
      })
    );

    const previewDetails = detailMap(intent.cards[0]?.details ?? []);

    expect(String(intent.summary)).toContain('not ready to save');
    expect(previewDetails.get('Target state')).toBe('Unsupported');
    expect(previewDetails.get('Finding kinds')).toBe('unsupported-target');
  });
});

function readModel(
  overrides: Partial<PolicyPreviewPanelReadModel>
): PolicyPreviewPanelReadModel {
  return {
    returned: 1,
    previewId: 'policy-preview-1',
    targetType: 'url',
    targetValue: 'https://example.test/learn',
    decisionAction: 'allow',
    parentRuleContextReferenceCount: 1,
    parentRuleContextRefIds: 'parent-rule-context-1',
    dryRun: true,
    ...overrides,
  };
}

function detailMap(
  details: readonly { readonly label: unknown; readonly value: unknown }[]
): Map<string, string> {
  return new Map(details.map((detail) => [String(detail.label), String(detail.value)]));
}
