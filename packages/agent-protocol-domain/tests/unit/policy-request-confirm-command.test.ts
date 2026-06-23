import { expect, it } from 'vitest';
import { AgentCommand, AgentEvent } from '@ocentra-parent/schema-domain/agent-command-event-contracts';
import { AgentProtocolDefaults } from '@ocentra-parent/schema-domain/agent-protocol-defaults';
import {
  PolicyRequestAssistantPreviewConfirmClaimState,
  PolicyRequestAssistantPreviewConfirmResultState,
  createPolicyRequestAssistantPreviewConfirmCommand,
  defaultPolicyRequestAssistantPreviewConfirmRequest,
  parsePolicyRequestAssistantPreviewConfirmResultEvent,
  type PolicyRequestAssistantPreviewConfirmResult,
} from '../../src/policy-request-confirm-command';

function sampleResult(): PolicyRequestAssistantPreviewConfirmResult {
  const request = defaultPolicyRequestAssistantPreviewConfirmRequest();
  return {
    schemaVersion: AgentProtocolDefaults.SchemaVersion,
    commandId: request.commandId,
    requestId: request.requestId,
    assistantPreviewId: request.assistantPreviewId,
    resultState: PolicyRequestAssistantPreviewConfirmResultState.Confirmed,
    policyRequestStatus: AgentProtocolDefaults.PolicyPreview.RequestStatus.PendingParentReview,
    policyAssistantConfirmationState: AgentProtocolDefaults.PolicyPreview.AssistantConfirmationState.ParentConfirmed,
    policyAuditReferenceId: request.confirmationAuditReferenceId,
    confirmedAt: request.confirmedAt,
    rejectionReason: null,
    commandTransportClaimState: PolicyRequestAssistantPreviewConfirmClaimState.Claimed,
    serviceValidationClaimState: PolicyRequestAssistantPreviewConfirmClaimState.Claimed,
    activityStoreMutationClaimState: PolicyRequestAssistantPreviewConfirmClaimState.Unclaimed,
    upstreamWriterClaimState: PolicyRequestAssistantPreviewConfirmClaimState.Unclaimed,
    readModelProjectionClaimState: PolicyRequestAssistantPreviewConfirmClaimState.Unclaimed,
    portalWritableUiClaimState: PolicyRequestAssistantPreviewConfirmClaimState.Unclaimed,
    childDeviceDeliveryClaimState: PolicyRequestAssistantPreviewConfirmClaimState.Unclaimed,
    providerDeliveryClaimState: PolicyRequestAssistantPreviewConfirmClaimState.Unclaimed,
    platformEnforcementClaimState: PolicyRequestAssistantPreviewConfirmClaimState.Unclaimed,
    productClaimState: PolicyRequestAssistantPreviewConfirmClaimState.Unclaimed,
  };
}

it('createPolicyRequestAssistantPreviewConfirmCommand: builds the typed confirmation command payload', () => {
  const request = defaultPolicyRequestAssistantPreviewConfirmRequest();
  const command = createPolicyRequestAssistantPreviewConfirmCommand({
    messageId: 'cmd-policy-confirm-1',
    sentAt: '2026-06-18T00:10:00Z',
    source: AgentProtocolDefaults.Peer.PortalDev,
    target: AgentProtocolDefaults.Target.LocalhostWindowsAgent,
    request,
  });

  expect(command.command).toBe(AgentCommand.PolicyRequestAssistantPreviewConfirm);
  expect(command.payload[AgentProtocolDefaults.Field.PolicyRequestAssistantPreviewConfirmRequest]).toBe(
    JSON.stringify(request)
  );
});

it('parsePolicyRequestAssistantPreviewConfirmResultEvent: decodes the typed confirmation result', () => {
  const result = sampleResult();
  const parsed = parsePolicyRequestAssistantPreviewConfirmResultEvent({
    schemaVersion: AgentProtocolDefaults.SchemaVersion,
    eventId: 'evt-policy-confirm-1',
    correlationId: 'cmd-policy-confirm-1',
    sentAt: '2026-06-18T00:10:01Z',
    source: {
      peerId: 'local-dev-agent',
      role: 'agent-service',
    },
    target: AgentProtocolDefaults.Peer.PortalDev,
    event: AgentEvent.PolicyRequestAssistantPreviewConfirmReported,
    severity: 'info',
    payload: {
      [AgentProtocolDefaults.Field.PolicyRequestAssistantPreviewConfirmResult]: JSON.stringify(result),
    },
    snapshot: null,
  });

  expect(parsed).toEqual({
    ok: true,
    value: result,
  });
});

it('parsePolicyRequestAssistantPreviewConfirmResultEvent: rejects other events', () => {
  const parsed = parsePolicyRequestAssistantPreviewConfirmResultEvent({
    schemaVersion: AgentProtocolDefaults.SchemaVersion,
    eventId: 'evt-policy-confirm-2',
    correlationId: 'cmd-policy-confirm-2',
    sentAt: '2026-06-18T00:10:01Z',
    source: {
      peerId: 'local-dev-agent',
      role: 'agent-service',
    },
    target: AgentProtocolDefaults.Peer.PortalDev,
    event: AgentEvent.PolicyPreviewReadModelReported,
    severity: 'info',
    payload: {},
    snapshot: null,
  });

  expect(parsed).toEqual({
    ok: false,
    reason: 'wrong-event',
  });
});
