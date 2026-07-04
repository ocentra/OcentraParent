/* compatibility shim over Rust-generated app risk detection contracts */

import { type GeneratedAppRiskDetectionCandidate } from './generated/app-riskdetection-contracts';
import { appRiskDetectionCandidateIsHonestGenerated } from './generated/app-riskdetection-contract-rules';

export function appRiskDetectionCandidateIsHonest(
  candidate: GeneratedAppRiskDetectionCandidate
): boolean {
  return appRiskDetectionCandidateIsHonestGenerated(candidate);
}
