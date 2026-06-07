import { describe, expect, it } from 'vitest';
import { AgentEvent, AgentEventEnvelopeSchema, AgentProtocolDefaults } from '../src/contracts';
import {
  createScreenSettingsCommand,
  createScreenSettingsGetRequest,
  createScreenSettingsReplaceRequest,
  parseScreenSettingsUpdateEvent,
} from '../src/screen-settings-adapter';

describe('screen settings protocol adapter', () => {
  registerGetCommandTest();
  registerReplaceCommandTest();
  registerAcceptedEventTest();
  registerRejectedEventTest();
  registerWrongEventTest();
});

function registerGetCommandTest() {
  it('creates typed screen settings get commands without duplicating settings truth', () => {
    const command = createScreenSettingsCommand({
      messageId: 'cmd-screen-settings-1',
      sentAt: '2026-06-07T04:50:00Z',
      source: AgentProtocolDefaults.Peer.PortalDev,
      target: AgentProtocolDefaults.Target.LocalhostWindowsAgent,
      request: createScreenSettingsGetRequest('screen-settings-get-1'),
    });

    expect(command.command).toBe('agent.screen-settings.get');
    expect(command.payload[AgentProtocolDefaults.Field.ScreenSettingsUpdateKind]).toBe('get');
    expect(JSON.parse(String(command.payload[AgentProtocolDefaults.Field.ScreenSettingsRequest]))).toEqual({
      schemaVersion: 1,
      requestId: 'screen-settings-get-1',
      kind: 'get',
    });
  });
}

function registerReplaceCommandTest() {
  it('creates typed screen settings replace commands from activity-domain settings', () => {
    const command = createScreenSettingsCommand({
      messageId: 'cmd-screen-settings-2',
      sentAt: '2026-06-07T04:51:00Z',
      source: AgentProtocolDefaults.Peer.PortalDev,
      target: AgentProtocolDefaults.Target.LocalhostWindowsAgent,
      request: createScreenSettingsReplaceRequest({
        requestId: 'screen-settings-replace-1',
        baseSettingVersion: null,
        setting: strictDryRunSetting(2),
      }),
    });

    expect(command.command).toBe('agent.screen-settings.replace');
    const request = JSON.parse(String(command.payload[AgentProtocolDefaults.Field.ScreenSettingsRequest]));
    expect(request.kind).toBe('replace');
    expect(request.setting.retainRawImage).toBe(false);
    expect(request.setting.policyUseEnabled).toBe(true);
  });
}

function registerAcceptedEventTest() {
  it('parses accepted screen settings responses from protocol events', () => {
    const result = parseScreenSettingsUpdateEvent(
      eventEnvelope(AgentEvent.ScreenSettingsReplaceAccepted, {
        schemaVersion: 1,
        requestId: 'screen-settings-replace-1',
        kind: 'replace',
        status: 'accepted',
        setting: strictDryRunSetting(2),
        auditEventId: 'screen-setting-audit-1',
        rejectionReason: null,
        message: 'Screen settings update accepted.',
      })
    );

    expect(result).toEqual({
      ok: true,
      value: {
        schemaVersion: 1,
        requestId: 'screen-settings-replace-1',
        kind: 'replace',
        status: 'accepted',
        setting: strictDryRunSetting(2),
        auditEventId: 'screen-setting-audit-1',
        rejectionReason: null,
        message: 'Screen settings update accepted.',
      },
    });
  });
}

function registerRejectedEventTest() {
  it('parses rejected screen settings responses without losing typed custody reason', () => {
    const result = parseScreenSettingsUpdateEvent(
      eventEnvelope(AgentEvent.ScreenSettingsReplaceRejected, {
        schemaVersion: 1,
        requestId: 'screen-settings-replace-1',
        kind: 'replace',
        status: 'rejected',
        setting: null,
        auditEventId: null,
        rejectionReason: 'raw-retention-forbidden',
        message: 'Screen settings value is inconsistent.',
      })
    );

    expect(result).toEqual({
      ok: true,
      value: {
        schemaVersion: 1,
        requestId: 'screen-settings-replace-1',
        kind: 'replace',
        status: 'rejected',
        setting: null,
        auditEventId: null,
        rejectionReason: 'raw-retention-forbidden',
        message: 'Screen settings value is inconsistent.',
      },
    });
  });
}

function registerWrongEventTest() {
  it('rejects non-screen-settings protocol events', () => {
    const result = parseScreenSettingsUpdateEvent(
      eventEnvelope(AgentEvent.HealthReported, {
        online: true,
      })
    );

    expect(result).toEqual({
      ok: false,
      reason: 'wrong-event',
    });
  });
}

function eventEnvelope(eventName: string, response: unknown) {
  return AgentEventEnvelopeSchema.parse({
    schemaVersion: 1,
    eventId: 'evt-screen-settings-1',
    correlationId: 'cmd-screen-settings-1',
    sentAt: '2026-06-07T04:51:01Z',
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
            [AgentProtocolDefaults.Field.ScreenSettingsResponse]: JSON.stringify(response),
          },
    snapshot: null,
  });
}

function strictDryRunSetting(settingVersion: number) {
  return {
    schemaVersion: 1,
    screenAnalysisEnabled: true,
    analysisMode: 'policyDryRun',
    cadenceCaptureEnabled: true,
    cadenceSeconds: 60,
    strictModeEnabled: true,
    triggerCaptureEnabled: true,
    enabledTriggers: ['timedCadence', 'nativeAppForegroundStart'],
    allowedCaptureScope: 'activeWindow',
    ocrTextEnabled: true,
    ocrTextSnippetLimit: 8,
    redactionMode: 'localSensitiveText',
    ocrTextRetentionMode: 'redactedSnippets',
    credentialSuppressionEnabled: true,
    piiRedactionEnabled: true,
    temporaryImageTtlSeconds: 300,
    maxRetryCount: 2,
    deleteAfterSuccess: true,
    deleteAfterExpiry: true,
    retainRawImage: false,
    policyUseEnabled: true,
    changedByParentRef: 'screen-parent-local-settings',
    changedAt: '2026-06-07T04:51:00Z',
    settingVersion,
    reason: 'parent-enabled-strict-dry-run',
  };
}
