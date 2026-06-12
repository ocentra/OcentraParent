import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { LanHouseholdProductProofStateSchema } from './lan-pairing-product-proof';
import {
  LanPairingProductionDiscoveryStateSchema,
  LanPairingRejectionReasonSchema,
  LanPairingResponseStateSchema,
  LanPairingRouteIdSchema,
  LanPairingSchemaVersionSchema,
  LanPairingTimestampSchema,
} from './lan-pairing-values';
import { LanProductionHouseholdProofRuntimeOwnerSchema } from './lan-production-household-proof';

const NonEmptySignedDiscoveryRelayText = Schema.String.pipe(Schema.minLength(1));

export const LanSignedDiscoveryRelayAdapterKindSchema = withParser(
  Schema.Literal(
    'passive-lan-neighbor',
    'router-infrastructure',
    'mdns-name',
    'ssdp-name',
    'router-dhcp-name',
    'manual-direct-address',
    'signed-child-agent-hello',
    'signed-child-agent-heartbeat'
  )
);

export const LanSignedDiscoveryRelaySourceConfidenceSchema = withParser(
  Schema.Literal('confirmed', 'strong', 'weak', 'manual-required', 'unavailable', 'rejected')
);

export const LanSignedDiscoveryRelayCustodyLabelSchema = withParser(
  Schema.Literal(
    'parent-local-service',
    'passive-lan-observation',
    'router-infrastructure-observation',
    'manual-parent-entry',
    'signed-child-agent-artifact',
    'no-ocentra-child-data-custody',
    'parent-owned-storage-unavailable'
  )
);

export const LanSignedDiscoveryRelaySignedProofCheckSchema = withParser(
  Schema.Literal(
    'signed-hello-manual-required',
    'signed-heartbeat-manual-required',
    'accepted-signed-child-agent-manual-required',
    'unauthenticated-caller-rejected',
    'expired-signed-proof-rejected',
    'replayed-signed-proof-rejected',
    'wrong-origin-signed-proof-rejected',
    'wrong-device-signed-proof-rejected',
    'revoked-signed-proof-rejected',
    'stale-signed-proof-rejected'
  )
);

export const LanSignedDiscoveryRelayRouteSafetyCheckSchema = withParser(
  Schema.Literal(
    'trusted-registry-restart-recovery',
    'selected-route-custody',
    'stale-selected-device-rejected',
    'offline-selected-device-rejected',
    'wrong-route-rejected',
    'revoked-route-rejected',
    'parent-assign-decision-audited',
    'parent-rename-decision-audited',
    'parent-ignore-decision-audited',
    'parent-restore-decision-audited',
    'parent-trust-decision-audited',
    'parent-revoke-decision-audited'
  )
);

export const LanSignedDiscoveryRelayCacheCheckSchema = withParser(
  Schema.Literal(
    'relay-route-unavailable',
    'relay-route-queued-not-configured',
    'cache-route-unavailable',
    'parent-owned-storage-unavailable',
    'ocentra-child-data-custody-not-claimed'
  )
);

export const LanSignedDiscoveryRelayDecisionStateSchema = withParser(
  Schema.Literal('local-first', 'unavailable', 'queued-not-configured', 'not-implemented')
);

export const LanSignedDiscoveryRelayAdapterRowSchema = withParser(
  Schema.Struct({
    schemaVersion: LanPairingSchemaVersionSchema,
    adapter: LanSignedDiscoveryRelayAdapterKindSchema,
    discoveryState: LanPairingProductionDiscoveryStateSchema,
    proofState: LanHouseholdProductProofStateSchema,
    sourceConfidence: LanSignedDiscoveryRelaySourceConfidenceSchema,
    custodyLabel: LanSignedDiscoveryRelayCustodyLabelSchema,
    runtimeOwner: LanProductionHouseholdProofRuntimeOwnerSchema,
    evidenceLabel: NonEmptySignedDiscoveryRelayText,
    requiredArtifactSummary: Schema.Union(NonEmptySignedDiscoveryRelayText, Schema.Null),
  })
);

export const LanSignedDiscoveryRelaySignedProofRowSchema = withParser(
  Schema.Struct({
    schemaVersion: LanPairingSchemaVersionSchema,
    check: LanSignedDiscoveryRelaySignedProofCheckSchema,
    discoveryState: LanPairingProductionDiscoveryStateSchema,
    responseState: LanPairingResponseStateSchema,
    rejectionReason: Schema.Union(LanPairingRejectionReasonSchema, Schema.Null),
    proofState: LanHouseholdProductProofStateSchema,
    runtimeOwner: LanProductionHouseholdProofRuntimeOwnerSchema,
    evidenceLabel: NonEmptySignedDiscoveryRelayText,
  })
);

export const LanSignedDiscoveryRelayRouteSafetyRowSchema = withParser(
  Schema.Struct({
    schemaVersion: LanPairingSchemaVersionSchema,
    check: LanSignedDiscoveryRelayRouteSafetyCheckSchema,
    routeId: Schema.Union(LanPairingRouteIdSchema, Schema.Null),
    discoveryState: LanPairingProductionDiscoveryStateSchema,
    responseState: LanPairingResponseStateSchema,
    rejectionReason: Schema.Union(LanPairingRejectionReasonSchema, Schema.Null),
    proofState: LanHouseholdProductProofStateSchema,
    runtimeOwner: LanProductionHouseholdProofRuntimeOwnerSchema,
    custodyLabel: LanSignedDiscoveryRelayCustodyLabelSchema,
    evidenceLabel: NonEmptySignedDiscoveryRelayText,
  })
);

export const LanSignedDiscoveryRelayCacheRowSchema = withParser(
  Schema.Struct({
    schemaVersion: LanPairingSchemaVersionSchema,
    check: LanSignedDiscoveryRelayCacheCheckSchema,
    decisionState: LanSignedDiscoveryRelayDecisionStateSchema,
    discoveryState: LanPairingProductionDiscoveryStateSchema,
    proofState: LanHouseholdProductProofStateSchema,
    runtimeOwner: LanProductionHouseholdProofRuntimeOwnerSchema,
    custodyLabel: LanSignedDiscoveryRelayCustodyLabelSchema,
    evidenceLabel: NonEmptySignedDiscoveryRelayText,
  })
);

const LanSignedDiscoveryRelaySpineBaseSchema = Schema.Struct({
  schemaVersion: LanPairingSchemaVersionSchema,
  generatedAt: LanPairingTimestampSchema,
  adapterRows: Schema.Array(LanSignedDiscoveryRelayAdapterRowSchema),
  signedProofRows: Schema.Array(LanSignedDiscoveryRelaySignedProofRowSchema),
  routeSafetyRows: Schema.Array(LanSignedDiscoveryRelayRouteSafetyRowSchema),
  relayCacheRows: Schema.Array(LanSignedDiscoveryRelayCacheRowSchema),
  manualProofRequired: Schema.Array(LanSignedDiscoveryRelayAdapterKindSchema),
  notImplemented: Schema.Array(LanSignedDiscoveryRelayCacheCheckSchema),
  claimsProved: Schema.Array(NonEmptySignedDiscoveryRelayText),
  claimsNotProved: Schema.Array(NonEmptySignedDiscoveryRelayText),
});

type LanSignedDiscoveryRelaySpineCandidate = Infer<typeof LanSignedDiscoveryRelaySpineBaseSchema>;

export const LanSignedDiscoveryRelaySpineSchema = withParser(
  LanSignedDiscoveryRelaySpineBaseSchema.pipe(
    Schema.filter(
      (spine) =>
        signedDiscoveryRelaySpineIsHonest(spine) ||
        'Expected signed LAN discovery relay spine to keep signed child-agent artifacts, physical household proof, relay/cache, and parent-owned storage as explicit manual or unavailable boundaries'
    )
  )
);

const RequiredAdapters = [
  'passive-lan-neighbor',
  'router-infrastructure',
  'mdns-name',
  'ssdp-name',
  'router-dhcp-name',
  'manual-direct-address',
  'signed-child-agent-hello',
  'signed-child-agent-heartbeat',
] as const satisfies ReadonlyArray<LanSignedDiscoveryRelayAdapterKind>;

const RequiredManualAdapters = [
  'mdns-name',
  'ssdp-name',
  'router-dhcp-name',
  'manual-direct-address',
  'signed-child-agent-hello',
  'signed-child-agent-heartbeat',
] as const satisfies ReadonlyArray<LanSignedDiscoveryRelayAdapterKind>;

const RequiredSignedProofChecks = [
  'signed-hello-manual-required',
  'signed-heartbeat-manual-required',
  'accepted-signed-child-agent-manual-required',
  'unauthenticated-caller-rejected',
  'expired-signed-proof-rejected',
  'replayed-signed-proof-rejected',
  'wrong-origin-signed-proof-rejected',
  'wrong-device-signed-proof-rejected',
  'revoked-signed-proof-rejected',
  'stale-signed-proof-rejected',
] as const satisfies ReadonlyArray<LanSignedDiscoveryRelaySignedProofCheck>;

const RequiredRouteSafetyChecks = [
  'trusted-registry-restart-recovery',
  'selected-route-custody',
  'stale-selected-device-rejected',
  'offline-selected-device-rejected',
  'wrong-route-rejected',
  'revoked-route-rejected',
  'parent-assign-decision-audited',
  'parent-rename-decision-audited',
  'parent-ignore-decision-audited',
  'parent-restore-decision-audited',
  'parent-trust-decision-audited',
  'parent-revoke-decision-audited',
] as const satisfies ReadonlyArray<LanSignedDiscoveryRelayRouteSafetyCheck>;

const RequiredRelayCacheChecks = [
  'relay-route-unavailable',
  'relay-route-queued-not-configured',
  'cache-route-unavailable',
  'parent-owned-storage-unavailable',
  'ocentra-child-data-custody-not-claimed',
] as const satisfies ReadonlyArray<LanSignedDiscoveryRelayCacheCheck>;

function signedDiscoveryRelaySpineIsHonest(spine: LanSignedDiscoveryRelaySpineCandidate): boolean {
  return (
    adaptersAreComplete(spine) &&
    signedProofRowsAreComplete(spine.signedProofRows) &&
    routeSafetyRowsAreComplete(spine.routeSafetyRows) &&
    relayCacheRowsAreHonest(spine.relayCacheRows, spine.notImplemented) &&
    spine.claimsNotProved.some((claim) => claim.includes('signed child-agent')) &&
    spine.claimsNotProved.some((claim) => claim.includes('physical household LAN')) &&
    spine.claimsNotProved.some((claim) => claim.includes('relay or cache')) &&
    spine.claimsNotProved.some((claim) => claim.includes('parent-owned storage'))
  );
}

function adaptersAreComplete(spine: LanSignedDiscoveryRelaySpineCandidate): boolean {
  const byAdapter = new Map(spine.adapterRows.map((row) => [row.adapter, row] as const));
  const manual = new Set(spine.manualProofRequired);

  return (
    RequiredAdapters.every((adapter) => byAdapter.has(adapter)) &&
    RequiredManualAdapters.every((adapter) => {
      const row = byAdapter.get(adapter);
      return row?.proofState === 'manual-required' && manual.has(adapter);
    }) &&
    byAdapter.get('passive-lan-neighbor')?.custodyLabel === 'passive-lan-observation' &&
    byAdapter.get('router-infrastructure')?.custodyLabel === 'router-infrastructure-observation' &&
    byAdapter.get('signed-child-agent-hello')?.custodyLabel === 'signed-child-agent-artifact' &&
    byAdapter.get('signed-child-agent-heartbeat')?.custodyLabel === 'signed-child-agent-artifact'
  );
}

function signedProofRowsAreComplete(rows: ReadonlyArray<LanSignedDiscoveryRelaySignedProofRow>): boolean {
  const byCheck = new Map(rows.map((row) => [row.check, row] as const));
  const rejectedRows = rows.filter((row) => row.responseState === 'rejected');
  return (
    RequiredSignedProofChecks.every((check) => byCheck.has(check)) &&
    manualSignedProofRowIsHonest(byCheck.get('signed-hello-manual-required')) &&
    manualSignedProofRowIsHonest(byCheck.get('signed-heartbeat-manual-required')) &&
    manualSignedProofRowIsHonest(byCheck.get('accepted-signed-child-agent-manual-required')) &&
    rejectedRows.length >= 7 &&
    rejectedRows.every((row) => row.rejectionReason !== null && row.proofState === 'ci-mechanical-proof')
  );
}

function manualSignedProofRowIsHonest(row: LanSignedDiscoveryRelaySignedProofRow | undefined): boolean {
  return row !== undefined && row.proofState === 'manual-required' && row.rejectionReason === null;
}

function routeSafetyRowsAreComplete(rows: ReadonlyArray<LanSignedDiscoveryRelayRouteSafetyRow>): boolean {
  const byCheck = new Map(rows.map((row) => [row.check, row] as const));
  return (
    RequiredRouteSafetyChecks.every((check) => byCheck.has(check)) &&
    byCheck.get('wrong-route-rejected')?.rejectionReason === 'wrong-device' &&
    byCheck.get('revoked-route-rejected')?.rejectionReason === 'revoked' &&
    byCheck.get('stale-selected-device-rejected')?.rejectionReason === 'stale' &&
    byCheck.get('offline-selected-device-rejected')?.rejectionReason === 'offline'
  );
}

function relayCacheRowsAreHonest(
  rows: ReadonlyArray<LanSignedDiscoveryRelayCacheRow>,
  notImplemented: ReadonlyArray<LanSignedDiscoveryRelayCacheCheck>
): boolean {
  const byCheck = new Map(rows.map((row) => [row.check, row] as const));
  const missing = new Set(notImplemented);
  return (
    RequiredRelayCacheChecks.every((check) => byCheck.has(check)) &&
    RequiredRelayCacheChecks.slice(0, 4).every((check) => {
      const row = byCheck.get(check);
      return missing.has(check) && row?.proofState === 'not-implemented';
    }) &&
    byCheck.get('relay-route-queued-not-configured')?.decisionState === 'queued-not-configured' &&
    byCheck.get('ocentra-child-data-custody-not-claimed')?.custodyLabel === 'no-ocentra-child-data-custody'
  );
}

export type LanSignedDiscoveryRelayAdapterKind = Infer<typeof LanSignedDiscoveryRelayAdapterKindSchema>;
export type LanSignedDiscoveryRelaySourceConfidence = Infer<typeof LanSignedDiscoveryRelaySourceConfidenceSchema>;
export type LanSignedDiscoveryRelayCustodyLabel = Infer<typeof LanSignedDiscoveryRelayCustodyLabelSchema>;
export type LanSignedDiscoveryRelaySignedProofCheck = Infer<typeof LanSignedDiscoveryRelaySignedProofCheckSchema>;
export type LanSignedDiscoveryRelayRouteSafetyCheck = Infer<typeof LanSignedDiscoveryRelayRouteSafetyCheckSchema>;
export type LanSignedDiscoveryRelayCacheCheck = Infer<typeof LanSignedDiscoveryRelayCacheCheckSchema>;
export type LanSignedDiscoveryRelayDecisionState = Infer<typeof LanSignedDiscoveryRelayDecisionStateSchema>;
export type LanSignedDiscoveryRelayAdapterRow = Infer<typeof LanSignedDiscoveryRelayAdapterRowSchema>;
export type LanSignedDiscoveryRelaySignedProofRow = Infer<typeof LanSignedDiscoveryRelaySignedProofRowSchema>;
export type LanSignedDiscoveryRelayRouteSafetyRow = Infer<typeof LanSignedDiscoveryRelayRouteSafetyRowSchema>;
export type LanSignedDiscoveryRelayCacheRow = Infer<typeof LanSignedDiscoveryRelayCacheRowSchema>;
export type LanSignedDiscoveryRelaySpine = Infer<typeof LanSignedDiscoveryRelaySpineSchema>;
