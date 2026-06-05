import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { LanDiscoveryEvidenceRecordSchema } from './lan-discovery-evidence';
import { LanPairingRouteIdSchema, LanPairingSchemaVersionSchema } from './lan-pairing-values';
import { ParentEvidenceReferenceIdSchema, ParentTimestampSchema } from './reference-primitives';

const NonEmptyScreenFamilyHubRuntimeText = Schema.String.pipe(Schema.minLength(1));

export const ScreenFamilyAiHubRuntimeDiscoveryProofSchemaVersionSchema = withParser(
  Schema.Literal('screen-family-ai-hub-runtime-discovery-proof')
);

export const ScreenFamilyAiHubRuntimeStateSchema = withParser(
  Schema.Literal('runtime-discovered', 'runtime-unavailable', 'manual-required')
);
export const ScreenFamilyAiHubHouseholdLanStateSchema = withParser(
  Schema.Literal('loopback-runtime-proof', 'physical-household-manual-required')
);
export const ScreenFamilyAiHubCloudRelayStateSchema = withParser(Schema.Literal('not-implemented'));
export const ScreenFamilyAiHubRuntimeExchangeStateSchema = withParser(
  Schema.Literal('accepted', 'completed', 'rejected')
);
export const ScreenFamilyAiHubRuntimeTransferModeSchema = withParser(Schema.Literal('summaryOnly', 'redactedCrop'));
export const ScreenFamilyAiHubRuntimeClaimSchema = NonEmptyScreenFamilyHubRuntimeText.pipe(
  Schema.brand('ScreenFamilyAiHubRuntimeClaim')
);
export const ScreenFamilyAiHubRouteLinkIdSchema = NonEmptyScreenFamilyHubRuntimeText.pipe(
  Schema.brand('ScreenFamilyAiHubRouteLinkId')
);

const ScreenFamilyAiHubRuntimeDiscoveryStateSchema = Schema.Struct({
  runtimeState: ScreenFamilyAiHubRuntimeStateSchema,
  householdLanState: ScreenFamilyAiHubHouseholdLanStateSchema,
  cloudRelayState: ScreenFamilyAiHubCloudRelayStateSchema,
  discoveredAt: ParentTimestampSchema,
  runtimeEndpointRef: NonEmptyScreenFamilyHubRuntimeText,
  discoveryEvidence: Schema.Array(LanDiscoveryEvidenceRecordSchema),
});

const ScreenFamilyAiHubRuntimeRouteLinkSchema = Schema.Struct({
  routeId: ScreenFamilyAiHubRouteLinkIdSchema,
  lanRouteId: LanPairingRouteIdSchema,
  routeExecutionState: Schema.Literal('selected'),
  destinationCustodyState: Schema.Literal('live-lan-child-agent'),
  localProviderAttempted: Schema.Literal(true),
  parentApprovedFamilyHub: Schema.Literal(true),
  remoteApiFallbackAllowed: Schema.Literal(false),
  rawImageRetentionAllowed: Schema.Literal(false),
  ocentraHostedProcessingAllowed: Schema.Literal(false),
});

const ScreenFamilyAiHubRuntimeExchangeSchema = Schema.Struct({
  exchangeState: ScreenFamilyAiHubRuntimeExchangeStateSchema,
  transferMode: ScreenFamilyAiHubRuntimeTransferModeSchema,
  requestEvidenceRef: ParentEvidenceReferenceIdSchema,
  responseEvidenceRef: ParentEvidenceReferenceIdSchema,
  rawFullScreenshotTransferred: Schema.Literal(false),
  rawImageRetained: Schema.Literal(false),
  remoteProviderUsed: Schema.Literal(false),
  ocentraHostedProcessingUsed: Schema.Literal(false),
});

const ScreenFamilyAiHubRuntimeDiscoveryReadModelBaseSchema = Schema.Struct({
  schemaVersion: ScreenFamilyAiHubRuntimeDiscoveryProofSchemaVersionSchema,
  lanSchemaVersion: LanPairingSchemaVersionSchema,
  discovery: ScreenFamilyAiHubRuntimeDiscoveryStateSchema,
  route: ScreenFamilyAiHubRuntimeRouteLinkSchema,
  exchange: ScreenFamilyAiHubRuntimeExchangeSchema,
  claimBoundaries: Schema.Array(ScreenFamilyAiHubRuntimeClaimSchema),
  updatedAt: ParentTimestampSchema,
});

type ScreenFamilyAiHubRuntimeDiscoveryReadModelCandidate = Infer<
  typeof ScreenFamilyAiHubRuntimeDiscoveryReadModelBaseSchema
>;

export const ScreenFamilyAiHubRuntimeDiscoveryReadModelSchema = withParser(
  ScreenFamilyAiHubRuntimeDiscoveryReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        screenFamilyAiHubRuntimeDiscoveryReadModelIsHonest(readModel) ||
        'Expected screen family AI hub runtime discovery proof to keep loopback runtime, physical LAN, cloud relay, raw-image, and remote/API boundaries honest'
    )
  )
);

function screenFamilyAiHubRuntimeDiscoveryReadModelIsHonest(
  readModel: ScreenFamilyAiHubRuntimeDiscoveryReadModelCandidate
): boolean {
  return (
    discoveryEvidenceCoversRuntimeRoute(readModel.discovery.discoveryEvidence) &&
    readModel.discovery.runtimeState === 'runtime-discovered' &&
    readModel.discovery.householdLanState === 'loopback-runtime-proof' &&
    readModel.discovery.cloudRelayState === 'not-implemented' &&
    readModel.route.destinationCustodyState === 'live-lan-child-agent' &&
    readModel.exchange.exchangeState === 'completed' &&
    readModel.exchange.rawFullScreenshotTransferred === false &&
    readModel.exchange.rawImageRetained === false &&
    readModel.exchange.remoteProviderUsed === false &&
    readModel.exchange.ocentraHostedProcessingUsed === false
  );
}

function discoveryEvidenceCoversRuntimeRoute(evidence: ReadonlyArray<ScreenFamilyAiHubRuntimeDiscoveryEvidence>) {
  const sources = new Set(evidence.map((record) => record.source));
  const kinds = new Set(evidence.map((record) => record.evidenceKind));
  return (
    sources.has('child-agent-hello') &&
    sources.has('child-agent-heartbeat') &&
    sources.has('local-service') &&
    kinds.has('child-agent-presence') &&
    kinds.has('route')
  );
}

export type ScreenFamilyAiHubRuntimeDiscoveryEvidence = Infer<typeof LanDiscoveryEvidenceRecordSchema>;
export type ScreenFamilyAiHubRuntimeState = Infer<typeof ScreenFamilyAiHubRuntimeStateSchema>;
export type ScreenFamilyAiHubHouseholdLanState = Infer<typeof ScreenFamilyAiHubHouseholdLanStateSchema>;
export type ScreenFamilyAiHubCloudRelayState = Infer<typeof ScreenFamilyAiHubCloudRelayStateSchema>;
export type ScreenFamilyAiHubRuntimeExchangeState = Infer<typeof ScreenFamilyAiHubRuntimeExchangeStateSchema>;
export type ScreenFamilyAiHubRuntimeTransferMode = Infer<typeof ScreenFamilyAiHubRuntimeTransferModeSchema>;
export type ScreenFamilyAiHubRuntimeDiscoveryReadModel = Infer<typeof ScreenFamilyAiHubRuntimeDiscoveryReadModelSchema>;
