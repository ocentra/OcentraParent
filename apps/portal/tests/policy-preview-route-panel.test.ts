import { createElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import { describe, expect, it } from 'vitest';
import {
  AgentEvent,
  AgentEventEnvelopeSchema,
  AgentProtocolDefaults,
} from '@ocentra-parent/agent-protocol-domain/contracts';
import { PortalRoute } from '@ocentra-parent/portal-domain/contracts';
import { createPolicyPreviewPanelIntent } from '@ocentra-parent/portal-domain/policy-preview-panel';
import { PolicyPreviewRoutePanel, shouldRenderPolicyPreviewRoute } from '../src/PolicyPreviewRoutePanel';
import { resolveLiveActivityState } from '../src/live-activity-state';

describe('policy preview portal route panel', () => {
  it('attaches only to the policy authoring routes', () => {
    expect(shouldRenderPolicyPreviewRoute(PortalRoute.RuleManagement)).toBe(true);
    expect(shouldRenderPolicyPreviewRoute(PortalRoute.Schedules)).toBe(true);
    expect(shouldRenderPolicyPreviewRoute(PortalRoute.Approvals)).toBe(true);
    expect(shouldRenderPolicyPreviewRoute(PortalRoute.Enforcement)).toBe(true);

    expect(shouldRenderPolicyPreviewRoute(PortalRoute.Overview)).toBe(false);
    expect(shouldRenderPolicyPreviewRoute(PortalRoute.AppGameSessions)).toBe(false);
    expect(shouldRenderPolicyPreviewRoute(PortalRoute.AiRuntime)).toBe(false);
  });

  it('uses the latest policy preview event and read model for the shared panel intent', () => {
    const liveActivity = resolveLiveActivityState([
      policyPreviewEvent({
        eventId: 'evt-policy-preview-stale',
        sentAt: '2026-06-12T10:00:00Z',
        previewId: 'policy-preview-stale',
        targetValue: 'https://example.test/stale',
        sourceStatus: 'preview',
        requestStatus: 'preview-only',
      }),
      policyPreviewEvent({
        eventId: 'evt-policy-preview-latest',
        sentAt: '2026-06-12T10:05:00Z',
        previewId: 'policy-preview-latest',
        targetValue: 'https://example.test/latest',
        sourceStatus: 'confirmed',
        requestStatus: 'approved',
      }),
    ]);

    expect(liveActivity.policyPreviewEvent?.eventId).toBe('evt-policy-preview-latest');
    expect(liveActivity.policyPreviewReadModel).toMatchObject({
      previewId: 'policy-preview-latest',
      targetValue: 'https://example.test/latest',
      policySourceStatus: 'confirmed',
      policyRequestStatus: 'approved',
    });

    const intent = createPolicyPreviewPanelIntent(liveActivity.policyPreviewEvent, liveActivity.policyPreviewReadModel);

    expect(intent.summaryDetails).toEqual(
      expect.arrayContaining([
        expect.objectContaining({ label: 'Policy check', value: 'policy-preview-latest' }),
        expect.objectContaining({ label: 'Parent rule context references', value: '1' }),
      ])
    );
    expect(intent.cards[0]?.details).toEqual(
      expect.arrayContaining([
        expect.objectContaining({ label: 'Target value', value: 'https://example.test/latest' }),
        expect.objectContaining({ label: 'Request status', value: 'Approved' }),
      ])
    );
    expect(intent.cards[1]?.details).toEqual(
      expect.arrayContaining([expect.objectContaining({ label: 'Source status', value: 'Confirmed' })])
    );

    const markup = renderToStaticMarkup(
      createElement(PolicyPreviewRoutePanel, {
        actions: policyPreviewActions(),
        commandEnabled: true,
        liveActivity,
      })
    );

    expect(markup).toContain('Policy preview parent authoring');
    expect(markup).toContain('Refresh policy decision');
    expect(markup).toContain('https://example.test/latest');
  });
});

function policyPreviewEvent({
  eventId,
  previewId,
  requestStatus,
  sentAt,
  sourceStatus,
  targetValue,
}: {
  readonly eventId: string;
  readonly previewId: string;
  readonly requestStatus: 'approved' | 'preview-only';
  readonly sentAt: string;
  readonly sourceStatus: 'confirmed' | 'preview';
  readonly targetValue: string;
}) {
  return AgentEventEnvelopeSchema.parse({
    schemaVersion: 1,
    eventId,
    correlationId: `cmd-${eventId}`,
    sentAt,
    source: {
      peerId: 'local-dev-agent',
      role: 'agent-service',
    },
    target: {
      peerId: 'portal-dev',
      role: 'portal',
    },
    event: AgentEvent.PolicyPreviewReadModelReported,
    severity: 'info',
    payload: {
      schemaVersion: 1,
      generatedAt: sentAt,
      custody: 'child-device-local',
      limit: 1,
      returned: 1,
      capabilityStatus: 'available',
      policyPreviewId: previewId,
      latestEventId: `${eventId}-source`,
      latestObservedAt: sentAt,
      targetId: 'browser-evidence-1',
      targetType: 'url',
      targetValue,
      evidenceReferenceCount: 1,
      parentRuleContextReferenceCount: 1,
      parentRuleContextRefIds: 'parent-rule-context-1',
      policyDecisionId: `${previewId}-decision`,
      policyAction: 'allow',
      reasonCodes: 'educational-domain',
      ruleIds: 'allow-learning-sites',
      localAiResultId: 'local-ai-result-1',
      dryRun: true,
      enforcementHandoffState: 'disabled-preview-only',
      [AgentProtocolDefaults.Field.PolicyPreviewSaveState]: 'ready-to-save',
      [AgentProtocolDefaults.Field.PolicyPreviewManualReviewState]: 'not-required',
      [AgentProtocolDefaults.Field.PolicyPreviewTargetState]: 'supported',
      [AgentProtocolDefaults.Field.PolicyPreviewTargetExplanationCode]: 'target-supported',
      [AgentProtocolDefaults.Field.PolicyPreviewFindingKinds]: 'policy-match',
      [AgentProtocolDefaults.Field.PolicySourceStatus]: sourceStatus,
      [AgentProtocolDefaults.Field.PolicySourceSurface]: 'parent-portal',
      [AgentProtocolDefaults.Field.PolicyRequestOrigin]: 'child',
      [AgentProtocolDefaults.Field.PolicyAssistantConfirmationState]: 'not-required',
      [AgentProtocolDefaults.Field.PolicyRequestStatus]: requestStatus,
      networkEvidenceGrade: null,
      networkRequestedPolicyAction: null,
      networkMappedPolicyAction: null,
      networkPolicyMappingMode: null,
      networkAdapterActionAuthorized: null,
      networkEnforcementCommandAuthorized: null,
    },
    snapshot: null,
  });
}

function policyPreviewActions() {
  return {
    reconnect() {},
    selectCommandResult() {},
    sendCommand() {},
  };
}
