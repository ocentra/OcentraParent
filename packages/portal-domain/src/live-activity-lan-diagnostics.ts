import { PortalDevTextToken, resolvePortalDevText } from '@ocentra-parent/schema-domain/text-portal-dev';

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

interface PortalLanTrustedDeviceRegistryEntryProjection {
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

interface PortalLanHouseholdDeviceDecisionProjection {
  readonly actionKind: string;
  readonly canonicalDeviceId: string;
  readonly childProfileId: string | null;
  readonly displayName: string | null;
  readonly deviceKind: string | null;
  readonly decidedAt: string;
  readonly revokedAt: string | null;
}

interface PortalLanProductionHouseholdProofProjection {
  readonly manualProofRequired: readonly unknown[];
  readonly notImplemented: readonly unknown[];
  readonly claimsProved: readonly unknown[];
  readonly claimsNotProved: readonly unknown[];
}

interface PortalLanSignedProofRowProjection {
  readonly proofState: string;
  readonly responseState?: string | null;
}

interface PortalLanRouteSafetyRowProjection {
  readonly custodyLabel: string;
  readonly responseState?: string | null;
}

interface PortalLanRelayCacheRowProjection {
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

function projectPortalLanDiagnosticsParts(readModel: PortalLanDiagnosticsReadModel): PortalLanDiagnosticsProjectionParts {
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

function portalLanEvidenceWindowSummary(records: readonly PortalLanDiscoveryEvidenceRecordProjection[]): string {
  return formatSummaryText([
    records.length === 0 ? null : `${records.length} evidence records`,
    labelWithValue('first', minIsoValue(records.map((record) => record.firstSeenAt))),
    labelWithValue('latest', maxIsoValue(records.map((record) => record.lastSeenAt))),
    labelWithValue(
      'next expiry',
      minIsoValue(compactStringValues(records.map((record) => record.expiresAt))) ??
        (records.length === 0 ? null : 'no-expiry')
    ),
  ]);
}

function portalLanTrustedRegistrySummary(
  trustedRegistry: readonly PortalLanTrustedDeviceRegistryEntryProjection[]
): string {
  return formatSummaryText([
    trustedRegistry.length === 0 ? null : `${trustedRegistry.length} trusted routes`,
    labelWithValue('latest trust', maxIsoValue(trustedRegistry.map((entry) => entry.trustedAt))),
    labelWithValue('next expiry', minIsoValue(trustedRegistry.map((entry) => entry.expiresAt))),
  ]);
}

function portalLanDecisionHistorySummary(
  decisions: readonly PortalLanHouseholdDeviceDecisionProjection[]
): string {
  return formatSummaryText([
    decisions.length === 0 ? null : `${decisions.length} parent decisions`,
    labelWithValue('latest', decisions[0]?.decidedAt ?? null),
    summarizeCounts(decisions.map((decision) => decision.actionKind)),
  ]);
}

function portalLanProductionProofSummary(proof: PortalLanProductionHouseholdProofProjection | null): string {
  if (proof === null) {
    return notReportedText();
  }
  return formatSummaryText([
    `${proof.manualProofRequired.length} manual proof required`,
    `${proof.notImplemented.length} not implemented`,
    `${proof.claimsProved.length} claims proved`,
    `${proof.claimsNotProved.length} claims not proved`,
  ]);
}

function portalLanSignedProofSummary(rows: readonly PortalLanSignedProofRowProjection[]): string {
  return formatSummaryText([
    rows.length === 0 ? null : `${rows.length} signed proof rows`,
    summarizeCounts(rows.map((row) => row.proofState)),
    summarizeCounts(rows.map((row) => row.responseState ?? null)),
  ]);
}

function portalLanRouteSafetySummary(rows: readonly PortalLanRouteSafetyRowProjection[]): string {
  return formatSummaryText([
    rows.length === 0 ? null : `${rows.length} route safety rows`,
    summarizeCounts(rows.map((row) => row.responseState ?? null)),
    optionalSummaryText(rows.map((row) => row.custodyLabel)),
  ]);
}

function portalLanRelayCacheSummary(rows: readonly PortalLanRelayCacheRowProjection[]): string {
  return formatSummaryText([
    rows.length === 0 ? null : `${rows.length} relay cache rows`,
    summarizeCounts(rows.map((row) => row.decisionState)),
    optionalSummaryText(compactStringValues(rows.map((row) => row.custodyLabel ?? null))),
  ]);
}

function projectEvidenceRecordRow(record: PortalLanDiscoveryEvidenceRecordProjection): PortalLanDiagnosticsRow {
  return {
    label: `${record.source} | ${record.evidenceKind}`,
    value: formatSummaryText([
      record.value,
      labelWithValue('first', record.firstSeenAt),
      labelWithValue('latest', record.lastSeenAt),
      labelWithValue('expiry', record.expiresAt ?? 'no-expiry'),
      record.confidence,
      record.note,
    ]),
  };
}

function projectTrustedRegistryRow(entry: PortalLanTrustedDeviceRegistryEntryProjection): PortalLanDiagnosticsRow {
  return {
    label: entry.pairingId,
    value: formatSummaryText([
      entry.childDevice.label,
      entry.trustState,
      entry.routeId,
      labelWithValue('trusted', entry.trustedAt),
      labelWithValue('expiry', entry.expiresAt),
      labelWithValue('revoked', entry.revokedAt ?? 'active'),
    ]),
  };
}

function projectHouseholdDecisionRow(decision: PortalLanHouseholdDeviceDecisionProjection): PortalLanDiagnosticsRow {
  return {
    label: `${decision.actionKind} | ${decision.displayName ?? decision.canonicalDeviceId}`,
    value: formatSummaryText([
      decision.deviceKind,
      decision.childProfileId,
      labelWithValue('decided', decision.decidedAt),
      labelWithValue('revoked', decision.revokedAt ?? 'active'),
    ]),
  };
}

function formatSummaryText(values: readonly (string | null | undefined)[]): string {
  return optionalSummaryText(values) ?? notReportedText();
}

function optionalSummaryText(values: readonly (string | null | undefined)[]): string | null {
  const normalized = uniqueStrings(
    values.filter((value): value is string => value !== null && value !== undefined && value.length > 0)
  );
  return normalized.length === 0 ? null : normalized.join(' | ');
}

function summarizeCounts(values: readonly (string | null | undefined)[]): string | null {
  const counts = new Map<string, number>();
  for (const value of values) {
    if (value === null || value === undefined || value.length === 0) {
      continue;
    }
    counts.set(value, (counts.get(value) ?? 0) + 1);
  }
  if (counts.size === 0) {
    return null;
  }
  return [...counts.entries()].map(([value, count]) => `${count} ${value}`).join(' | ');
}

function labelWithValue(label: string, value: string | null | undefined): string | null {
  return value === null || value === undefined || value.length === 0 ? null : `${label} ${value}`;
}

function notReportedText(): string {
  return resolvePortalDevText(PortalDevTextToken.NotReported);
}

function minIsoValue(values: readonly string[]): string | null {
  return values.length === 0 ? null : [...values].sort()[0] ?? null;
}

function maxIsoValue(values: readonly string[]): string | null {
  return values.length === 0 ? null : [...values].sort().at(-1) ?? null;
}

function compareIsoDesc(left: string, right: string): number {
  return right.localeCompare(left);
}

function uniqueStrings(values: readonly string[]): readonly string[] {
  return [...new Set(values)];
}

function compactStringValues(values: readonly (string | null | undefined)[]): readonly string[] {
  return values.filter((value): value is string => typeof value === 'string');
}
