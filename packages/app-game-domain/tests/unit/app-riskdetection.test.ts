import { describe, expect, it } from 'vitest';
import {
  AppGamePolicyTargetKind,
  appGamePolicyTargetRequiresCategory,
} from '../../src/app-game-policy-target-compiler-rules';
import { AppRiskDetectionMatrix } from '../../src/app-riskdetection-data';
import { AppRiskDetectionCandidateSchema, AppRiskDetectionMatrixSchema } from '../../src/app-riskdetection';

const candidateFor = (candidateId: string) => {
  const candidate = AppRiskDetectionMatrix.candidates.find((entry) => entry.candidateId === candidateId);

  if (candidate === undefined) {
    throw new Error(`Missing app risk detection candidate ${candidateId}`);
  }

  return candidate;
};

describe('native app risk detection contracts', () => {
  registerKnownCatalogRiskTests();
  registerUnknownAndAiCandidateTests();
  registerSafetyAndSurfaceDisclosureTests();
});

function registerKnownCatalogRiskTests() {
  it('classifies known VPN, remote desktop, torrent, and AI app risks with evidence refs', () => {
    const matrix = AppRiskDetectionMatrixSchema.parse(AppRiskDetectionMatrix);
    const signals = matrix.candidates.map((candidate) => candidate.riskSignal);

    expect(signals).toContain('vpnProxy');
    expect(signals).toContain('remoteDesktop');
    expect(signals).toContain('downloadTorrent');
    expect(signals).toContain('aiChatbot');
    expect(matrix.candidates.filter((candidate) => candidate.sourceKind === 'knownCatalog')).toHaveLength(4);
    expect(matrix.candidates.every((candidate) => candidate.evidenceReferences.length === 1)).toBe(true);
  });

  it('keeps known catalog risk rows as policy candidates, not enforcement decisions', () => {
    const vpn = candidateFor('known-vpn-proxy-risk');

    expect(vpn.candidateState).toBe('catalogMatch');
    expect(vpn.confidence).toBe(0.94);
    expect(vpn.policyTargetKind).toBe(AppGamePolicyTargetKind.RiskApp);
    expect(vpn.notDirectEnforcement).toBe(true);
  });
}

function registerUnknownAndAiCandidateTests() {
  it('keeps unknown risklike names and hash-derived rows as candidates rather than facts', () => {
    const vpnName = candidateFor('unknown-vpn-name-candidate');
    const hashCandidate = candidateFor('unknown-publisher-hash-candidate');

    expect(vpnName.candidateState).toBe('heuristicCandidate');
    expect(vpnName.identityRef).toBeNull();
    expect(vpnName.confidence).toBeLessThanOrEqual(0.5);
    expect(hashCandidate.sourceKind).toBe('executableHash');
    expect(hashCandidate.policyCandidateAction).toBe('manualReview');
  });

  it('requires local AI app-risk candidates to cite a digest and stay review-routed', () => {
    const aiCandidate = candidateFor('local-ai-social-video-messaging-risk');
    const missingDigest = AppRiskDetectionCandidateSchema.safeParse({
      ...aiCandidate,
      localAiDigestRef: null,
    });

    expect(aiCandidate.sourceKind).toBe('localAiDigest');
    expect(aiCandidate.localAiDigestRef).toBe('local-ai-digest-social-video-messaging');
    expect(aiCandidate.askParentRouting).toBe('available');
    expect(missingDigest.success).toBe(false);
  });

  it('keeps parent display overrides from mutating raw app identity', () => {
    const parentOverride = candidateFor('parent-display-override-ai-tool');
    const rawIdentityMutation = AppRiskDetectionCandidateSchema.safeParse({
      ...parentOverride,
      parentOverride: { ...parentOverride.parentOverride, rawIdentityChanged: true },
    });

    expect(parentOverride.parentOverride?.parentDisplayLabel).toBe('Homework AI tool');
    expect(parentOverride.parentOverride?.rawIdentityChanged).toBe(false);
    expect(rawIdentityMutation.success).toBe(false);
  });
}

function registerSafetyAndSurfaceDisclosureTests() {
  it('rejects risk candidates that try to become direct enforcement', () => {
    const vpn = candidateFor('known-vpn-proxy-risk');

    expect(AppRiskDetectionCandidateSchema.safeParse({ ...vpn, notDirectEnforcement: false }).success).toBe(false);
    expect(AppRiskDetectionCandidateSchema.safeParse({ ...vpn, noContentClaim: false }).success).toBe(false);
    expect(AppRiskDetectionCandidateSchema.safeParse({ ...vpn, policyCandidateAction: 'block' }).success).toBe(false);
  });

  it('exposes parent-surface confidence, source evidence count, and no-content disclosure', () => {
    const matrix = AppRiskDetectionMatrixSchema.parse(AppRiskDetectionMatrix);

    expect(
      matrix.candidates.every(
        (candidate) =>
          candidate.surfaceDisclosure.confidencePercent === Math.round(candidate.confidence * 100) &&
          candidate.surfaceDisclosure.sourceEvidenceCount === candidate.evidenceReferences.length &&
          candidate.surfaceDisclosure.noContentClaimState === 'no-content-captured'
      )
    ).toBe(true);
  });

  it('routes risk app policy targets through category proof instead of direct action', () => {
    expect(appGamePolicyTargetRequiresCategory({ targetKind: AppGamePolicyTargetKind.RiskApp })).toBe(true);
  });
}
