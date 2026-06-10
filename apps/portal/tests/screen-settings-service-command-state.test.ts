import { describe, expect, it } from 'vitest';
import {
  screenEvidenceSettingsWritableUiProof,
  type ScreenEvidenceSettingsUiProof,
} from '@ocentra-parent/activity-domain/screen-evidence';
import {
  AgentEvent,
  AgentEventEnvelopeSchema,
  AgentProtocolDefaults,
  type AgentEventEnvelope,
} from '@ocentra-parent/agent-protocol-domain/contracts';
import {
  createScreenSettingsGetCommandDraft,
  createScreenSettingsReplaceCommandDraft,
  latestScreenSettingsServiceResponse,
  screenSettingsBaseVersionForReplace,
  screenSettingsServiceStatusText,
} from '../src/screen-settings-service-command-state';

describe('screen settings service command state', () => {
  it('builds typed get and replace command payloads for portal actions', () => {
    const proof = screenEvidenceSettingsWritableUiProof();
    const setting = requireStrictSetting(proof);
    const getDraft = createScreenSettingsGetCommandDraft(3);
    const replaceDraft = createScreenSettingsReplaceCommandDraft({
      baseSettingVersion: setting.settingVersion,
      sequence: 4,
      setting,
    });

    expect(getDraft.command).toBe('agent.screen-settings.get');
    expect(getDraft.requestId).toBe('screen-settings-request-3');
    expect(getDraft.payload[AgentProtocolDefaults.Field.ScreenSettingsUpdateKind]).toBe('get');
    expect(replaceDraft.command).toBe('agent.screen-settings.replace');
    expect(replaceDraft.requestId).toBe('screen-settings-request-4');
    expect(JSON.parse(String(replaceDraft.payload[AgentProtocolDefaults.Field.ScreenSettingsRequest]))).toEqual({
      schemaVersion: 1,
      requestId: 'screen-settings-request-4',
      kind: 'replace',
      baseSettingVersion: 3,
      setting,
    });
  });

  it('selects matching service responses and derives visible status text', () => {
    const proof = screenEvidenceSettingsWritableUiProof();
    const setting = requireStrictSetting(proof);
    const response = latestScreenSettingsServiceResponse(
      [
        screenSettingsEvent('screen-settings-request-other', setting),
        screenSettingsEvent('screen-settings-request-9', setting),
      ],
      'screen-settings-request-9'
    );

    expect(response).toEqual({
      schemaVersion: 1,
      requestId: 'screen-settings-request-9',
      kind: 'replace',
      status: 'accepted',
      setting,
      auditEventId: 'screen-settings-audit-screen-settings-request-9',
      rejectionReason: null,
      message: 'Screen settings update accepted.',
    });
    expect(
      screenSettingsServiceStatusText({
        commandEnabled: true,
        pendingRequestId: 'screen-settings-request-9',
        proof,
        response,
      })
    ).toBe(proof.serviceAcceptedStatus);
    expect(screenSettingsBaseVersionForReplace(response)).toBe(3);
  });
});

function requireStrictSetting(proof: ScreenEvidenceSettingsUiProof) {
  const setting = proof.intents[2]?.setting;
  if (setting === undefined) {
    throw new Error('missing strict screen setting');
  }
  return setting;
}

function screenSettingsEvent(requestId: string, setting: unknown): AgentEventEnvelope {
  return AgentEventEnvelopeSchema.parse({
    schemaVersion: 1,
    eventId: `evt-${requestId}`,
    correlationId: `cmd-${requestId}`,
    sentAt: '2026-06-07T05:30:00Z',
    source: {
      peerId: 'local-dev-agent',
      role: 'agent-service',
    },
    target: AgentProtocolDefaults.Peer.PortalDev,
    event: AgentEvent.ScreenSettingsReplaceAccepted,
    severity: 'info',
    payload: {
      [AgentProtocolDefaults.Field.ScreenSettingsResponse]: JSON.stringify({
        schemaVersion: 1,
        requestId,
        kind: 'replace',
        status: 'accepted',
        setting,
        auditEventId: `screen-settings-audit-${requestId}`,
        rejectionReason: null,
        message: 'Screen settings update accepted.',
      }),
    },
    snapshot: null,
  });
}
