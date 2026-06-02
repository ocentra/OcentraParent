import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { AgentProtocolSchemaVersion, AgentTimestampSchema } from './primitives';
import { AgentLanPairingProductionDiscoveryStateSchema } from './security';

const NonEmptyLanSourceMatrixText = Schema.String.pipe(Schema.minLength(1));

export const AgentLanPlanWorkpackIdSchema = withParser(
  Schema.Literal(
    '01',
    '02',
    '03',
    '04',
    '05',
    '06',
    '07',
    '08',
    '09',
    '10',
    '11',
    '12',
    '13',
    '14',
    '15',
    '16',
    '17',
    '18',
    '19',
    '20'
  )
);

export const AgentLanDiscoverySourceKindSchema = withParser(
  Schema.Literal(
    'contract-boundary',
    'evidence-model',
    'interface-selection',
    'windows-neighbor-table',
    'linux-proc-net-arp',
    'linux-ip-neigh',
    'macos-arp',
    'targeted-arp-refresh',
    'bounded-arp-sweep',
    'passive-arp-listener',
    'passive-mdns-listener',
    'passive-ssdp-listener',
    'passive-llmnr-listener',
    'passive-netbios-listener',
    'mdns-dns-sd-query',
    'ssdp-upnp-query',
    'netbios-name-cache',
    'llmnr-name-query',
    'reverse-dns-query',
    'service-identity-probe',
    'oui-vendor-lookup',
    'merge-deduplication',
    'explainable-classification',
    'household-device-store',
    'read-model-event-stream',
    'parent-mdns-advertisement',
    'child-mdns-advertisement',
    'signed-child-agent-hello',
    'signed-child-agent-heartbeat',
    'assignment-revocation-audit',
    'proof-gate-rollout'
  )
);

export const AgentLanDiscoverySourceStatusSchema = withParser(
  Schema.Literal('implemented', 'partial', 'parser-proof', 'manual-required', 'not-implemented')
);

export const AgentLanDiscoverySourceAuthoritySchema = withParser(
  Schema.Literal(
    'strong-identity',
    'weak-identity',
    'name-only',
    'classification-only',
    'presence-only',
    'manual-parent-decision',
    'route-custody',
    'proof-gate',
    'no-child-confirmation'
  )
);

export const AgentLanDiscoverySourceRuntimePathSchema = withParser(
  Schema.Literal(
    'typescript-contract',
    'agent-protocol',
    'rust-service-read-model',
    'portal-read-model',
    'proof-harness',
    'manual-artifact'
  )
);

export const AgentLanDiscoverySourceUiSurfaceSchema = withParser(
  Schema.Literal('devices-lan', 'activity-network', 'policy-network', 'setup-flow', 'proof-report', 'not-visible')
);

export const AgentLanDiscoverySourceProofStateSchema = withParser(
  Schema.Literal('ci-mechanical-proof', 'manual-required', 'not-implemented')
);

export const AgentLanDiscoverySourceRuntimeOwnerSchema = withParser(
  Schema.Literal('parent-domain-contract', 'agent-protocol', 'rust-service-read-model', 'proof-harness', 'manual-proof')
);

export const AgentLanPlanWorkpackStatusRowSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(AgentProtocolSchemaVersion),
    workpackId: AgentLanPlanWorkpackIdSchema,
    title: NonEmptyLanSourceMatrixText,
    discoveryState: AgentLanPairingProductionDiscoveryStateSchema,
    proofState: AgentLanDiscoverySourceProofStateSchema,
    runtimeOwner: AgentLanDiscoverySourceRuntimeOwnerSchema,
    status: AgentLanDiscoverySourceStatusSchema,
    readModelVisible: Schema.Boolean,
    requiredArtifactSummary: Schema.Union(NonEmptyLanSourceMatrixText, Schema.Null),
  })
);

export const AgentLanDiscoverySourceRowSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(AgentProtocolSchemaVersion),
    source: AgentLanDiscoverySourceKindSchema,
    workpackId: AgentLanPlanWorkpackIdSchema,
    status: AgentLanDiscoverySourceStatusSchema,
    authority: AgentLanDiscoverySourceAuthoritySchema,
    runtimePath: AgentLanDiscoverySourceRuntimePathSchema,
    uiSurface: AgentLanDiscoverySourceUiSurfaceSchema,
    canConfirmChildAgent: Schema.Boolean,
    canAssignChildProfile: Schema.Boolean,
    canControlRoute: Schema.Boolean,
    requiresSelectedInterface: Schema.Boolean,
    persistsAcrossRestart: Schema.Boolean,
    evidenceLabel: NonEmptyLanSourceMatrixText,
    requiredArtifactSummary: Schema.Union(NonEmptyLanSourceMatrixText, Schema.Null),
  })
);

export const AgentLanDiscoverySourceMatrixSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(AgentProtocolSchemaVersion),
    generatedAt: AgentTimestampSchema,
    workpackRows: Schema.Array(AgentLanPlanWorkpackStatusRowSchema),
    sourceRows: Schema.Array(AgentLanDiscoverySourceRowSchema),
    claimsProved: Schema.Array(NonEmptyLanSourceMatrixText),
    claimsNotProved: Schema.Array(NonEmptyLanSourceMatrixText),
  })
);

export type AgentLanPlanWorkpackId = Infer<typeof AgentLanPlanWorkpackIdSchema>;
export type AgentLanDiscoverySourceKind = Infer<typeof AgentLanDiscoverySourceKindSchema>;
export type AgentLanDiscoverySourceStatus = Infer<typeof AgentLanDiscoverySourceStatusSchema>;
export type AgentLanDiscoverySourceAuthority = Infer<typeof AgentLanDiscoverySourceAuthoritySchema>;
export type AgentLanDiscoverySourceRuntimePath = Infer<typeof AgentLanDiscoverySourceRuntimePathSchema>;
export type AgentLanDiscoverySourceUiSurface = Infer<typeof AgentLanDiscoverySourceUiSurfaceSchema>;
export type AgentLanDiscoverySourceProofState = Infer<typeof AgentLanDiscoverySourceProofStateSchema>;
export type AgentLanDiscoverySourceRuntimeOwner = Infer<typeof AgentLanDiscoverySourceRuntimeOwnerSchema>;
export type AgentLanPlanWorkpackStatusRow = Infer<typeof AgentLanPlanWorkpackStatusRowSchema>;
export type AgentLanDiscoverySourceRow = Infer<typeof AgentLanDiscoverySourceRowSchema>;
export type AgentLanDiscoverySourceMatrix = Infer<typeof AgentLanDiscoverySourceMatrixSchema>;
