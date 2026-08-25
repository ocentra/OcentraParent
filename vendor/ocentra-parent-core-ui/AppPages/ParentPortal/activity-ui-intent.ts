import type {
  DeviceKind,
  DevicePlatformKind,
  DeviceSlot,
  SelectableDeviceStatus,
} from './DeviceChoiceGrid/DeviceChoiceGridTypes';
import {
  createParentPortalAppGameDashboardIntent,
  type ParentPortalAppGameDashboardIntent,
} from './app-game-dashboard-intent';

type ActivityAdapterResultLike = {
  readonly ok?: unknown;
  readonly state?: unknown;
  readonly value?: unknown;
  readonly reason?: unknown;
};

type ParentPortalServiceRowLike = {
  readonly label?: unknown;
  readonly primaryArea?: unknown;
  readonly readyCount?: unknown;
  readonly signalScore?: unknown;
  readonly trend?: unknown;
};

export type ParentPortalActivityStateLike = {
  readonly activityReport?: ActivityAdapterResultLike | null;
  readonly activityReportHistory?: ActivityAdapterResultLike | null;
  readonly activityScreenReadModel?: ActivityAdapterResultLike | null;
  readonly activityAppUseReadModel?: ActivityAdapterResultLike | null;
  readonly activityAppGamePlatformExtensionReadModel?: ActivityAdapterResultLike | null;
  readonly activityBrowserReadModel?: ActivityAdapterResultLike | null;
  readonly activityGamesReadModel?: ActivityAdapterResultLike | null;
  readonly activityNetworkReadModel?: ActivityAdapterResultLike | null;
  readonly lanAddDeviceReadModel?: Record<string, unknown> | null;
};

export type ParentPortalActivityReportFile = {
  readonly id: string;
  readonly fileName: string;
  readonly dateLabel: string;
  readonly rangeLabel: string;
  readonly summary: string;
  readonly saved: boolean;
  readonly report: ParentPortalActivityReportView;
};

export type ParentPortalActivityReportView = {
  readonly title: string;
  readonly summary: string;
  readonly targetLabel: string;
  readonly saved: boolean;
  readonly fileName: string;
  readonly sections: readonly ParentPortalActivityReportSection[];
};

export type ParentPortalActivityReportSection = {
  readonly title: string;
  readonly lines: readonly string[];
};

export type ParentPortalActivityUiIntent = {
  readonly hasServiceBackedDeviceRows: boolean;
  readonly deviceSlots: readonly DeviceSlot[];
  readonly reportFiles: readonly ParentPortalActivityReportFile[];
  readonly reportDocument: Record<string, unknown> | null;
  readonly reportHistory: Record<string, unknown> | null;
  readonly screenReadModel: Record<string, unknown> | null;
  readonly appUseReadModel: Record<string, unknown> | null;
  readonly appGamePlatformExtensionReadModel: Record<string, unknown> | null;
  readonly browserReadModel: Record<string, unknown> | null;
  readonly gamesReadModel: Record<string, unknown> | null;
  readonly networkReadModel: Record<string, unknown> | null;
  readonly appGameDashboard: ParentPortalAppGameDashboardIntent;
};

export function createParentPortalActivityUiIntent(
  activityState: ParentPortalActivityStateLike | null | undefined,
  planSeatLimit: number
): ParentPortalActivityUiIntent {
  const reportDocument = parentPortalActivityAdapterRecord(activityState?.activityReport);
  const reportHistory = parentPortalActivityAdapterRecord(activityState?.activityReportHistory);
  const screenReadModel = parentPortalActivityAdapterRecord(activityState?.activityScreenReadModel);
  const appUseReadModel = parentPortalActivityAdapterRecord(activityState?.activityAppUseReadModel);
  const appGamePlatformExtensionReadModel = parentPortalActivityAdapterRecord(
    activityState?.activityAppGamePlatformExtensionReadModel
  );
  const browserReadModel = parentPortalActivityAdapterRecord(activityState?.activityBrowserReadModel);
  const gamesReadModel = parentPortalActivityAdapterRecord(activityState?.activityGamesReadModel);
  const networkReadModel = parentPortalActivityAdapterRecord(activityState?.activityNetworkReadModel);
  const serviceDeviceStates = collectActivityDeviceStates([
    reportDocument,
    reportHistory,
    screenReadModel,
    appUseReadModel,
    appGamePlatformExtensionReadModel,
    browserReadModel,
    gamesReadModel,
    networkReadModel,
  ]);
  const hasServiceBackedDeviceRows = serviceDeviceStates.size > 0;
  const seatCount = Math.max(0, Math.floor(planSeatLimit));

  return {
    hasServiceBackedDeviceRows,
    deviceSlots: activityDeviceSlots(serviceDeviceStates, seatCount),
    reportFiles: activityReportFiles(reportDocument, reportHistory),
    reportDocument,
    reportHistory,
    screenReadModel,
    appUseReadModel,
    appGamePlatformExtensionReadModel,
    browserReadModel,
    gamesReadModel,
    networkReadModel,
    appGameDashboard: createParentPortalAppGameDashboardIntent(
      appUseReadModel,
      gamesReadModel,
      appGamePlatformExtensionReadModel
    ),
  };
}

export function parentPortalActivityAdapterRecord(
  result: ActivityAdapterResultLike | null | undefined
): Record<string, unknown> | null {
  if (!isRecord(result) || result.ok !== true || !isRecord(result.value)) {
    return null;
  }
  return result.value;
}

export function createParentPortalLanPairingUiSlots(
  rows: readonly ParentPortalServiceRowLike[],
  addDeviceReadModel?: Record<string, unknown> | null
): readonly DeviceSlot[] {
  const deviceSlots = addDeviceReadModel ? lanDeviceSlots(addDeviceReadModel) : [];
  if (deviceSlots.length > 0) {
    return deviceSlots;
  }

  const deviceRow = serviceRow(rows, 'Current device', 'Device pairing');
  const discoveryRow = serviceRow(rows, 'LAN', 'LAN discovery');
  const state = pendingServiceScan(deviceRow, discoveryRow, addDeviceReadModel)
    ? 'pending'
    : stringValue(addDeviceReadModel?.['addDeviceState']) ||
      stringValue(addDeviceReadModel?.['localServiceDiscoveryState']) ||
      stringValue(deviceRow?.trend) ||
      stringValue(discoveryRow?.trend) ||
      '';
  if (!deviceRow && !discoveryRow && !state) return [];
  return [lanServiceSlot(state)];
}

export function createParentPortalLanPairingPortalIds(slots: readonly DeviceSlot[]): readonly string[] {
  return slots
    .filter((slot) => controllableLanDeviceSlot(slot) && slot.status === 'connected')
    .map((slot) => slot.value)
    .slice(0, 1);
}

export function createParentPortalCanonicalDeviceSlots(
  activitySlots: readonly DeviceSlot[],
  lanPairingSlots: readonly DeviceSlot[]
): readonly DeviceSlot[] {
  const devices = new Map<string, DeviceSlot>();
  for (const slot of lanPairingSlots) {
    upsertCanonicalDeviceSlot(devices, slot, 'lan');
  }
  for (const slot of activitySlots) {
    upsertCanonicalDeviceSlot(devices, slot, 'activity');
  }
  return Array.from(devices.values()).map((slot, slotIndex) => ({ ...slot, slotIndex }));
}

type CanonicalDeviceSlotSource = 'activity' | 'lan';

function upsertCanonicalDeviceSlot(
  devices: Map<string, DeviceSlot>,
  slot: DeviceSlot,
  source: CanonicalDeviceSlotSource
): void {
  if (!canonicalDeviceSlotSelectable(slot, source)) return;
  const existing = devices.get(slot.value) ?? matchingCanonicalDeviceSlot(devices, slot);
  const merged = existing ? mergedCanonicalDeviceSlot(existing, slot) : slot;
  if (existing && existing.value !== merged.value) {
    devices.delete(existing.value);
  }
  devices.set(merged.value, merged);
}

function canonicalDeviceSlotSelectable(slot: DeviceSlot, source: CanonicalDeviceSlotSource): boolean {
  if (!slot.device || slot.status === 'empty' || infrastructureDeviceSlot(slot)) return false;
  return source === 'activity' || controllableLanDeviceSlot(slot);
}

function controllableLanDeviceSlot(slot: DeviceSlot): boolean {
  return !!slot.device && slot.status !== 'empty' && !infrastructureDeviceSlot(slot) && slotHasAgentFacet(slot);
}

function infrastructureDeviceSlot(slot: DeviceSlot): boolean {
  return slot.badge === 'infrastructure' || slot.device?.platform === 'router' || slot.device?.type === 'router';
}

function matchingCanonicalDeviceSlot(devices: Map<string, DeviceSlot>, slot: DeviceSlot): DeviceSlot | undefined {
  for (const existing of devices.values()) {
    if (samePhysicalDeviceValue(slot.value, existing.value)) return existing;
    if (samePhysicalDeviceValue(slot.device?.id, existing.device?.id)) return existing;
    if (samePhysicalDeviceValue(slot.device?.mac, existing.device?.mac)) return existing;
    if (
      (slotHasAgentFacet(slot) || slotHasAgentFacet(existing)) &&
      samePhysicalDeviceValue(slot.device?.ip, existing.device?.ip) &&
      !conflictingPhysicalMac(slot.device?.mac, existing.device?.mac)
    ) {
      return existing;
    }
  }
  return undefined;
}

function mergedCanonicalDeviceSlot(existing: DeviceSlot, incoming: DeviceSlot): DeviceSlot {
  const incomingPreferred =
    slotHasAgentFacet(incoming) ||
    (!slotHasAgentFacet(existing) && deviceSlotStatusRank(incoming.status) > deviceSlotStatusRank(existing.status));
  const preferred = incomingPreferred ? incoming : existing;
  const fallback = incomingPreferred ? existing : incoming;
  const status =
    deviceSlotStatusRank(incoming.status) > deviceSlotStatusRank(existing.status) ? incoming.status : existing.status;
  const badge = preferred.badge ?? fallback.badge;
  const device = preferred.device
    ? {
        ...fallback.device,
        ...preferred.device,
        status: selectableDeviceStatus(status),
      }
    : fallback.device;
  return {
    ...fallback,
    ...preferred,
    status,
    ...(badge ? { badge } : {}),
    ...(device ? { device } : {}),
  };
}

function selectableDeviceStatus(status: DeviceSlot['status']): SelectableDeviceStatus {
  return status === 'empty' ? 'unsupported' : status;
}

function deviceSlotStatusRank(status: DeviceSlot['status']): number {
  if (status === 'connected') return 4;
  if (status === 'available') return 3;
  if (status === 'offline') return 2;
  if (status === 'unsupported') return 1;
  return 0;
}

function serviceRow(
  rows: readonly ParentPortalServiceRowLike[],
  primaryArea: string,
  label: string
): ParentPortalServiceRowLike | null {
  return (
    rows.find((row) => stringValue(row.primaryArea) === primaryArea) ??
    rows.find((row) => stringValue(row.label) === label) ??
    null
  );
}

function lanServiceSlot(state: string): DeviceSlot {
  const status = activityDeviceChoiceStatus(state || 'unavailable');
  return {
    value: 'lan-pairing-service-state',
    label: state === 'pending' ? 'Scanning LAN' : 'LAN',
    status,
    slotIndex: 0,
    badge: state === 'pending' ? 'scanning' : state || 'unavailable',
  };
}

function pendingServiceScan(
  deviceRow: ParentPortalServiceRowLike | null,
  discoveryRow: ParentPortalServiceRowLike | null,
  addDeviceReadModel?: Record<string, unknown> | null
): boolean {
  if (addDeviceReadModel) return false;
  const trend = `${stringValue(deviceRow?.trend) ?? ''} ${stringValue(discoveryRow?.trend) ?? ''}`.toLowerCase();
  if (trend.includes('scan') || trend.includes('pending')) {
    return true;
  }
  const hasLanRow = deviceRow !== null || discoveryRow !== null;
  return hasLanRow && !trend.includes('offline') && !trend.includes('unavailable');
}

function lanDeviceSlots(readModel: Record<string, unknown>): readonly DeviceSlot[] {
  const devices = new Map<string, DeviceSlot>();
  collectCanonicalLanDevices(readModel, devices);
  collectDiscoveredLanDevices(readModel, devices);
  collectTrustedLanDevices(readModel, devices);
  collectPairingRequestDevices(readModel, devices);
  collectSelectedLanDevice(readModel, devices);
  return Array.from(devices.values()).map((slot, slotIndex) => ({ ...slot, slotIndex }));
}

function collectCanonicalLanDevices(readModel: Record<string, unknown>, devices: Map<string, DeviceSlot>): void {
  const canonicalDevices = arrayValue(readModel['canonicalHouseholdDevices']);
  for (const canonicalDevice of canonicalDevices) {
    const item = recordValue(canonicalDevice);
    const deviceId = stringValue(item?.['canonicalDeviceId']);
    if (!deviceId) continue;
    const networkIdentity = recordValue(item?.['networkIdentity']);
    const childAgentInventory = recordValue(item?.['childAgentInventory']);
    const classification = stringValue(item?.['classification']);
    const childAgentBacked = classification === 'child-agent' || childAgentInventory !== null;
    const reachability = stringValue(networkIdentity?.['reachability']);
    const discoveryState = stringValue(item?.['discoveryState']);
    const trustState = stringValue(item?.['trustState']);
    const routeId = stringValue(item?.['routeId']);
    const evidence = lanDeviceEvidence(readModel, deviceId, routeId);
    upsertLanDeviceSlot(devices, {
      deviceId,
      label: stringValue(item?.['displayName']) || deviceId,
      platform: normalizeDevicePlatform(
        stringValue(childAgentInventory?.['platform']) || canonicalDevicePlatform(classification)
      ),
      ip: firstString(networkIdentity?.['ipAddresses']),
      mac: stringValue(networkIdentity?.['macAddress']),
      hostname: stringValue(networkIdentity?.['hostname']),
      networkInterface: firstString(networkIdentity?.['networkInterfaces']),
      agentStatus: childAgentBacked ? 'ocentra-child-agent' : undefined,
      manufacturer: stringValue(networkIdentity?.['macVendor']),
      portalEligible: childAgentBacked,
      cpuModel: stringValue(childAgentInventory?.['cpuModel']),
      cpuCores: stringValue(childAgentInventory?.['cpuCores']),
      memoryTotal: stringValue(childAgentInventory?.['memoryTotal']),
      gpuModel: stringValue(childAgentInventory?.['gpuModel']),
      gpuDriver: stringValue(childAgentInventory?.['gpuDriver']),
      gpuMemory: stringValue(childAgentInventory?.['gpuMemory']),
      nvidiaSmi: stringValue(childAgentInventory?.['nvidiaSmi']),
      routeId,
      routeState: stringValue(item?.['routeState']),
      trustState,
      discoveryState,
      sourceConfidence: stringValue(networkIdentity?.['confidence']),
      evidenceLabel: firstString(item?.['sourceLabels']),
      ...evidence,
      state: evidence.state || canonicalLanDeviceState(classification, reachability, discoveryState, trustState),
      preferState: evidence.preferState === true,
    });
  }
}

function canonicalDevicePlatform(classification: string): string {
  if (classification === 'network-infrastructure') return 'router';
  return 'unknown';
}

function canonicalLanDeviceState(
  classification: string,
  reachability: string,
  discoveryState: string,
  trustState: string
): string {
  if (classification === 'network-infrastructure') return 'infrastructure';
  if (classification === 'unsupported-lan-device' || classification === 'unknown-lan-device') {
    return passiveLanDeviceState(reachability, discoveryState);
  }
  return reachability || trustState || discoveryState || 'unavailable';
}

function collectDiscoveredLanDevices(readModel: Record<string, unknown>, devices: Map<string, DeviceSlot>): void {
  const discoveredDevices = arrayValue(readModel['discoveredDevices']);
  for (const discoveredDevice of discoveredDevices) {
    const item = recordValue(discoveredDevice);
    const childDevice = recordValue(item?.['childDevice']);
    const deviceId = stringValue(childDevice?.['deviceId']);
    if (!deviceId) continue;
    const hardwareProfile = recordValue(childDevice?.['hardwareProfile']);
    const routeId = stringValue(item?.['routeId']);
    const discoveryState = stringValue(item?.['discoveryState']);
    const evidence = lanDeviceEvidence(readModel, deviceId, routeId);
    upsertLanDeviceSlot(devices, {
      deviceId,
      label: stringValue(childDevice?.['label']) || deviceId,
      platform: normalizeDevicePlatform(stringValue(childDevice?.['platform'])),
      ip: stringValue(childDevice?.['ipAddress']),
      mac: stringValue(childDevice?.['macAddress']),
      hostname: stringValue(childDevice?.['hostname']),
      networkInterface: stringValue(childDevice?.['networkInterface']),
      agentStatus: stringValue(childDevice?.['agentStatus']),
      manufacturer: stringValue(hardwareProfile?.['manufacturer']),
      model: stringValue(hardwareProfile?.['model']),
      cpuModel: stringValue(hardwareProfile?.['cpuModel']),
      cpuCores: stringValue(hardwareProfile?.['cpuCores']),
      memoryTotal: stringValue(hardwareProfile?.['memoryTotal']),
      gpuModel: stringValue(hardwareProfile?.['gpuModel']),
      gpuDriver: stringValue(hardwareProfile?.['gpuDriver']),
      gpuMemory: stringValue(hardwareProfile?.['gpuMemory']),
      nvidiaSmi: stringValue(hardwareProfile?.['nvidiaSmi']),
      routeId,
      discoveryState,
      sourceConfidence: stringValue(item?.['discoveryStatus']),
      evidenceLabel: stringValue(item?.['addressRef']),
      ...evidence,
      state: evidence.state || lanDiscoveryDeviceState(item, childDevice),
      preferState: evidence.preferState === true,
    });
  }
}

function lanDiscoveryDeviceState(
  discoveredDevice: Record<string, unknown> | null,
  childDevice: Record<string, unknown> | null
): string {
  const discoveryStatus = stringValue(discoveredDevice?.['discoveryStatus']);
  const discoveryState = stringValue(discoveredDevice?.['discoveryState']);
  const platform = stringValue(childDevice?.['platform']);
  const agentStatus = stringValue(childDevice?.['agentStatus']);
  if (platform === 'router' && !agentStatus) {
    return 'infrastructure';
  }
  if (!agentStatus && discoveryStatus === 'network-neighbor') {
    return passiveLanDeviceState(stringValue(discoveredDevice?.['reachability']), discoveryState);
  }
  if (
    discoveryStatus === 'manual-required' ||
    discoveryStatus === 'unavailable' ||
    discoveryState === 'manual-required' ||
    discoveryState === 'unavailable' ||
    discoveryState === 'rejected' ||
    discoveryState === 'expired' ||
    discoveryState === 'revoked'
  ) {
    return discoveryState || discoveryStatus;
  }
  return stringValue(discoveredDevice?.['reachability']) || discoveryState || discoveryStatus || 'unavailable';
}

function passiveLanDeviceState(reachability: string, discoveryState: string): string {
  if (reachability === 'offline') return 'offline';
  if (reachability === 'stale') return 'stale';
  return discoveryState === 'unavailable' || discoveryState === 'manual-required' ? discoveryState : 'discovered';
}

function collectTrustedLanDevices(readModel: Record<string, unknown>, devices: Map<string, DeviceSlot>): void {
  const registry = arrayValue(readModel['trustedDeviceRegistry']);
  for (const registryEntry of registry) {
    const item = recordValue(registryEntry);
    const childDevice = recordValue(item?.['childDevice']);
    const deviceId = stringValue(childDevice?.['deviceId']);
    if (!deviceId) continue;
    const revokedAt = stringValue(item?.['revokedAt']);
    const hardwareProfile = recordValue(childDevice?.['hardwareProfile']);
    const routeId = stringValue(item?.['routeId']);
    const trustState = stringValue(item?.['trustState']);
    const existingSlot = selectedLanDeviceSlot(devices, deviceId, routeId);
    const evidence = lanDeviceEvidence(readModel, deviceId, routeId);
    upsertLanDeviceSlot(devices, {
      deviceId: existingSlot?.value || deviceId,
      label: stringValue(childDevice?.['label']) || existingSlot?.label || deviceId,
      platform: existingSlot?.device?.platform || normalizeDevicePlatform(stringValue(childDevice?.['platform'])),
      ip: stringValue(childDevice?.['ipAddress']),
      mac: stringValue(childDevice?.['macAddress']),
      hostname: stringValue(childDevice?.['hostname']),
      networkInterface: stringValue(childDevice?.['networkInterface']),
      agentStatus: stringValue(childDevice?.['agentStatus']),
      manufacturer: stringValue(hardwareProfile?.['manufacturer']),
      model: stringValue(hardwareProfile?.['model']),
      cpuModel: stringValue(hardwareProfile?.['cpuModel']),
      cpuCores: stringValue(hardwareProfile?.['cpuCores']),
      memoryTotal: stringValue(hardwareProfile?.['memoryTotal']),
      gpuModel: stringValue(hardwareProfile?.['gpuModel']),
      gpuDriver: stringValue(hardwareProfile?.['gpuDriver']),
      gpuMemory: stringValue(hardwareProfile?.['gpuMemory']),
      nvidiaSmi: stringValue(hardwareProfile?.['nvidiaSmi']),
      routeId,
      trustState,
      ...evidence,
      state: evidence.state || (revokedAt ? 'revoked' : trustState || 'paired'),
      preferState: evidence.preferState === true,
    });
  }
}

function collectPairingRequestDevices(readModel: Record<string, unknown>, devices: Map<string, DeviceSlot>): void {
  const pairingRequests = arrayValue(readModel['pairingRequests']);
  for (const pairingRequest of pairingRequests) {
    const item = recordValue(pairingRequest);
    const deviceId = stringValue(item?.['childDeviceId']);
    if (!deviceId) continue;
    const routeId = stringValue(item?.['routeId']);
    const evidence = lanDeviceEvidence(readModel, deviceId, routeId);
    upsertLanDeviceSlot(devices, {
      deviceId,
      label: activityDeviceShortLabel(deviceId, devices.size),
      platform: 'unknown',
      routeId,
      ...evidence,
      state: evidence.state || stringValue(item?.['pairingState']) || 'manual-required',
      preferState: evidence.preferState === true,
    });
  }
}

function collectSelectedLanDevice(readModel: Record<string, unknown>, devices: Map<string, DeviceSlot>): void {
  const selected = recordValue(readModel['selectedDeviceReadiness']);
  const deviceId = stringValue(selected?.['selectedChildDeviceId']);
  if (!deviceId) return;
  const routeId = stringValue(selected?.['routeId']);
  const existingSlot = selectedLanDeviceSlot(devices, deviceId, routeId);
  const evidence = lanDeviceEvidence(readModel, deviceId, routeId);
  upsertLanDeviceSlot(devices, {
    deviceId: existingSlot?.value || deviceId,
    label: existingSlot?.label || activityDeviceShortLabel(deviceId, devices.size),
    platform: existingSlot?.device?.platform || 'unknown',
    routeId,
    trustState: stringValue(selected?.['trustState']),
    readinessState:
      selected?.['readyForControl'] === true ? 'ready-for-control' : stringValue(selected?.['reachability']),
    ...evidence,
    state:
      evidence.state ||
      (selected?.['readyForControl'] === true
        ? 'ready'
        : stringValue(selected?.['reachability']) || stringValue(selected?.['trustState']) || 'unavailable'),
    preferState: true,
  });
}

function selectedLanDeviceSlot(
  devices: Map<string, DeviceSlot>,
  deviceId: string,
  routeId: string
): DeviceSlot | undefined {
  const direct = devices.get(deviceId);
  if (direct) {
    return direct;
  }
  for (const slot of devices.values()) {
    if (!slotHasAgentFacet(slot)) continue;
    if (samePhysicalDeviceValue(slot.device?.routeId, routeId)) {
      return slot;
    }
  }
  return undefined;
}

type LanDeviceEvidenceInput = {
  readonly pairingId?: string | undefined;
  readonly proofDigest?: string | undefined;
  readonly origin?: string | undefined;
  readonly expiresAt?: string | undefined;
  readonly trustedAt?: string | undefined;
  readonly parentDeviceId?: string | undefined;
  readonly childProfileId?: string | undefined;
  readonly routeState?: string | undefined;
  readonly trustState?: string | undefined;
  readonly discoveryState?: string | undefined;
  readonly readinessState?: string | undefined;
  readonly sourceConfidence?: string | undefined;
  readonly custodyLabel?: string | undefined;
  readonly signedProofCheck?: string | undefined;
  readonly signedProofState?: string | undefined;
  readonly routeSafety?: string | undefined;
  readonly routeSafetyState?: string | undefined;
  readonly routeSafetyReason?: string | undefined;
  readonly relayCacheCheck?: string | undefined;
  readonly relayCacheState?: string | undefined;
  readonly relayCacheCustody?: string | undefined;
  readonly manualProof?: string | undefined;
  readonly claimsNotProved?: string | undefined;
  readonly lanWorkpackStatus?: string | undefined;
  readonly lanSourceProof?: string | undefined;
  readonly lanWeakSourceProof?: string | undefined;
  readonly parentDecision?: string | undefined;
  readonly householdName?: string | undefined;
  readonly parentDeviceKind?: DeviceKind | undefined;
  readonly auditLabel?: string | undefined;
  readonly requirementLabel?: string | undefined;
  readonly evidenceLabel?: string | undefined;
  readonly state?: string | undefined;
  readonly preferState?: boolean | undefined;
};

function lanDeviceEvidence(
  readModel: Record<string, unknown>,
  deviceId: string,
  routeId: string
): LanDeviceEvidenceInput {
  const selectedReadiness = recordValue(readModel['selectedDeviceReadiness']);
  const selectedMatches =
    samePhysicalDeviceValue(deviceId, stringValue(selectedReadiness?.['selectedChildDeviceId'])) ||
    samePhysicalDeviceValue(routeId, stringValue(selectedReadiness?.['routeId']));
  const signedSpine = recordValue(readModel['signedDiscoveryRelaySpine']);
  const routeSafetyRow = preferredLanSpineRouteRow(signedSpine, routeId);
  const signedProofRow = firstLanSpineRecord(signedSpine, 'signedProofRows');
  const relayCacheRow = firstLanSpineRecord(signedSpine, 'relayCacheRows');
  const adapterRow = preferredLanSpineAdapterRow(signedSpine, selectedMatches);
  const sourceMatrix = recordValue(readModel['lanDiscoverySourceMatrix']);
  const registryEntry = preferredLanTrustedRegistryEntry(readModel, deviceId, routeId);
  const childDevice = recordValue(registryEntry?.['childDevice']);
  const parentDevice = recordValue(registryEntry?.['parentDevice']);
  const decision = latestLanHouseholdDecision(readModel, deviceId);
  return compactLanDeviceEvidence({
    pairingId:
      (selectedMatches ? stringValue(selectedReadiness?.['pairingId']) : '') ||
      stringValue(registryEntry?.['pairingId']),
    proofDigest: stringValue(registryEntry?.['proofDigest']),
    origin: stringValue(registryEntry?.['origin']),
    expiresAt: stringValue(registryEntry?.['expiresAt']),
    trustedAt: stringValue(registryEntry?.['trustedAt']),
    parentDeviceId: stringValue(parentDevice?.['deviceId']),
    childProfileId: stringValue(childDevice?.['childProfileId']) || stringValue(decision?.['childProfileId']),
    readinessState: selectedMatches ? lanSelectedReadinessLabel(selectedReadiness) : undefined,
    sourceConfidence: stringValue(adapterRow?.['sourceConfidence']),
    custodyLabel: stringValue(routeSafetyRow?.['custodyLabel']) || stringValue(relayCacheRow?.['custodyLabel']),
    signedProofCheck: stringValue(signedProofRow?.['check']),
    signedProofState: stringValue(signedProofRow?.['proofState']) || stringValue(adapterRow?.['proofState']),
    routeSafety: stringValue(routeSafetyRow?.['check']),
    routeSafetyState: stringValue(routeSafetyRow?.['responseState']) || stringValue(routeSafetyRow?.['discoveryState']),
    routeSafetyReason: stringValue(routeSafetyRow?.['rejectionReason']),
    relayCacheCheck: stringValue(relayCacheRow?.['check']),
    relayCacheState: stringValue(relayCacheRow?.['decisionState']) || stringValue(relayCacheRow?.['proofState']),
    relayCacheCustody: stringValue(relayCacheRow?.['custodyLabel']),
    manualProof: lanEvidenceSummary(signedSpine?.['manualProofRequired']),
    claimsNotProved: lanEvidenceSummary(signedSpine?.['claimsNotProved']),
    lanWorkpackStatus: lanSourceMatrixWorkpackSummary(sourceMatrix),
    lanSourceProof: lanSourceMatrixImplementedSummary(sourceMatrix),
    lanWeakSourceProof: lanSourceMatrixWeakSourceSummary(sourceMatrix),
    parentDecision: lanHouseholdDecisionLabel(decision),
    householdName: stringValue(decision?.['displayName']),
    parentDeviceKind: normalizedLanDeviceKindValue(stringValue(decision?.['deviceKind'])),
    auditLabel: lanEvidenceSummary(readModel['auditCheckLabels']),
    requirementLabel: lanEvidenceSummary(readModel['routeRequirementLabels']),
    evidenceLabel:
      stringValue(routeSafetyRow?.['evidenceLabel']) ||
      stringValue(signedProofRow?.['evidenceLabel']) ||
      stringValue(adapterRow?.['evidenceLabel']),
    state: lanHouseholdDecisionState(decision),
    preferState: lanHouseholdDecisionState(decision).length > 0,
  });
}

function preferredLanSpineRouteRow(
  signedSpine: Record<string, unknown> | null,
  routeId: string
): Record<string, unknown> | null {
  const rows = recordArrayValue(signedSpine?.['routeSafetyRows']);
  return rows.find((row) => samePhysicalDeviceValue(stringValue(row['routeId']), routeId)) ?? rows[0] ?? null;
}

function preferredLanSpineAdapterRow(
  signedSpine: Record<string, unknown> | null,
  selectedMatches: boolean
): Record<string, unknown> | null {
  const rows = recordArrayValue(signedSpine?.['adapterRows']);
  if (selectedMatches) {
    return (
      rows.find((row) => stringValue(row['adapter']) === 'signed-child-agent-hello') ??
      rows.find((row) => stringValue(row['adapter']) === 'signed-child-agent-heartbeat') ??
      rows[0] ??
      null
    );
  }
  return rows[0] ?? null;
}

function firstLanSpineRecord(signedSpine: Record<string, unknown> | null, key: string): Record<string, unknown> | null {
  return recordArrayValue(signedSpine?.[key])[0] ?? null;
}

function latestLanHouseholdDecision(
  readModel: Record<string, unknown>,
  deviceId: string
): Record<string, unknown> | null {
  const decisions = recordArrayValue(readModel['householdDeviceDecisions']);
  for (let index = decisions.length - 1; index >= 0; index -= 1) {
    const decision = decisions[index];
    if (samePhysicalDeviceValue(stringValue(decision?.['canonicalDeviceId']), deviceId)) {
      return decision ?? null;
    }
  }
  return null;
}

function preferredLanTrustedRegistryEntry(
  readModel: Record<string, unknown>,
  deviceId: string,
  routeId: string
): Record<string, unknown> | null {
  const entries = recordArrayValue(readModel['trustedDeviceRegistry']);
  return (
    entries.find((entry) => {
      const childDevice = recordValue(entry['childDevice']);
      return (
        samePhysicalDeviceValue(stringValue(childDevice?.['deviceId']), deviceId) ||
        samePhysicalDeviceValue(stringValue(entry['routeId']), routeId)
      );
    }) ?? null
  );
}

function lanSelectedReadinessLabel(selectedReadiness: Record<string, unknown> | null): string {
  if (!selectedReadiness) return '';
  if (selectedReadiness['readyForControl'] === true) return 'ready-for-control';
  return stringValue(selectedReadiness['reachability']) || stringValue(selectedReadiness['trustState']);
}

function lanHouseholdDecisionLabel(decision: Record<string, unknown> | null): string {
  if (!decision) return '';
  const actionKind = stringValue(decision['actionKind']);
  const displayName = stringValue(decision['displayName']);
  const revokedAt = stringValue(decision['revokedAt']);
  if (revokedAt && actionKind) return `${actionKind} revoked`;
  if (actionKind && displayName) return `${actionKind}: ${displayName}`;
  return actionKind || displayName;
}

function lanHouseholdDecisionState(decision: Record<string, unknown> | null): string {
  if (!decision) return '';
  const actionKind = stringValue(decision['actionKind']);
  if (actionKind === 'revoke' || stringValue(decision['revokedAt'])) {
    return 'revoked';
  }
  if (actionKind === 'ignore') {
    return 'ignored';
  }
  return '';
}

function lanEvidenceSummary(value: unknown): string {
  const items = stringArrayValue(value);
  if (items.length <= 2) return items.join(', ');
  return `${items.slice(0, 2).join(', ')} +${items.length - 2}`;
}

function lanSourceMatrixWorkpackSummary(sourceMatrix: Record<string, unknown> | null): string {
  const rows = recordArrayValue(sourceMatrix?.['workpackRows']);
  if (rows.length === 0) return '';
  const complete = rows.filter((row) => stringValue(row['status']) === 'implemented').length;
  const partial = rows.filter((row) => stringValue(row['status']) === 'partial').length;
  const manual = rows.filter((row) => stringValue(row['status']) === 'manual-required').length;
  const missing = rows.filter((row) => stringValue(row['status']) === 'not-implemented').length;
  return `workpacks ${complete}/${rows.length} implemented; ${partial} partial; ${manual} manual; ${missing} missing`;
}

function lanSourceMatrixImplementedSummary(sourceMatrix: Record<string, unknown> | null): string {
  const labels = recordArrayValue(sourceMatrix?.['sourceRows'])
    .filter((row) => stringValue(row['status']) === 'implemented')
    .map((row) => stringValue(row['source']))
    .filter((source) => source.length > 0);
  return lanEvidenceSummary(labels);
}

function lanSourceMatrixWeakSourceSummary(sourceMatrix: Record<string, unknown> | null): string {
  const weak = recordArrayValue(sourceMatrix?.['sourceRows']).filter(
    (row) => row['canConfirmChildAgent'] !== true && row['canAssignChildProfile'] !== true
  ).length;
  const total = recordArrayValue(sourceMatrix?.['sourceRows']).length;
  return total > 0 ? `weak sources fenced ${weak}/${total}` : '';
}

function compactLanDeviceEvidence(input: LanDeviceEvidenceInput): LanDeviceEvidenceInput {
  return Object.fromEntries(
    Object.entries(input).filter(([, value]) => value !== undefined && value !== '')
  ) as LanDeviceEvidenceInput;
}

type LanAgentFacetInput = {
  readonly agentStatus?: string | undefined;
  readonly cpuModel?: string | undefined;
  readonly memoryTotal?: string | undefined;
  readonly gpuModel?: string | undefined;
};

function upsertLanDeviceSlot(
  devices: Map<string, DeviceSlot>,
  input: {
    readonly deviceId: string;
    readonly label: string;
    readonly platform: DevicePlatformKind;
    readonly ip?: string | undefined;
    readonly mac?: string | undefined;
    readonly hostname?: string | undefined;
    readonly networkInterface?: string | undefined;
    readonly agentStatus?: string | undefined;
    readonly manufacturer?: string | undefined;
    readonly model?: string | undefined;
    readonly cpuModel?: string | undefined;
    readonly cpuCores?: string | undefined;
    readonly memoryTotal?: string | undefined;
    readonly gpuModel?: string | undefined;
    readonly gpuDriver?: string | undefined;
    readonly gpuMemory?: string | undefined;
    readonly nvidiaSmi?: string | undefined;
    readonly routeId?: string | undefined;
    readonly pairingId?: string | undefined;
    readonly proofDigest?: string | undefined;
    readonly origin?: string | undefined;
    readonly expiresAt?: string | undefined;
    readonly trustedAt?: string | undefined;
    readonly parentDeviceId?: string | undefined;
    readonly childProfileId?: string | undefined;
    readonly routeState?: string | undefined;
    readonly trustState?: string | undefined;
    readonly discoveryState?: string | undefined;
    readonly readinessState?: string | undefined;
    readonly sourceConfidence?: string | undefined;
    readonly custodyLabel?: string | undefined;
    readonly signedProofCheck?: string | undefined;
    readonly signedProofState?: string | undefined;
    readonly routeSafety?: string | undefined;
    readonly routeSafetyState?: string | undefined;
    readonly routeSafetyReason?: string | undefined;
    readonly relayCacheCheck?: string | undefined;
    readonly relayCacheState?: string | undefined;
    readonly relayCacheCustody?: string | undefined;
    readonly manualProof?: string | undefined;
    readonly claimsNotProved?: string | undefined;
    readonly lanWorkpackStatus?: string | undefined;
    readonly lanSourceProof?: string | undefined;
    readonly lanWeakSourceProof?: string | undefined;
    readonly parentDecision?: string | undefined;
    readonly householdName?: string | undefined;
    readonly parentDeviceKind?: DeviceKind | undefined;
    readonly auditLabel?: string | undefined;
    readonly requirementLabel?: string | undefined;
    readonly evidenceLabel?: string | undefined;
    readonly portalEligible?: boolean | undefined;
    readonly state: string;
    readonly preferState?: boolean;
  }
): void {
  const existing = devices.get(input.deviceId) ?? matchingPhysicalDeviceSlot(devices, input);
  const slotValue = mergedLanDeviceSlotValue(existing, input);
  const incomingHasAgentFacet = hasAgentFacet(input);
  const existingHasAgentFacet = existing ? slotHasAgentFacet(existing) : false;
  const preserveCanonicalIdentity = !!existing && hasCanonicalLanPhysicalSlotValue(existing.value);
  const portalEligible = (input.portalEligible ?? incomingHasAgentFacet) || existing?.device?.portalEligible === true;
  const householdName = input.householdName || existing?.device?.householdName;
  const detectedName = detectedLanDeviceName(input) || existing?.device?.detectedName;
  const slotLabel = householdName || mergedLanDeviceLabel(existing, input, slotValue, devices.size);
  if (existing && existing.value !== slotValue) {
    devices.delete(existing.value);
  }
  const preserveCanonicalState =
    !!existing &&
    hasCanonicalLanPhysicalSlotValue(existing.value) &&
    !incomingHasAgentFacet &&
    input.preferState !== true;
  const state =
    input.state === 'revoked'
      ? input.state
      : preserveCanonicalState
        ? stringValue(existing.badge) || input.state
        : input.preferState ||
            !existing ||
            activityDeviceStateRank(input.state) >= activityDeviceStateRank(stringValue(existing.badge))
          ? input.state
          : stringValue(existing.badge);
  const status = activityDeviceChoiceStatus(state);
  const inferredDeviceType = inferLanDeviceKind(input);
  const parentDeviceKind = input.parentDeviceKind ?? existing?.device?.parentDeviceKind;
  const deviceType =
    parentDeviceKind ??
    (inferredDeviceType === 'unknown' ? (existing?.device?.type ?? inferredDeviceType) : inferredDeviceType);
  devices.set(slotValue, {
    value: slotValue,
    label: slotLabel,
    status,
    slotIndex: existing?.slotIndex ?? devices.size,
    badge: state,
    device: {
      id: slotValue,
      name: slotLabel,
      ip: input.ip || existing?.device?.ip,
      mac: input.mac || existing?.device?.mac,
      hostname: preferredLanHostname(
        existing?.device?.hostname,
        input.hostname,
        existingHasAgentFacet,
        incomingHasAgentFacet
      ),
      networkInterface: input.networkInterface || existing?.device?.networkInterface,
      agentStatus: preserveCanonicalIdentity
        ? nonEmptyString(existing?.device?.agentStatus) || nonEmptyString(input.agentStatus)
        : nonEmptyString(input.agentStatus) || nonEmptyString(existing?.device?.agentStatus),
      manufacturer: input.manufacturer || existing?.device?.manufacturer,
      model: input.model || existing?.device?.model,
      cpuModel: input.cpuModel || existing?.device?.cpuModel,
      cpuCores: input.cpuCores || existing?.device?.cpuCores,
      memoryTotal: input.memoryTotal || existing?.device?.memoryTotal,
      gpuModel: input.gpuModel || existing?.device?.gpuModel,
      gpuDriver: input.gpuDriver || existing?.device?.gpuDriver,
      gpuMemory: input.gpuMemory || existing?.device?.gpuMemory,
      nvidiaSmi: input.nvidiaSmi || existing?.device?.nvidiaSmi,
      routeId: input.routeId || existing?.device?.routeId,
      pairingId: input.pairingId || existing?.device?.pairingId,
      proofDigest: input.proofDigest || existing?.device?.proofDigest,
      origin: input.origin || existing?.device?.origin,
      expiresAt: input.expiresAt || existing?.device?.expiresAt,
      trustedAt: input.trustedAt || existing?.device?.trustedAt,
      parentDeviceId: input.parentDeviceId || existing?.device?.parentDeviceId,
      childProfileId: input.childProfileId || existing?.device?.childProfileId,
      routeState: input.routeState || existing?.device?.routeState,
      trustState: input.trustState || existing?.device?.trustState,
      discoveryState: input.discoveryState || existing?.device?.discoveryState,
      readinessState: input.readinessState || existing?.device?.readinessState,
      sourceState: state || existing?.device?.sourceState,
      sourceConfidence: input.sourceConfidence || existing?.device?.sourceConfidence,
      custodyLabel: input.custodyLabel || existing?.device?.custodyLabel,
      signedProofCheck: input.signedProofCheck || existing?.device?.signedProofCheck,
      signedProofState: input.signedProofState || existing?.device?.signedProofState,
      routeSafety: input.routeSafety || existing?.device?.routeSafety,
      routeSafetyState: input.routeSafetyState || existing?.device?.routeSafetyState,
      routeSafetyReason: input.routeSafetyReason || existing?.device?.routeSafetyReason,
      relayCacheCheck: input.relayCacheCheck || existing?.device?.relayCacheCheck,
      relayCacheState: input.relayCacheState || existing?.device?.relayCacheState,
      relayCacheCustody: input.relayCacheCustody || existing?.device?.relayCacheCustody,
      manualProof: input.manualProof || existing?.device?.manualProof,
      claimsNotProved: input.claimsNotProved || existing?.device?.claimsNotProved,
      lanWorkpackStatus: input.lanWorkpackStatus || existing?.device?.lanWorkpackStatus,
      lanSourceProof: input.lanSourceProof || existing?.device?.lanSourceProof,
      lanWeakSourceProof: input.lanWeakSourceProof || existing?.device?.lanWeakSourceProof,
      parentDecision: input.parentDecision || existing?.device?.parentDecision,
      householdName,
      detectedName,
      parentDeviceKind,
      auditLabel: input.auditLabel || existing?.device?.auditLabel,
      requirementLabel: input.requirementLabel || existing?.device?.requirementLabel,
      evidenceLabel: input.evidenceLabel || existing?.device?.evidenceLabel,
      portalEligible,
      type: deviceType,
      platform: input.platform,
      status,
    },
  });
}

function matchingPhysicalDeviceSlot(
  devices: Map<string, DeviceSlot>,
  input: {
    readonly ip?: string | undefined;
    readonly mac?: string | undefined;
  } & LanAgentFacetInput
): DeviceSlot | undefined {
  for (const slot of devices.values()) {
    if (samePhysicalDeviceValue(input.mac, slot.device?.mac)) return slot;
    if (!hasAgentFacet(input) && !slotHasAgentFacet(slot)) continue;
    if (samePhysicalDeviceValue(input.ip, slot.device?.ip) && !conflictingPhysicalMac(input.mac, slot.device?.mac)) {
      return slot;
    }
  }
  return undefined;
}

function samePhysicalDeviceValue(left: string | undefined, right: string | undefined): boolean {
  const normalizedLeft = left?.trim().toLowerCase();
  const normalizedRight = right?.trim().toLowerCase();
  return !!normalizedLeft && normalizedLeft === normalizedRight;
}

function conflictingPhysicalMac(left: string | undefined, right: string | undefined): boolean {
  const normalizedLeft = left?.trim().toLowerCase();
  const normalizedRight = right?.trim().toLowerCase();
  return !!normalizedLeft && !!normalizedRight && normalizedLeft !== normalizedRight;
}

function mergedLanDeviceSlotValue(
  existing: DeviceSlot | undefined,
  input: { readonly deviceId: string } & LanAgentFacetInput
): string {
  if (!existing) return input.deviceId;
  if (hasCanonicalLanPhysicalSlotValue(existing.value)) return existing.value;
  if (hasAgentFacet(input)) return input.deviceId;
  return existing.value;
}

function hasCanonicalLanPhysicalSlotValue(value: string | undefined): boolean {
  return typeof value === 'string' && value.startsWith('lan-physical-');
}

function nonEmptyString(value: string | undefined): string | undefined {
  return typeof value === 'string' && value.length > 0 ? value : undefined;
}

function preferredLanHostname(
  existingHostname: string | undefined,
  incomingHostname: string | undefined,
  existingHasAgentFacet: boolean,
  incomingHasAgentFacet: boolean
): string | undefined {
  const incomingWeak =
    incomingHostname === undefined ||
    incomingHostname.length === 0 ||
    incomingHostname.toLowerCase() === 'unknown-host';
  if (existingHasAgentFacet && !incomingHasAgentFacet) {
    return existingHostname || incomingHostname;
  }
  if (incomingWeak) {
    return existingHostname || incomingHostname;
  }
  return incomingHostname || existingHostname;
}

function mergedLanDeviceLabel(
  existing: DeviceSlot | undefined,
  input: {
    readonly deviceId: string;
    readonly label: string;
    readonly hostname?: string | undefined;
    readonly model?: string | undefined;
  } & LanAgentFacetInput,
  slotValue: string,
  slotIndex: number
): string {
  const preferredLabel = preferredLanDeviceLabel(input, slotValue, slotIndex);
  if (!existing || hasAgentFacet(input)) {
    return preferredLabel;
  }
  if (existing.label && !existing.label.startsWith('LAN ')) return existing.label;
  return preferredLabel || existing.label || activityDeviceShortLabel(slotValue, slotIndex);
}

function preferredLanDeviceLabel(
  input: {
    readonly deviceId: string;
    readonly label: string;
    readonly hostname?: string | undefined;
    readonly model?: string | undefined;
  },
  slotValue: string,
  slotIndex: number
): string {
  const label = input.label.trim();
  const hostLabel = usableLanDeviceName(input.hostname);
  const modelLabel = usableLanDeviceName(input.model);
  if (rawLanDeviceLabel(label, input.deviceId) && hostLabel) return hostLabel;
  if (rawLanDeviceLabel(label, input.deviceId) && modelLabel) return modelLabel;
  return label || hostLabel || modelLabel || activityDeviceShortLabel(slotValue, slotIndex);
}

function detectedLanDeviceName(input: {
  readonly deviceId: string;
  readonly label: string;
  readonly hostname?: string | undefined;
  readonly model?: string | undefined;
  readonly householdName?: string | undefined;
}): string {
  const hostLabel = usableLanDeviceName(input.hostname);
  if (hostLabel) return hostLabel;
  const modelLabel = usableLanDeviceName(input.model);
  if (modelLabel) return modelLabel;
  const label = input.label.trim();
  if (input.householdName && label.toLowerCase() === input.householdName.toLowerCase()) return '';
  return rawLanDeviceLabel(label, input.deviceId) ? '' : label;
}

function rawLanDeviceLabel(label: string, deviceId: string): boolean {
  const normalized = label.trim().toLowerCase();
  return (
    !normalized ||
    normalized === deviceId.toLowerCase() ||
    normalized === 'local-dev-agent' ||
    normalized.startsWith('lan ')
  );
}

function usableLanDeviceName(value?: string): string {
  const trimmed = value?.trim();
  if (!trimmed) return '';
  const normalized = trimmed.toLowerCase();
  if (normalized === 'unknown' || normalized === 'unknown-host' || normalized === 'not reported') return '';
  return trimmed;
}

function slotHasAgentFacet(slot: DeviceSlot): boolean {
  return hasAgentFacet({
    agentStatus: slot.device?.agentStatus,
    cpuModel: slot.device?.cpuModel,
    memoryTotal: slot.device?.memoryTotal,
    gpuModel: slot.device?.gpuModel,
  });
}

function hasAgentFacet(input: LanAgentFacetInput): boolean {
  return !!input.agentStatus;
}

function normalizeDevicePlatform(value: string): DevicePlatformKind {
  if (
    value === 'windows' ||
    value === 'macos' ||
    value === 'linux' ||
    value === 'android' ||
    value === 'ios' ||
    value === 'router'
  ) {
    return value;
  }
  return 'unknown';
}

function normalizeDeviceKind(platform: DevicePlatformKind): DeviceKind {
  if (platform === 'windows' || platform === 'macos' || platform === 'linux') return 'desktop';
  if (platform === 'android' || platform === 'ios') return 'mobile';
  if (platform === 'router') return 'router';
  return 'unknown';
}

function normalizedLanDeviceKindValue(value: string): DeviceKind | undefined {
  if (
    value === 'mobile' ||
    value === 'desktop' ||
    value === 'laptop' ||
    value === 'tablet' ||
    value === 'router' ||
    value === 'unknown'
  ) {
    return value;
  }
  return undefined;
}

function inferLanDeviceKind(input: {
  readonly label: string;
  readonly platform: DevicePlatformKind;
  readonly hostname?: string | undefined;
  readonly manufacturer?: string | undefined;
  readonly model?: string | undefined;
  readonly cpuModel?: string | undefined;
}): DeviceKind {
  const evidenceText = [input.label, input.hostname, input.manufacturer, input.model, input.cpuModel]
    .filter(Boolean)
    .join(' ')
    .toLowerCase();

  if (input.platform === 'router' || /\b(router|gateway|access point|ap)\b/u.test(evidenceText)) return 'router';
  if (/\b(ipad|tablet|galaxy tab|kindle|tab a|tab s)\b/u.test(evidenceText)) return 'tablet';
  if (/\b(iphone|android phone|pixel|galaxy|phone)\b/u.test(evidenceText)) return 'mobile';
  if (/\b(laptop|notebook|macbook|thinkpad|elitebook|probook|latitude|xps|vivobook|yoga)\b/u.test(evidenceText)) {
    return 'laptop';
  }
  if (/\b(desktop|workstation|tower|imac|nuc|mini pc)\b/u.test(evidenceText)) return 'desktop';
  return normalizeDeviceKind(input.platform);
}

function activityDeviceSlots(deviceStates: ReadonlyMap<string, string>, planSeatLimit: number): readonly DeviceSlot[] {
  const devices = Array.from(deviceStates.entries())
    .slice(0, planSeatLimit)
    .map(([deviceId, state], index) => activityDeviceSlot(deviceId, state, index));

  while (devices.length < planSeatLimit) {
    devices.push(emptyActivityDeviceSlot(devices.length));
  }

  return devices;
}

function collectActivityDeviceStates(readModels: readonly (Record<string, unknown> | null)[]): Map<string, string> {
  const deviceStates = new Map<string, string>();
  for (const readModel of readModels) {
    if (!readModel) continue;
    collectRequestScope(readModel, deviceStates);
    collectReportSourceStates(readModel, deviceStates);
    collectReportHistorySourceStates(readModel, deviceStates);
    collectReadModelRows(readModel, deviceStates);
  }
  return deviceStates;
}

function collectRequestScope(readModel: Record<string, unknown>, deviceStates: Map<string, string>): void {
  const request = recordValue(readModel['request']);
  const scope = recordValue(request?.['scope']);
  const deviceId = stringValue(scope?.['deviceId']);
  if (deviceId) {
    mergeDeviceState(deviceStates, deviceId, stringValue(readModel['state']) || 'unavailable');
  }
}

function collectReportSourceStates(readModel: Record<string, unknown>, deviceStates: Map<string, string>): void {
  const sourceStates = readModel['sourceStates'];
  if (!Array.isArray(sourceStates)) return;
  for (const sourceState of sourceStates) {
    if (!isRecord(sourceState)) continue;
    const deviceId = stringValue(sourceState['deviceId']);
    if (!deviceId) continue;
    const state = stringValue(sourceState['state']) || stringValue(sourceState['reachabilityState']) || 'unavailable';
    mergeDeviceState(deviceStates, deviceId, state);
  }
}

function collectReportHistorySourceStates(readModel: Record<string, unknown>, deviceStates: Map<string, string>): void {
  const reports = readModel['reports'];
  if (!Array.isArray(reports)) return;
  for (const item of reports) {
    const report = recordValue(recordValue(item)?.['parsedReport']);
    if (report) {
      collectReportSourceStates(report, deviceStates);
    }
  }
}

function collectReadModelRows(readModel: Record<string, unknown>, deviceStates: Map<string, string>): void {
  const rows = readModel['rows'];
  if (!Array.isArray(rows)) return;
  for (const row of rows) {
    if (!isRecord(row)) continue;
    const deviceId = stringValue(row['deviceId']);
    if (!deviceId) continue;
    mergeDeviceState(
      deviceStates,
      deviceId,
      stringValue(row['state']) || stringValue(readModel['state']) || 'unavailable'
    );
  }
}

function mergeDeviceState(deviceStates: Map<string, string>, deviceId: string, state: string): void {
  const existing = deviceStates.get(deviceId);
  if (!existing || activityDeviceStateRank(state) > activityDeviceStateRank(existing)) {
    deviceStates.set(deviceId, state);
  }
}

function activityDeviceStateRank(state: string): number {
  switch (state) {
    case 'ready':
    case 'reachable':
    case 'online':
    case 'paired':
      return 5;
    case 'discovered':
    case 'pending':
    case 'pairing':
    case 'empty':
      return 4;
    case 'stale':
      return 3;
    case 'offline':
    case 'unreachable':
      return 2;
    case 'permission-required':
    case 'scaffold-only':
    case 'unavailable':
    case 'infrastructure':
    case 'manual-required':
    case 'rejected':
    case 'expired':
    case 'ignored':
    case 'revoked':
    case 'error':
      return 1;
    default:
      return 0;
  }
}

function activityDeviceSlot(deviceId: string, state: string, slotIndex: number): DeviceSlot {
  const status = activityDeviceChoiceStatus(state);
  return {
    value: deviceId,
    label: activityDeviceShortLabel(deviceId, slotIndex),
    status,
    slotIndex,
    badge: state,
    device: {
      id: deviceId,
      name: deviceId,
      type: 'unknown',
      platform: 'unknown',
      status,
    },
  };
}

function activityDeviceChoiceStatus(state: string): 'connected' | 'available' | 'offline' | 'unsupported' {
  if (state === 'ready' || state === 'reachable' || state === 'online' || state === 'paired') return 'connected';
  if (state === 'empty' || state === 'stale' || state === 'discovered' || state === 'pending' || state === 'pairing') {
    return 'available';
  }
  if (state === 'offline' || state === 'unreachable') return 'offline';
  return 'unsupported';
}

function emptyActivityDeviceSlot(slotIndex: number): DeviceSlot {
  return {
    value: `activity-empty-seat-${slotIndex + 1}`,
    label: '',
    status: 'empty',
    slotIndex,
  };
}

function activityDeviceShortLabel(deviceId: string, slotIndex: number): string {
  const suffix = deviceId
    .replace(/[^A-Za-z0-9]/g, '')
    .slice(-3)
    .toUpperCase();
  return suffix || `D${String(slotIndex + 1).padStart(3, '0')}`;
}

function activityReportFiles(
  reportDocument: Record<string, unknown> | null,
  reportHistory: Record<string, unknown> | null
): readonly ParentPortalActivityReportFile[] {
  const currentReport = reportDocument ? [activityReportFileFromDocument(reportDocument, null)] : [];
  const reports = reportHistory?.['reports'];
  if (!Array.isArray(reports)) return currentReport;
  return currentReport.concat(
    reports.flatMap((item) => {
      if (!isRecord(item)) return [];
      const report = recordValue(item['parsedReport']);
      if (!report) return [];
      return [activityReportFileFromDocument(report, item)];
    })
  );
}

function activityReportFileFromDocument(
  report: Record<string, unknown>,
  savedItem: Record<string, unknown> | null
): ParentPortalActivityReportFile {
  const reportId = stringValue(report['reportId']) || stringValue(savedItem?.['reportId']) || 'activity-report';
  const fileName = stringValue(savedItem?.['fileName']) || `${reportId}.json`;
  const saved = savedItem !== null || isRecord(report['savedMetadata']);
  return {
    id: reportId,
    fileName,
    dateLabel: dateLabel(stringValue(savedItem?.['reportDate']) || stringValue(report['generatedAt'])),
    rangeLabel: rangeLabel(stringValue(report['rangeStart']), stringValue(report['rangeEnd'])),
    summary: reportSummary(report),
    saved,
    report: activityReportView(report, fileName, saved),
  };
}

function activityReportView(
  report: Record<string, unknown>,
  fileName: string,
  saved: boolean
): ParentPortalActivityReportView {
  return {
    title: stringValue(report['reportId']) || fileName,
    summary: reportSummary(report),
    targetLabel: reportTargetLabel(report),
    saved,
    fileName,
    sections: reportSections(report),
  };
}

function reportSummary(report: Record<string, unknown>): string {
  const sections = report['sections'];
  if (!Array.isArray(sections)) return 'Activity report returned by the local service';
  for (const section of sections) {
    const summary = stringValue(recordValue(section)?.['summary']);
    if (summary) return summary;
  }
  return 'Activity report returned by the local service';
}

function reportTargetLabel(report: Record<string, unknown>): string {
  const scope = recordValue(report['scope']);
  const scopeKind = stringValue(scope?.['scopeKind']);
  const deviceId = stringValue(scope?.['deviceId']);
  if (scopeKind === 'device' && deviceId) return `Device ${deviceId}`;
  return 'Family';
}

function reportSections(report: Record<string, unknown>): readonly ParentPortalActivityReportSection[] {
  const sections = report['sections'];
  if (!Array.isArray(sections)) return [];
  return sections.flatMap((section) => {
    if (!isRecord(section)) return [];
    const title = stringValue(section['title']) || stringValue(section['sectionKind']) || 'Report section';
    const state = stringValue(section['state']) || 'not-reported';
    const itemCount = numberValue(section['itemCount']);
    const summary = stringValue(section['summary']) || `${title} is ${state}`;
    return [
      {
        title,
        lines: [summary, itemCount === null ? `State ${state}` : `State ${state}; ${itemCount} items`],
      },
    ];
  });
}

function dateLabel(value: string): string {
  return value ? value.slice(0, 10) : 'No date';
}

function rangeLabel(start: string, end: string): string {
  const startLabel = dateLabel(start);
  const endLabel = dateLabel(end);
  if (startLabel === 'No date' && endLabel === 'No date') return 'No range';
  if (startLabel === endLabel) return startLabel;
  return `${startLabel} to ${endLabel}`;
}

function numberValue(value: unknown): number | null {
  return typeof value === 'number' && Number.isFinite(value) ? value : null;
}

function stringValue(value: unknown): string {
  return typeof value === 'string' ? value : '';
}

function firstString(value: unknown): string {
  if (!Array.isArray(value)) return '';
  for (const item of value) {
    const next = stringValue(item);
    if (next) return next;
  }
  return '';
}

function stringArrayValue(value: unknown): readonly string[] {
  return arrayValue(value)
    .map(stringValue)
    .filter((item) => item.length > 0);
}

function recordValue(value: unknown): Record<string, unknown> | null {
  return isRecord(value) ? value : null;
}

function recordArrayValue(value: unknown): readonly Record<string, unknown>[] {
  return arrayValue(value)
    .map(recordValue)
    .filter((record): record is Record<string, unknown> => record !== null);
}

function arrayValue(value: unknown): readonly unknown[] {
  return Array.isArray(value) ? value : [];
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === 'object' && value !== null && !Array.isArray(value);
}
