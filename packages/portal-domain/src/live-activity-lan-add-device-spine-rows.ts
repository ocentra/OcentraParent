import type {
  GeneratedPortalLanSignedDiscoveryRelayCacheRowSnapshot,
  GeneratedPortalLanSignedDiscoveryRelayRouteSafetyRowSnapshot,
  GeneratedPortalLanSignedDiscoveryRelaySignedProofRowSnapshot,
} from './generated-portal-contracts';
import { isRecord, isString, notNull } from './live-activity-lan-add-device-shared';

export type PortalLanSignedProofRow = GeneratedPortalLanSignedDiscoveryRelaySignedProofRowSnapshot;
export type PortalLanRouteSafetyRow = GeneratedPortalLanSignedDiscoveryRelayRouteSafetyRowSnapshot;
export type PortalLanRelayCacheRow = GeneratedPortalLanSignedDiscoveryRelayCacheRowSnapshot;

export function normalizePortalLanSignedProofRows(value: unknown): readonly PortalLanSignedProofRow[] | null {
  if (!Array.isArray(value)) {
    return null;
  }

  const rows = value.map((entry) => normalizePortalLanSignedProofRow(entry)).filter(notNull);
  return rows.length === value.length ? rows : null;
}

export function normalizePortalLanRouteSafetyRows(value: unknown): readonly PortalLanRouteSafetyRow[] | null {
  if (!Array.isArray(value)) {
    return null;
  }

  const rows = value.map((entry) => normalizePortalLanRouteSafetyRow(entry)).filter(notNull);
  return rows.length === value.length ? rows : null;
}

export function normalizePortalLanRelayCacheRows(value: unknown): readonly PortalLanRelayCacheRow[] | null {
  if (!Array.isArray(value)) {
    return null;
  }

  const rows = value.map((entry) => normalizePortalLanRelayCacheRow(entry)).filter(notNull);
  return rows.length === value.length ? rows : null;
}

function normalizePortalLanSignedProofRow(value: unknown): PortalLanSignedProofRow | null {
  if (!isRecord(value)) {
    return null;
  }

  const raw = value as Record<string, unknown> & {
    check?: unknown;
    proofState?: unknown;
  };
  const { check, proofState } = raw;
  if (!isString(check) || !isString(proofState)) {
    return null;
  }

  return {
    ...raw,
    check,
    proofState,
  } as PortalLanSignedProofRow;
}

function normalizePortalLanRouteSafetyRow(value: unknown): PortalLanRouteSafetyRow | null {
  if (!isRecord(value)) {
    return null;
  }

  const raw = value as Record<string, unknown> & {
    check?: unknown;
    custodyLabel?: unknown;
  };
  const { check, custodyLabel } = raw;
  if (!isString(check) || !isString(custodyLabel)) {
    return null;
  }

  return {
    ...raw,
    check,
    custodyLabel,
  } as PortalLanRouteSafetyRow;
}

function normalizePortalLanRelayCacheRow(value: unknown): PortalLanRelayCacheRow | null {
  if (!isRecord(value)) {
    return null;
  }

  const raw = value as Record<string, unknown> & {
    check?: unknown;
    decisionState?: unknown;
  };
  const { check, decisionState } = raw;
  if (!isString(check) || !isString(decisionState)) {
    return null;
  }

  return {
    ...raw,
    check,
    decisionState,
  } as PortalLanRelayCacheRow;
}
