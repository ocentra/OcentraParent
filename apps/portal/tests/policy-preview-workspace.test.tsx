import { createElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import { describe, expect, it } from 'vitest';
import { AgentEvent, AgentEventEnvelopeSchema } from '@ocentra-parent/schema-domain/agent-command-event-contracts';
import { AgentProtocolDefaults } from '@ocentra-parent/schema-domain/agent-protocol-defaults';
import { parentPortalRouteContext } from '@ocentra-parent/portal-domain/parent-portal-data';
import { PortalRoute } from '@ocentra-parent/schema-domain/portal-contracts';
import { resolveParentPortalServiceState } from '@ocentra-parent/portal-domain/parent-portal-service-state';
import { ParentPortalSvgSurface } from '../../../vendor/ocentra-parent-core-ui/AppPages/ParentPortal/ParentPortalSvgSurface';
import { DEFAULT_PARENT_PORTAL_SVG_CONTROLS } from '../../../vendor/ocentra-parent-core-ui/AppPages/ParentPortal/ParentPortalSvgSurfaceControls';
import { resolveLiveActivityState } from '../src/live-activity-state';

describe('policy preview workspace banner', () => {
  it('does not render the preview banner inside the main policy workspace', () => {
    const liveActivity = resolveLiveActivityState([policyPreviewEvent()]);
    const routeContext = parentPortalRouteContext(PortalRoute.RuleManagement);
    const serviceState = resolveParentPortalServiceState({
      connectionState: 'connected',
      events: [policyPreviewEvent()],
    });

    const html = renderToStaticMarkup(
      createElement(ParentPortalSvgSurface, {
        pageMode: routeContext.pageMode,
        parentPortalRows: serviceState.parentPortalRows,
        userEntry: serviceState.userEntry,
        content: serviceState.content,
        controls: DEFAULT_PARENT_PORTAL_SVG_CONTROLS,
        initialNavLabel: routeContext.navLabel,
        initialSelectedControlId: routeContext.selectedControlId,
        activityState: liveActivity,
        onRefreshParentPortal() {},
        onMatchmaking() {},
      })
    );

    expect(html).not.toContain('Policy preview workspace banner');
    expect(html).not.toContain('https://example.test/learn');
    expect(html).not.toContain('Parent confirmation required');
    expect(html).toContain('RULES');
  });
});

function policyPreviewEvent() {
  return AgentEventEnvelopeSchema.parse({
    schemaVersion: 1,
    eventId: 'evt-policy-preview',
    correlationId: 'cmd-policy-preview',
    sentAt: '2026-06-18T09:00:02Z',
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
      generatedAt: '2026-06-18T09:00:02Z',
      custody: 'child-device-local',
      limit: 1,
      returned: 1,
      capabilityStatus: 'available',
      policyPreviewId: 'policy-preview-1',
      latestEventId: 'activity-browser-url-observed-1',
      latestObservedAt: '2026-06-18T09:00:00Z',
      targetId: 'browser-evidence-1',
      targetType: 'url',
      targetValue: 'https://example.test/learn',
      evidenceReferenceCount: 1,
      parentRuleContextReferenceCount: 1,
      parentRuleContextRefIds: 'parent-rule-context-1',
      policyDecisionId: 'policy-decision-1',
      policyAction: 'allow',
      reasonCodes: 'educational-domain',
      ruleIds: 'allow-learning-sites',
      localAiResultId: 'local-ai-result-1',
      dryRun: true,
      enforcementHandoffState: 'disabled-preview-only',
      [AgentProtocolDefaults.Field.PolicyPreviewSaveState]: 'preview-required',
      [AgentProtocolDefaults.Field.PolicyPreviewManualReviewState]: 'not-required',
      [AgentProtocolDefaults.Field.PolicyPreviewTargetState]: 'supported',
      [AgentProtocolDefaults.Field.PolicyPreviewTargetExplanationCode]: 'target-supported',
      [AgentProtocolDefaults.Field.PolicyPreviewFindingKinds]: 'policy-match',
      [AgentProtocolDefaults.Field.PolicySourceStatus]: 'preview',
      [AgentProtocolDefaults.Field.PolicySourceSurface]: 'ai-preview',
      [AgentProtocolDefaults.Field.PolicyRequestOrigin]: 'assistant-draft',
      [AgentProtocolDefaults.Field.PolicyAssistantConfirmationState]: 'parent-confirmation-required',
      [AgentProtocolDefaults.Field.PolicyRequestStatus]: 'preview-only',
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
