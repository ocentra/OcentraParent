import { describe, expect, it } from 'vitest';
import { AgentEvent, AgentProtocolDefaults, type AgentEventEnvelope } from '../src/contracts';
import { parseAgentBrowserAiUxReadModelEvent } from '../src/browser-ai-ux-read-model';
import { AgentProtocolSchemaVersion } from '../src/primitives';

const Source = {
  peerId: 'agent-service',
  role: 'agent-service',
} as const;

const Target = {
  peerId: 'portal-dev',
  role: 'portal',
} as const;

const BrowserAiUxReadModel = {
  schemaVersion: 1,
  generatedAt: '2026-06-04T03:12:00Z',
  custodyLabel: 'child-device-service-modeled',
  capabilityStatus: 'service-backed-manual-required',
  returned: 2,
  latestEventId: 'browser-ai-ux-row-checking',
  rows: [
    {
      schemaVersion: 1,
      rowId: 'browser-ai-ux-row-checking',
      sourceEvidenceIds: ['browser-evidence-youtube-video'],
      childSnapshotId: 'browser-child-ux-youtube-video',
      childState: 'checking',
      childPrimaryTextToken: 'browser.child.checking.title',
      childDeliveryState: 'checking-hold-rendered',
      adapterProofRef: 'managed-browser-checking-page-proof',
      parentExplanationId: 'browser-parent-explanation-youtube-video',
      parentExplanationState: 'preview',
      parentTitleTextToken: 'browser.parent.explanation.title',
      explanationAuditRefs: ['browser-parent-explanation-audit-youtube-video'],
      modelRuntimeVisible: true,
      policyRuleVisible: true,
      actionVisible: true,
      childExperienceVisible: true,
      degradedStateVisible: false,
      manualFallbackVisible: false,
      runtimeDeliveryClaimed: false,
      renderedUiClaimed: false,
      directEnforcementClaimed: false,
    },
    {
      schemaVersion: 1,
      rowId: 'browser-ai-ux-row-manual-required',
      sourceEvidenceIds: ['browser-evidence-generic-video'],
      childSnapshotId: 'browser-child-ux-generic-video',
      childState: 'manual_required',
      childPrimaryTextToken: 'browser.child.manual.title',
      childDeliveryState: 'portal-row-only',
      adapterProofRef: null,
      parentExplanationId: 'browser-parent-explanation-generic-video',
      parentExplanationState: 'manual_required',
      parentTitleTextToken: 'browser.parent.explanation.degraded',
      explanationAuditRefs: ['browser-parent-explanation-audit-generic-video'],
      modelRuntimeVisible: true,
      policyRuleVisible: true,
      actionVisible: true,
      childExperienceVisible: true,
      degradedStateVisible: true,
      manualFallbackVisible: true,
      runtimeDeliveryClaimed: false,
      renderedUiClaimed: false,
      directEnforcementClaimed: false,
    },
  ],
} as const;

describe('agent browser AI UX read-model parser', () => {
  it('parses the service-backed browser AI UX read-model event payload', () => {
    const parsed = parseAgentBrowserAiUxReadModelEvent(browserAiUxEvent(JSON.stringify(BrowserAiUxReadModel)));

    expect(parsed).toEqual({
      ok: true,
      value: BrowserAiUxReadModel,
    });
  });

  it('rejects wrong events and invalid payloads without inventing UX rows', () => {
    expect(
      parseAgentBrowserAiUxReadModelEvent({
        ...browserAiUxEvent(JSON.stringify(BrowserAiUxReadModel)),
        event: AgentEvent.HealthReported,
      })
    ).toEqual({
      ok: false,
      reason: 'wrong-event',
    });
    expect(parseAgentBrowserAiUxReadModelEvent(browserAiUxEvent('{'))).toEqual({
      ok: false,
      reason: 'invalid-json',
    });
    expect(
      parseAgentBrowserAiUxReadModelEvent(browserAiUxEvent(JSON.stringify({ ...BrowserAiUxReadModel, returned: 1 })))
    ).toEqual({
      ok: false,
      reason: 'invalid-payload',
    });
  });

  it('rejects runtime delivery, rendered UI, enforcement, and unproved rendered child-state claims', () => {
    expect(
      parseAgentBrowserAiUxReadModelEvent(
        browserAiUxEvent(
          JSON.stringify({
            ...BrowserAiUxReadModel,
            rows: [{ ...BrowserAiUxReadModel.rows[0], adapterProofRef: null }],
          })
        )
      )
    ).toEqual({
      ok: false,
      reason: 'invalid-payload',
    });
    expect(
      parseAgentBrowserAiUxReadModelEvent(
        browserAiUxEvent(
          JSON.stringify({
            ...BrowserAiUxReadModel,
            rows: [{ ...BrowserAiUxReadModel.rows[0], runtimeDeliveryClaimed: true, renderedUiClaimed: true }],
          })
        )
      )
    ).toEqual({
      ok: false,
      reason: 'invalid-payload',
    });
  });
});

function browserAiUxEvent(serializedReadModel: string): AgentEventEnvelope {
  return {
    schemaVersion: AgentProtocolSchemaVersion,
    eventId: 'browser-ai-ux-read-model-event',
    correlationId: 'browser-ai-ux-read-model-command',
    sentAt: '2026-06-04T03:12:01Z',
    source: Source,
    target: Target,
    event: AgentEvent.BrowserAiUxReadModelReported,
    severity: 'info',
    payload: {
      [AgentProtocolDefaults.Field.BrowserAiUxReadModel]: serializedReadModel,
    },
    snapshot: null,
  };
}
