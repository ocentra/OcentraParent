import type {
  PortalLanDiscoveryEvidenceRecordProjection,
  PortalLanHouseholdDeviceDecisionProjection,
  PortalLanTrustedDeviceRegistryEntryProjection,
} from './live-activity-lan-diagnostics';
import { formatSummaryText } from './live-activity-lan-diagnostics-text';

export function portalLanEvidenceWindowSummary(records: readonly PortalLanDiscoveryEvidenceRecordProjection[]): string {
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

export function portalLanTrustedRegistrySummary(
  trustedRegistry: readonly PortalLanTrustedDeviceRegistryEntryProjection[]
): string {
  return formatSummaryText([
    trustedRegistry.length === 0 ? null : `${trustedRegistry.length} trusted routes`,
    labelWithValue('latest trust', maxIsoValue(trustedRegistry.map((entry) => entry.trustedAt))),
    labelWithValue('next expiry', minIsoValue(trustedRegistry.map((entry) => entry.expiresAt))),
  ]);
}

export function portalLanDecisionHistorySummary(
  decisions: readonly PortalLanHouseholdDeviceDecisionProjection[]
): string {
  return formatSummaryText([
    decisions.length === 0 ? null : `${decisions.length} parent decisions`,
    labelWithValue('latest', decisions[0]?.decidedAt ?? null),
    summarizeCounts(decisions.map((decision) => decision.actionKind)),
  ]);
}

function labelWithValue(label: string, value: string | null | undefined): string | null {
  return value === null || value === undefined || value.length === 0 ? null : `${label} ${value}`;
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

function minIsoValue(values: readonly string[]): string | null {
  return values.length === 0 ? null : ([...values].sort()[0] ?? null);
}

function maxIsoValue(values: readonly string[]): string | null {
  return values.length === 0 ? null : ([...values].sort().at(-1) ?? null);
}

function compactStringValues(values: readonly (string | null | undefined)[]): readonly string[] {
  return values.filter((value): value is string => typeof value === 'string');
}
