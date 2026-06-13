import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { PolicyDecisionSchema } from '@ocentra-parent/policy-domain/policy';

export const PolicyAuthoritySourceLiteral = {
  ParentPolicy: 'parent-policy',
  LocalAiResult: 'local-ai-result',
  TrackingSignal: 'tracking-signal',
  ActivityEvidence: 'activity-evidence',
} as const;

export const PolicyAuthorityStateLiteral = {
  Authorized: 'authorized',
  EvidenceOnly: 'evidence-only',
  DryRun: 'dry-run',
} as const;

export const PolicyAuthoritySourceSchema = withParser(
  Schema.Literal(
    PolicyAuthoritySourceLiteral.ParentPolicy,
    PolicyAuthoritySourceLiteral.LocalAiResult,
    PolicyAuthoritySourceLiteral.TrackingSignal,
    PolicyAuthoritySourceLiteral.ActivityEvidence
  )
);

export const PolicyAuthorityStateSchema = withParser(
  Schema.Literal(
    PolicyAuthorityStateLiteral.Authorized,
    PolicyAuthorityStateLiteral.EvidenceOnly,
    PolicyAuthorityStateLiteral.DryRun
  )
);

export const PolicyAuthorityRequestSchema = withParser(
  Schema.Struct({
    source: PolicyAuthoritySourceSchema,
    decision: PolicyDecisionSchema,
  })
);

export const PolicyAuthorityDecisionSchema = withParser(
  Schema.Struct({
    source: PolicyAuthoritySourceSchema,
    state: PolicyAuthorityStateSchema,
    decision: PolicyDecisionSchema,
  })
);

export type PolicyAuthoritySource = Infer<typeof PolicyAuthoritySourceSchema>;
export type PolicyAuthorityState = Infer<typeof PolicyAuthorityStateSchema>;
export type PolicyAuthorityRequest = Infer<typeof PolicyAuthorityRequestSchema>;
export type PolicyAuthorityDecision = Infer<typeof PolicyAuthorityDecisionSchema>;

export const PolicyAuthoritySource = {
  ParentPolicy: PolicyAuthoritySourceSchema.parse(PolicyAuthoritySourceLiteral.ParentPolicy),
  LocalAiResult: PolicyAuthoritySourceSchema.parse(PolicyAuthoritySourceLiteral.LocalAiResult),
  TrackingSignal: PolicyAuthoritySourceSchema.parse(PolicyAuthoritySourceLiteral.TrackingSignal),
  ActivityEvidence: PolicyAuthoritySourceSchema.parse(PolicyAuthoritySourceLiteral.ActivityEvidence),
} as const;

export const PolicyAuthorityState = {
  Authorized: PolicyAuthorityStateSchema.parse(PolicyAuthorityStateLiteral.Authorized),
  EvidenceOnly: PolicyAuthorityStateSchema.parse(PolicyAuthorityStateLiteral.EvidenceOnly),
  DryRun: PolicyAuthorityStateSchema.parse(PolicyAuthorityStateLiteral.DryRun),
} as const;

export function resolvePolicyAuthority(input: PolicyAuthorityRequest): PolicyAuthorityDecision {
  const request = PolicyAuthorityRequestSchema.parse(input);
  const state =
    request.decision.dryRun ? PolicyAuthorityState.DryRun
    : request.source === PolicyAuthoritySource.ParentPolicy ? PolicyAuthorityState.Authorized
    : PolicyAuthorityState.EvidenceOnly;

  return PolicyAuthorityDecisionSchema.parse({
    source: request.source,
    state,
    decision: request.decision,
  });
}
