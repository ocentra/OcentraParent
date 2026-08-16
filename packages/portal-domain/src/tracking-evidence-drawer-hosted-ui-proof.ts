import { PortalDevTextToken, resolvePortalDevText, type DisplayText } from './display-text';
import {
  decodePortalDetailValue,
  type PortalDetailValue,
  type TrackingStatusProofArtifact,
} from './portal-contract-text-contracts';
import { PortalDetails } from './details';
import { TrackingStatusProofArtifacts } from './tracking-status-proof-artifacts';

type PortalDisplayText = DisplayText;

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
    title: resolvePortalDevText(PortalDevTextToken.TrackingEvidenceDrawerHostedUi),
    body: resolvePortalDevText(PortalDevTextToken.TrackingEvidenceDrawerHostedUiBody),
    proofTier: resolvePortalDevText(PortalDevTextToken.TrackingProofService),
    drawerMode: resolvePortalDevText(PortalDevTextToken.TrackingEvidenceDrawerReadOnly),
    sourceEventId: citation?.eventId ?? notReported(),
    evidenceReferences: citation?.evidenceReferences ?? notReported(),
    deletedEvidence: citation?.deletedEvidence ?? notReported(),
    proofArtifact: TrackingStatusProofArtifacts.HostedEvidenceDrawer,
    boundary: resolvePortalDevText(PortalDevTextToken.TrackingEvidenceDrawerBoundary),
    missingProof: resolvePortalDevText(PortalDevTextToken.TrackingManualRequired),
    productClaim: citation?.productClaim ?? resolvePortalDevText(PortalDevTextToken.TrackingNoProductClaim),
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
  return decodePortalDetailValue(resolvePortalDevText(PortalDevTextToken.NotReported));
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
