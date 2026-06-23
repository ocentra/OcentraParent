import { type Infer, NonEmptyStringSchema, Schema, withParser } from './effect';
import {
  AgentPolicyControlDeliveryDomainSchema,
  AgentPolicyControlDeliveryParentVisibleStateSchema,
} from './agent-policy-control-delivery-read-model';

const PolicyControlAuditCountSchema = Schema.Number.pipe(Schema.nonNegative(), Schema.int());
const NullablePolicyControlAuditTextSchema = Schema.Union(NonEmptyStringSchema, Schema.Null);

export const PolicyControlAuditPayloadSchemaVersion = 'policy-control-audit.v1' as const;
export const PolicyControlRedactedAuditPayloadSchemaVersion = 'policy-control-audit-redacted.v1' as const;

export const PolicyControlAuditEventKindSchema = withParser(
  Schema.Literal(
    'queued',
    'delivered',
    'acknowledged',
    'applied',
    'rejected',
    'superseded',
    'rolled-back',
    'degraded',
    'manual-required',
    'expired-before-delivery'
  )
);
export const PolicyControlAuditActorRoleSchema = withParser(
  Schema.Literal('parent-service', 'child-agent', 'policy-control-plane', 'adapter-runtime')
);
export const PolicyControlAuditSensitiveFieldKindSchema = withParser(
  Schema.Literal(
    'child-display-name',
    'account-locator',
    'policy-target-value',
    'raw-url',
    'secret-token',
    'provider-detail'
  )
);

export const PolicyControlAuditRedactionPlaceholder = {
  ChildIdentity: 'redacted-child-identity',
  AccountIdentity: 'redacted-account-identity',
  PolicyTarget: 'redacted-policy-target',
  RawUrl: 'redacted-url',
  Secret: 'redacted-secret',
  ProviderDetail: 'redacted-provider-detail',
} as const;

export const PolicyControlAuditPayloadSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(PolicyControlAuditPayloadSchemaVersion),
    auditEventId: NonEmptyStringSchema,
    deliveryRowId: NonEmptyStringSchema,
    policyVersionRef: NonEmptyStringSchema,
    policyDomain: AgentPolicyControlDeliveryDomainSchema,
    eventKind: PolicyControlAuditEventKindSchema,
    parentVisibleState: AgentPolicyControlDeliveryParentVisibleStateSchema,
    childDeviceId: NonEmptyStringSchema,
    actorRole: PolicyControlAuditActorRoleSchema,
    reasonCodes: Schema.Array(NonEmptyStringSchema),
    auditRefs: Schema.Array(NonEmptyStringSchema).pipe(
      Schema.filter((value) => value.length > 0 || 'Expected audit refs for policy control audit payload')
    ),
    manualProofRequirements: Schema.Array(NonEmptyStringSchema),
    retryScheduleRefs: Schema.Array(NonEmptyStringSchema),
    childDisplayName: NullablePolicyControlAuditTextSchema,
    accountLocator: NullablePolicyControlAuditTextSchema,
    policyTargetValue: NullablePolicyControlAuditTextSchema,
    rawUrl: NullablePolicyControlAuditTextSchema,
    secretToken: NullablePolicyControlAuditTextSchema,
    providerDetail: NullablePolicyControlAuditTextSchema,
  })
);

const PolicyControlRedactedAuditPayloadBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(PolicyControlRedactedAuditPayloadSchemaVersion),
  auditEventId: NonEmptyStringSchema,
  deliveryRowId: NonEmptyStringSchema,
  policyVersionRef: NonEmptyStringSchema,
  policyDomain: AgentPolicyControlDeliveryDomainSchema,
  eventKind: PolicyControlAuditEventKindSchema,
  parentVisibleState: AgentPolicyControlDeliveryParentVisibleStateSchema,
  childDeviceId: NonEmptyStringSchema,
  actorRole: PolicyControlAuditActorRoleSchema,
  reasonCodes: Schema.Array(NonEmptyStringSchema),
  auditRefs: Schema.Array(NonEmptyStringSchema).pipe(
    Schema.filter((value) => value.length > 0 || 'Expected audit refs for redacted policy control audit payload')
  ),
  manualProofRequirements: Schema.Array(NonEmptyStringSchema),
  retryScheduleRefs: Schema.Array(NonEmptyStringSchema),
  childDisplayName: Schema.Union(Schema.Literal(PolicyControlAuditRedactionPlaceholder.ChildIdentity), Schema.Null),
  accountLocator: Schema.Union(Schema.Literal(PolicyControlAuditRedactionPlaceholder.AccountIdentity), Schema.Null),
  policyTargetValue: Schema.Union(Schema.Literal(PolicyControlAuditRedactionPlaceholder.PolicyTarget), Schema.Null),
  rawUrl: Schema.Union(Schema.Literal(PolicyControlAuditRedactionPlaceholder.RawUrl), Schema.Null),
  secretToken: Schema.Union(Schema.Literal(PolicyControlAuditRedactionPlaceholder.Secret), Schema.Null),
  providerDetail: Schema.Union(Schema.Literal(PolicyControlAuditRedactionPlaceholder.ProviderDetail), Schema.Null),
  protectedFieldKinds: Schema.Array(PolicyControlAuditSensitiveFieldKindSchema),
  redactedSensitiveFieldCount: PolicyControlAuditCountSchema,
  redactionApplied: Schema.Boolean,
});

type PolicyControlRedactedAuditPayloadCandidate = Infer<typeof PolicyControlRedactedAuditPayloadBaseSchema>;

export const PolicyControlRedactedAuditPayloadSchema = withParser(
  PolicyControlRedactedAuditPayloadBaseSchema.pipe(
    Schema.filter((payload: PolicyControlRedactedAuditPayloadCandidate) => {
      const validation = validateRedactedAuditPayload(payload);
      return validation === true
        ? true
        : 'Expected redacted policy control audit payloads to keep sensitive fields redacted and counts aligned';
    })
  )
);

export type PolicyControlAuditEventKind = Infer<typeof PolicyControlAuditEventKindSchema>;
export type PolicyControlAuditActorRole = Infer<typeof PolicyControlAuditActorRoleSchema>;
export type PolicyControlAuditSensitiveFieldKind = Infer<typeof PolicyControlAuditSensitiveFieldKindSchema>;
export type PolicyControlAuditPayload = Infer<typeof PolicyControlAuditPayloadSchema>;
export type PolicyControlRedactedAuditPayload = Infer<typeof PolicyControlRedactedAuditPayloadSchema>;

export function redactPolicyControlAuditPayload(payload: PolicyControlAuditPayload): PolicyControlRedactedAuditPayload {
  const parsed = PolicyControlAuditPayloadSchema.parse(payload);
  const protectedFieldKinds: PolicyControlAuditSensitiveFieldKind[] = [];

  const redacted = PolicyControlRedactedAuditPayloadSchema.parse({
    schemaVersion: PolicyControlRedactedAuditPayloadSchemaVersion,
    auditEventId: parsed.auditEventId,
    deliveryRowId: parsed.deliveryRowId,
    policyVersionRef: parsed.policyVersionRef,
    policyDomain: parsed.policyDomain,
    eventKind: parsed.eventKind,
    parentVisibleState: parsed.parentVisibleState,
    childDeviceId: parsed.childDeviceId,
    actorRole: parsed.actorRole,
    reasonCodes: parsed.reasonCodes,
    auditRefs: parsed.auditRefs,
    manualProofRequirements: parsed.manualProofRequirements,
    retryScheduleRefs: parsed.retryScheduleRefs,
    childDisplayName: redactField(
      parsed.childDisplayName,
      'child-display-name',
      PolicyControlAuditRedactionPlaceholder.ChildIdentity,
      protectedFieldKinds
    ),
    accountLocator: redactField(
      parsed.accountLocator,
      'account-locator',
      PolicyControlAuditRedactionPlaceholder.AccountIdentity,
      protectedFieldKinds
    ),
    policyTargetValue: redactField(
      parsed.policyTargetValue,
      'policy-target-value',
      PolicyControlAuditRedactionPlaceholder.PolicyTarget,
      protectedFieldKinds
    ),
    rawUrl: redactField(parsed.rawUrl, 'raw-url', PolicyControlAuditRedactionPlaceholder.RawUrl, protectedFieldKinds),
    secretToken: redactField(
      parsed.secretToken,
      'secret-token',
      PolicyControlAuditRedactionPlaceholder.Secret,
      protectedFieldKinds
    ),
    providerDetail: redactField(
      parsed.providerDetail,
      'provider-detail',
      PolicyControlAuditRedactionPlaceholder.ProviderDetail,
      protectedFieldKinds
    ),
    protectedFieldKinds,
    redactedSensitiveFieldCount: protectedFieldKinds.length,
    redactionApplied: protectedFieldKinds.length > 0,
  });

  return redacted;
}

function redactField<TPlaceholder extends string>(
  value: string | null,
  kind: PolicyControlAuditSensitiveFieldKind,
  placeholder: TPlaceholder,
  protectedFieldKinds: PolicyControlAuditSensitiveFieldKind[]
): TPlaceholder | null {
  if (value === null) {
    return null;
  }
  protectedFieldKinds.push(kind);
  return placeholder;
}

function validateRedactedAuditPayload(payload: PolicyControlRedactedAuditPayloadCandidate): true | string {
  const hasProtectedFields = payload.redactedSensitiveFieldCount > 0;
  if (payload.redactionApplied !== hasProtectedFields) {
    return 'Redaction applied flag must match sensitive field count';
  }
  if (payload.redactedSensitiveFieldCount !== payload.protectedFieldKinds.length) {
    return 'Protected field kinds must match redacted sensitive field count';
  }
  if (!matchesPlaceholder(payload.childDisplayName, payload.protectedFieldKinds, 'child-display-name')) {
    return 'Child display name must be null or the redacted placeholder';
  }
  if (!matchesPlaceholder(payload.accountLocator, payload.protectedFieldKinds, 'account-locator')) {
    return 'Account locator must be null or the redacted placeholder';
  }
  if (!matchesPlaceholder(payload.policyTargetValue, payload.protectedFieldKinds, 'policy-target-value')) {
    return 'Policy target value must be null or the redacted placeholder';
  }
  if (!matchesPlaceholder(payload.rawUrl, payload.protectedFieldKinds, 'raw-url')) {
    return 'Raw URL must be null or the redacted placeholder';
  }
  if (!matchesPlaceholder(payload.secretToken, payload.protectedFieldKinds, 'secret-token')) {
    return 'Secret token must be null or the redacted placeholder';
  }
  if (!matchesPlaceholder(payload.providerDetail, payload.protectedFieldKinds, 'provider-detail')) {
    return 'Provider detail must be null or the redacted placeholder';
  }
  return true;
}

function matchesPlaceholder(
  value: string | null,
  protectedFieldKinds: readonly PolicyControlAuditSensitiveFieldKind[],
  kind: PolicyControlAuditSensitiveFieldKind
): boolean {
  return protectedFieldKinds.includes(kind) ? value !== null : value === null;
}
