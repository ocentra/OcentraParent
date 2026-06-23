import { type Infer, Schema, withParser, brandedNonEmptyStringSchema, NonEmptyStringSchema } from './effect';
const ProviderAdCountSchema = Schema.Number.pipe(Schema.nonNegative(), Schema.int());
const ProviderAdPositiveCountSchema = Schema.Number.pipe(Schema.positive(), Schema.int());

export const HouseholdAiProviderAdvertisementHeartbeatProofIdSchema = brandedNonEmptyStringSchema(
  'HouseholdAiProviderAdvertisementHeartbeatProofId'
);

export const HouseholdAiProviderAdvertisementStateSchema = withParser(
  Schema.Literal('eligible', 'stale', 'offline', 'revoked', 'unsupported')
);

export const HouseholdAiProviderCapabilitySchema = withParser(
  Schema.Literal('screen-ai-analysis', 'text-classification', 'parent-assistant-report')
);

export const HouseholdAiProviderResourceClassSchema = withParser(
  Schema.Literal('desktop-gpu', 'desktop-cpu', 'laptop-cpu', 'mobile-light')
);

export const HouseholdAiProviderHeartbeatBoundarySchema = withParser(
  Schema.Struct({
    physicalLanExecutionClaimed: Schema.Boolean,
    providerGossipRuntimeClaimed: Schema.Boolean,
    modelExecutionClaimed: Schema.Boolean,
    modelQualityClaimed: Schema.Boolean,
    policyAuthorityClaimed: Schema.Boolean,
    enforcementClaimed: Schema.Boolean,
    rawScreenshotTransferred: Schema.Boolean,
    remoteApiAiUsed: Schema.Boolean,
  }).pipe(
    Schema.filter(
      (boundary) =>
        Object.values(boundary).every((claim) => claim === false) ||
        'Expected household AI provider advertisement proof to keep physical LAN, gossip runtime, model, policy, enforcement, raw-transfer, and remote/API claims false'
    )
  )
);

const HouseholdAiProviderAdvertisementRowSchema = Schema.Struct({
  providerId: NonEmptyStringSchema,
  deviceId: NonEmptyStringSchema,
  providerTrustRef: NonEmptyStringSchema,
  advertisementId: NonEmptyStringSchema,
  heartbeatId: NonEmptyStringSchema,
  state: HouseholdAiProviderAdvertisementStateSchema,
  capabilities: Schema.Array(HouseholdAiProviderCapabilitySchema),
  resourceClass: HouseholdAiProviderResourceClassSchema,
  privacyMode: Schema.Literal('local-only'),
  heartbeatAgeMs: ProviderAdCountSchema,
  heartbeatTtlMs: ProviderAdPositiveCountSchema,
  rawPayloadAdvertised: Schema.Literal(false),
  remoteApiAdvertised: Schema.Literal(false),
  rejectionReason: Schema.NullOr(NonEmptyStringSchema),
});

const HouseholdAiProviderAdvertisementHeartbeatProofBaseSchema = Schema.Struct({
  proofId: HouseholdAiProviderAdvertisementHeartbeatProofIdSchema,
  generatedAt: NonEmptyStringSchema,
  requestedCapability: Schema.Literal('screen-ai-analysis'),
  advertisements: Schema.Array(HouseholdAiProviderAdvertisementRowSchema),
  validationSummary: Schema.Struct({
    eligibleProviderCount: ProviderAdCountSchema,
    staleProviderRejectedCount: ProviderAdCountSchema,
    offlineProviderRejectedCount: ProviderAdCountSchema,
    revokedProviderRejectedCount: ProviderAdCountSchema,
    unsupportedProviderRejectedCount: ProviderAdCountSchema,
    rawPayloadAdvertisementCount: ProviderAdCountSchema,
    remoteApiAdvertisementCount: ProviderAdCountSchema,
  }),
  claimBoundaries: HouseholdAiProviderHeartbeatBoundarySchema,
});

type HouseholdAiProviderAdvertisementHeartbeatProofCandidate = Infer<
  typeof HouseholdAiProviderAdvertisementHeartbeatProofBaseSchema
>;

export const HouseholdAiProviderAdvertisementHeartbeatProofSchema = withParser(
  HouseholdAiProviderAdvertisementHeartbeatProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        householdAiProviderAdvertisementHeartbeatProofIsReady(proof) ||
        'Expected household AI provider advertisement proof to accept only fresh trusted local providers with screen capability and reject stale/offline/revoked/unsupported rows'
    )
  )
);

export type HouseholdAiProviderAdvertisementHeartbeatProof = Infer<
  typeof HouseholdAiProviderAdvertisementHeartbeatProofSchema
>;

const generatedAt = '2026-06-08T04:54:00.000Z';

export const HouseholdAiProviderAdvertisementHeartbeatProof =
  HouseholdAiProviderAdvertisementHeartbeatProofSchema.parse({
    proofId: 'household-ai-provider-advertisement-heartbeat-proof',
    generatedAt,
    requestedCapability: 'screen-ai-analysis',
    advertisements: [
      {
        providerId: 'household-provider-parent-desktop-1',
        deviceId: 'parent-desktop-device-1',
        providerTrustRef: 'trusted-household-provider-parent-desktop-1',
        advertisementId: 'provider-ad-parent-desktop-1',
        heartbeatId: 'provider-heartbeat-parent-desktop-1',
        state: 'eligible',
        capabilities: ['screen-ai-analysis', 'text-classification'],
        resourceClass: 'desktop-gpu',
        privacyMode: 'local-only',
        heartbeatAgeMs: 2_000,
        heartbeatTtlMs: 30_000,
        rawPayloadAdvertised: false,
        remoteApiAdvertised: false,
        rejectionReason: null,
      },
      {
        providerId: 'household-provider-stale-laptop-1',
        deviceId: 'laptop-device-1',
        providerTrustRef: 'trusted-household-provider-laptop-1',
        advertisementId: 'provider-ad-stale-laptop-1',
        heartbeatId: 'provider-heartbeat-stale-laptop-1',
        state: 'stale',
        capabilities: ['screen-ai-analysis'],
        resourceClass: 'laptop-cpu',
        privacyMode: 'local-only',
        heartbeatAgeMs: 61_000,
        heartbeatTtlMs: 30_000,
        rawPayloadAdvertised: false,
        remoteApiAdvertised: false,
        rejectionReason: 'heartbeat-expired',
      },
      {
        providerId: 'household-provider-offline-desktop-1',
        deviceId: 'offline-desktop-device-1',
        providerTrustRef: 'trusted-household-provider-offline-desktop-1',
        advertisementId: 'provider-ad-offline-desktop-1',
        heartbeatId: 'provider-heartbeat-offline-desktop-1',
        state: 'offline',
        capabilities: ['screen-ai-analysis'],
        resourceClass: 'desktop-cpu',
        privacyMode: 'local-only',
        heartbeatAgeMs: 0,
        heartbeatTtlMs: 30_000,
        rawPayloadAdvertised: false,
        remoteApiAdvertised: false,
        rejectionReason: 'provider-offline',
      },
      {
        providerId: 'household-provider-revoked-mobile-1',
        deviceId: 'mobile-device-1',
        providerTrustRef: 'revoked-household-provider-mobile-1',
        advertisementId: 'provider-ad-revoked-mobile-1',
        heartbeatId: 'provider-heartbeat-revoked-mobile-1',
        state: 'revoked',
        capabilities: ['screen-ai-analysis'],
        resourceClass: 'mobile-light',
        privacyMode: 'local-only',
        heartbeatAgeMs: 1_000,
        heartbeatTtlMs: 30_000,
        rawPayloadAdvertised: false,
        remoteApiAdvertised: false,
        rejectionReason: 'provider-revoked',
      },
      {
        providerId: 'household-provider-text-only-1',
        deviceId: 'text-only-device-1',
        providerTrustRef: 'trusted-household-provider-text-only-1',
        advertisementId: 'provider-ad-text-only-1',
        heartbeatId: 'provider-heartbeat-text-only-1',
        state: 'unsupported',
        capabilities: ['text-classification'],
        resourceClass: 'desktop-cpu',
        privacyMode: 'local-only',
        heartbeatAgeMs: 1_000,
        heartbeatTtlMs: 30_000,
        rawPayloadAdvertised: false,
        remoteApiAdvertised: false,
        rejectionReason: 'missing-screen-ai-analysis-capability',
      },
    ],
    validationSummary: {
      eligibleProviderCount: 1,
      staleProviderRejectedCount: 1,
      offlineProviderRejectedCount: 1,
      revokedProviderRejectedCount: 1,
      unsupportedProviderRejectedCount: 1,
      rawPayloadAdvertisementCount: 0,
      remoteApiAdvertisementCount: 0,
    },
    claimBoundaries: {
      physicalLanExecutionClaimed: false,
      providerGossipRuntimeClaimed: false,
      modelExecutionClaimed: false,
      modelQualityClaimed: false,
      policyAuthorityClaimed: false,
      enforcementClaimed: false,
      rawScreenshotTransferred: false,
      remoteApiAiUsed: false,
    },
  });

function householdAiProviderAdvertisementHeartbeatProofIsReady(
  proof: HouseholdAiProviderAdvertisementHeartbeatProofCandidate
): boolean {
  return (
    summaryMatchesRows(proof) &&
    onlyEligibleRowsCanServeRequestedCapability(proof) &&
    rejectedRowsNameReasons(proof) &&
    noRawOrRemoteAdvertisement(proof)
  );
}

function summaryMatchesRows(proof: HouseholdAiProviderAdvertisementHeartbeatProofCandidate): boolean {
  return (
    proof.validationSummary.eligibleProviderCount === rowsByState(proof, 'eligible') &&
    proof.validationSummary.staleProviderRejectedCount === rowsByState(proof, 'stale') &&
    proof.validationSummary.offlineProviderRejectedCount === rowsByState(proof, 'offline') &&
    proof.validationSummary.revokedProviderRejectedCount === rowsByState(proof, 'revoked') &&
    proof.validationSummary.unsupportedProviderRejectedCount === rowsByState(proof, 'unsupported')
  );
}

function rowsByState(
  proof: HouseholdAiProviderAdvertisementHeartbeatProofCandidate,
  state: Infer<typeof HouseholdAiProviderAdvertisementStateSchema>
) {
  return proof.advertisements.filter((row) => row.state === state).length;
}

function onlyEligibleRowsCanServeRequestedCapability(
  proof: HouseholdAiProviderAdvertisementHeartbeatProofCandidate
): boolean {
  return proof.advertisements.every(
    (row) =>
      row.state !== 'eligible' ||
      (row.capabilities.includes(proof.requestedCapability) && row.heartbeatAgeMs <= row.heartbeatTtlMs)
  );
}

function rejectedRowsNameReasons(proof: HouseholdAiProviderAdvertisementHeartbeatProofCandidate): boolean {
  return proof.advertisements.every((row) => row.state === 'eligible' || row.rejectionReason !== null);
}

function noRawOrRemoteAdvertisement(proof: HouseholdAiProviderAdvertisementHeartbeatProofCandidate): boolean {
  return (
    proof.validationSummary.rawPayloadAdvertisementCount === 0 &&
    proof.validationSummary.remoteApiAdvertisementCount === 0 &&
    proof.advertisements.every((row) => row.rawPayloadAdvertised === false && row.remoteApiAdvertised === false)
  );
}

export const decodeHouseholdAiProviderAdvertisementHeartbeatProof = Schema.decodeUnknownSync(
  HouseholdAiProviderAdvertisementHeartbeatProofSchema
);
