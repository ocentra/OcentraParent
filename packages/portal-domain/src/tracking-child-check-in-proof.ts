import { PortalDevTextToken, resolvePortalDevText, type DisplayText } from './display-text';
import { TrackingStatusProofArtifacts } from './tracking-status-proof-artifacts';
import { type TrackingStatusProofArtifact } from './portal-contract-text-contracts';

type PortalDisplayText = DisplayText;

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
    title: resolvePortalDevText(PortalDevTextToken.TrackingChildCheckInProofTitle),
    body: resolvePortalDevText(PortalDevTextToken.TrackingChildCheckInProofBody),
    proofTier: resolvePortalDevText(PortalDevTextToken.TrackingProofFixture),
    evidence: resolvePortalDevText(PortalDevTextToken.TrackingEvidenceUiFixture),
    proofArtifact: TrackingStatusProofArtifacts.ChildCheckIn,
    copyBoundary: resolvePortalDevText(PortalDevTextToken.TrackingChildCheckInCopyBoundary),
    safeAction: resolvePortalDevText(PortalDevTextToken.TrackingChildCheckInSafeAction),
    helpAction: resolvePortalDevText(PortalDevTextToken.TrackingChildCheckInHelpAction),
    shareLocationAction: resolvePortalDevText(PortalDevTextToken.TrackingChildCheckInShareLocationAction),
    callParentAction: resolvePortalDevText(PortalDevTextToken.TrackingChildCheckInCallParentAction),
    deliveryBoundary: resolvePortalDevText(PortalDevTextToken.TrackingChildCheckInDeliveryBoundary),
    missingProof: resolvePortalDevText(PortalDevTextToken.TrackingManualRequired),
    productClaim: resolvePortalDevText(PortalDevTextToken.TrackingNoProductClaim),
  };
}

export function trackingChildRuntimeUiProof(): TrackingChildRuntimeUiProof {
  return {
    title: resolvePortalDevText(PortalDevTextToken.TrackingChildRuntimeUiProofTitle),
    body: resolvePortalDevText(PortalDevTextToken.TrackingChildRuntimeUiProofBody),
    proofTier: resolvePortalDevText(PortalDevTextToken.TrackingProofService),
    evidence: resolvePortalDevText(PortalDevTextToken.TrackingEvidenceUiFixture),
    proofArtifact: TrackingStatusProofArtifacts.ChildRuntimeUi,
    disclosure: resolvePortalDevText(PortalDevTextToken.TrackingChildRuntimeDisclosure),
    safeResponse: resolvePortalDevText(PortalDevTextToken.TrackingChildRuntimeSafeResponse),
    helpResponse: resolvePortalDevText(PortalDevTextToken.TrackingChildRuntimeHelpResponse),
    locationShareConsent: resolvePortalDevText(PortalDevTextToken.TrackingChildRuntimeLocationConsent),
    runtimeBoundary: resolvePortalDevText(PortalDevTextToken.TrackingChildRuntimeBoundary),
    deliveryBoundary: resolvePortalDevText(PortalDevTextToken.TrackingChildCheckInDeliveryBoundary),
    missingProof: resolvePortalDevText(PortalDevTextToken.TrackingManualRequired),
    productClaim: resolvePortalDevText(PortalDevTextToken.TrackingNoProductClaim),
  };
}
