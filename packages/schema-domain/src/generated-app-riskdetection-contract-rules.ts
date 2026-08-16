/* generated from crates/schema/src/app_risk_detection.rs */

import {
  GeneratedAppRiskDetectionCandidateState,
  GeneratedAppRiskDetectionNoContentClaimState,
  GeneratedAppRiskDetectionPolicyCandidateAction,
  GeneratedAppRiskDetectionSourceKind,
  type GeneratedAppRiskDetectionCandidate,
} from './generated-app-riskdetection-contracts';

export function appRiskDetectionCandidateIsHonestGenerated(candidate: GeneratedAppRiskDetectionCandidate): boolean {
  return (
    appRiskDetectionCandidateCitesEvidenceGenerated(candidate) &&
    appRiskDetectionCandidateStateMatchesSourceGenerated(candidate) &&
    appRiskDetectionUnknownPublisherLowersConfidenceGenerated(candidate) &&
    appRiskDetectionAiCandidateCitesDigestGenerated(candidate) &&
    appRiskDetectionParentOverrideIsDisplayOnlyGenerated(candidate) &&
    appRiskDetectionCandidateCannotDirectlyEnforceGenerated(candidate) &&
    appRiskDetectionSurfaceDisclosureMatchesEvidenceGenerated(candidate)
  );
}

function appRiskDetectionCandidateCitesEvidenceGenerated(candidate: GeneratedAppRiskDetectionCandidate): boolean {
  return candidate.evidenceReferences.length > 0 && candidate.sourceRefs.length > 0;
}

function appRiskDetectionCandidateStateMatchesSourceGenerated(candidate: GeneratedAppRiskDetectionCandidate): boolean {
  switch (candidate.sourceKind) {
    case GeneratedAppRiskDetectionSourceKind.KnownCatalog:
      return (
        candidate.candidateState === GeneratedAppRiskDetectionCandidateState.CatalogMatch && candidate.confidence >= 0.7
      );
    case GeneratedAppRiskDetectionSourceKind.ExecutableName:
    case GeneratedAppRiskDetectionSourceKind.PublisherMetadata:
    case GeneratedAppRiskDetectionSourceKind.ExecutableHash:
      return (
        candidate.candidateState === GeneratedAppRiskDetectionCandidateState.HeuristicCandidate ||
        candidate.candidateState === GeneratedAppRiskDetectionCandidateState.ParentReviewCandidate
      );
    case GeneratedAppRiskDetectionSourceKind.LocalAiDigest:
      return candidate.candidateState === GeneratedAppRiskDetectionCandidateState.AiCandidate;
    case GeneratedAppRiskDetectionSourceKind.ParentOverride:
      return candidate.candidateState === GeneratedAppRiskDetectionCandidateState.ParentDisplayOverride;
  }
}

function appRiskDetectionUnknownPublisherLowersConfidenceGenerated(
  candidate: GeneratedAppRiskDetectionCandidate
): boolean {
  if (candidate.publisherTrustState === 'knownPublisher' || candidate.publisherTrustState === 'parentTrusted') {
    return true;
  }

  return candidate.confidence <= 0.5;
}

function appRiskDetectionAiCandidateCitesDigestGenerated(candidate: GeneratedAppRiskDetectionCandidate): boolean {
  if (candidate.sourceKind !== GeneratedAppRiskDetectionSourceKind.LocalAiDigest) {
    return true;
  }

  return (
    candidate.localAiDigestRef !== null &&
    candidate.policyCandidateAction !== GeneratedAppRiskDetectionPolicyCandidateAction.None
  );
}

function appRiskDetectionParentOverrideIsDisplayOnlyGenerated(candidate: GeneratedAppRiskDetectionCandidate): boolean {
  if (candidate.sourceKind !== GeneratedAppRiskDetectionSourceKind.ParentOverride) {
    return true;
  }

  return candidate.parentOverride !== null && candidate.parentOverride.rawIdentityChanged === false;
}

function appRiskDetectionCandidateCannotDirectlyEnforceGenerated(
  candidate: GeneratedAppRiskDetectionCandidate
): boolean {
  return candidate.notDirectEnforcement && candidate.noContentClaim;
}

function appRiskDetectionSurfaceDisclosureMatchesEvidenceGenerated(
  candidate: GeneratedAppRiskDetectionCandidate
): boolean {
  return (
    candidate.surfaceDisclosure.sourceEvidenceCount === candidate.evidenceReferences.length &&
    candidate.surfaceDisclosure.confidencePercent === Math.round(candidate.confidence * 100) &&
    candidate.surfaceDisclosure.noContentClaimState === GeneratedAppRiskDetectionNoContentClaimState.NoContentCaptured
  );
}
