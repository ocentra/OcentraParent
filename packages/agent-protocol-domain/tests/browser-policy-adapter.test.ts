import { describe, expect, it } from 'vitest';
import { createBrowserPolicyCommand, parseBrowserPolicyUpdateEvent } from '../src/browser-policy-adapter';
import { AgentEvent, AgentEventEnvelopeSchema, AgentProtocolDefaults } from '../src/contracts';

describe('browser policy protocol adapter', () => {
  it('creates typed browser policy patch commands from policy request contracts', () => {
    const command = createBrowserPolicyCommand({
      messageId: 'cmd-browser-policy-1',
      sentAt: '2026-05-28T17:25:00Z',
      source: AgentProtocolDefaults.Peer.PortalDev,
      target: AgentProtocolDefaults.Target.LocalhostWindowsAgent,
      request: patchRequest(),
    });

    expect(command.command).toBe('agent.browser-policy.patch');
    expect(command.payload[AgentProtocolDefaults.Field.BrowserPolicyUpdateKind]).toBe('patch');
    expect(JSON.parse(String(command.payload[AgentProtocolDefaults.Field.BrowserPolicyRequest]))).toMatchObject({
      kind: 'patch',
      policyId: 'browser-policy-child-1',
    });
  });

  it('parses accepted browser policy responses from protocol events', () => {
    const result = parseBrowserPolicyUpdateEvent(
      eventEnvelope(AgentEvent.BrowserPolicyPatchAccepted, {
        schemaVersion: 'v0.6',
        requestId: 'browser-control-request-1',
        kind: 'patch',
        status: 'accepted',
        policy: null,
        effectivePolicy: null,
        capabilityRegistry: null,
        rejectionReason: null,
        auditEventId: 'browser-policy-audit-1',
        message: 'Browser policy patch accepted.',
      })
    );

    expect(result).toMatchObject({
      ok: true,
      value: {
        status: 'accepted',
        auditEventId: 'browser-policy-audit-1',
      },
    });
  });
});

describe('browser policy protocol adapter rejection paths', () => {
  it('parses rejected browser policy responses without losing typed reason', () => {
    const result = parseBrowserPolicyUpdateEvent(
      eventEnvelope(AgentEvent.BrowserPolicyPatchRejected, {
        schemaVersion: 'v0.6',
        requestId: 'browser-control-request-1',
        kind: 'patch',
        status: 'rejected',
        policy: null,
        effectivePolicy: null,
        capabilityRegistry: null,
        rejectionReason: 'missing-managed-proof-or-fallback',
        auditEventId: null,
        message: 'Exact URL policy requires managed browser proof or fallback.',
      })
    );

    expect(result).toMatchObject({
      ok: true,
      value: {
        status: 'rejected',
        rejectionReason: 'missing-managed-proof-or-fallback',
      },
    });
  });

  it('rejects non-browser-policy protocol events', () => {
    const result = parseBrowserPolicyUpdateEvent(
      eventEnvelope(AgentEvent.HealthReported, {
        online: true,
      })
    );

    expect(result).toEqual({
      ok: false,
      reason: 'wrong-event',
    });
  });
});

function patchRequest() {
  return {
    schemaVersion: 'v0.6',
    requestId: 'browser-control-request-1',
    kind: 'patch',
    policyId: 'browser-policy-child-1',
    baseRevisionId: 'browser-policy-revision-1',
    patches: [
      {
        op: 'replace',
        fieldId: 'browser.enabled',
        writesTo: '/browserPolicy/enabled',
        value: true,
      },
    ],
  };
}

function eventEnvelope(eventName: string, response: unknown) {
  return AgentEventEnvelopeSchema.parse({
    schemaVersion: 1,
    eventId: 'evt-browser-policy-1',
    correlationId: 'cmd-browser-policy-1',
    sentAt: '2026-05-28T17:25:01Z',
    source: {
      peerId: 'local-dev-agent',
      role: 'agent-service',
    },
    target: AgentProtocolDefaults.Peer.PortalDev,
    event: eventName,
    severity: 'info',
    payload:
      eventName === AgentEvent.HealthReported
        ? response
        : {
            [AgentProtocolDefaults.Field.BrowserPolicyResponse]: JSON.stringify(response),
          },
    snapshot: null,
  });
}
