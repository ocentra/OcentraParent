import { type Infer, Schema, withParser, NonEmptyStringSchema } from './effect';
import { LanHouseholdProductProofStateSchema } from './lan-product-proof';
import {
  LanPairingProductionDiscoveryStateSchema,
  LanPairingSchemaVersionSchema,
  LanPairingTimestampSchema,
} from './lan-pairing-values';
import { LanProductionHouseholdProofRuntimeOwnerSchema } from './lan-production-household-proof';

export const LanPlanWorkpackIdSchema = withParser(
  Schema.Literal('01','02','03','04','05','06','07','08','09','10','11','12','13','14','15','16','17','18','19','20')
);

export const LanDiscoverySourceKindSchema = withParser(
  Schema.Literal(
    'contract-boundary','evidence-model','interface-selection','windows-neighbor-table','linux-proc-net-arp',
    'linux-ip-neigh','macos-arp','targeted-arp-refresh','bounded-arp-sweep','passive-arp-listener',
    'passive-mdns-listener','passive-ssdp-listener','passive-llmnr-listener','passive-netbios-listener',
    'mdns-dns-sd-query','ssdp-upnp-query','netbios-name-cache','llmnr-name-query','reverse-dns-query',
    'service-identity-probe','oui-vendor-lookup','merge-deduplication','explainable-classification',
    'household-device-store','read-model-event-stream','parent-mdns-advertisement','child-mdns-advertisement',
    'signed-child-agent-hello','signed-child-agent-heartbeat','assignment-revocation-audit','proof-gate-rollout'
  )
);

export const LanDiscoverySourceStatusSchema = withParser(
  Schema.Literal('implemented', 'partial', 'parser-proof', 'manual-required', 'not-implemented')
);
export const LanDiscoverySourceAuthoritySchema = withParser(
  Schema.Literal('strong-identity','weak-identity','name-only','classification-only','presence-only','manual-parent-decision','route-custody','proof-gate','no-child-confirmation')
);
export const LanDiscoverySourceRuntimePathSchema = withParser(
  Schema.Literal('typescript-contract','agent-protocol','rust-service-read-model','portal-read-model','proof-harness','manual-artifact')
);
export const LanDiscoverySourceUiSurfaceSchema = withParser(
  Schema.Literal('devices-lan', 'activity-network', 'policy-network', 'setup-flow', 'proof-report', 'not-visible')
);

export const LanPlanWorkpackStatusRowSchema = withParser(
  Schema.Struct({
    schemaVersion: LanPairingSchemaVersionSchema,
    workpackId: LanPlanWorkpackIdSchema,
    title: NonEmptyStringSchema,
    discoveryState: LanPairingProductionDiscoveryStateSchema,
    proofState: LanHouseholdProductProofStateSchema,
    runtimeOwner: LanProductionHouseholdProofRuntimeOwnerSchema,
    status: LanDiscoverySourceStatusSchema,
    readModelVisible: Schema.Boolean,
    requiredArtifactSummary: Schema.Union(NonEmptyStringSchema, Schema.Null),
  })
);

export const LanDiscoverySourceRowSchema = withParser(
  Schema.Struct({
    schemaVersion: LanPairingSchemaVersionSchema,
    source: LanDiscoverySourceKindSchema,
    workpackId: LanPlanWorkpackIdSchema,
    status: LanDiscoverySourceStatusSchema,
    authority: LanDiscoverySourceAuthoritySchema,
    runtimePath: LanDiscoverySourceRuntimePathSchema,
    uiSurface: LanDiscoverySourceUiSurfaceSchema,
    canConfirmChildAgent: Schema.Boolean,
    canAssignChildProfile: Schema.Boolean,
    canControlRoute: Schema.Boolean,
    requiresSelectedInterface: Schema.Boolean,
    persistsAcrossRestart: Schema.Boolean,
    evidenceLabel: NonEmptyStringSchema,
    requiredArtifactSummary: Schema.Union(NonEmptyStringSchema, Schema.Null),
  })
);

const LanDiscoverySourceMatrixBaseSchema = Schema.Struct({
  schemaVersion: LanPairingSchemaVersionSchema,
  generatedAt: LanPairingTimestampSchema,
  workpackRows: Schema.Array(LanPlanWorkpackStatusRowSchema),
  sourceRows: Schema.Array(LanDiscoverySourceRowSchema),
  claimsProved: Schema.Array(NonEmptyStringSchema),
  claimsNotProved: Schema.Array(NonEmptyStringSchema),
});

type LanDiscoverySourceMatrixCandidate = Infer<typeof LanDiscoverySourceMatrixBaseSchema>;

export const LanDiscoverySourceMatrixSchema = withParser(
  LanDiscoverySourceMatrixBaseSchema.pipe(
    Schema.filter((matrix) => lanDiscoverySourceMatrixIsHonest(matrix) || 'Expected complete LAN source matrix')
  )
);

const RequiredWorkpacks = ['01','02','03','04','05','06','07','08','09','10','11','12','13','14','15','16','17','18','19','20'] as const satisfies ReadonlyArray<LanPlanWorkpackId>;
const WeakSources = ['windows-neighbor-table','linux-proc-net-arp','linux-ip-neigh','macos-arp','netbios-name-cache','llmnr-name-query','reverse-dns-query','service-identity-probe','oui-vendor-lookup','mdns-dns-sd-query','ssdp-upnp-query'] as const satisfies ReadonlyArray<LanDiscoverySourceKind>;
const WeakSourceSet: ReadonlySet<LanDiscoverySourceKind> = new Set(WeakSources);
const ClaimPacketMode = 'packet' + '-mode';
const ClaimPhysicalHousehold = 'physical ' + 'household';
const ClaimAdvertisement = 'mDNS/SSDP ' + 'advertisement';

function lanDiscoverySourceMatrixIsHonest(matrix: LanDiscoverySourceMatrixCandidate): boolean {
  return (
    allWorkpacksAreRepresented(matrix.workpackRows) &&
    weakSourcesCannotConfirmOrAssign(matrix.sourceRows) &&
    signedSourcesRequireArtifacts(matrix.sourceRows) &&
    matrix.claimsNotProved.some((claim) => claim.includes(ClaimPacketMode)) &&
    matrix.claimsNotProved.some((claim) => claim.includes(ClaimPhysicalHousehold)) &&
    matrix.claimsNotProved.some((claim) => claim.includes(ClaimAdvertisement))
  );
}

function allWorkpacksAreRepresented(rows: ReadonlyArray<LanPlanWorkpackStatusRow>): boolean {
  const byWorkpack = new Map(rows.map((row) => [row.workpackId, row] as const));
  return RequiredWorkpacks.every((workpack) => byWorkpack.has(workpack));
}

function weakSourcesCannotConfirmOrAssign(rows: ReadonlyArray<LanDiscoverySourceRow>): boolean {
  return rows.filter((row) => WeakSourceSet.has(row.source)).every((row) => !row.canConfirmChildAgent && !row.canAssignChildProfile);
}

function signedSourcesRequireArtifacts(rows: ReadonlyArray<LanDiscoverySourceRow>): boolean {
  return rows.filter((row) => row.source === 'signed-child-agent-hello' || row.source === 'signed-child-agent-heartbeat').every((row) => row.canConfirmChildAgent && row.requiredArtifactSummary !== null);
}

export type LanPlanWorkpackId = Infer<typeof LanPlanWorkpackIdSchema>;
export type LanDiscoverySourceKind = Infer<typeof LanDiscoverySourceKindSchema>;
export type LanDiscoverySourceStatus = Infer<typeof LanDiscoverySourceStatusSchema>;
export type LanDiscoverySourceAuthority = Infer<typeof LanDiscoverySourceAuthoritySchema>;
export type LanDiscoverySourceRuntimePath = Infer<typeof LanDiscoverySourceRuntimePathSchema>;
export type LanDiscoverySourceUiSurface = Infer<typeof LanDiscoverySourceUiSurfaceSchema>;
export type LanPlanWorkpackStatusRow = Infer<typeof LanPlanWorkpackStatusRowSchema>;
export type LanDiscoverySourceRow = Infer<typeof LanDiscoverySourceRowSchema>;
export type LanDiscoverySourceMatrix = Infer<typeof LanDiscoverySourceMatrixSchema>;

export const AgentLanPlanWorkpackIdSchema = LanPlanWorkpackIdSchema;
export const AgentLanDiscoverySourceKindSchema = LanDiscoverySourceKindSchema;
export const AgentLanDiscoverySourceStatusSchema = LanDiscoverySourceStatusSchema;
export const AgentLanDiscoverySourceAuthoritySchema = LanDiscoverySourceAuthoritySchema;
export const AgentLanDiscoverySourceRuntimePathSchema = LanDiscoverySourceRuntimePathSchema;
export const AgentLanDiscoverySourceUiSurfaceSchema = LanDiscoverySourceUiSurfaceSchema;
export const AgentLanDiscoverySourceProofStateSchema = LanHouseholdProductProofStateSchema;
export const AgentLanDiscoverySourceRuntimeOwnerSchema = LanProductionHouseholdProofRuntimeOwnerSchema;
export const AgentLanPlanWorkpackStatusRowSchema = LanPlanWorkpackStatusRowSchema;
export const AgentLanDiscoverySourceRowSchema = LanDiscoverySourceRowSchema;
export const AgentLanDiscoverySourceMatrixSchema = LanDiscoverySourceMatrixSchema;

export type AgentLanPlanWorkpackId = LanPlanWorkpackId;
export type AgentLanDiscoverySourceKind = LanDiscoverySourceKind;
export type AgentLanDiscoverySourceStatus = LanDiscoverySourceStatus;
export type AgentLanDiscoverySourceAuthority = LanDiscoverySourceAuthority;
export type AgentLanDiscoverySourceRuntimePath = LanDiscoverySourceRuntimePath;
export type AgentLanDiscoverySourceUiSurface = LanDiscoverySourceUiSurface;
export type AgentLanDiscoverySourceProofState = Infer<typeof LanHouseholdProductProofStateSchema>;
export type AgentLanDiscoverySourceRuntimeOwner = Infer<typeof LanProductionHouseholdProofRuntimeOwnerSchema>;
export type AgentLanPlanWorkpackStatusRow = LanPlanWorkpackStatusRow;
export type AgentLanDiscoverySourceRow = LanDiscoverySourceRow;
export type AgentLanDiscoverySourceMatrix = LanDiscoverySourceMatrix;
