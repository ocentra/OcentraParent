import { AgentProtocolSchemaVersion, type AgentPeerRole, type AgentRoute } from '@ocentra-parent/schema-domain/event-primitives';
import { AgentPolicyPreviewDefaults } from '@ocentra-parent/schema-domain/agent-policy-preview-read-model';
import * as PolicyRequestContracts from '@ocentra-parent/schema-domain/agent-policy-request-confirm-command';
import {
  AgentCommand,
  AgentCommandEnvelopeSchema,
  AgentEvent,
  type AgentCommandEnvelope,
  type AgentEventEnvelope,
  type AgentProtocolLogFields,
} from './contracts';

export type PolicyRequestAssistantPreviewConfirmAdapterFailureReason =
  | 'wrong-event'
  | 'missing-json-field'
  | 'invalid-json'
  | 'invalid-payload';

export type PolicyRequestAssistantPreviewConfirmResult =
  PolicyRequestContracts.PolicyRequestAssistantPreviewConfirmResult;
export const PolicyRequestAssistantPreviewConfirmResultState =
  PolicyRequestContracts.PolicyRequestAssistantPreviewConfirmResultState;
export const PolicyRequestAssistantPreviewConfirmClaimState =
  PolicyRequestContracts.PolicyRequestAssistantPreviewConfirmClaimState;

export type PolicyRequestAssistantPreviewConfirmAdapterResult =
  | {
      readonly ok: true;
      readonly value: PolicyRequestContracts.PolicyRequestAssistantPreviewConfirmResult;
    }
  | {
      readonly ok: false;
      readonly reason: PolicyRequestAssistantPreviewConfirmAdapterFailureReason;
    };

export type PolicyRequestAssistantPreviewConfirmCommandPeerInput = {
  readonly peerId: string;
  readonly role: AgentPeerRole;
};

export type PolicyRequestAssistantPreviewConfirmCommandTargetInput = {
  readonly deviceId: string;
  readonly platform: string;
  readonly route: AgentRoute;
};

export type CreatePolicyRequestAssistantPreviewConfirmCommandInput = {
  readonly messageId: string;
  readonly sentAt: string;
  readonly source: PolicyRequestAssistantPreviewConfirmCommandPeerInput;
  readonly target: PolicyRequestAssistantPreviewConfirmCommandTargetInput;
  readonly request: PolicyRequestContracts.PolicyRequestAssistantPreviewConfirmRequest;
};

const PolicyRequestAssistantPreviewConfirmDefaults = {
  CommandId: 'policy-request-assistant-preview-confirm-command',
  RequestId: 'policy-request-1',
  SubmissionKey: 'policy-request-submission-1',
  HouseholdId: 'family-local',
  ChildProfileId: 'child-profile-1',
  DeviceId: 'local-dev-agent',
  SourceDocumentId: 'policy-document-1',
  PolicyVersion: 1,
  TargetReferenceId: 'example.test',
  RuleId: 'browser-rule-1',
  RequestedAt: '2026-06-18T00:00:00Z',
  ExpiresAt: '2026-06-18T01:00:00Z',
  AssistantPreviewId: 'assistant-preview-1',
  AuditReferenceId: 'audit.policy-request.preview',
  ConfirmationActorId: 'parent-1',
  ConfirmationAuditReferenceId: 'audit.policy-request.confirmed',
  ConfirmedAt: '2026-06-18T00:05:00Z',
} as const;

export function defaultPolicyRequestAssistantPreviewConfirmRequest():
  PolicyRequestContracts.PolicyRequestAssistantPreviewConfirmRequest {
  return PolicyRequestContracts.PolicyRequestAssistantPreviewConfirmRequestSchema.parse({
    schemaVersion: AgentProtocolSchemaVersion,
    commandId: PolicyRequestAssistantPreviewConfirmDefaults.CommandId,
    requestId: PolicyRequestAssistantPreviewConfirmDefaults.RequestId,
    submissionKey: PolicyRequestAssistantPreviewConfirmDefaults.SubmissionKey,
    householdId: PolicyRequestAssistantPreviewConfirmDefaults.HouseholdId,
    childProfileId: PolicyRequestAssistantPreviewConfirmDefaults.ChildProfileId,
    deviceId: PolicyRequestAssistantPreviewConfirmDefaults.DeviceId,
    sourceDocumentId: PolicyRequestAssistantPreviewConfirmDefaults.SourceDocumentId,
    policyVersion: PolicyRequestAssistantPreviewConfirmDefaults.PolicyVersion,
    requestKind: 'ask-parent',
    targetKind: 'site',
    targetReferenceId: PolicyRequestAssistantPreviewConfirmDefaults.TargetReferenceId,
    requestedAction: 'ask-parent',
    ruleId: PolicyRequestAssistantPreviewConfirmDefaults.RuleId,
    requestedBonusMinutes: null,
    requestedAt: PolicyRequestAssistantPreviewConfirmDefaults.RequestedAt,
    expiresAt: PolicyRequestAssistantPreviewConfirmDefaults.ExpiresAt,
    origin: AgentPolicyPreviewDefaults.RequestOrigin.AssistantDraft,
    assistantPreviewId: PolicyRequestAssistantPreviewConfirmDefaults.AssistantPreviewId,
    assistantConfirmationState: AgentPolicyPreviewDefaults.AssistantConfirmationState.ParentConfirmationRequired,
    requestStatus: AgentPolicyPreviewDefaults.RequestStatus.PreviewOnly,
    auditReferenceIds: [PolicyRequestAssistantPreviewConfirmDefaults.AuditReferenceId],
    confirmationActorId: PolicyRequestAssistantPreviewConfirmDefaults.ConfirmationActorId,
    confirmationActorRole: 'parent',
    confirmationActorState: 'active',
    confirmationAuditReferenceId:
      PolicyRequestAssistantPreviewConfirmDefaults.ConfirmationAuditReferenceId,
    confirmedAt: PolicyRequestAssistantPreviewConfirmDefaults.ConfirmedAt,
  });
}

export function createPolicyRequestAssistantPreviewConfirmCommand(
  input: CreatePolicyRequestAssistantPreviewConfirmCommandInput
): AgentCommandEnvelope {
  const parsedRequest = PolicyRequestContracts.PolicyRequestAssistantPreviewConfirmRequestSchema.safeParse(
    input.request
  );
  if (!parsedRequest.success) {
    throw new Error('invalid policy request assistant preview confirm request');
  }

  return AgentCommandEnvelopeSchema.parse({
    schemaVersion: AgentProtocolSchemaVersion,
    messageId: input.messageId,
    sentAt: input.sentAt,
    source: input.source,
    target: input.target,
    command: AgentCommand.PolicyRequestAssistantPreviewConfirm,
    payload: createPolicyRequestAssistantPreviewConfirmPayload(parsedRequest.data),
  });
}

export function createPolicyRequestAssistantPreviewConfirmPayload(
  request: PolicyRequestContracts.PolicyRequestAssistantPreviewConfirmRequest
): AgentProtocolLogFields {
  const parsedRequest = PolicyRequestContracts.PolicyRequestAssistantPreviewConfirmRequestSchema.safeParse(request);
  if (!parsedRequest.success) {
    throw new Error('invalid policy request assistant preview confirm request');
  }

  return {
    [PolicyRequestContracts.PolicyRequestAssistantPreviewConfirmPayloadField.Request]:
      JSON.stringify(parsedRequest.data),
  };
}

export function parsePolicyRequestAssistantPreviewConfirmResultEvent(
  event: AgentEventEnvelope
): PolicyRequestAssistantPreviewConfirmAdapterResult {
  if (event.event !== AgentEvent.PolicyRequestAssistantPreviewConfirmReported) {
    return adapterFailure('wrong-event');
  }

  const raw = event.payload[PolicyRequestContracts.PolicyRequestAssistantPreviewConfirmPayloadField.Result];
  if (typeof raw !== 'string') {
    return adapterFailure('missing-json-field');
  }

  let decoded: unknown;
  try {
    decoded = JSON.parse(raw);
  } catch {
    return adapterFailure('invalid-json');
  }

  const parsed = PolicyRequestContracts.PolicyRequestAssistantPreviewConfirmResultSchema.safeParse(decoded);
  if (!parsed.success) {
    return adapterFailure('invalid-payload');
  }

  return {
    ok: true,
    value: parsed.data,
  };
}

function adapterFailure(
  reason: PolicyRequestAssistantPreviewConfirmAdapterFailureReason
): PolicyRequestAssistantPreviewConfirmAdapterResult {
  return {
    ok: false,
    reason,
  };
}
