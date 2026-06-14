import { describe, expect, it } from 'vitest';
import { parsePolicyPreviewReadModel } from '../../src/policy-preview-read-model';

describe('policy preview parser defaults', () => {
  it('accepts the new parent authoring preview fields', () => {
    const parsed = parsePolicyPreviewReadModel({
      returned: 1,
      policyPreviewSaveState: 'preview-required',
      policyPreviewManualReviewState: 'not-required',
      policyPreviewTargetState: 'supported',
      policyPreviewTargetExplanationCode: 'assistant-preview',
      policyPreviewFindingKinds: 'assistant-draft',
      policySourceStatus: 'preview',
      policySourceSurface: 'ai-preview',
      policyRequestOrigin: 'assistant-draft',
      policyAssistantConfirmationState: 'parent-confirmation-required',
      policyRequestStatus: 'preview-only',
      dryRun: true,
      enforcementHandoffState: 'disabled-preview-only',
      networkAdapterActionAuthorized: false,
      networkEnforcementCommandAuthorized: false,
    });

    expect(parsed).not.toBeNull();
    expect(parsed).toMatchObject({
      policyPreviewSaveState: 'preview-required',
      policyPreviewManualReviewState: 'not-required',
      policyPreviewTargetState: 'supported',
      policyPreviewTargetExplanationCode: 'assistant-preview',
      policyPreviewFindingKinds: 'assistant-draft',
      policySourceStatus: 'preview',
      policySourceSurface: 'ai-preview',
      policyRequestOrigin: 'assistant-draft',
      policyAssistantConfirmationState: 'parent-confirmation-required',
      policyRequestStatus: 'preview-only',
    });
  });

  it('accepts null WP02 preview-state fields and rejects invalid enums', () => {
    const nullable = parsePolicyPreviewReadModel({
      returned: 1,
      policyPreviewSaveState: null,
      policyPreviewManualReviewState: null,
      policyPreviewTargetState: null,
      policyPreviewTargetExplanationCode: null,
      policyPreviewFindingKinds: null,
      policySourceStatus: null,
      policySourceSurface: null,
      policyRequestOrigin: null,
      policyAssistantConfirmationState: null,
      policyRequestStatus: null,
      networkAdapterActionAuthorized: false,
      networkEnforcementCommandAuthorized: false,
    });
    const invalid = parsePolicyPreviewReadModel({
      returned: 1,
      policyPreviewSaveState: 'green-lit',
      networkAdapterActionAuthorized: false,
      networkEnforcementCommandAuthorized: false,
    });

    expect(nullable).not.toBeNull();
    expect(nullable).toMatchObject({
      policyPreviewSaveState: null,
      policyPreviewManualReviewState: null,
      policyPreviewTargetState: null,
      policyPreviewTargetExplanationCode: null,
      policyPreviewFindingKinds: null,
      policySourceStatus: null,
      policySourceSurface: null,
      policyRequestOrigin: null,
      policyAssistantConfirmationState: null,
      policyRequestStatus: null,
    });
    expect(invalid).toBeNull();
  });
});
