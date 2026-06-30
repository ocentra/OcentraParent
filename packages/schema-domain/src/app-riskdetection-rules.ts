/* compatibility shim over Rust-generated app risk detection contracts */

import {
  GeneratedAppRiskDetectionAskParentRouting,
  GeneratedAppRiskDetectionCandidateState,
  GeneratedAppRiskDetectionConfidenceBand,
  GeneratedAppRiskDetectionNoContentClaimState,
  GeneratedAppRiskDetectionPolicyCandidateAction,
  GeneratedAppRiskDetectionPolicyTargetKind,
  GeneratedAppRiskDetectionPublisherTrustState,
  GeneratedAppRiskDetectionRiskSignal,
  GeneratedAppRiskDetectionSourceKind,
  GeneratedAppRiskDetectionSurfaceState,
  type GeneratedAppRiskDetectionCandidate,
} from './generated/app-riskdetection-contracts';
import { appRiskDetectionCandidateIsHonestGenerated } from './generated/app-riskdetection-contract-rules';

export const AppRiskDetectionRiskSignal = GeneratedAppRiskDetectionRiskSignal;
export const AppRiskDetectionSourceKind = GeneratedAppRiskDetectionSourceKind;
export const AppRiskDetectionCandidateState = GeneratedAppRiskDetectionCandidateState;
export const AppRiskDetectionPublisherTrustState = GeneratedAppRiskDetectionPublisherTrustState;
export const AppRiskDetectionPolicyCandidateAction = GeneratedAppRiskDetectionPolicyCandidateAction;
export const AppRiskDetectionConfidenceBand = GeneratedAppRiskDetectionConfidenceBand;
export const AppRiskDetectionPolicyTargetKind = GeneratedAppRiskDetectionPolicyTargetKind;
export const AppRiskDetectionAskParentRouting = GeneratedAppRiskDetectionAskParentRouting;
export const AppRiskDetectionSurfaceState = GeneratedAppRiskDetectionSurfaceState;
export const AppRiskDetectionNoContentClaimState = GeneratedAppRiskDetectionNoContentClaimState;

export function appRiskDetectionCandidateIsHonest(candidate: GeneratedAppRiskDetectionCandidate): boolean {
  return appRiskDetectionCandidateIsHonestGenerated(candidate);
}
