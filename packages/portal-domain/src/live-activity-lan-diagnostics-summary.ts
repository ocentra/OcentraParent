import { PortalDevTextToken, resolvePortalDevText } from './display-text';
import type {
  PortalLanProductionHouseholdProofProjection,
  PortalLanRelayCacheRowProjection,
  PortalLanRouteSafetyRowProjection,
  PortalLanSignedProofRowProjection,
} from './live-activity-lan-diagnostics';
import { formatSummaryText } from './live-activity-lan-diagnostics-text';

export function portalLanProductionProofSummary(proof: PortalLanProductionHouseholdProofProjection | null): string {
  if (proof === null) {
    return resolvePortalDevText(PortalDevTextToken.NotReported);
  }
  return formatSummaryText([
    `${proof.manualProofRequired.length} manual proof required`,
    `${proof.notImplemented.length} not implemented`,
    `${proof.claimsProved.length} claims proved`,
    `${proof.claimsNotProved.length} claims not proved`,
  ]);
}

export function portalLanSignedProofSummary(rows: readonly PortalLanSignedProofRowProjection[]): string {
  return formatSummaryText([
    rows.length === 0 ? null : `${rows.length} signed proof rows`,
    summarizeCounts(rows.map((row) => row.proofState)),
    summarizeCounts(rows.map((row) => row.responseState ?? null)),
  ]);
}

export function portalLanRouteSafetySummary(rows: readonly PortalLanRouteSafetyRowProjection[]): string {
  return formatSummaryText([
    rows.length === 0 ? null : `${rows.length} route safety rows`,
    summarizeCounts(rows.map((row) => row.responseState ?? null)),
    optionalSummaryText(rows.map((row) => row.custodyLabel)),
  ]);
}

export function portalLanRelayCacheSummary(rows: readonly PortalLanRelayCacheRowProjection[]): string {
  return formatSummaryText([
    rows.length === 0 ? null : `${rows.length} relay cache rows`,
    summarizeCounts(rows.map((row) => row.decisionState)),
    optionalSummaryText(compactStringValues(rows.map((row) => row.custodyLabel ?? null))),
  ]);
}

function optionalSummaryText(values: readonly (string | null | undefined)[]): string | null {
  const normalized = [
    ...new Set(values.filter((value): value is string => value !== null && value !== undefined && value.length > 0)),
  ];
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

function compactStringValues(values: readonly (string | null | undefined)[]): readonly string[] {
  return values.filter((value): value is string => typeof value === 'string');
}
