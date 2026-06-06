import type { AgentEventEnvelope } from '@ocentra-parent/agent-protocol-domain/contracts';
import { parseEnforcementSupportedAdapterRuntimeProofReadModel } from '@ocentra-parent/agent-protocol-domain/enforcement-supported-adapter-runtime-proof-adapter';
import {
  V08SupportedAdapterRuntimeBoundary,
  type V08SupportedAdapterRuntimeBoundary as V08SupportedAdapterRuntimeBoundaryValue,
  type V08SupportedAdapterRuntimeProofEntry,
  type V08SupportedAdapterRuntimeProofReadModel,
} from '@ocentra-parent/parent-domain/v0-8-supported-adapter-runtime-proof';
import {
  PortalFormatting,
  PortalText,
  PortalTextToken,
  decodePortalDetailValue,
  type PortalDetailValue,
} from '@ocentra-parent/portal-domain/contracts';

export type NetworkAdapterCapabilityStatusSummary = {
  readonly sourceReadModel: PortalDetailValue;
  readonly generatedAt: PortalDetailValue;
  readonly reportedRows: PortalDetailValue;
  readonly platformMatrix: PortalDetailValue;
  readonly observePolicyHandoff: PortalDetailValue;
  readonly hostDomainManualGate: PortalDetailValue;
  readonly hostDomainArtifactStatus: PortalDetailValue;
  readonly exactUrlCapability: PortalDetailValue;
  readonly degradedState: PortalDetailValue;
  readonly unavailableState: PortalDetailValue;
  readonly unsupportedState: PortalDetailValue;
  readonly manualProofRequirements: PortalDetailValue;
  readonly proofArtifacts: PortalDetailValue;
  readonly noClaimBoundary: PortalDetailValue;
};

export function parseNetworkAdapterCapabilityStatus(
  event: AgentEventEnvelope | null
): NetworkAdapterCapabilityStatusSummary | null {
  if (event === null) {
    return null;
  }

  const readModel = parseEnforcementSupportedAdapterRuntimeProofReadModel(event);
  if (readModel === null) {
    return null;
  }

  return networkAdapterCapabilityStatusSummary(readModel);
}

export function emptyNetworkAdapterCapabilityStatusSummary(): NetworkAdapterCapabilityStatusSummary {
  return {
    sourceReadModel: notReported(),
    generatedAt: notReported(),
    reportedRows: notReported(),
    platformMatrix: notReported(),
    observePolicyHandoff: notReported(),
    hostDomainManualGate: notReported(),
    hostDomainArtifactStatus: notReported(),
    exactUrlCapability: notReported(),
    degradedState: notReported(),
    unavailableState: notReported(),
    unsupportedState: notReported(),
    manualProofRequirements: notReported(),
    proofArtifacts: notReported(),
    noClaimBoundary: notReported(),
  };
}

function networkAdapterCapabilityStatusSummary(
  readModel: V08SupportedAdapterRuntimeProofReadModel
): NetworkAdapterCapabilityStatusSummary {
  const networkRows = networkCapabilityRows(readModel.entries);
  return {
    sourceReadModel: detailFromValue(readModel.readModelId),
    generatedAt: detailFromValue(readModel.generatedAt),
    reportedRows: detailFromValue(networkRows.length),
    platformMatrix: platformMatrixDetail(networkRows),
    observePolicyHandoff: entryDetail(
      rowForBoundary(networkRows, V08SupportedAdapterRuntimeBoundary.WindowsNetworkFlowObservePolicyHandoff)
    ),
    hostDomainManualGate: entryDetail(
      rowForBoundary(networkRows, V08SupportedAdapterRuntimeBoundary.WindowsHostNetworkDomainBlockingManualGate)
    ),
    hostDomainArtifactStatus: entryDetail(
      rowForBoundary(networkRows, V08SupportedAdapterRuntimeBoundary.WindowsHostNetworkDomainArtifactStatus)
    ),
    exactUrlCapability: entryDetail(
      rowForBoundary(networkRows, V08SupportedAdapterRuntimeBoundary.WindowsManagedExactActiveTabNotClaimed)
    ),
    degradedState: entryDetail(
      rowForBoundary(networkRows, V08SupportedAdapterRuntimeBoundary.WindowsAdapterPermissionDependencyDegraded)
    ),
    unavailableState: entryDetail(
      rowForBoundary(networkRows, V08SupportedAdapterRuntimeBoundary.LinuxHostAdapterUnavailable)
    ),
    unsupportedState: entryDetail(
      rowForBoundary(networkRows, V08SupportedAdapterRuntimeBoundary.MacosHostAdapterUnsupported)
    ),
    manualProofRequirements: joinedDetail(networkRows.flatMap((entry) => entry.manualProofRequirements)),
    proofArtifacts: joinedDetail(networkRows.flatMap((entry) => entry.linkedProofArtifacts)),
    noClaimBoundary: noClaimBoundaryDetail(networkRows),
  };
}

function networkCapabilityRows(
  entries: readonly V08SupportedAdapterRuntimeProofEntry[]
): readonly V08SupportedAdapterRuntimeProofEntry[] {
  const networkBoundaries = new Set([
    V08SupportedAdapterRuntimeBoundary.WindowsNetworkFlowObservePolicyHandoff,
    V08SupportedAdapterRuntimeBoundary.WindowsHostNetworkDomainBlockingManualGate,
    V08SupportedAdapterRuntimeBoundary.WindowsHostNetworkDomainArtifactStatus,
    V08SupportedAdapterRuntimeBoundary.WindowsManagedExactActiveTabNotClaimed,
    V08SupportedAdapterRuntimeBoundary.WindowsAdapterPermissionDependencyDegraded,
    V08SupportedAdapterRuntimeBoundary.LinuxHostAdapterUnavailable,
    V08SupportedAdapterRuntimeBoundary.MacosHostAdapterUnsupported,
    V08SupportedAdapterRuntimeBoundary.AndroidMobileControlManualGate,
    V08SupportedAdapterRuntimeBoundary.IosMobileControlManualGate,
  ]);
  return entries.filter((entry) => networkBoundaries.has(entry.runtimeBoundary));
}

function rowForBoundary(
  rows: readonly V08SupportedAdapterRuntimeProofEntry[],
  boundary: V08SupportedAdapterRuntimeBoundaryValue
): V08SupportedAdapterRuntimeProofEntry | null {
  return rows.find((entry) => entry.runtimeBoundary === boundary) ?? null;
}

function entryDetail(entry: V08SupportedAdapterRuntimeProofEntry | null): PortalDetailValue {
  if (entry === null) {
    return notReported();
  }

  return joinedDetail([
    entry.runtimeBoundary,
    entry.platform,
    entry.adapterCapability,
    entry.runtimeState,
    entry.adapterResult,
    entry.platformSupportState,
    entry.refusalReason,
  ]);
}

function platformMatrixDetail(rows: readonly V08SupportedAdapterRuntimeProofEntry[]): PortalDetailValue {
  return joinedDetail(rows.flatMap((entry) => [entry.platform, entry.runtimeState]));
}

function noClaimBoundaryDetail(rows: readonly V08SupportedAdapterRuntimeProofEntry[]): PortalDetailValue {
  if (rows.length === 0) {
    return notReported();
  }

  const claimUpgrade = rows.some(
    (entry) =>
      entry.broadInstalledAppBlockingClaimed ||
      entry.networkDomainBlockingClaimed ||
      entry.exactActiveTabEnforcementClaimed ||
      entry.notificationDeliveryClaimed ||
      entry.tamperHardeningClaimed ||
      entry.mobileControlClaimed ||
      entry.unsupportedPlatformBehaviorClaimed
  );
  return detailFromValue(claimUpgrade);
}

function joinedDetail(values: readonly unknown[]): PortalDetailValue {
  const normalized = values.filter(isReportedValue).map((value) => String(value));
  if (normalized.length === 0) {
    return notReported();
  }
  return decodePortalDetailValue(normalized.join(PortalFormatting.EventDetailSeparator));
}

function detailFromValue(value: unknown): PortalDetailValue {
  if (!isReportedValue(value)) {
    return notReported();
  }
  return decodePortalDetailValue(String(value));
}

function isReportedValue(value: unknown): boolean {
  return value !== undefined && value !== null && String(value).length > 0;
}

function notReported(): PortalDetailValue {
  return decodePortalDetailValue(PortalText.Resolve(PortalTextToken.NotReported));
}
