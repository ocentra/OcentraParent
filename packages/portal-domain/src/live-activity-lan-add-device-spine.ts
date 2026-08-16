import type {
  GeneratedPortalLanSignedDiscoveryRelayAdapterRowSnapshot,
  GeneratedPortalLanSignedDiscoveryRelaySpineSummarySnapshot,
} from './generated-portal-contracts';
import {
  isNumber,
  isRecord,
  isString,
  normalizeGeneratedArray,
  normalizeStringArray,
} from './live-activity-lan-add-device-shared';
import {
  normalizePortalLanRelayCacheRows,
  normalizePortalLanRouteSafetyRows,
  normalizePortalLanSignedProofRows,
  type PortalLanRelayCacheRow,
  type PortalLanRouteSafetyRow,
  type PortalLanSignedProofRow,
} from './live-activity-lan-add-device-spine-rows';

export type PortalLanSignedRelayAdapterRow = GeneratedPortalLanSignedDiscoveryRelayAdapterRowSnapshot;
export type PortalLanSignedDiscoveryRelaySpine = GeneratedPortalLanSignedDiscoveryRelaySpineSummarySnapshot;

export function normalizePortalLanSignedDiscoveryRelaySpine(value: unknown): PortalLanSignedDiscoveryRelaySpine | null {
  if (value === null || value === undefined) {
    return null;
  }
  if (!isRecord(value)) {
    return null;
  }

  const raw = value as Record<string, unknown> & {
    adapterRows?: unknown;
    claimsNotProved?: unknown;
    claimsProved?: unknown;
    generatedAt?: unknown;
    manualProofRequired?: unknown;
    notImplemented?: unknown;
    relayCacheRows?: unknown;
    routeSafetyRows?: unknown;
    schemaVersion?: unknown;
    signedProofRows?: unknown;
  };

  const normalizedFields = normalizePortalLanSignedDiscoveryRelaySpineFields(raw);
  if (normalizedFields === null) {
    return null;
  }

  return {
    ...raw,
    ...normalizedFields,
  } as PortalLanSignedDiscoveryRelaySpine;
}

type PortalLanSignedDiscoveryRelaySpineFields = {
  readonly schemaVersion: number;
  readonly generatedAt: string;
  readonly adapterRows: readonly PortalLanSignedRelayAdapterRow[];
  readonly signedProofRows: readonly PortalLanSignedProofRow[];
  readonly routeSafetyRows: readonly PortalLanRouteSafetyRow[];
  readonly relayCacheRows: readonly PortalLanRelayCacheRow[];
  readonly manualProofRequired: readonly string[];
  readonly notImplemented: readonly string[];
  readonly claimsProved: readonly string[];
  readonly claimsNotProved: readonly string[];
};

function normalizePortalLanSignedDiscoveryRelaySpineFields(
  raw: Record<string, unknown> & {
    adapterRows?: unknown;
    claimsNotProved?: unknown;
    claimsProved?: unknown;
    generatedAt?: unknown;
    manualProofRequired?: unknown;
    notImplemented?: unknown;
    relayCacheRows?: unknown;
    routeSafetyRows?: unknown;
    schemaVersion?: unknown;
    signedProofRows?: unknown;
  }
): PortalLanSignedDiscoveryRelaySpineFields | null {
  const { schemaVersion, generatedAt } = raw;
  const adapterRows = normalizeGeneratedArray<PortalLanSignedRelayAdapterRow>(raw.adapterRows);
  const signedProofRows = normalizePortalLanSignedProofRows(raw.signedProofRows);
  const routeSafetyRows = normalizePortalLanRouteSafetyRows(raw.routeSafetyRows);
  const relayCacheRows = normalizePortalLanRelayCacheRows(raw.relayCacheRows);
  const manualProofRequired = normalizeStringArray(raw.manualProofRequired);
  const notImplemented = normalizeStringArray(raw.notImplemented);
  const claimsProved = normalizeStringArray(raw.claimsProved);
  const claimsNotProved = normalizeStringArray(raw.claimsNotProved);
  if (
    !isNumber(schemaVersion) ||
    !isString(generatedAt) ||
    adapterRows === null ||
    signedProofRows === null ||
    routeSafetyRows === null ||
    relayCacheRows === null ||
    manualProofRequired === null ||
    notImplemented === null ||
    claimsProved === null ||
    claimsNotProved === null
  ) {
    return null;
  }

  return {
    schemaVersion,
    generatedAt,
    adapterRows,
    signedProofRows,
    routeSafetyRows,
    relayCacheRows,
    manualProofRequired,
    notImplemented,
    claimsProved,
    claimsNotProved,
  };
}
