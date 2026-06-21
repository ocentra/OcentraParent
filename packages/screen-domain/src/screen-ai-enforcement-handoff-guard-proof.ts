import {
  ScreenAiEnforcementHandoffGuardInputSchema,
  ScreenAiEnforcementHandoffGuardPayloadSchema,
  type ScreenAiEnforcementHandoffGuardPayload,
} from '@ocentra-parent/schema-domain/screen-ai-enforcement-handoff-guard-proof';

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
