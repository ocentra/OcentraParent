import { formatSummaryText, compareIsoDesc, uniqueStrings } from './live-activity-lan-diagnostics-text';
import {
  portalLanDecisionHistorySummary,
  portalLanEvidenceWindowSummary,
  portalLanTrustedRegistrySummary,
} from './live-activity-lan-diagnostics-formatters';
import {
  portalLanProductionProofSummary,
  portalLanRelayCacheSummary,
  portalLanRouteSafetySummary,
  portalLanSignedProofSummary,
} from './live-activity-lan-diagnostics-summary';
import {
  projectEvidenceRecordRow,
  projectHouseholdDecisionRow,
  projectTrustedRegistryRow,
} from './live-activity-lan-diagnostics-rows';

export interface PortalLanDiscoveryEvidenceRecordProjection {
  readonly source: string;
  readonly evidenceKind: string;
  readonly value: string;
  readonly firstSeenAt: string;
  readonly lastSeenAt: string;
  readonly expiresAt: string | null;
  readonly confidence: string;
  readonly note: string | null;
}

export interface PortalLanDiagnosticsRow {
  readonly label: string;
  readonly value: string;
}

export interface PortalLanDiagnosticsViewModel {
  readonly evidenceWindowSummary: string;
  readonly trustedRegistrySummary: string;
  readonly decisionHistorySummary: string;
  readonly policyTargetSurfaceSummary: string;
  readonly productionProofSummary: string;
  readonly signedProofSummary: string;
  readonly routeSafetySummary: string;
  readonly relayCacheSummary: string;
  readonly evidenceRecordRows: readonly PortalLanDiagnosticsRow[];
  readonly trustedRegistryRows: readonly PortalLanDiagnosticsRow[];
  readonly decisionRows: readonly PortalLanDiagnosticsRow[];
}

interface PortalLanCanonicalHouseholdDeviceProjection {
  readonly policyTargetSurfaces?: readonly string[];
  readonly networkIdentity: {
    readonly evidenceRecords?: readonly PortalLanDiscoveryEvidenceRecordProjection[];
  };
}

export interface PortalLanTrustedDeviceRegistryEntryProjection {
  readonly pairingId: string;
  readonly routeId: string;
  readonly trustState: string;
  readonly trustedAt: string;
  readonly expiresAt: string;
  readonly revokedAt: string | null;
  readonly childDevice: {
    readonly label: string;
  };
}

export interface PortalLanHouseholdDeviceDecisionProjection {
  readonly actionKind: string;
  readonly canonicalDeviceId: string;
  readonly childProfileId: string | null;
  readonly displayName: string | null;
  readonly deviceKind: string | null;
  readonly decidedAt: string;
  readonly revokedAt: string | null;
}

export interface PortalLanProductionHouseholdProofProjection {
  readonly manualProofRequired: readonly unknown[];
  readonly notImplemented: readonly unknown[];
  readonly claimsProved: readonly unknown[];
  readonly claimsNotProved: readonly unknown[];
}

export interface PortalLanSignedProofRowProjection {
  readonly proofState: string;
  readonly responseState?: string | null;
}

export interface PortalLanRouteSafetyRowProjection {
  readonly custodyLabel: string;
  readonly responseState?: string | null;
}

export interface PortalLanRelayCacheRowProjection {
  readonly decisionState: string;
  readonly custodyLabel?: string | null;
}

interface PortalLanSignedDiscoveryRelaySpineProjection {
  readonly signedProofRows: readonly PortalLanSignedProofRowProjection[];
  readonly routeSafetyRows: readonly PortalLanRouteSafetyRowProjection[];
  readonly relayCacheRows: readonly PortalLanRelayCacheRowProjection[];
}

export interface PortalLanDiagnosticsReadModel {
  readonly canonicalHouseholdDevices: readonly PortalLanCanonicalHouseholdDeviceProjection[];
  readonly trustedDeviceRegistry: readonly PortalLanTrustedDeviceRegistryEntryProjection[];
  readonly householdDeviceDecisions: readonly PortalLanHouseholdDeviceDecisionProjection[];
  readonly productionHouseholdProof?: PortalLanProductionHouseholdProofProjection | null;
  readonly signedDiscoveryRelaySpine: PortalLanSignedDiscoveryRelaySpineProjection | null;
}

type PortalLanDiagnosticsProjectionParts = {
  readonly evidenceRecords: readonly PortalLanDiscoveryEvidenceRecordProjection[];
  readonly trustedRegistry: readonly PortalLanTrustedDeviceRegistryEntryProjection[];
  readonly householdDeviceDecisions: readonly PortalLanHouseholdDeviceDecisionProjection[];
  readonly policyTargetSurfaces: readonly string[];
  readonly productionHouseholdProof: PortalLanProductionHouseholdProofProjection | null;
  readonly signedProofRows: readonly PortalLanSignedProofRowProjection[];
  readonly routeSafetyRows: readonly PortalLanRouteSafetyRowProjection[];
  readonly relayCacheRows: readonly PortalLanRelayCacheRowProjection[];
};

export function projectPortalLanDiagnosticsViewModel(
  readModel: PortalLanDiagnosticsReadModel | null
): PortalLanDiagnosticsViewModel | null {
  if (readModel === null) {
    return null;
  }

  const parts = projectPortalLanDiagnosticsParts(readModel);
  return {
    evidenceWindowSummary: portalLanEvidenceWindowSummary(parts.evidenceRecords),
    trustedRegistrySummary: portalLanTrustedRegistrySummary(parts.trustedRegistry),
    decisionHistorySummary: portalLanDecisionHistorySummary(parts.householdDeviceDecisions),
    policyTargetSurfaceSummary: formatSummaryText(parts.policyTargetSurfaces),
    productionProofSummary: portalLanProductionProofSummary(parts.productionHouseholdProof),
    signedProofSummary: portalLanSignedProofSummary(parts.signedProofRows),
    routeSafetySummary: portalLanRouteSafetySummary(parts.routeSafetyRows),
    relayCacheSummary: portalLanRelayCacheSummary(parts.relayCacheRows),
    evidenceRecordRows: parts.evidenceRecords
      .slice()
      .sort((left, right) => compareIsoDesc(left.lastSeenAt, right.lastSeenAt))
      .slice(0, 4)
      .map(projectEvidenceRecordRow),
    trustedRegistryRows: parts.trustedRegistry.slice(0, 4).map(projectTrustedRegistryRow),
    decisionRows: parts.householdDeviceDecisions.slice(0, 4).map(projectHouseholdDecisionRow),
  };
}

function projectPortalLanDiagnosticsParts(
  readModel: PortalLanDiagnosticsReadModel
): PortalLanDiagnosticsProjectionParts {
  const trustedRegistry = [...readModel.trustedDeviceRegistry].sort((left, right) =>
    compareIsoDesc(left.trustedAt, right.trustedAt)
  );
  const householdDeviceDecisions = [...readModel.householdDeviceDecisions].sort((left, right) =>
    compareIsoDesc(left.decidedAt, right.decidedAt)
  );
  return {
    evidenceRecords: readModel.canonicalHouseholdDevices.flatMap(
      (device) => device.networkIdentity.evidenceRecords ?? []
    ),
    trustedRegistry,
    householdDeviceDecisions,
    policyTargetSurfaces: uniqueStrings(
      readModel.canonicalHouseholdDevices.flatMap((device) => device.policyTargetSurfaces ?? [])
    ),
    productionHouseholdProof: readModel.productionHouseholdProof ?? null,
    signedProofRows: readModel.signedDiscoveryRelaySpine?.signedProofRows ?? [],
    routeSafetyRows: readModel.signedDiscoveryRelaySpine?.routeSafetyRows ?? [],
    relayCacheRows: readModel.signedDiscoveryRelaySpine?.relayCacheRows ?? [],
  };
}
