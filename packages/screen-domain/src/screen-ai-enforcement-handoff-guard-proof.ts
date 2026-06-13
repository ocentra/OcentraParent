import { EventingEventTypeSchema } from '@ocentra-parent/event-domain/eventing';
import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import { PolicyActionSchema, PolicyDecisionSchema, PolicyRuleSchema } from '@ocentra-parent/policy-domain/policy';
import { ParentContractSchemaVersionSchema, ParentTimestampSchema } from '@ocentra-parent/family-domain/reference-primitives';
import { ParentEvidenceReferenceSchema } from '@ocentra-parent/family-domain/references';

export const ScreenAiEnforcementHandoffGuardPayloadIdSchema = brandedNonEmptyStringSchema('ScreenAiEnforcementHandoffGuardPayloadId');
export const ScreenAiEnforcementHandoffGuardAuditEventIdSchema = brandedNonEmptyStringSchema('ScreenAiEnforcementHandoffGuardAuditEventId');
export const ScreenAiEnforcementHandoffGuardBoundarySchema = brandedNonEmptyStringSchema('ScreenAiEnforcementHandoffGuardBoundary');

export const ScreenAiEnforcementHandoffConfidenceStateSchema = withParser(
  Schema.Literal('high', 'medium', 'low', 'unknown')
);

export const ScreenAiEnforcementHandoffModeSchema = withParser(Schema.Literal('dry-run', 'manual-required'));
export const ScreenAiEnforcementHandoffEventNameLiteral = {
  Accepted: 'screen.enforcement.handoff.guard.accepted',
} as const;
export const ScreenAiEnforcementHandoffAcceptedEventType = EventingEventTypeSchema.parse(
  ScreenAiEnforcementHandoffEventNameLiteral.Accepted
);

export const ScreenAiEnforcementHandoffAuditEventSchema = withParser(
  Schema.Struct({
    auditEventId: ScreenAiEnforcementHandoffGuardAuditEventIdSchema,
    eventType: Schema.Literal(ScreenAiEnforcementHandoffAcceptedEventType),
    emittedAt: ParentTimestampSchema,
    evidenceReference: ParentEvidenceReferenceSchema,
  })
);

export const ScreenAiEnforcementHandoffInputMaterialSchema = withParser(
  Schema.Struct({
    summaryReference: ParentEvidenceReferenceSchema,
    localAiResultReference: ParentEvidenceReferenceSchema,
    auditReference: ParentEvidenceReferenceSchema,
    rawPixelsIncluded: Schema.Literal(false),
    rawModelTextIncluded: Schema.Literal(false),
    rawScreenshotRetained: Schema.Literal(false),
    localAiAuthorityClaimed: Schema.Literal(false),
  })
);

const ScreenAiEnforcementHandoffGuardInputBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  payloadId: ScreenAiEnforcementHandoffGuardPayloadIdSchema,
  generatedAt: ParentTimestampSchema,
  sourcePolicyDecision: PolicyDecisionSchema,
  parentPolicyRule: PolicyRuleSchema,
  requestedAction: PolicyActionSchema,
  confidenceState: ScreenAiEnforcementHandoffConfidenceStateSchema,
  handoffMode: ScreenAiEnforcementHandoffModeSchema,
  inputMaterial: ScreenAiEnforcementHandoffInputMaterialSchema,
  auditEvent: ScreenAiEnforcementHandoffAuditEventSchema,
  claimBoundary: ScreenAiEnforcementHandoffGuardBoundarySchema,
});

type ScreenAiEnforcementHandoffGuardInputCandidate = Infer<typeof ScreenAiEnforcementHandoffGuardInputBaseSchema>;

export const ScreenAiEnforcementHandoffGuardInputSchema = withParser(
  ScreenAiEnforcementHandoffGuardInputBaseSchema.pipe(
    Schema.filter(
      (input) =>
        screenAiEnforcementHandoffInputIsReady(input) ||
        'Expected screen AI enforcement handoff guard input to use dry-run policy with summary, AI, parent-rule, and audit refs'
    )
  )
);

const ScreenAiEnforcementHandoffGuardPayloadBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  payloadId: ScreenAiEnforcementHandoffGuardPayloadIdSchema,
  generatedAt: ParentTimestampSchema,
  sourcePolicyDecision: PolicyDecisionSchema,
  parentPolicyRule: PolicyRuleSchema,
  requestedAction: PolicyActionSchema,
  confidenceState: ScreenAiEnforcementHandoffConfidenceStateSchema,
  handoffMode: ScreenAiEnforcementHandoffModeSchema,
  summaryReference: ParentEvidenceReferenceSchema,
  localAiResultReference: ParentEvidenceReferenceSchema,
  auditReference: ParentEvidenceReferenceSchema,
  auditEvent: ScreenAiEnforcementHandoffAuditEventSchema,
  rawPixelsIncluded: Schema.Literal(false),
  rawModelTextIncluded: Schema.Literal(false),
  rawScreenshotRetained: Schema.Literal(false),
  localAiAuthorityClaimed: Schema.Literal(false),
  claimBoundary: ScreenAiEnforcementHandoffGuardBoundarySchema,
});

type ScreenAiEnforcementHandoffGuardPayloadCandidate = Infer<typeof ScreenAiEnforcementHandoffGuardPayloadBaseSchema>;

export const ScreenAiEnforcementHandoffGuardPayloadSchema = withParser(
  ScreenAiEnforcementHandoffGuardPayloadBaseSchema.pipe(
    Schema.filter(
      (payload) =>
        screenAiEnforcementHandoffPayloadIsHonest(payload) ||
        'Expected screen AI enforcement handoff payload to preserve dry-run custody and reject raw AI/pixel handoff'
    )
  )
);

export function buildScreenAiEnforcementHandoffGuardPayload(input: unknown): ScreenAiEnforcementHandoffGuardPayload {
  const parsed = ScreenAiEnforcementHandoffGuardInputSchema.parse(input);
  return ScreenAiEnforcementHandoffGuardPayloadSchema.parse({
    schemaVersion: parsed.schemaVersion,
    payloadId: parsed.payloadId,
    generatedAt: parsed.generatedAt,
    sourcePolicyDecision: parsed.sourcePolicyDecision,
    parentPolicyRule: parsed.parentPolicyRule,
    requestedAction: parsed.requestedAction,
    confidenceState: parsed.confidenceState,
    handoffMode: parsed.handoffMode,
    summaryReference: parsed.inputMaterial.summaryReference,
    localAiResultReference: parsed.inputMaterial.localAiResultReference,
    auditReference: parsed.inputMaterial.auditReference,
    auditEvent: parsed.auditEvent,
    rawPixelsIncluded: false,
    rawModelTextIncluded: false,
    rawScreenshotRetained: false,
    localAiAuthorityClaimed: false,
    claimBoundary: parsed.claimBoundary,
  });
}

function screenAiEnforcementHandoffInputIsReady(input: ScreenAiEnforcementHandoffGuardInputCandidate): boolean {
  return (
    screenAiPolicyDecisionIsGuarded(input) &&
    screenAiHandoffMaterialMatchesDecision(input) &&
    input.auditEvent.evidenceReference.evidenceReferenceId === input.inputMaterial.auditReference.evidenceReferenceId &&
    input.auditEvent.evidenceReference.kind === 'journal-event'
  );
}

function screenAiPolicyDecisionIsGuarded(input: ScreenAiEnforcementHandoffGuardInputCandidate): boolean {
  return (
    input.sourcePolicyDecision.dryRun === true &&
    input.sourcePolicyDecision.enforcementHandoffState !== 'handed-off' &&
    input.sourcePolicyDecision.localAiResultId !== null &&
    input.sourcePolicyDecision.ruleIds.includes(input.parentPolicyRule.ruleId) &&
    input.parentPolicyRule.enabled === true &&
    input.requestedAction === input.sourcePolicyDecision.action
  );
}

function screenAiHandoffMaterialMatchesDecision(input: ScreenAiEnforcementHandoffGuardInputCandidate): boolean {
  return (
    evidenceIncludes(input.sourcePolicyDecision.evidenceReferences, input.inputMaterial.summaryReference) &&
    evidenceIncludes(input.sourcePolicyDecision.evidenceReferences, input.inputMaterial.localAiResultReference) &&
    evidenceIncludes(input.sourcePolicyDecision.evidenceReferences, input.inputMaterial.auditReference) &&
    input.inputMaterial.summaryReference.kind === 'query-store-summary' &&
    input.inputMaterial.localAiResultReference.kind === 'local-ai-result' &&
    input.inputMaterial.auditReference.kind === 'journal-event'
  );
}

function screenAiEnforcementHandoffPayloadIsHonest(payload: ScreenAiEnforcementHandoffGuardPayloadCandidate): boolean {
  return (
    screenAiPolicyDecisionIsGuarded({
      ...payload,
      inputMaterial: {
        summaryReference: payload.summaryReference,
        localAiResultReference: payload.localAiResultReference,
        auditReference: payload.auditReference,
        rawPixelsIncluded: payload.rawPixelsIncluded,
        rawModelTextIncluded: payload.rawModelTextIncluded,
        rawScreenshotRetained: payload.rawScreenshotRetained,
        localAiAuthorityClaimed: payload.localAiAuthorityClaimed,
      },
    }) &&
    payload.auditEvent.evidenceReference.evidenceReferenceId === payload.auditReference.evidenceReferenceId &&
    payload.auditEvent.evidenceReference.kind === 'journal-event'
  );
}

function evidenceIncludes(
  evidenceReferences: ReadonlyArray<Infer<typeof ParentEvidenceReferenceSchema>>,
  expectedReference: Infer<typeof ParentEvidenceReferenceSchema>
): boolean {
  return evidenceReferences.some(
    (reference) =>
      reference.evidenceReferenceId === expectedReference.evidenceReferenceId &&
      reference.kind === expectedReference.kind
  );
}

export type ScreenAiEnforcementHandoffGuardPayloadId = typeof ScreenAiEnforcementHandoffGuardPayloadIdSchema.Type;
export type ScreenAiEnforcementHandoffGuardAuditEventId = typeof ScreenAiEnforcementHandoffGuardAuditEventIdSchema.Type;
export type ScreenAiEnforcementHandoffGuardBoundary = typeof ScreenAiEnforcementHandoffGuardBoundarySchema.Type;
export type ScreenAiEnforcementHandoffConfidenceState = Infer<typeof ScreenAiEnforcementHandoffConfidenceStateSchema>;
export type ScreenAiEnforcementHandoffMode = Infer<typeof ScreenAiEnforcementHandoffModeSchema>;
export type ScreenAiEnforcementHandoffAuditEvent = Infer<typeof ScreenAiEnforcementHandoffAuditEventSchema>;
export type ScreenAiEnforcementHandoffInputMaterial = Infer<typeof ScreenAiEnforcementHandoffInputMaterialSchema>;
export type ScreenAiEnforcementHandoffGuardInput = Infer<typeof ScreenAiEnforcementHandoffGuardInputSchema>;
export type ScreenAiEnforcementHandoffGuardPayload = Infer<typeof ScreenAiEnforcementHandoffGuardPayloadSchema>;

