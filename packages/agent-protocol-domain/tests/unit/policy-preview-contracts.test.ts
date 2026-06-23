import { describe, expect, it } from 'vitest';
import { AgentProtocolDefaults } from '@ocentra-parent/schema-domain/agent-protocol-defaults';
import { parsePolicyPreviewReadModel } from '../../src/policy-preview-read-model';

describe('policy preview parser defaults', () => {
  it('accepts the new parent authoring preview fields', assertParentAuthoringPreviewFields);
  it('accepts null WP02 preview-state fields and rejects invalid enums', assertNullablePreviewFieldsAndInvalidEnums);
  it('accepts replay-rejected request status with replay lineage fields', assertReplayRejectedRequestStatus);
  it('accepts app and device target types for confirmed preview rows', assertConfirmedPreviewTargetTypes);
});

function assertParentAuthoringPreviewFields() {
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
    policyApprovalId: 'approval-1',
    policyOverrideId: 'override-1',
    policyReplayOfApprovalId: null,
    policyReviewedByActorId: 'parent-1',
    policyReviewedByActorRole: 'parent',
    policyReviewedAt: '2026-06-18T10:00:00.000Z',
    policyAuditReferenceId: 'audit-1',
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
    policyApprovalId: 'approval-1',
    policyOverrideId: 'override-1',
    policyReplayOfApprovalId: null,
    policyReviewedByActorId: 'parent-1',
    policyReviewedByActorRole: 'parent',
    policyReviewedAt: '2026-06-18T10:00:00.000Z',
    policyAuditReferenceId: 'audit-1',
  });
}

function assertNullablePreviewFieldsAndInvalidEnums() {
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
    policyApprovalId: null,
    policyOverrideId: null,
    policyReplayOfApprovalId: null,
    policyReviewedByActorId: null,
    policyReviewedByActorRole: null,
    policyReviewedAt: null,
    policyAuditReferenceId: null,
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
    policyApprovalId: null,
    policyOverrideId: null,
    policyReplayOfApprovalId: null,
    policyReviewedByActorId: null,
    policyReviewedByActorRole: null,
    policyReviewedAt: null,
    policyAuditReferenceId: null,
  });
  expect(invalid).toBeNull();
}

function assertReplayRejectedRequestStatus() {
  const parsed = parsePolicyPreviewReadModel({
    returned: 1,
    policyRequestStatus: 'replay-rejected',
    policyApprovalId: 'approval-3',
    policyReplayOfApprovalId: 'approval-1',
    networkAdapterActionAuthorized: false,
    networkEnforcementCommandAuthorized: false,
  });

  expect(parsed).not.toBeNull();
  expect(parsed).toMatchObject({
    policyRequestStatus: 'replay-rejected',
    policyApprovalId: 'approval-3',
    policyReplayOfApprovalId: 'approval-1',
  });
}

function assertConfirmedPreviewTargetTypes() {
  const appTarget = parsePolicyPreviewReadModel({
    returned: 1,
    targetType: AgentProtocolDefaults.PolicyPreview.TargetType.App,
    targetValue: 'discord.exe',
    networkAdapterActionAuthorized: false,
    networkEnforcementCommandAuthorized: false,
  });
  const deviceTarget = parsePolicyPreviewReadModel({
    returned: 1,
    targetType: AgentProtocolDefaults.PolicyPreview.TargetType.Device,
    targetValue: 'child-phone-1',
    networkAdapterActionAuthorized: false,
    networkEnforcementCommandAuthorized: false,
  });

  expect(appTarget).not.toBeNull();
  expect(appTarget).toMatchObject({
    targetType: 'app',
    targetValue: 'discord.exe',
  });
  expect(deviceTarget).not.toBeNull();
  expect(deviceTarget).toMatchObject({
    targetType: 'device',
    targetValue: 'child-phone-1',
  });
}
