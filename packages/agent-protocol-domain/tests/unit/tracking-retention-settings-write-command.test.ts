import { describe, expect, it } from 'vitest';
import { AgentEvent, AgentProtocolDefaults, type AgentEventEnvelope } from '../../src/contracts';
import { AgentProtocolSchemaVersion } from '@ocentra-parent/schema-domain/event-primitives';
import {
  AgentTrackingConfigCommandFlowEventType,
  AgentTrackingConfigUpdateRequestSchema,
  AgentTrackingConfigUpdateEventType,
  AgentTrackingConfigAckState,
  AgentTrackingAiBoundaryMode,
  AgentTrackingConfigAuditOutcomeLiteral,
  AgentTrackingConfigPolicyDecisionStateLiteral,
  AgentTrackingConfigPortalUpdateKindLiteral,
  AgentTrackingConfigUpdateResponseStateLiteral,
  AgentTrackingDeleteAfterAlertResolutionState,
  AgentTrackingDurableSettingsPersistenceState,
  AgentTrackingEffectiveStateLiteral,
  AgentTrackingExecutionClaimState,
  AgentTrackingParentExportState,
  AgentTrackingNotificationMode,
  AgentTrackingRemoteAiState,
  AgentTrackingRemoteSyncState,
  AgentTrackingRetentionSettingsWriteDefaults,
  AgentTrackingRetentionSettingsWriteRequestSchema,
  AgentTrackingRuntimeEnabledState,
  AgentTrackingRuntimeMode,
  ChildTrackingConfigUpdatedEventSchema,
  ParentTrackingConfigUpdatedEventSchema,
  TrackingConfigAuditEntryCommittedEventSchema,
  TrackingConfigChangeApprovedEventSchema,
  TrackingConfigChangeRejectedEventSchema,
  TrackingConfigChangeRequestedEventSchema,
  TrackingConfigPolicyDecisionCompletedEventSchema,
  TrackingConfigPolicyEvaluationRequestedEventSchema,
  TrackingConfigPortalReadModelUpdatedEventSchema,
  TrackingConfigUpdateAppliedEventSchema,
} from '@ocentra-parent/schema-domain/agent-tracking-retention-settings-write-command';
import {
  defaultAgentTrackingConfigUpdateRequest,
  defaultAgentTrackingRetentionSettingsWriteRequest,
  parseAgentTrackingRetentionSettingsWriteResultEvent,
  AgentTrackingRetentionSettingsWriteResultParseState,
} from '../../src/tracking-retention-settings-write-command';

const Source = {
  peerId: 'agent-service',
  role: 'agent-service',
} as const;

const Target = {
  peerId: 'portal-dev',
  role: 'portal',
} as const;

const TrackingRetentionSettingsWriteResult = {
  schemaVersion: AgentProtocolSchemaVersion,
  commandId: 'tracking-retention-write-command-1',
  settingsKind: AgentTrackingRetentionSettingsWriteDefaults.SettingsKindRetentionWindow,
  writeState: AgentTrackingRetentionSettingsWriteDefaults.WriteStateAccepted,
  acceptedAt: AgentTrackingRetentionSettingsWriteDefaults.AcceptedAt,
  sourceWriterIntentRefs: [AgentTrackingRetentionSettingsWriteDefaults.WriterIntentRef],
  sourceReadModelProofRefs: [AgentTrackingRetentionSettingsWriteDefaults.ReadModelProofRefs[0]],
  sourceMutationProofRefs: [AgentTrackingRetentionSettingsWriteDefaults.MutationProofRef],
  appliedRetentionWindowHours: 168,
  appliedDeleteAfterAlertResolutionState: AgentTrackingDeleteAfterAlertResolutionState.RetainAfterAlertResolved,
  parentExportState: AgentTrackingParentExportState.NotPrepared,
  remoteSyncState: AgentTrackingRemoteSyncState.Disabled,
  remoteAiState: AgentTrackingRemoteAiState.Disabled,
  localServiceStateRevision: 1,
  localServiceStateSnapshotRef: AgentTrackingRetentionSettingsWriteDefaults.LocalServiceStateSnapshotRef,
  durableSettingsStoreRef: AgentTrackingRetentionSettingsWriteDefaults.DurableSettingsStoreRef,
  durableSettingsPersistenceState: AgentTrackingDurableSettingsPersistenceState.Persisted,
  childConfigAckState: AgentTrackingConfigAckState.Received,
  commandTransportClaimState: AgentTrackingExecutionClaimState.Claimed,
  serviceWritePreflightClaimState: AgentTrackingExecutionClaimState.Claimed,
  serviceMutationExecutionState: AgentTrackingExecutionClaimState.Claimed,
  portalWritableUiClaimState: AgentTrackingExecutionClaimState.Unclaimed,
  platformRuntimeClaimState: AgentTrackingExecutionClaimState.Unclaimed,
  childDeviceDeliveryClaimState: AgentTrackingExecutionClaimState.Unclaimed,
  providerDeliveryClaimState: AgentTrackingExecutionClaimState.Unclaimed,
  notificationReceiptClaimState: AgentTrackingExecutionClaimState.Unclaimed,
  physicalDeviceClaimState: AgentTrackingExecutionClaimState.Unclaimed,
  authorityClaimState: AgentTrackingExecutionClaimState.Unclaimed,
  productClaimState: AgentTrackingExecutionClaimState.Unclaimed,
} as const;

describe('agent tracking retention settings write result parser', () => {
  it('parses local service write requests without remote sync or remote AI claims', () => {
    expect(
      AgentTrackingRetentionSettingsWriteRequestSchema.parse(defaultAgentTrackingRetentionSettingsWriteRequest())
    ).toEqual({
      schemaVersion: AgentProtocolSchemaVersion,
      commandId: AgentTrackingRetentionSettingsWriteDefaults.CommandId,
      settingsKind: AgentTrackingRetentionSettingsWriteDefaults.SettingsKindRetentionWindow,
      requestedRetentionWindowHours: 168,
      requestedDeleteAfterAlertResolutionState: AgentTrackingDeleteAfterAlertResolutionState.RetainAfterAlertResolved,
      requestedParentExportState: AgentTrackingParentExportState.NotPrepared,
      requestedRemoteSyncState: AgentTrackingRemoteSyncState.Disabled,
      requestedRemoteAiState: AgentTrackingRemoteAiState.Disabled,
      sourceWriterIntentRefs: [AgentTrackingRetentionSettingsWriteDefaults.WriterIntentRef],
      sourceReadModelProofRefs: AgentTrackingRetentionSettingsWriteDefaults.ReadModelProofRefs,
    });
  });

  it('parses parent and child tracking config update events as canonical protocol contracts', () => {
    const config = defaultAgentTrackingConfigUpdateRequest();
    const target = {
      scope: 'child-device',
      deviceId: 'local-dev-agent',
      platform: 'windows',
      route: 'localhost',
    } as const;
    const parentEvent = ParentTrackingConfigUpdatedEventSchema.parse({
      sourceCommandId: config.commandId,
      sourceMessageId: AgentTrackingRetentionSettingsWriteDefaults.CommandId,
      sourcePeerId: 'portal-dev',
      target,
      config,
    });

    expect(parentEvent.target.scope).toBe('child-device');
    expect(
      ChildTrackingConfigUpdatedEventSchema.parse({
        parentEventType: AgentTrackingConfigUpdateEventType.Parent,
        sourceCommandId: parentEvent.sourceCommandId,
        target: parentEvent.target,
        config: parentEvent.config,
      })
    ).toEqual({
      parentEventType: AgentTrackingConfigUpdateEventType.Parent,
      sourceCommandId: config.commandId,
      target,
      config,
    });
    expect(
      TrackingConfigUpdateAppliedEventSchema.parse({
        parentEventType: AgentTrackingConfigUpdateEventType.Parent,
        childEventType: AgentTrackingConfigUpdateEventType.Child,
        sourceCommandId: parentEvent.sourceCommandId,
        target: parentEvent.target,
        responseState: AgentTrackingConfigUpdateResponseStateLiteral.Applied,
        effectiveTrackingState: AgentTrackingEffectiveStateLiteral.Enabled,
        localServiceStateRevision: 1,
        durableSettingsPersistenceState:
          AgentTrackingDurableSettingsPersistenceState.Persisted,
      })
    ).toEqual({
      parentEventType: AgentTrackingConfigUpdateEventType.Parent,
      childEventType: AgentTrackingConfigUpdateEventType.Child,
      sourceCommandId: config.commandId,
      target,
      responseState: AgentTrackingConfigUpdateResponseStateLiteral.Applied,
      effectiveTrackingState: AgentTrackingEffectiveStateLiteral.Enabled,
      localServiceStateRevision: 1,
      durableSettingsPersistenceState:
        AgentTrackingDurableSettingsPersistenceState.Persisted,
    });
    expect(
      AgentTrackingConfigUpdateRequestSchema.parse({
        commandId: config.commandId,
        runtimeConfig: {
          trackingEnabledState: AgentTrackingRuntimeEnabledState.Enabled,
          trackingMode: AgentTrackingRuntimeMode.ObserveOnly,
          aiBoundaryMode: AgentTrackingAiBoundaryMode.RequestWhenUncertain,
          notificationMode: AgentTrackingNotificationMode.ParentPortalOnly,
        },
        retentionSettings: config.retentionSettings,
      })
    ).toEqual(config);
  });

  it('parses tracking config command-flow payload contracts with shared policy and audit event types', () => {
    const config = defaultAgentTrackingConfigUpdateRequest();
    const target = {
      scope: 'child-device',
      deviceId: 'local-dev-agent',
      platform: 'windows',
      route: 'localhost',
    } as const;
    const changeRequested = TrackingConfigChangeRequestedEventSchema.parse({
      changeRequestedEventRef: 'event.tracking-retention-settings-write.command-1.change-requested',
      previousEventRef: 'event.parent-controller.parent-action.received.1',
      sourceCommandId: config.commandId,
      sourceMessageId: AgentTrackingRetentionSettingsWriteDefaults.CommandId,
      sourcePeerId: 'portal-dev',
      target,
      config,
      requestedAt: AgentTrackingRetentionSettingsWriteDefaults.AcceptedAt,
    });
    const policyEvaluation = TrackingConfigPolicyEvaluationRequestedEventSchema.parse({
      policyEvaluationRef: 'event.tracking-retention-settings-write.command-1.policy-evaluation',
      previousEventRef: changeRequested.changeRequestedEventRef,
      sourceCommandId: config.commandId,
      target,
      parentRuleRefs: [
        'policy.rule.tracking.local-child-runtime',
        'policy.rule.tracking.remote-sync-disabled',
      ],
      dryRun: false,
    });
    const policyDecision = TrackingConfigPolicyDecisionCompletedEventSchema.parse({
      policyDecisionRef: 'event.tracking-retention-settings-write.command-1.policy-decision',
      previousEventRef: policyEvaluation.policyEvaluationRef,
      sourceCommandId: config.commandId,
      target,
      decisionState: AgentTrackingConfigPolicyDecisionStateLiteral.Approved,
      parentRuleRefs: policyEvaluation.parentRuleRefs,
      childRuntimePublishRequired: true,
    });
    const changeApproved = TrackingConfigChangeApprovedEventSchema.parse({
      changeApprovedEventRef: 'event.tracking-retention-settings-write.command-1.change-approved',
      previousEventRef: policyDecision.policyDecisionRef,
      sourceCommandId: config.commandId,
      target,
      approvedAt: AgentTrackingRetentionSettingsWriteDefaults.AcceptedAt,
      childRuntimePublishRequired: true,
    });
    const audit = TrackingConfigAuditEntryCommittedEventSchema.parse({
      auditEntryRef: 'event.tracking-retention-settings-write.command-1.audit-entry',
      previousEventRef: changeApproved.changeApprovedEventRef,
      sourceCommandId: config.commandId,
      policyDecisionRef: policyDecision.policyDecisionRef,
      target,
      auditOutcome: AgentTrackingConfigAuditOutcomeLiteral.Committed,
    });
    const readModelUpdated = TrackingConfigPortalReadModelUpdatedEventSchema.parse({
      readModelRef: 'event.tracking-retention-settings-write.command-1.portal-read-model',
      previousEventRef: audit.auditEntryRef,
      auditEntryRef: audit.auditEntryRef,
      sourceCommandId: config.commandId,
      target,
      updateKind: AgentTrackingConfigPortalUpdateKindLiteral.TrackingConfigState,
      visibleManualRequired: false,
      visibleUnavailable: false,
    });
    const changeRejected = TrackingConfigChangeRejectedEventSchema.parse({
      changeRejectedEventRef: 'event.tracking-retention-settings-write.command-1.change-rejected',
      previousEventRef: policyDecision.policyDecisionRef,
      sourceCommandId: config.commandId,
      target,
      rejectedAt: AgentTrackingRetentionSettingsWriteDefaults.AcceptedAt,
      rejectionReasonCode: 'invalid-tracking-config-request',
    });

    expect(AgentTrackingConfigCommandFlowEventType.ChangeRequested).toBe('tracking.config.change_requested');
    expect(policyDecision.decisionState).toBe(
      AgentTrackingConfigPolicyDecisionStateLiteral.Approved
    );
    expect(readModelUpdated.updateKind).toBe(
      AgentTrackingConfigPortalUpdateKindLiteral.TrackingConfigState
    );
    expect(changeRejected.rejectionReasonCode).toBe('invalid-tracking-config-request');
  });

  it('parses accepted service write command results without product overclaims', () => {
    const parsed = parseAgentTrackingRetentionSettingsWriteResultEvent(
      writeResultEvent(JSON.stringify(TrackingRetentionSettingsWriteResult))
    );

    expect(parsed).toEqual({
      parseState: AgentTrackingRetentionSettingsWriteResultParseState.Parsed,
      value: TrackingRetentionSettingsWriteResult,
    });
  });

  it('rejects wrong events and invalid write-result payloads', () => {
    expect(
      parseAgentTrackingRetentionSettingsWriteResultEvent({
        ...writeResultEvent(JSON.stringify(TrackingRetentionSettingsWriteResult)),
        event: AgentEvent.HealthReported,
      })
    ).toEqual({
      parseState: AgentTrackingRetentionSettingsWriteResultParseState.Failed,
      reason: 'wrong-event',
    });
    expect(parseAgentTrackingRetentionSettingsWriteResultEvent(writeResultEvent('{'))).toEqual({
      parseState: AgentTrackingRetentionSettingsWriteResultParseState.Failed,
      reason: 'invalid-json',
    });
    expect(
      parseAgentTrackingRetentionSettingsWriteResultEvent(
        writeResultEvent(
          JSON.stringify({
            ...TrackingRetentionSettingsWriteResult,
            productClaimState: AgentTrackingExecutionClaimState.Claimed,
          })
        )
      )
    ).toEqual({
      parseState: AgentTrackingRetentionSettingsWriteResultParseState.Failed,
      reason: 'invalid-payload',
    });
    expect(
      parseAgentTrackingRetentionSettingsWriteResultEvent(
        writeResultEvent(
          JSON.stringify({
            ...TrackingRetentionSettingsWriteResult,
            serviceMutationExecutionState: AgentTrackingExecutionClaimState.Unclaimed,
          })
        )
      )
    ).toEqual({
      parseState: AgentTrackingRetentionSettingsWriteResultParseState.Failed,
      reason: 'invalid-payload',
    });
    expect(
      parseAgentTrackingRetentionSettingsWriteResultEvent(
        writeResultEvent(
          JSON.stringify({
            ...TrackingRetentionSettingsWriteResult,
            durableSettingsPersistenceState: AgentTrackingDurableSettingsPersistenceState.NotPersisted,
          })
        )
      )
    ).toEqual({
      parseState: AgentTrackingRetentionSettingsWriteResultParseState.Failed,
      reason: 'invalid-payload',
    });
  });
});

function writeResultEvent(serializedResult: string): AgentEventEnvelope {
  return {
    schemaVersion: AgentProtocolSchemaVersion,
    eventId: 'tracking-retention-settings-write-result-event',
    correlationId: AgentTrackingRetentionSettingsWriteDefaults.CommandId,
    sentAt: '2026-06-06T19:50:01Z',
    source: Source,
    target: Target,
    event: AgentEvent.ActivityTrackingRetentionSettingsWriteReported,
    severity: 'info',
    payload: {
      [AgentProtocolDefaults.Field.ActivityTrackingRetentionSettingsWriteResult]: serializedResult,
    },
    snapshot: null,
  };
}
