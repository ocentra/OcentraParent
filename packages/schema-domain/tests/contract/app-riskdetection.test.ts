import { describe, expect, it } from 'vitest';
import {
  AppRiskDetectionCandidateSchema,
  AppRiskDetectionMatrixReadModel,
  AppRiskDetectionMatrixSchema,
  AppRiskDetectionPolicyTargetKind,
} from '@ocentra-parent/schema-domain/app-riskdetection';
import { AppRiskDetectionMatrix } from '@ocentra-parent/schema-domain/app-riskdetection-data';

const candidateFor = (candidateId: string) => {
  const candidate = AppRiskDetectionMatrix.candidates.find((entry) => entry.candidateId === candidateId);

  if (candidate === undefined) {
    throw new Error(`Missing app risk detection candidate ${candidateId}`);
  }

  return candidate;
};

describe('app risk detection contracts', () => {
  it('round trips the generated matrix through the thin schema-domain adapter', () => {
    const matrix = AppRiskDetectionMatrixSchema.parse(AppRiskDetectionMatrixReadModel);

    expect(matrix.matrixId).toBe('app-riskdetection-proof-matrix');
    expect(matrix.candidates).toHaveLength(8);
    expect(matrix.candidates.filter((candidate) => candidate.sourceKind === 'knownCatalog')).toHaveLength(4);
  });

  it('keeps known, heuristic, local-ai, and parent-override candidates explicit and advisory', () => {
    const knownVpn = candidateFor('known-vpn-proxy-risk');
    const aiCandidate = candidateFor('local-ai-social-video-messaging-risk');
    const override = candidateFor('parent-display-override-ai-tool');

    expect(knownVpn.policyTargetKind).toBe(AppRiskDetectionPolicyTargetKind.RiskApp);
    expect(knownVpn.notDirectEnforcement).toBe(true);
    expect(aiCandidate.localAiDigestRef).toBe('local-ai-digest-social-video-messaging');
    expect(override.parentOverride?.rawIdentityChanged).toBe(false);
  });

  it('rejects local-ai rows without digests and direct-enforcement overclaims', () => {
    const aiCandidate = candidateFor('local-ai-social-video-messaging-risk');
    const knownVpn = candidateFor('known-vpn-proxy-risk');

    expect(
      AppRiskDetectionCandidateSchema.safeParse({
        ...aiCandidate,
        localAiDigestRef: null,
      }).success
    ).toBe(false);
    expect(
      AppRiskDetectionCandidateSchema.safeParse({
        ...knownVpn,
        notDirectEnforcement: false,
      }).success
    ).toBe(false);
    expect(
      AppRiskDetectionCandidateSchema.safeParse({
        ...knownVpn,
        noContentClaim: false,
      }).success
    ).toBe(false);
  });
});
