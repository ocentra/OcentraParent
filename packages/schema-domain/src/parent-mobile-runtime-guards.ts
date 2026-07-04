import type { ParentMobileRuntimeReadModelCandidate } from './parent-mobile-runtime';
import {
  parentMobileCapabilityProofsAreConsistent,
  parentMobileControllerProofIsConsistent,
  parentMobileLanAiProviderJobIsConsistent,
  parentMobilePackageProofIsConsistent,
} from './parent-mobile-runtime-package-guards';
import { parentMobileRouteStatusesAreConsistent } from './parent-mobile-runtime-route-guards';

export function parentMobileRuntimeReadModelIsConsistent(readModel: ParentMobileRuntimeReadModelCandidate): boolean {
  if (readModel.packageProof.platform !== readModel.platform) {
    return false;
  }

  if (!parentMobilePackageProofIsConsistent(readModel.packageProof)) {
    return false;
  }

  if (readModel.localModelExecutionAllowed !== false) {
    return false;
  }

  if (readModel.childAgentBehaviorClaim !== 'not-claimed') {
    return false;
  }

  if (readModel.serviceAvailability.cloudRelay !== 'not-implemented') {
    return false;
  }

  if (!parentMobileRouteStatusesAreConsistent(readModel.serviceAvailability)) {
    return false;
  }

  if (!parentMobileControllerProofIsConsistent(readModel.controllerProof)) {
    return false;
  }

  if (!parentMobileCapabilityProofsAreConsistent(readModel)) {
    return false;
  }

  if (readModel.assistantJobProof.route === 'lan-ai-provider') {
    return parentMobileLanAiProviderJobIsConsistent(readModel.assistantJobProof);
  }

  return readModel.assistantJobProof.providerId === null && readModel.assistantJobProof.unavailableReason !== null;
}
