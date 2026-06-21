import { describe, expect, it } from 'vitest';
import {
  LocalAiContractCompletenessProof,
  LocalAiContractCompletenessProofSchema,
} from '@ocentra-parent/schema-domain/local-ai-contract-completeness-proof';
import { LocalAiProviderSchedulerDecisionSchema } from '@ocentra-parent/schema-domain/local-ai-provider-scheduler';

describe('local AI contract completeness proof', () => {
  it('proves input, result, provider capability, queue, and route contracts without runtime overclaims', () => {
    expect(LocalAiContractCompletenessProof.provedContractKinds).toEqual([
      'input',
      'result',
      'provider-capability',
      'job-queue',
      'provider-route',
    ]);
    expect(LocalAiContractCompletenessProof.evaluationInput.requestId).toBe(
      LocalAiContractCompletenessProof.safetyResult.requestId
    );
    expect(LocalAiContractCompletenessProof.routeDecision.selectedRuntimeReferenceId).toBe(
      LocalAiContractCompletenessProof.runtimeStatus.runtimeReferenceId
    );
    expect(LocalAiContractCompletenessProof.safetyResult.modelRuntime.privacyMode).toBe('local-only');
    expect(LocalAiContractCompletenessProof.claimBoundaries).toEqual({
      modelExecutionClaimed: false,
      modelQualityClaimed: false,
      policyAuthorityClaimed: false,
      enforcementClaimed: false,
      portalUiClaimed: false,
      remoteApiAiUsed: false,
      rawPromptRetained: false,
      rawEvidenceRetained: false,
    });
  });

  it('rejects proof rows that promote local AI contracts into execution or enforcement claims', () => {
    const result = LocalAiContractCompletenessProofSchema.safeParse({
      ...LocalAiContractCompletenessProof,
      claimBoundaries: {
        ...LocalAiContractCompletenessProof.claimBoundaries,
        enforcementClaimed: true,
      },
    });

    expect(result.success).toBe(false);
  });

  it('rejects a provider route that no longer points at the selected runtime', () => {
    const result = LocalAiContractCompletenessProofSchema.safeParse({
      ...LocalAiContractCompletenessProof,
      routeDecision: {
        ...LocalAiContractCompletenessProof.routeDecision,
        selectedRuntimeReferenceId: 'different-runtime',
      },
    });

    expect(result.success).toBe(false);
  });

  it('rejects queued local AI jobs without a queue position', () => {
    const result = LocalAiProviderSchedulerDecisionSchema.safeParse({
      ...LocalAiContractCompletenessProof.routeDecision,
      queuePosition: null,
    });

    expect(result.success).toBe(false);
  });
});
