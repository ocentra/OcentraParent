import {
  projectPortalLanDiagnosticsViewModel as projectPortalLanDiagnosticsViewModelFromReadModel,
  type PortalLanDiagnosticsReadModel,
  type PortalLanDiagnosticsRow as PortalLanDiagnosticsRowSource,
  type PortalLanDiagnosticsViewModel as PortalLanDiagnosticsViewModelSource,
} from './live-activity-lan-diagnostics';
import {
  normalizePortalLanAddDeviceReadModel as normalizePortalLanAddDeviceReadModelFromValue,
  type PortalLanAddDeviceReadModel as PortalLanAddDeviceReadModelSource,
} from './live-activity-lan-add-device-records';
import type { PortalRouteEventPayloadRecord } from './portal-contract-adapter';

export type PortalLanAddDeviceReadModel = PortalLanAddDeviceReadModelSource;
export type PortalLanDiagnosticsRow = PortalLanDiagnosticsRowSource;
export type PortalLanDiagnosticsViewModel = PortalLanDiagnosticsViewModelSource;

const PortalLanAddDeviceRouteSnapshotField = 'lanAddDeviceReadModel';

export function parsePortalLanAddDeviceReadModel(
  payload: PortalRouteEventPayloadRecord | unknown
): PortalLanAddDeviceReadModel | null {
  const readModel = extractPortalLanAddDeviceReadModel(payload);
  return normalizePortalLanAddDeviceReadModelFromValue(readModel);
}

function extractPortalLanAddDeviceReadModel(value: unknown): unknown {
  const parsed = parseJsonRecord(value);
  if (!isRecord(parsed)) {
    return parsed;
  }
  return parsed[PortalLanAddDeviceRouteSnapshotField] ?? parsed;
}

function parseJsonRecord(value: unknown): unknown {
  if (typeof value !== 'string') {
    return value;
  }

  try {
    return JSON.parse(String(value));
  } catch {
    return null;
  }
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === 'object' && value !== null;
}

export function projectPortalLanDiagnosticsViewModel(
  readModel: PortalLanDiagnosticsReadModel | null
): PortalLanDiagnosticsViewModel | null {
  return projectPortalLanDiagnosticsViewModelFromReadModel(readModel);
}
