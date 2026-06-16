import { describe, expect, it } from 'vitest';
import { PolicyAuthoritySource, PolicyAuthorityState, resolvePolicyAuthority } from '../../src/authority';
import {
  PolicyDecisionHandoffState,
  type LocalAiResultReferenceId,
  type PolicyDecisionId,
  type PolicyReasonCode,
  type PolicyRuleId,
} from '../../src/policy';
import {
  ParentEvidenceReferenceIdSchema,
  type ParentTimestamp,
} from '@ocentra-parent/family-domain/reference-primitives';

const policyDecision = {
  schemaVersion: 'v0.6',
  decisionId: 'decision-1' as PolicyDecisionId,
  action: 'ask-parent',
  reasonCodes: ['tracking-signal-review' as PolicyReasonCode],
  evidenceReferences: [
    {
      evidenceReferenceId: 'evidence-1' as typeof ParentEvidenceReferenceIdSchema.Type,
      kind: 'activity-event',
      observedAt: '2026-06-12T10:00:00.000Z' as ParentTimestamp,
    },
  ],
  ruleIds: ['rule-1' as PolicyRuleId],
  localAiResultId: 'ai-result-1' as LocalAiResultReferenceId,
  dryRun: false,
  enforcementHandoffState: PolicyDecisionHandoffState.Pending,
  expiresAt: null,
} as const;

describe('policy authority contracts', () => {
  it('allows only parent policy decisions to authorize enforcement handoff', () => {
    const decision = resolvePolicyAuthority({
      source: PolicyAuthoritySource.ParentPolicy,
      decision: policyDecision,
    });

    expect(decision.state).toBe(PolicyAuthorityState.Authorized);
  });

  it('keeps AI and tracking signals as evidence-only inputs', () => {
    const aiDecision = resolvePolicyAuthority({
      source: PolicyAuthoritySource.LocalAiResult,
      decision: policyDecision,
    });
    const trackingDecision = resolvePolicyAuthority({
      source: PolicyAuthoritySource.TrackingSignal,
      decision: policyDecision,
    });

    expect(aiDecision.state).toBe(PolicyAuthorityState.EvidenceOnly);
    expect(trackingDecision.state).toBe(PolicyAuthorityState.EvidenceOnly);
  });

  it('keeps dry-run parent policy decisions out of enforcement authority', () => {
    const decision = resolvePolicyAuthority({
      source: PolicyAuthoritySource.ParentPolicy,
      decision: { ...policyDecision, dryRun: true },
    });

    expect(decision.state).toBe(PolicyAuthorityState.DryRun);
  });
});
