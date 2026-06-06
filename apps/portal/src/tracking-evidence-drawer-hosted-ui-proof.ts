import {
  PortalDetails,
  PortalText,
  PortalTextToken,
  TrackingStatusProofArtifacts,
  decodePortalDetailValue,
  type PortalDetailValue,
  type PortalDisplayText,
  type TrackingStatusProofArtifact,
} from '@ocentra-parent/portal-domain/contracts';

export type TrackingEvidenceDrawerSourceCitation = {
  readonly eventId: PortalDetailValue;
  readonly evidenceReferences: PortalDetailValue;
  readonly deletedEvidence: PortalDetailValue;
  readonly productClaim: PortalDisplayText;
};

export type TrackingEvidenceDrawerHostedUiProof = {
  readonly title: PortalDisplayText;
  readonly body: PortalDisplayText;
  readonly proofTier: PortalDisplayText;
  readonly drawerMode: PortalDisplayText;
  readonly sourceEventId: PortalDetailValue;
  readonly evidenceReferences: PortalDetailValue;
  readonly deletedEvidence: PortalDetailValue;
  readonly proofArtifact: TrackingStatusProofArtifact;
  readonly boundary: PortalDisplayText;
  readonly missingProof: PortalDisplayText;
  readonly productClaim: PortalDisplayText;
  readonly policyEvaluatorClaimedRows: PortalDetailValue;
  readonly actionDispatchClaimedRows: PortalDetailValue;
  readonly childDeviceDeliveryClaimedRows: PortalDetailValue;
  readonly providerDeliveryClaimedRows: PortalDetailValue;
  readonly physicalDeviceClaimedRows: PortalDetailValue;
  readonly authorityClaimedRows: PortalDetailValue;
};

export function trackingEvidenceDrawerHostedUiProof(
  citation: TrackingEvidenceDrawerSourceCitation | null
): TrackingEvidenceDrawerHostedUiProof {
  return {
    title: PortalText.Resolve(PortalTextToken.TrackingEvidenceDrawerHostedUi),
    body: PortalText.Resolve(PortalTextToken.TrackingEvidenceDrawerHostedUiBody),
    proofTier: PortalText.Resolve(PortalTextToken.TrackingProofService),
    drawerMode: PortalText.Resolve(PortalTextToken.TrackingEvidenceDrawerReadOnly),
    sourceEventId: citation?.eventId ?? notReported(),
    evidenceReferences: citation?.evidenceReferences ?? notReported(),
    deletedEvidence: citation?.deletedEvidence ?? notReported(),
    proofArtifact: TrackingStatusProofArtifacts.HostedEvidenceDrawer,
    boundary: PortalText.Resolve(PortalTextToken.TrackingEvidenceDrawerBoundary),
    missingProof: PortalText.Resolve(PortalTextToken.TrackingManualRequired),
    productClaim: citation?.productClaim ?? PortalText.Resolve(PortalTextToken.TrackingNoProductClaim),
    policyEvaluatorClaimedRows: zeroRows(),
    actionDispatchClaimedRows: zeroRows(),
    childDeviceDeliveryClaimedRows: zeroRows(),
    providerDeliveryClaimedRows: zeroRows(),
    physicalDeviceClaimedRows: zeroRows(),
    authorityClaimedRows: zeroRows(),
  };
}

function zeroRows(): PortalDetailValue {
  return decodePortalDetailValue(String(0));
}

function notReported(): PortalDetailValue {
  return decodePortalDetailValue(PortalText.Resolve(PortalTextToken.NotReported));
}

export const TrackingEvidenceDrawerHostedUiProofDetails = {
  ActionDispatch: PortalDetails.AdapterDispatch,
  Authority: PortalDetails.Enforcement,
  Boundary: PortalDetails.AdapterBoundary,
  ChildDelivery: PortalDetails.ChildDelivery,
  DeletedEvidence: PortalDetails.DeletedEvidence,
  DrawerMode: PortalDetails.Status,
  EvidenceReferences: PortalDetails.EvidenceReferences,
  MissingProof: PortalDetails.MissingProof,
  PhysicalDevice: PortalDetails.Device,
  PolicyEvaluator: PortalDetails.PolicyReadiness,
  ProductClaim: PortalDetails.ProductClaim,
  ProofArtifact: PortalDetails.RuntimeReference,
  ProofTier: PortalDetails.ProofTier,
  ProviderDelivery: PortalDetails.Provider,
  SourceEvent: PortalDetails.EventId,
} as const;
