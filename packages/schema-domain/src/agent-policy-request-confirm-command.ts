import {
  AgentDeviceIdSchema,
  AgentMessageIdSchema,
  AgentPeerIdSchema,
  AgentPlatformSchema,
  AgentProtocolSchemaVersion,
  AgentRouteSchema,
} from './event-primitives';
import { type Infer, NonEmptyStringSchema, Schema, withParser } from './effect';
import {
  AgentPolicyPreviewAssistantConfirmationStateSchema,
  AgentPolicyPreviewDefaults,
  AgentPolicyPreviewRequestStatusSchema,
} from './agent-policy-preview-read-model';

export const PolicyRequestAssistantPreviewConfirmPayloadField = {
  Request: 'policyRequestAssistantPreviewConfirmRequest',
  Result: 'policyRequestAssistantPreviewConfirmResult',
} as const;

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
export const PolicyRequestAssistantPreviewConfirmActorStateSchema = withParser(Schema.Literal('active', 'revoked'));
export const PolicyRequestAssistantPreviewConfirmResultStateSchema = withParser(
  Schema.Literal('confirmed', 'rejected')
);
export const PolicyRequestAssistantPreviewConfirmClaimStateSchema = withParser(Schema.Literal('claimed', 'unclaimed'));

export const PolicyRequestAssistantPreviewConfirmRequestSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(AgentProtocolSchemaVersion),
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
    requestedBonusMinutes: Schema.Union(Schema.Number.pipe(Schema.int(), Schema.positive()), Schema.Null),
    requestedAt: NonEmptyStringSchema,
    expiresAt: NonEmptyStringSchema,
    origin: Schema.Literal(AgentPolicyPreviewDefaults.RequestOrigin.AssistantDraft),
    assistantPreviewId: NonEmptyStringSchema,
    assistantConfirmationState: Schema.Literal(
      AgentPolicyPreviewDefaults.AssistantConfirmationState.ParentConfirmationRequired
    ),
    requestStatus: Schema.Literal(AgentPolicyPreviewDefaults.RequestStatus.PreviewOnly),
    auditReferenceIds: Schema.Array(NonEmptyStringSchema),
    confirmationActorId: NonEmptyStringSchema,
    confirmationActorRole: PolicyRequestAssistantPreviewConfirmActorRoleSchema,
    confirmationActorState: PolicyRequestAssistantPreviewConfirmActorStateSchema,
    confirmationAuditReferenceId: NonEmptyStringSchema,
    confirmedAt: NonEmptyStringSchema,
  })
    .pipe(
      Schema.filter(
        (request) => request.auditReferenceIds.length > 0 || 'Policy request confirmation needs audit reference IDs'
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
    schemaVersion: Schema.Literal(AgentProtocolSchemaVersion),
    commandId: NonEmptyStringSchema,
    requestId: NonEmptyStringSchema,
    assistantPreviewId: Schema.Union(NonEmptyStringSchema, Schema.Null),
    resultState: PolicyRequestAssistantPreviewConfirmResultStateSchema,
    policyRequestStatus: AgentPolicyPreviewRequestStatusSchema,
    policyAssistantConfirmationState: AgentPolicyPreviewAssistantConfirmationStateSchema,
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

export const PolicyRequestAssistantPreviewConfirmCommandTargetSchema = withParser(
  Schema.Struct({
    scope: Schema.Literal('device'),
    deviceId: AgentDeviceIdSchema,
    platform: AgentPlatformSchema,
    route: AgentRouteSchema,
  })
);

export const PolicyRequestAssistantPreviewConfirmCommandSourceSchema = withParser(
  Schema.Struct({
    messageId: AgentMessageIdSchema,
    peerId: AgentPeerIdSchema,
  })
);

export type PolicyRequestAssistantPreviewConfirmRequest = Infer<
  typeof PolicyRequestAssistantPreviewConfirmRequestSchema
>;
export type PolicyRequestAssistantPreviewConfirmResult = Infer<typeof PolicyRequestAssistantPreviewConfirmResultSchema>;
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
