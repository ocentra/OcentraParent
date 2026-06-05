import {
  PortalText,
  PortalTextToken,
  TrackingStatusProofArtifacts,
  type PortalDisplayText,
  type TrackingStatusProofArtifact,
} from '@ocentra-parent/portal-domain/contracts';

export type TrackingChildCheckInProof = {
  readonly title: PortalDisplayText;
  readonly body: PortalDisplayText;
  readonly proofTier: PortalDisplayText;
  readonly evidence: PortalDisplayText;
  readonly proofArtifact: TrackingStatusProofArtifact;
  readonly copyBoundary: PortalDisplayText;
  readonly safeAction: PortalDisplayText;
  readonly helpAction: PortalDisplayText;
  readonly shareLocationAction: PortalDisplayText;
  readonly callParentAction: PortalDisplayText;
  readonly deliveryBoundary: PortalDisplayText;
  readonly missingProof: PortalDisplayText;
  readonly productClaim: PortalDisplayText;
};

export type TrackingChildRuntimeUiProof = {
  readonly title: PortalDisplayText;
  readonly body: PortalDisplayText;
  readonly proofTier: PortalDisplayText;
  readonly evidence: PortalDisplayText;
  readonly proofArtifact: TrackingStatusProofArtifact;
  readonly disclosure: PortalDisplayText;
  readonly safeResponse: PortalDisplayText;
  readonly helpResponse: PortalDisplayText;
  readonly locationShareConsent: PortalDisplayText;
  readonly runtimeBoundary: PortalDisplayText;
  readonly deliveryBoundary: PortalDisplayText;
  readonly missingProof: PortalDisplayText;
  readonly productClaim: PortalDisplayText;
};

export function trackingChildCheckInProof(): TrackingChildCheckInProof {
  return {
    title: PortalText.Resolve(PortalTextToken.TrackingChildCheckInProofTitle),
    body: PortalText.Resolve(PortalTextToken.TrackingChildCheckInProofBody),
    proofTier: PortalText.Resolve(PortalTextToken.TrackingProofFixture),
    evidence: PortalText.Resolve(PortalTextToken.TrackingEvidenceUiFixture),
    proofArtifact: TrackingStatusProofArtifacts.ChildCheckIn,
    copyBoundary: PortalText.Resolve(PortalTextToken.TrackingChildCheckInCopyBoundary),
    safeAction: PortalText.Resolve(PortalTextToken.TrackingChildCheckInSafeAction),
    helpAction: PortalText.Resolve(PortalTextToken.TrackingChildCheckInHelpAction),
    shareLocationAction: PortalText.Resolve(PortalTextToken.TrackingChildCheckInShareLocationAction),
    callParentAction: PortalText.Resolve(PortalTextToken.TrackingChildCheckInCallParentAction),
    deliveryBoundary: PortalText.Resolve(PortalTextToken.TrackingChildCheckInDeliveryBoundary),
    missingProof: PortalText.Resolve(PortalTextToken.TrackingManualRequired),
    productClaim: PortalText.Resolve(PortalTextToken.TrackingNoProductClaim),
  };
}

export function trackingChildRuntimeUiProof(): TrackingChildRuntimeUiProof {
  return {
    title: PortalText.Resolve(PortalTextToken.TrackingChildRuntimeUiProofTitle),
    body: PortalText.Resolve(PortalTextToken.TrackingChildRuntimeUiProofBody),
    proofTier: PortalText.Resolve(PortalTextToken.TrackingProofService),
    evidence: PortalText.Resolve(PortalTextToken.TrackingEvidenceUiFixture),
    proofArtifact: TrackingStatusProofArtifacts.ChildRuntimeUi,
    disclosure: PortalText.Resolve(PortalTextToken.TrackingChildRuntimeDisclosure),
    safeResponse: PortalText.Resolve(PortalTextToken.TrackingChildRuntimeSafeResponse),
    helpResponse: PortalText.Resolve(PortalTextToken.TrackingChildRuntimeHelpResponse),
    locationShareConsent: PortalText.Resolve(PortalTextToken.TrackingChildRuntimeLocationConsent),
    runtimeBoundary: PortalText.Resolve(PortalTextToken.TrackingChildRuntimeBoundary),
    deliveryBoundary: PortalText.Resolve(PortalTextToken.TrackingChildCheckInDeliveryBoundary),
    missingProof: PortalText.Resolve(PortalTextToken.TrackingManualRequired),
    productClaim: PortalText.Resolve(PortalTextToken.TrackingNoProductClaim),
  };
}
