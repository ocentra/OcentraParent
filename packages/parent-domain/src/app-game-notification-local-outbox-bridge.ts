import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  AppGameNotificationDeliveryClaimState,
  AppGameNotificationIntentKind,
  AppGameNotificationIntentSchema,
  AppGameNotificationIntentStatus,
  AppGameNotificationReasonCode,
  AppGameNotificationReferenceSchema,
  type AppGameNotificationIntent,
} from './app-game-notification-intent';
import {
  NotificationLocalOutboxRecordSchema,
  type NotificationLocalOutboxRecord,
} from './notification-local-outbox-adapter-proof';
import {
  ParentContractSchemaVersion,
  ParentContractSchemaVersionSchema,
  ParentTimestampSchema,
  type ParentTimestamp,
} from './reference-primitives';
import {
  FamilyReferenceSchema,
  ParentActionReferenceSchema,
  type FamilyReference,
  type ParentActionReference,
} from './references';
import {
  V3NotificationProviderChannelSchema,
  V3NotificationRuleReasonCodeSchema,
  type V3NotificationProviderChannel,
  type V3NotificationRuleReasonCode,
} from './v3-notification-rule-provider-retry-contract';

const NonEmptyAppGameNotificationBridgeText = Schema.String.pipe(Schema.minLength(1));

export const AppGameNotificationLocalOutboxBridgeSchemaVersionSchema = withParser(
  Schema.Literal('app-game-notification-local-outbox-bridge')
);
export const AppGameNotificationLocalOutboxBridgeReadModelIdSchema = NonEmptyAppGameNotificationBridgeText.pipe(
  Schema.brand('AppGameNotificationLocalOutboxBridgeReadModelId')
);
export const AppGameNotificationLocalOutboxBridgeReferenceSchema = NonEmptyAppGameNotificationBridgeText.pipe(
  Schema.brand('AppGameNotificationLocalOutboxBridgeReference')
);
export const AppGameNotificationLocalOutboxBridgeNonClaimSchema = withParser(
  Schema.Literal(
    'no-provider-delivery',
    'no-provider-receipt-ingestion',
    'no-cloud-routing',
    'no-parent-notification-ui',
    'no-adapter-dispatch',
    'no-durable-service-persistence',
    'no-child-device-delivery',
    'no-broad-app-blocking',
    'no-platform-support',
    'no-raw-child-evidence'
  )
);
export const AppGameNotificationLocalOutboxBridgeBlockReasonSchema = withParser(
  Schema.Literal(
    'manual-required-no-local-outbox',
    'capability-unavailable-no-local-outbox',
    'intent-only-no-local-outbox'
  )
);

export const AppGameNotificationLocalOutboxBridgeNonClaims = [
  'no-provider-delivery',
  'no-provider-receipt-ingestion',
  'no-cloud-routing',
  'no-parent-notification-ui',
  'no-adapter-dispatch',
  'no-durable-service-persistence',
  'no-child-device-delivery',
  'no-broad-app-blocking',
  'no-platform-support',
  'no-raw-child-evidence',
] as const;

const AppGameNotificationLocalOutboxBridgeRecordLinkSchema = withParser(
  Schema.Struct({
    notificationIntentRef: AppGameNotificationReferenceSchema,
    localOutboxRecordRef: AppGameNotificationReferenceSchema,
    outboxEntryRef: AppGameNotificationLocalOutboxBridgeReferenceSchema,
    intentKind: Schema.Literal(
      AppGameNotificationIntentKind.TimeLimitReached,
      AppGameNotificationIntentKind.ApprovalRequested,
      AppGameNotificationIntentKind.SuspiciousUnknown
    ),
    notificationReasonCode: Schema.Literal(
      AppGameNotificationReasonCode.TimeLimit,
      AppGameNotificationReasonCode.ApprovalRequest,
      AppGameNotificationReasonCode.SuspiciousUnknown
    ),
    outboxReasonCode: V3NotificationRuleReasonCodeSchema,
    providerChannel: V3NotificationProviderChannelSchema,
  })
);

const AppGameNotificationLocalOutboxBridgeBlockedIntentSchema = withParser(
  Schema.Struct({
    notificationIntentRef: AppGameNotificationReferenceSchema,
    intentKind: Schema.Literal(...Object.values(AppGameNotificationIntentKind)),
    intentStatus: Schema.Literal(
      AppGameNotificationIntentStatus.IntentOnly,
      AppGameNotificationIntentStatus.ManualRequired,
      AppGameNotificationIntentStatus.Unavailable
    ),
    deliveryClaimState: Schema.Literal(
      AppGameNotificationDeliveryClaimState.NotClaimed,
      AppGameNotificationDeliveryClaimState.ManualRequired
    ),
    blockReason: AppGameNotificationLocalOutboxBridgeBlockReasonSchema,
    manualProofRequirements: Schema.Array(AppGameNotificationReferenceSchema),
  })
);

const AppGameNotificationLocalOutboxBridgeProofBaseSchema = Schema.Struct({
  schemaVersion: AppGameNotificationLocalOutboxBridgeSchemaVersionSchema,
  contractVersion: ParentContractSchemaVersionSchema,
  readModelId: AppGameNotificationLocalOutboxBridgeReadModelIdSchema,
  generatedAt: ParentTimestampSchema,
  sourceIntentReadModelRef: AppGameNotificationLocalOutboxBridgeReferenceSchema,
  localOutboxReadModelRef: AppGameNotificationLocalOutboxBridgeReferenceSchema,
  outboxRootRef: AppGameNotificationLocalOutboxBridgeReferenceSchema,
  bridgedIntentRefs: Schema.Array(AppGameNotificationLocalOutboxBridgeRecordLinkSchema),
  blockedIntentRefs: Schema.Array(AppGameNotificationLocalOutboxBridgeBlockedIntentSchema),
  records: Schema.Array(NotificationLocalOutboxRecordSchema),
  nonClaims: Schema.Array(AppGameNotificationLocalOutboxBridgeNonClaimSchema),
  providerDeliveryRuntimeClaimed: Schema.Boolean,
  providerReceiptIngestionClaimed: Schema.Boolean,
  cloudRoutingClaimed: Schema.Boolean,
  parentNotificationUiClaimed: Schema.Boolean,
  adapterDispatchClaimed: Schema.Boolean,
  durableServicePersistenceClaimed: Schema.Boolean,
  childDeviceDeliveryClaimed: Schema.Boolean,
  broadAppBlockingClaimed: Schema.Boolean,
  platformSupportClaimed: Schema.Boolean,
});

type AppGameNotificationLocalOutboxBridgeProofCandidate = Infer<
  typeof AppGameNotificationLocalOutboxBridgeProofBaseSchema
>;

export const AppGameNotificationLocalOutboxBridgeProofSchema = withParser(
  AppGameNotificationLocalOutboxBridgeProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        appGameNotificationLocalOutboxBridgeProofIsHonest(proof) ||
        'Expected app/game notification local-outbox bridge proof to map only local-outbox-eligible intents into local records, block manual/unavailable intents, and keep provider, UI, service persistence, adapter, broad-blocking, and platform claims false'
    )
  )
);

export type AppGameNotificationLocalOutboxBridgeNonClaim = Infer<
  typeof AppGameNotificationLocalOutboxBridgeNonClaimSchema
>;
export type AppGameNotificationLocalOutboxBridgeBlockReason = Infer<
  typeof AppGameNotificationLocalOutboxBridgeBlockReasonSchema
>;
export type AppGameNotificationLocalOutboxBridgeRecordLink = Infer<
  typeof AppGameNotificationLocalOutboxBridgeRecordLinkSchema
>;
export type AppGameNotificationLocalOutboxBridgeBlockedIntent = Infer<
  typeof AppGameNotificationLocalOutboxBridgeBlockedIntentSchema
>;
export type AppGameNotificationLocalOutboxBridgeProof = Infer<typeof AppGameNotificationLocalOutboxBridgeProofSchema>;

export type AppGameNotificationLocalOutboxBridgeInput = {
  readonly generatedAt: ParentTimestamp;
  readonly family: FamilyReference;
  readonly parentAction: ParentActionReference;
  readonly sourceIntentReadModelRef: string;
  readonly localOutboxReadModelRef: string;
  readonly outboxRootRef: string;
  readonly outboxFileRef: string;
  readonly localDataPathRef: string;
  readonly intents: ReadonlyArray<unknown>;
};

const BridgeClaimFlags = [
  'providerDeliveryRuntimeClaimed',
  'providerReceiptIngestionClaimed',
  'cloudRoutingClaimed',
  'parentNotificationUiClaimed',
  'adapterDispatchClaimed',
  'durableServicePersistenceClaimed',
  'childDeviceDeliveryClaimed',
  'broadAppBlockingClaimed',
  'platformSupportClaimed',
] as const;

export function buildAppGameNotificationLocalOutboxBridgeProof(
  input: AppGameNotificationLocalOutboxBridgeInput
): AppGameNotificationLocalOutboxBridgeProof {
  const parsedIntents = input.intents.map((intent) => AppGameNotificationIntentSchema.parse(intent));
  const bridgedIntents = parsedIntents.filter(appGameNotificationIntentCanBridgeToLocalOutbox);
  const blockedIntents = parsedIntents.filter((intent) => !appGameNotificationIntentCanBridgeToLocalOutbox(intent));

  return AppGameNotificationLocalOutboxBridgeProofSchema.parse({
    schemaVersion: 'app-game-notification-local-outbox-bridge',
    contractVersion: ParentContractSchemaVersion.V0_6,
    readModelId: 'app-game-notification-local-outbox-bridge',
    generatedAt: input.generatedAt,
    sourceIntentReadModelRef: input.sourceIntentReadModelRef,
    localOutboxReadModelRef: input.localOutboxReadModelRef,
    outboxRootRef: input.outboxRootRef,
    bridgedIntentRefs: bridgedIntents.map((intent) => bridgeLinkForIntent(intent)),
    blockedIntentRefs: blockedIntents.map((intent) => blockedIntentForIntent(intent)),
    records: bridgedIntents.map((intent) => outboxRecordForIntent(input, intent)),
    nonClaims: AppGameNotificationLocalOutboxBridgeNonClaims,
    providerDeliveryRuntimeClaimed: false,
    providerReceiptIngestionClaimed: false,
    cloudRoutingClaimed: false,
    parentNotificationUiClaimed: false,
    adapterDispatchClaimed: false,
    durableServicePersistenceClaimed: false,
    childDeviceDeliveryClaimed: false,
    broadAppBlockingClaimed: false,
    platformSupportClaimed: false,
  });
}

export function summarizeAppGameNotificationLocalOutboxBridgeReasons(
  proof: AppGameNotificationLocalOutboxBridgeProof
): Record<V3NotificationRuleReasonCode, number> {
  return countBy(
    proof.bridgedIntentRefs.map((link) => link.outboxReasonCode),
    ['policy-violation', 'parent-request', 'suspicious-unknown', 'device-offline', 'sync-failure', 'provider-failure']
  );
}

export function summarizeAppGameNotificationLocalOutboxBridgeChannels(
  proof: AppGameNotificationLocalOutboxBridgeProof
): Record<V3NotificationProviderChannel, number> {
  return countBy(
    proof.records.map((record) => record.envelope.providerChannel),
    ['push', 'email', 'sms', 'whatsapp', 'in-app']
  );
}

function appGameNotificationLocalOutboxBridgeProofIsHonest(
  proof: AppGameNotificationLocalOutboxBridgeProofCandidate
): boolean {
  return (
    proof.records.length > 0 &&
    proof.bridgedIntentRefs.length === proof.records.length &&
    AppGameNotificationLocalOutboxBridgeNonClaims.every((claim) => proof.nonClaims.includes(claim)) &&
    BridgeClaimFlags.every((flag) => proof[flag] === false) &&
    appGameNotificationLocalOutboxBridgeLinksMatchRecords(proof) &&
    proof.blockedIntentRefs.every(
      (blocked) =>
        blocked.intentStatus === AppGameNotificationIntentStatus.IntentOnly ||
        blocked.manualProofRequirements.length > 0
    )
  );
}

function appGameNotificationLocalOutboxBridgeLinksMatchRecords(
  proof: AppGameNotificationLocalOutboxBridgeProofCandidate
): boolean {
  const recordIds = new Set(proof.records.map((record) => String(record.entryId)));
  return proof.bridgedIntentRefs.every(
    (link) =>
      String(link.localOutboxRecordRef) === String(link.outboxEntryRef) &&
      recordIds.has(String(link.outboxEntryRef)) &&
      outboxReasonMatchesNotificationReason(link)
  );
}

function outboxReasonMatchesNotificationReason(link: AppGameNotificationLocalOutboxBridgeRecordLink): boolean {
  return outboxReasonForNotificationReason(link.notificationReasonCode) === link.outboxReasonCode;
}

function appGameNotificationIntentCanBridgeToLocalOutbox(intent: AppGameNotificationIntent): boolean {
  return (
    intent.intentStatus === AppGameNotificationIntentStatus.LocalOutboxEligible &&
    intent.deliveryClaimState === AppGameNotificationDeliveryClaimState.LocalOutboxOnly &&
    intent.localOutboxRecordRef !== null &&
    outboxReasonForNotificationReason(intent.notificationReasonCode) !== undefined
  );
}

function bridgeLinkForIntent(intent: AppGameNotificationIntent): AppGameNotificationLocalOutboxBridgeRecordLink {
  if (intent.localOutboxRecordRef === null) {
    throw new Error('Expected local-outbox-eligible notification intent to include a local outbox record ref');
  }
  const outboxReasonCode = outboxReasonForNotificationReason(intent.notificationReasonCode);
  if (outboxReasonCode === undefined) {
    throw new Error(
      `Unsupported app/game notification reason for local outbox bridge: ${intent.notificationReasonCode}`
    );
  }
  return AppGameNotificationLocalOutboxBridgeRecordLinkSchema.parse({
    notificationIntentRef: intent.notificationIntentId,
    localOutboxRecordRef: intent.localOutboxRecordRef,
    outboxEntryRef: intent.localOutboxRecordRef,
    intentKind: intent.intentKind,
    notificationReasonCode: intent.notificationReasonCode,
    outboxReasonCode,
    providerChannel: intent.providerChannelPreference,
  });
}

function outboxReasonForNotificationReason(
  notificationReasonCode: AppGameNotificationIntent['notificationReasonCode']
): V3NotificationRuleReasonCode | undefined {
  switch (notificationReasonCode) {
    case AppGameNotificationReasonCode.TimeLimit:
      return 'policy-violation';
    case AppGameNotificationReasonCode.ApprovalRequest:
      return 'parent-request';
    case AppGameNotificationReasonCode.SuspiciousUnknown:
      return 'suspicious-unknown';
    default:
      return undefined;
  }
}

function blockedIntentForIntent(intent: AppGameNotificationIntent): AppGameNotificationLocalOutboxBridgeBlockedIntent {
  return AppGameNotificationLocalOutboxBridgeBlockedIntentSchema.parse({
    notificationIntentRef: intent.notificationIntentId,
    intentKind: intent.intentKind,
    intentStatus: intent.intentStatus,
    deliveryClaimState: intent.deliveryClaimState,
    blockReason: blockReasonForIntent(intent),
    manualProofRequirements: intent.manualProofRequirements,
  });
}

function blockReasonForIntent(intent: AppGameNotificationIntent): AppGameNotificationLocalOutboxBridgeBlockReason {
  if (intent.intentStatus === AppGameNotificationIntentStatus.ManualRequired) {
    return 'manual-required-no-local-outbox';
  }
  if (intent.intentStatus === AppGameNotificationIntentStatus.Unavailable) {
    return 'capability-unavailable-no-local-outbox';
  }
  return 'intent-only-no-local-outbox';
}

function outboxRecordForIntent(
  input: AppGameNotificationLocalOutboxBridgeInput,
  intent: AppGameNotificationIntent
): NotificationLocalOutboxRecord {
  const link = bridgeLinkForIntent(intent);
  return NotificationLocalOutboxRecordSchema.parse({
    entryId: link.outboxEntryRef,
    state: 'queued-local',
    envelope: {
      alertRef: intent.notificationIntentId,
      family: FamilyReferenceSchema.parse(input.family),
      device: intent.device,
      parentAction: ParentActionReferenceSchema.parse(input.parentAction),
      severity: intent.priority,
      reasonCode: link.outboxReasonCode,
      providerChannel: intent.providerChannelPreference,
      evidenceRefs: intent.evidenceReferences,
      policyRefs: intent.policyRefs,
      auditRefs: intent.auditRefs,
      payloadTemplateRef: `app-game-notification-template-${intent.parentTitleToken}`,
      providerPayloadPreview: `app-game alert ref ${intent.notificationIntentId} reason ${link.outboxReasonCode} evidence and policy refs only`,
      sensitiveDetailMinimized: true,
      rawChildEvidenceIncluded: false,
      rawUrlOrTitleIncluded: false,
      rawMessageTextIncluded: false,
      screenshotOrReportIncluded: false,
    },
    outboxFileRef: input.outboxFileRef,
    localDataPathRef: input.localDataPathRef,
    deliveryClaimState: 'local-outbox-only',
    visibleAfterAt: null,
    retryAttemptCount: 0,
    quietHoursRef: null,
    retryPolicyRef: null,
    deadLetterRef: null,
    providerReceiptRef: null,
    manualProofRequirements: [],
    manualActionRequired: false,
    providerDeliveryAttempted: false,
    providerDeliveryObserved: false,
    providerReceiptIngested: false,
    providerCredentialsStored: false,
    cloudRoutingClaimed: false,
    parentNotificationUiClaimed: false,
    sensitiveProviderMetadataStored: false,
  });
}

function countBy<const T extends string>(values: ReadonlyArray<T>, keys: readonly T[]): Record<T, number> {
  return Object.fromEntries(keys.map((key) => [key, values.filter((value) => value === key).length])) as Record<
    T,
    number
  >;
}

export const decodeAppGameNotificationLocalOutboxBridgeProof = Schema.decodeUnknownSync(
  AppGameNotificationLocalOutboxBridgeProofSchema
);
