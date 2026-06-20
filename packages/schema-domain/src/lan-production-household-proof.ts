import { type Infer, Schema, withParser, NonEmptyStringSchema } from './effect';
import {
  LanPairingProductionDiscoveryStateSchema,
  LanPairingSchemaVersionSchema,
  LanPairingTimestampSchema,
} from './lan-pairing-values';
import { LanHouseholdProductProofStateSchema } from './lan-product-proof';

export const LanProductionHouseholdProofCapabilitySchema = withParser(
  Schema.Literal(
    'signed-lan-hello',
    'signed-lan-heartbeat',
    'passive-neighbor-discovery',
    'router-neighbor-discovery',
    'mdns-name-discovery',
    'ssdp-name-discovery',
    'router-dhcp-name-discovery',
    'trusted-registry',
    'parent-assignment',
    'parent-rename',
    'parent-ignore',
    'parent-revocation',
    'route-custody',
    'stale-selected-device',
    'offline-selected-device',
    'relay-route',
    'cache-route',
    'second-physical-child-agent',
    'android-child-agent-parity',
    'ios-child-agent-parity',
    'store-signing'
  )
);

export const LanProductionHouseholdProofRuntimeOwnerSchema = withParser(
  Schema.Literal('parent-domain-contract', 'agent-protocol', 'rust-service-read-model', 'proof-harness', 'manual-proof')
);

export const LanProductionHouseholdProofStatusSchema = withParser(
  Schema.Struct({
    schemaVersion: LanPairingSchemaVersionSchema,
    capability: LanProductionHouseholdProofCapabilitySchema,
    discoveryState: LanPairingProductionDiscoveryStateSchema,
    proofState: LanHouseholdProductProofStateSchema,
    runtimeOwner: LanProductionHouseholdProofRuntimeOwnerSchema,
    evidenceLabel: NonEmptyStringSchema,
    requiredArtifactSummary: Schema.Union(NonEmptyStringSchema, Schema.Null),
  })
);

const LanProductionHouseholdProofSummaryBaseSchema = Schema.Struct({
  schemaVersion: LanPairingSchemaVersionSchema,
  generatedAt: LanPairingTimestampSchema,
  statusRows: Schema.Array(LanProductionHouseholdProofStatusSchema),
  manualProofRequired: Schema.Array(LanProductionHouseholdProofCapabilitySchema),
  notImplemented: Schema.Array(LanProductionHouseholdProofCapabilitySchema),
  claimsProved: Schema.Array(NonEmptyStringSchema),
  claimsNotProved: Schema.Array(NonEmptyStringSchema),
});

type LanProductionHouseholdProofSummaryCandidate = Infer<typeof LanProductionHouseholdProofSummaryBaseSchema>;

export const LanProductionHouseholdProofSummarySchema = withParser(
  LanProductionHouseholdProofSummaryBaseSchema.pipe(
    Schema.filter(
      (summary) =>
        productionHouseholdProofSummaryIsHonest(summary) ||
        'Expected production LAN household proof to keep signed discovery, physical second-agent, mobile parity, relay, cache, and store/signing gaps explicit'
    )
  )
);

const RequiredCapabilities = [
  'signed-lan-hello',
  'signed-lan-heartbeat',
  'passive-neighbor-discovery',
  'router-neighbor-discovery',
  'mdns-name-discovery',
  'ssdp-name-discovery',
  'router-dhcp-name-discovery',
  'trusted-registry',
  'parent-assignment',
  'parent-rename',
  'parent-ignore',
  'parent-revocation',
  'route-custody',
  'stale-selected-device',
  'offline-selected-device',
  'relay-route',
  'cache-route',
  'second-physical-child-agent',
  'android-child-agent-parity',
  'ios-child-agent-parity',
  'store-signing',
] as const satisfies ReadonlyArray<LanProductionHouseholdProofCapability>;

const RequiredManualCapabilities = [
  'signed-lan-hello',
  'signed-lan-heartbeat',
  'mdns-name-discovery',
  'ssdp-name-discovery',
  'router-dhcp-name-discovery',
  'second-physical-child-agent',
  'android-child-agent-parity',
  'ios-child-agent-parity',
  'store-signing',
] as const satisfies ReadonlyArray<LanProductionHouseholdProofCapability>;

const RequiredNotImplementedCapabilities = [
  'relay-route',
  'cache-route',
] as const satisfies ReadonlyArray<LanProductionHouseholdProofCapability>;

function productionHouseholdProofSummaryIsHonest(summary: LanProductionHouseholdProofSummaryCandidate): boolean {
  const rows = new Map(summary.statusRows.map((row) => [row.capability, row] as const));
  const manualProof = new Set(summary.manualProofRequired);
  const notImplemented = new Set(summary.notImplemented);

  return (
    RequiredCapabilities.every((capability) => rows.has(capability)) &&
    RequiredManualCapabilities.every((capability) => {
      const row = rows.get(capability);
      return row?.proofState === 'manual-required' && manualProof.has(capability);
    }) &&
    RequiredNotImplementedCapabilities.every((capability) => {
      const row = rows.get(capability);
      return row?.proofState === 'not-implemented' && notImplemented.has(capability);
    }) &&
    summary.statusRows.some(
      (row) => row.capability === 'passive-neighbor-discovery' && row.proofState === 'ci-mechanical-proof'
    ) &&
    summary.statusRows.some((row) => row.capability === 'route-custody' && row.proofState === 'ci-mechanical-proof') &&
    summary.claimsNotProved.some((claim) => claim.includes('physical household LAN')) &&
    summary.claimsNotProved.some((claim) => claim.includes('signed LAN hello')) &&
    summary.claimsNotProved.some((claim) => claim.includes('cloud relay')) &&
    summary.claimsNotProved.some((claim) => claim.includes('Android child-agent parity')) &&
    summary.claimsNotProved.some((claim) => claim.includes('iOS child-agent parity')) &&
    summary.claimsNotProved.some((claim) => claim.includes('store signing'))
  );
}

export type LanProductionHouseholdProofCapability = Infer<typeof LanProductionHouseholdProofCapabilitySchema>;
export type LanProductionHouseholdProofRuntimeOwner = Infer<typeof LanProductionHouseholdProofRuntimeOwnerSchema>;
export type LanProductionHouseholdProofStatus = Infer<typeof LanProductionHouseholdProofStatusSchema>;
export type LanProductionHouseholdProofSummary = Infer<typeof LanProductionHouseholdProofSummarySchema>;
