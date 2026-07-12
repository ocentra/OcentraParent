import type {
  PortalLanDiscoveryEvidenceRecordProjection,
  PortalLanDiagnosticsRow,
  PortalLanHouseholdDeviceDecisionProjection,
  PortalLanTrustedDeviceRegistryEntryProjection,
} from './live-activity-lan-diagnostics';
import { formatSummaryText } from './live-activity-lan-diagnostics-text';

export function projectEvidenceRecordRow(record: PortalLanDiscoveryEvidenceRecordProjection): PortalLanDiagnosticsRow {
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

export function projectTrustedRegistryRow(
  entry: PortalLanTrustedDeviceRegistryEntryProjection
): PortalLanDiagnosticsRow {
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

export function projectHouseholdDecisionRow(
  decision: PortalLanHouseholdDeviceDecisionProjection
): PortalLanDiagnosticsRow {
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

function labelWithValue(label: string, value: string | null | undefined): string | null {
  return value === null || value === undefined || value.length === 0 ? null : `${label} ${value}`;
}
