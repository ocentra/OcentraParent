import { describe, expect, it } from 'vitest';
import { AgentEvent, AgentProtocolDefaults, type AgentEventEnvelope } from '../src/contracts';
import { AgentProtocolSchemaVersion } from '../src/primitives';
import {
  createTrackingRetentionSettingsMutationPayload,
  parseTrackingRetentionSettingsMutationEvent,
  trackingRetentionSettingsMutationCommandName,
} from '../src/tracking-retention-settings-service-mutation';

const Source = {
  peerId: 'agent-service',
  role: 'agent-service',
} as const;

const Target = {
  peerId: 'portal-dev',
  role: 'portal',
} as const;

const MutationResult = {
  requestId: 'tracking-retention-settings-service-mutation-request',
  mutationId: 'tracking-retention-settings-service-mutation-request-result',
  intentId: 'tracking-retention-settings-write-retention-window',
  settingsKind: 'retention-window-setting',
  writeAction: 'set-retention-window',
  requestedValue: '168',
  mutationState: 'accepted',
  rejectionReason: null,
  serviceMutationExecuted: true,
  durablePersistenceClaimed: false,
  portalUiClaimed: false,
  platformRuntimeClaimed: false,
  childDeviceDeliveryClaimed: false,
  providerDeliveryClaimed: false,
  notificationReceiptClaimed: false,
  physicalDeviceClaimed: false,
  authorityClaimed: false,
  productClaimReady: false,
  evidenceReferenceIds: ['tracking-retention-writer-evidence-window'],
  sourceReadModelProofRefs: [
    'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/24-retention-settings-read-model-proof.json',
  ],
  writerBoundaryProofRefs: [
    'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/25-retention-settings-writer-boundary-proof.json',
  ],
  auditRefs: ['tracking-retention-service-mutation-audit-window'],
} as const;

describe('tracking retention settings service mutation protocol', () => {
  it('creates the mutation command payload with the dedicated protocol field', () => {
    const payload = createTrackingRetentionSettingsMutationPayload({
      requestId: MutationResult.requestId,
      intentId: MutationResult.intentId,
      settingsKind: MutationResult.settingsKind,
      writeAction: MutationResult.writeAction,
      requestedValue: MutationResult.requestedValue,
      evidenceReferenceIds: MutationResult.evidenceReferenceIds,
      sourceReadModelProofRefs: MutationResult.sourceReadModelProofRefs,
      writerBoundaryProofRefs: MutationResult.writerBoundaryProofRefs,
      auditRefs: MutationResult.auditRefs,
    });

    expect(trackingRetentionSettingsMutationCommandName()).toBe('agent.activity.tracking.retention-settings.mutate');
    expect(Object.keys(payload)).toEqual([AgentProtocolDefaults.Field.ActivityTrackingRetentionSettingsMutation]);
    expect(JSON.parse(payload[AgentProtocolDefaults.Field.ActivityTrackingRetentionSettingsMutation])).toEqual({
      requestId: MutationResult.requestId,
      intentId: MutationResult.intentId,
      settingsKind: MutationResult.settingsKind,
      writeAction: MutationResult.writeAction,
      requestedValue: MutationResult.requestedValue,
      evidenceReferenceIds: MutationResult.evidenceReferenceIds,
      sourceReadModelProofRefs: MutationResult.sourceReadModelProofRefs,
      writerBoundaryProofRefs: MutationResult.writerBoundaryProofRefs,
      auditRefs: MutationResult.auditRefs,
    });
  });

  it('parses accepted service mutation results without product overclaims', () => {
    expect(parseTrackingRetentionSettingsMutationEvent(mutationEvent(JSON.stringify(MutationResult)))).toEqual({
      ok: true,
      value: MutationResult,
    });
  });

  it('rejects wrong events and invalid payloads', () => {
    expect(
      parseTrackingRetentionSettingsMutationEvent({
        ...mutationEvent(JSON.stringify(MutationResult)),
        event: AgentEvent.HealthReported,
      })
    ).toEqual({
      ok: false,
      reason: 'wrong-event',
    });
    expect(parseTrackingRetentionSettingsMutationEvent(mutationEvent('{'))).toEqual({
      ok: false,
      reason: 'invalid-json',
    });
    expect(
      parseTrackingRetentionSettingsMutationEvent(
        mutationEvent(JSON.stringify({ ...MutationResult, productClaimReady: true }))
      )
    ).toEqual({
      ok: false,
      reason: 'invalid-payload',
    });
  });
});

function mutationEvent(serializedMutation: string): AgentEventEnvelope {
  return {
    schemaVersion: AgentProtocolSchemaVersion,
    eventId: 'tracking-retention-settings-service-mutation-event',
    correlationId: 'tracking-retention-settings-service-mutation-command',
    sentAt: '2026-06-06T15:30:01Z',
    source: Source,
    target: Target,
    event: AgentEvent.ActivityTrackingRetentionSettingsMutationReported,
    severity: 'info',
    payload: {
      [AgentProtocolDefaults.Field.ActivityTrackingRetentionSettingsMutation]: serializedMutation,
    },
    snapshot: null,
  };
}
