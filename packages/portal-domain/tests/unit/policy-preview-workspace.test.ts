import { describe, expect, it } from 'vitest';
import { createPolicyWorkspacePreviewRows, type PolicyWorkspacePreviewRow } from '../../src/policy-preview-workspace';
import type { PortalPolicyPreviewReadModel } from '@ocentra-parent/schema-domain/agent-policy-preview-read-model';

describe('policy preview workspace rows', () => {
  it('keeps assistant drafts preview-only with visible confirmation and lifecycle boundaries', () => {
    const rows = createPolicyWorkspacePreviewRows(
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

    expect(rowValue(rows, 'Preview')).toContain('preview-only');
    expect(rowBody(rows, 'Preview')).toContain('https://example.test/learn');
    expect(rowBody(rows, 'Preview')).toContain('Proof tier not reported');
    expect(rowValue(rows, 'Approval')).toBe('Parent confirmation required');
    expect(rowBody(rows, 'Approval')).toContain('Assistant draft');
    expect(rowBody(rows, 'Approval')).toContain('Parent confirmation is required before any write.');
    expect(rowValue(rows, 'Lifecycle')).toBe('Preview');
    expect(rowBody(rows, 'Lifecycle')).toContain('AI preview');
    expect(rowBody(rows, 'Lifecycle')).toContain('Delivery and acknowledgment stay separate from active enforcement.');
    expect(rowValue(rows, 'Boundary')).toBe('Advisory only');
  });

  it('keeps unavailable preview state explicit instead of inventing control readiness', () => {
    const rows = createPolicyWorkspacePreviewRows(null);

    expect(rowValue(rows, 'Preview')).toBe('Policy preview read-model is unavailable.');
    expect(rowBody(rows, 'Preview')).toContain('Refresh the policy preview read model');
    expect(rowValue(rows, 'Approval')).toBe('Not reported');
    expect(rowValue(rows, 'Lifecycle')).toBe('Not reported');
    expect(rowValue(rows, 'Boundary')).toBe('Advisory only');
    expect(rowBody(rows, 'Boundary')).toContain('typed confirmation and adapter-owned handoff');
  });

  it('keeps delivered lifecycle state separate from active enforcement claims', () => {
    const rows = createPolicyWorkspacePreviewRows(
      readModel({
        policyPreviewSaveState: 'ready-to-save',
        policyPreviewManualReviewState: 'not-required',
        policyPreviewTargetState: 'supported',
        policySourceStatus: 'delivered',
        policySourceSurface: 'parent-portal',
        policyRequestOrigin: 'child',
        policyAssistantConfirmationState: 'parent-confirmed',
        policyRequestStatus: 'approved',
        networkEvidenceGrade: 'B',
      })
    );

    expect(rowValue(rows, 'Preview')).toContain('ready to save');
    expect(rowBody(rows, 'Preview')).toContain('Proof tier B');
    expect(rowValue(rows, 'Approval')).toBe('Parent confirmed');
    expect(rowValue(rows, 'Lifecycle')).toBe('Delivered');
    expect(rowBody(rows, 'Lifecycle')).toContain('Parent portal');
    expect(rowBody(rows, 'Lifecycle')).toContain('separate from active enforcement');
  });
});

function readModel(overrides: Partial<PortalPolicyPreviewReadModel>): PortalPolicyPreviewReadModel {
  return {
    schemaVersion: 1,
    generatedAt: '2026-06-18T09:00:00Z',
    custody: 'child-device-local',
    limit: 1,
    returned: 1,
    capabilityStatus: 'available',
    previewId: 'policy-preview-1',
    latestEventId: 'activity-browser-url-observed-1',
    latestObservedAt: '2026-06-18T08:59:59Z',
    targetId: 'browser-evidence-1',
    targetType: 'url',
    targetValue: 'https://example.test/learn',
    evidenceReferenceCount: 1,
    parentRuleContextReferenceCount: 1,
    parentRuleContextRefIds: 'parent-rule-context-1',
    decisionId: 'policy-decision-1',
    decisionAction: 'allow',
    reasonCodes: 'educational-domain',
    ruleIds: 'allow-learning-sites',
    localAiResultId: 'local-ai-result-1',
    dryRun: true,
    enforcementHandoffState: 'disabled-preview-only',
    policyPreviewTargetExplanationCode: 'target-supported',
    policyPreviewFindingKinds: 'policy-match',
    networkEvidenceGrade: null,
    networkRequestedPolicyAction: null,
    networkMappedPolicyAction: null,
    networkPolicyMappingMode: null,
    networkAdapterActionAuthorized: null,
    networkEnforcementCommandAuthorized: null,
    ...overrides,
  };
}

function rowValue(rows: readonly PolicyWorkspacePreviewRow[], label: string): string {
  return String(findRow(rows, label).value);
}

function rowBody(rows: readonly PolicyWorkspacePreviewRow[], label: string): string {
  return String(findRow(rows, label).body);
}

function findRow(rows: readonly PolicyWorkspacePreviewRow[], label: string): PolicyWorkspacePreviewRow {
  const row = rows.find((entry) => String(entry.label) === label);
  if (row === undefined) {
    throw new Error(`Missing row: ${label}`);
  }
  return row;
}
