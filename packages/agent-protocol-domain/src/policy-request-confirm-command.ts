import { AgentProtocolSchemaVersion, type AgentPeerRole, type AgentRoute } from '@ocentra-parent/schema-domain/event-primitives';
import { type Infer, NonEmptyStringSchema, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  AgentCommand,
  AgentCommandEnvelopeSchema,
  AgentEvent,
  AgentProtocolDefaults,
  type AgentCommandEnvelope,
  type AgentEventEnvelope,
  type AgentEventName,
  type AgentProtocolLogFields,
} from './contracts';

export const PolicyRequestAssistantPreviewConfirmRequestKindSchema = withParser(
  Schema.Literal('ask-parent', 'bonus-time', 'temporary-override')
);
export const PolicyRequestAssistantPreviewConfirmTargetKindSchema = withParser(
  Schema.Literal('child-profile', 'device', 'app', 'site', 'category', 'resource')
);
export const PolicyRequestAssistantPreviewConfirmActionSchema = withParser(
  Schema.Literal('allow', 'warn', 'ask-parent', 'time-limit', 'block')
);
export const PolicyRequestAssistantPreviewConfirmActorRoleSchema = withParser(
  Schema.Literal('parent', 'co-parent', 'observer', 'child', 'support')
);
export const PolicyRequestAssistantPreviewConfirmActorStateSchema = withParser(
  Schema.Literal('active', 'revoked')
);
export const PolicyRequestAssistantPreviewConfirmResultStateSchema = withParser(
  Schema.Literal('confirmed', 'rejected')
);
export const PolicyRequestAssistantPreviewConfirmClaimStateSchema = withParser(
  Schema.Literal('claimed', 'unclaimed')
);

const PolicyRequestStatusSchema = withParser(
  Schema.Literal(
    AgentProtocolDefaults.PolicyPreview.RequestStatus.PreviewOnly,
    AgentProtocolDefaults.PolicyPreview.RequestStatus.PendingParentReview,
    AgentProtocolDefaults.PolicyPreview.RequestStatus.Approved,
    AgentProtocolDefaults.PolicyPreview.RequestStatus.Denied,
    AgentProtocolDefaults.PolicyPreview.RequestStatus.Modified,
    AgentProtocolDefaults.PolicyPreview.RequestStatus.Expired,
    AgentProtocolDefaults.PolicyPreview.RequestStatus.ReplayRejected
  )
);

const PolicyAssistantConfirmationStateSchema = withParser(
  Schema.Literal(
    AgentProtocolDefaults.PolicyPreview.AssistantConfirmationState.NotRequired,
    AgentProtocolDefaults.PolicyPreview.AssistantConfirmationState.ParentConfirmationRequired,
    AgentProtocolDefaults.PolicyPreview.AssistantConfirmationState.ParentConfirmed
  )
);

export const PolicyRequestAssistantPreviewConfirmRequestSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(AgentProtocolDefaults.SchemaVersion),
    commandId: NonEmptyStringSchema,
    requestId: NonEmptyStringSchema,
    submissionKey: NonEmptyStringSchema,
    householdId: NonEmptyStringSchema,
    childProfileId: NonEmptyStringSchema,
    deviceId: Schema.Union(NonEmptyStringSchema, Schema.Null),
    sourceDocumentId: NonEmptyStringSchema,
    policyVersion: Schema.Number.pipe(Schema.int(), Schema.positive()),
    requestKind: PolicyRequestAssistantPreviewConfirmRequestKindSchema,
    targetKind: PolicyRequestAssistantPreviewConfirmTargetKindSchema,
    targetReferenceId: NonEmptyStringSchema,
    requestedAction: PolicyRequestAssistantPreviewConfirmActionSchema,
    ruleId: Schema.Union(NonEmptyStringSchema, Schema.Null),
    requestedBonusMinutes: Schema.Union(
      Schema.Number.pipe(Schema.int(), Schema.positive()),
      Schema.Null
    ),
    requestedAt: NonEmptyStringSchema,
    expiresAt: NonEmptyStringSchema,
    origin: Schema.Literal(AgentProtocolDefaults.PolicyPreview.RequestOrigin.AssistantDraft),
    assistantPreviewId: NonEmptyStringSchema,
    assistantConfirmationState: Schema.Literal(
      AgentProtocolDefaults.PolicyPreview.AssistantConfirmationState.ParentConfirmationRequired
    ),
    requestStatus: Schema.Literal(AgentProtocolDefaults.PolicyPreview.RequestStatus.PreviewOnly),
    auditReferenceIds: Schema.Array(NonEmptyStringSchema),
    confirmationActorId: NonEmptyStringSchema,
    confirmationActorRole: PolicyRequestAssistantPreviewConfirmActorRoleSchema,
    confirmationActorState: PolicyRequestAssistantPreviewConfirmActorStateSchema,
    confirmationAuditReferenceId: NonEmptyStringSchema,
    confirmedAt: NonEmptyStringSchema,
  })
    .pipe(
      Schema.filter(
        (request) =>
          request.auditReferenceIds.length > 0 || 'Policy request confirmation needs audit reference IDs'
      )
    )
    .pipe(
      Schema.filter(
        (request) =>
          request.requestKind !== 'bonus-time' ||
          request.requestedBonusMinutes !== null ||
          'Bonus-time confirmation requests must include requested bonus minutes'
      )
    )
);

export const PolicyRequestAssistantPreviewConfirmResultSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(AgentProtocolDefaults.SchemaVersion),
    commandId: NonEmptyStringSchema,
    requestId: NonEmptyStringSchema,
    assistantPreviewId: Schema.Union(NonEmptyStringSchema, Schema.Null),
    resultState: PolicyRequestAssistantPreviewConfirmResultStateSchema,
    policyRequestStatus: PolicyRequestStatusSchema,
    policyAssistantConfirmationState: PolicyAssistantConfirmationStateSchema,
    policyAuditReferenceId: Schema.Union(NonEmptyStringSchema, Schema.Null),
    confirmedAt: Schema.Union(NonEmptyStringSchema, Schema.Null),
    rejectionReason: Schema.Union(NonEmptyStringSchema, Schema.Null),
    commandTransportClaimState: Schema.Literal('claimed'),
    serviceValidationClaimState: Schema.Literal('claimed'),
    activityStoreMutationClaimState: PolicyRequestAssistantPreviewConfirmClaimStateSchema,
    upstreamWriterClaimState: PolicyRequestAssistantPreviewConfirmClaimStateSchema,
    readModelProjectionClaimState: PolicyRequestAssistantPreviewConfirmClaimStateSchema,
    portalWritableUiClaimState: Schema.Literal('unclaimed'),
    childDeviceDeliveryClaimState: Schema.Literal('unclaimed'),
    providerDeliveryClaimState: Schema.Literal('unclaimed'),
    platformEnforcementClaimState: Schema.Literal('unclaimed'),
    productClaimState: Schema.Literal('unclaimed'),
  })
);

export type PolicyRequestAssistantPreviewConfirmRequest = Infer<
  typeof PolicyRequestAssistantPreviewConfirmRequestSchema
>;
export type PolicyRequestAssistantPreviewConfirmResult = Infer<
  typeof PolicyRequestAssistantPreviewConfirmResultSchema
>;
export type PolicyRequestAssistantPreviewConfirmResultState = Infer<
  typeof PolicyRequestAssistantPreviewConfirmResultStateSchema
>;
export type PolicyRequestAssistantPreviewConfirmClaimState = Infer<
  typeof PolicyRequestAssistantPreviewConfirmClaimStateSchema
>;

export const PolicyRequestAssistantPreviewConfirmResultState = {
  Confirmed: PolicyRequestAssistantPreviewConfirmResultStateSchema.parse('confirmed'),
  Rejected: PolicyRequestAssistantPreviewConfirmResultStateSchema.parse('rejected'),
} as const;

export const PolicyRequestAssistantPreviewConfirmClaimState = {
  Claimed: PolicyRequestAssistantPreviewConfirmClaimStateSchema.parse('claimed'),
  Unclaimed: PolicyRequestAssistantPreviewConfirmClaimStateSchema.parse('unclaimed'),
} as const;

export type PolicyRequestAssistantPreviewConfirmAdapterFailureReason =
  | 'wrong-event'
  | 'missing-json-field'
  | 'invalid-json'
  | 'invalid-payload';

export type PolicyRequestAssistantPreviewConfirmAdapterResult =
  | {
      readonly ok: true;
      readonly value: PolicyRequestAssistantPreviewConfirmResult;
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
  readonly request: PolicyRequestAssistantPreviewConfirmRequest;
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

export function defaultPolicyRequestAssistantPreviewConfirmRequest(): PolicyRequestAssistantPreviewConfirmRequest {
  return PolicyRequestAssistantPreviewConfirmRequestSchema.parse({
    schemaVersion: AgentProtocolDefaults.SchemaVersion,
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
    origin: AgentProtocolDefaults.PolicyPreview.RequestOrigin.AssistantDraft,
    assistantPreviewId: PolicyRequestAssistantPreviewConfirmDefaults.AssistantPreviewId,
    assistantConfirmationState:
      AgentProtocolDefaults.PolicyPreview.AssistantConfirmationState.ParentConfirmationRequired,
    requestStatus: AgentProtocolDefaults.PolicyPreview.RequestStatus.PreviewOnly,
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
  const parsedRequest = PolicyRequestAssistantPreviewConfirmRequestSchema.safeParse(input.request);
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
  request: PolicyRequestAssistantPreviewConfirmRequest
): AgentProtocolLogFields {
  const parsedRequest = PolicyRequestAssistantPreviewConfirmRequestSchema.safeParse(request);
  if (!parsedRequest.success) {
    throw new Error('invalid policy request assistant preview confirm request');
  }

  return {
    [AgentProtocolDefaults.Field.PolicyRequestAssistantPreviewConfirmRequest]: JSON.stringify(
      parsedRequest.data
    ),
  };
}

export function parsePolicyRequestAssistantPreviewConfirmResultEvent(
  event: AgentEventEnvelope
): PolicyRequestAssistantPreviewConfirmAdapterResult {
  if (!policyRequestAssistantPreviewConfirmEventNames().includes(event.event)) {
    return adapterFailure('wrong-event');
  }

  const raw = event.payload[AgentProtocolDefaults.Field.PolicyRequestAssistantPreviewConfirmResult];
  if (typeof raw !== 'string') {
    return adapterFailure('missing-json-field');
  }

  let decoded: unknown;
  try {
    decoded = JSON.parse(raw);
  } catch {
    return adapterFailure('invalid-json');
  }

  const parsed = PolicyRequestAssistantPreviewConfirmResultSchema.safeParse(decoded);
  if (!parsed.success) {
    return adapterFailure('invalid-payload');
  }

  return {
    ok: true,
    value: parsed.data,
  };
}

function policyRequestAssistantPreviewConfirmEventNames(): AgentEventName[] {
  return [AgentEvent.PolicyRequestAssistantPreviewConfirmReported];
}

function adapterFailure(
  reason: PolicyRequestAssistantPreviewConfirmAdapterFailureReason
): PolicyRequestAssistantPreviewConfirmAdapterResult {
  return {
    ok: false,
    reason,
  };
}
