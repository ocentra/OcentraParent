import { PortalDevTextToken, resolvePortalDevText } from './display-text';
import {
  decodePortalDetailValue,
  type PortalDetailValue,
  type TrackingStatusProofArtifact,
} from './portal-contract-text-contracts';
import { PortalFormatting } from './formatting';
import type {
  TrackingFamilyDashboardHostedRollupRow,
  TrackingStatusProofRow,
  TrackingUnsupportedManualPlatformRow,
} from './tracking-status-panel';

type PortalTextTokenValue = (typeof PortalDevTextToken)[keyof typeof PortalDevTextToken];

type TrackingStatusRetentionProofDefinition = {
  readonly historyVisibility: PortalTextTokenValue;
  readonly deletedEvidence: PortalTextTokenValue;
};

type TrackingStatusProofRowDefinition = {
  readonly titleToken: PortalTextTokenValue;
  readonly evidenceToken: PortalTextTokenValue;
  readonly proofArtifact: TrackingStatusProofArtifact;
  readonly retentionProof?: TrackingStatusRetentionProofDefinition;
};

type TrackingUnsupportedManualPlatformDefinition = {
  readonly titleToken: PortalTextTokenValue;
  readonly supportStateToken: PortalTextTokenValue;
  readonly renderedStateToken: PortalTextTokenValue;
};

type TrackingFamilyDashboardHostedRollupDefinition = {
  readonly titleToken: PortalTextTokenValue;
  readonly evidenceToken: PortalTextTokenValue;
  readonly visibleChildren: number;
  readonly attentionItems: number;
  readonly retainedAuditItems: number;
};

export function row(definition: TrackingStatusProofRowDefinition): TrackingStatusProofRow {
  const { titleToken, evidenceToken, proofArtifact } = definition;
  const baseRow = {
    title: resolvePortalDevText(titleToken),
    state: resolvePortalDevText(titleToken),
    proofTier: resolvePortalDevText(PortalDevTextToken.TrackingProofFixture),
    evidence: resolvePortalDevText(evidenceToken),
    proofArtifact,
    missingProof: missingProofForEvidence(evidenceToken),
    productClaim: resolvePortalDevText(PortalDevTextToken.TrackingNoProductClaim),
  };
  const retentionProof = definition.retentionProof;
  if (retentionProof === undefined) {
    return baseRow;
  }
  return {
    ...baseRow,
    historyVisibility: resolvePortalDevText(retentionProof.historyVisibility),
    deletedEvidence: resolvePortalDevText(retentionProof.deletedEvidence),
  };
}

export function unsupportedManualRow(
  definition: TrackingUnsupportedManualPlatformDefinition
): TrackingUnsupportedManualPlatformRow {
  return {
    title: resolvePortalDevText(definition.titleToken),
    supportState: resolvePortalDevText(definition.supportStateToken),
    renderedState: resolvePortalDevText(definition.renderedStateToken),
  };
}

export function familyDashboardRollupRow(
  definition: TrackingFamilyDashboardHostedRollupDefinition
): TrackingFamilyDashboardHostedRollupRow {
  return {
    title: resolvePortalDevText(definition.titleToken),
    status: resolvePortalDevText(PortalDevTextToken.TrackingFamilyDashboardRollupReady),
    visibleChildren: detailFromValue(definition.visibleChildren),
    attentionItems: detailFromValue(definition.attentionItems),
    retainedAuditItems: detailFromValue(definition.retainedAuditItems),
    evidence: resolvePortalDevText(definition.evidenceToken),
  };
}

export function renderedStateCount(
  rows: readonly TrackingUnsupportedManualPlatformRow[],
  renderedStateToken: PortalTextTokenValue
): PortalDetailValue {
  const renderedState = resolvePortalDevText(renderedStateToken);
  return detailFromValue(rows.filter((rowValue) => rowValue.renderedState === renderedState).length);
}

export function missingProofForEvidence(evidenceToken: PortalTextTokenValue): string {
  if (evidenceToken === PortalDevTextToken.TrackingEvidencePhysicalMissing) {
    return resolvePortalDevText(PortalDevTextToken.TrackingPhysicalDeviceRequired);
  }
  return resolvePortalDevText(PortalDevTextToken.TrackingManualRequired);
}

export function detailFromValue(value: unknown): PortalDetailValue {
  if (value === undefined || value === null) {
    return notReported();
  }
  return decodePortalDetailValue(String(value));
}

export function notReported(): PortalDetailValue {
  return detailFromValue(resolvePortalDevText(PortalDevTextToken.NotReported));
}

export function preferredActiveSummaryDetail(activeValue: unknown, legacyValue: unknown): PortalDetailValue {
  return detailFromValue(activeValue ?? legacyValue);
}

export function evidenceReferenceDetail(evidenceReferenceIds: readonly string[] | undefined): PortalDetailValue {
  if (evidenceReferenceIds === undefined || evidenceReferenceIds.length === 0) {
    return notReported();
  }
  return detailFromValue(evidenceReferenceIds.join(PortalFormatting.EventDetailSeparator));
}
