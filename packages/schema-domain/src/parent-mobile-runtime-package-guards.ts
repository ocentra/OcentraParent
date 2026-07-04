import {
  AndroidParentMobileCapabilityStatuses,
  IosParentMobileCapabilityStatuses,
} from './parent-mobile-runtime-capability-statuses';
import type {
  ParentMobileAssistantJobProof,
  ParentMobileControllerProof,
  ParentMobileRuntimeReadModelCandidate,
} from './parent-mobile-runtime';

const ParentMobileControllerStateExpectations = {
  'active-controller': {
    controllerLease: 'required',
    takeoverRequestAllowed: true,
    commandAuthorityState: 'active-controller-backend-proof',
    requestBoundary: 'backend-controller-owned',
  },
  observer: {
    controllerLease: 'absent',
    takeoverRequestAllowed: false,
    commandAuthorityState: 'observer-read-only',
    requestBoundary: 'observer-read-only',
  },
  'manual-required': {
    controllerLease: 'absent',
    takeoverRequestAllowed: true,
    commandAuthorityState: 'controller-takeover-manual-required',
    requestBoundary: 'request-first-manual-required',
  },
  unavailable: {
    controllerLease: 'absent',
    takeoverRequestAllowed: false,
    commandAuthorityState: 'unavailable',
    requestBoundary: 'unavailable',
  },
} as const;

export function parentMobilePackageProofIsConsistent(
  packageProof: ParentMobileRuntimeReadModelCandidate['packageProof']
): boolean {
  return packageProof.packageLifecycleState ===
    (packageProof.packageState === 'unavailable' ? 'unavailable' : 'manual-required');
}

export function parentMobileControllerProofIsConsistent(controllerProof: ParentMobileControllerProof): boolean {
  const expected = ParentMobileControllerStateExpectations[controllerProof.controllerState];

  return (
    (expected.controllerLease === 'required'
      ? controllerProof.controllerLeaseId !== null
      : controllerProof.controllerLeaseId === null) &&
    controllerProof.takeoverRequestAllowed === expected.takeoverRequestAllowed &&
    controllerProof.commandAuthorityState === expected.commandAuthorityState &&
    controllerProof.requestBoundary === expected.requestBoundary
  );
}

export function parentMobileLanAiProviderJobIsConsistent(assistantJobProof: ParentMobileAssistantJobProof): boolean {
  return (
    assistantJobProof.requiredCapabilities.length > 0 &&
    (assistantJobProof.jobState === 'submitted'
      ? assistantJobProof.providerId !== null && assistantJobProof.unavailableReason === null
      : assistantJobProof.providerId === null && assistantJobProof.unavailableReason !== null)
  );
}

export function parentMobileCapabilityProofsAreConsistent(
  readModel: ParentMobileRuntimeReadModelCandidate
): boolean {
  const expected =
    readModel.platform === 'android' ? AndroidParentMobileCapabilityStatuses : IosParentMobileCapabilityStatuses;
  const capabilityStatuses = new Map(
    readModel.platformCapabilities.map((entry) => [entry.capability, entry.status] as const)
  );

  return (
    capabilityStatuses.size === readModel.platformCapabilities.length &&
    capabilityStatuses.size === expected.length &&
    !readModel.platformCapabilities.some((entry) => entry.status === 'supported' || entry.status === 'implemented') &&
    expected.every(([capability, status]) => capabilityStatuses.get(capability) === status)
  );
}
