/* generated from crates/schema/src/eventing_contracts_ts.rs */

import { type Infer, NonEmptyStringSchema, Schema, brandedNonEmptyStringSchema, withParser } from './effect';

const EventingTaxonomyTextSchema = NonEmptyStringSchema.pipe(
  Schema.filter((value) => isEventingTaxonomyText(value) || 'Expected event type or namespace taxonomy text')
);
const EventingSchemaVersionNumberSchema = Schema.Number.pipe(Schema.int(), Schema.between(1, 65535));

export const EventingEventTypeSchema = withParser(EventingTaxonomyTextSchema.pipe(Schema.brand('EventingEventType')));
export const EventingEventNamespaceSchema = withParser(
  EventingTaxonomyTextSchema.pipe(Schema.brand('EventingEventNamespace'))
);
export const EventingEventIdSchema = brandedNonEmptyStringSchema('EventingEventId');
export const EventingCorrelationIdSchema = brandedNonEmptyStringSchema('EventingCorrelationId');
export const EventingCausationIdSchema = brandedNonEmptyStringSchema('EventingCausationId');
export const EventingRequestIdSchema = brandedNonEmptyStringSchema('EventingRequestId');
export const EventingJournalHashSchema = brandedNonEmptyStringSchema('EventingJournalHash');
export const EventingAggregateKeySchema = brandedNonEmptyStringSchema('EventingAggregateKey');
export const EventingIdempotencyKeySchema = brandedNonEmptyStringSchema('EventingIdempotencyKey');
export const EventingSubscriberIdSchema = brandedNonEmptyStringSchema('EventingSubscriberId');
export const EventingTargetHandlerSchema = brandedNonEmptyStringSchema('EventingTargetHandler');
export const EventingEventCustodySchema = brandedNonEmptyStringSchema('EventingEventCustody');
export const EventingRuntimeRoleSchema = brandedNonEmptyStringSchema('EventingRuntimeRole');
export const EventingSourceServiceSchema = brandedNonEmptyStringSchema('EventingSourceService');
export const EventingSourceComponentSchema = brandedNonEmptyStringSchema('EventingSourceComponent');
export const EventingRuntimeInstanceIdSchema = brandedNonEmptyStringSchema('EventingRuntimeInstanceId');
export const EventingRecordedAtSchema = brandedNonEmptyStringSchema('EventingRecordedAt');
export const EventingRustTypeSchema = brandedNonEmptyStringSchema('EventingRustType');
export const EventingSchemaVersionSchema = EventingSchemaVersionNumberSchema.pipe(
  Schema.brand('EventingSchemaVersion')
);

export const EventingEventPrioritySchema = withParser(Schema.Literal('low', 'normal', 'high', 'critical'));

export const EventingTopologyStatusSchema = withParser(
  Schema.Literal('covered', 'no-publisher', 'no-subscriber', 'accepted-one-sided')
);

export const EventingDeliveryRouteKindSchema = withParser(
  Schema.Literal('local-in-process', 'local-service', 'external-transport', 'external-relay')
);

export const EventingRequestCompletionOutcomeSchema = withParser(Schema.Literal('completed', 'duplicate', 'late'));

export const EventingEventContractSchema = withParser(
  Schema.Struct({
    eventType: EventingEventTypeSchema,
    schemaVersion: EventingSchemaVersionSchema,
  })
);

export const EventingEventSourceSchema = withParser(
  Schema.Struct({
    custody: EventingEventCustodySchema,
    role: EventingRuntimeRoleSchema,
    service: EventingSourceServiceSchema,
    component: EventingSourceComponentSchema,
    instanceId: EventingRuntimeInstanceIdSchema,
  })
);

export const EventingEnvelopeMetadataSchema = withParser(
  Schema.Struct({
    eventId: EventingEventIdSchema,
    correlationId: EventingCorrelationIdSchema,
    causationId: Schema.Union(EventingCausationIdSchema, Schema.Null),
    aggregateKey: EventingAggregateKeySchema,
    idempotencyKey: EventingIdempotencyKeySchema,
    source: EventingEventSourceSchema,
    observedAt: EventingRecordedAtSchema,
    targetHandler: Schema.Union(EventingTargetHandlerSchema, Schema.Null),
    priority: EventingEventPrioritySchema,
    deadline: Schema.Union(EventingRecordedAtSchema, Schema.Null),
  })
);

export const EventingStoredEnvelopeHeaderSchema = withParser(
  Schema.Struct({
    contract: EventingEventContractSchema,
    metadata: EventingEnvelopeMetadataSchema,
    journalHash: Schema.Union(EventingJournalHashSchema, Schema.Null),
  })
);

export const EventingTopologySubscriberTargetSchema = withParser(
  Schema.Struct({
    subscriberId: EventingSubscriberIdSchema,
    targetHandler: EventingTargetHandlerSchema,
  })
);

export const EventingTopologyEntrySchema = withParser(
  Schema.Struct({
    contract: EventingEventContractSchema,
    rustType: EventingRustTypeSchema,
    publishers: Schema.Array(EventingSourceComponentSchema),
    subscribers: Schema.Array(EventingTopologySubscriberTargetSchema),
    families: Schema.Array(EventingEventNamespaceSchema),
    status: EventingTopologyStatusSchema,
  })
);

export const EventingTopologyManifestSchema = withParser(
  Schema.Struct({
    entries: Schema.Array(EventingTopologyEntrySchema),
  })
);

export const EventingRequestCompletionReportSchema = withParser(
  Schema.Struct({
    requestId: EventingRequestIdSchema,
    outcome: EventingRequestCompletionOutcomeSchema,
  })
);

export type EventingEventPriority = Infer<typeof EventingEventPrioritySchema>;
export type EventingTopologyStatus = Infer<typeof EventingTopologyStatusSchema>;
export type EventingDeliveryRouteKind = Infer<typeof EventingDeliveryRouteKindSchema>;
export type EventingRequestCompletionOutcome = Infer<typeof EventingRequestCompletionOutcomeSchema>;
export type EventingEventType = typeof EventingEventTypeSchema.Type;
export type EventingEventNamespace = typeof EventingEventNamespaceSchema.Type;
export type EventingEventId = typeof EventingEventIdSchema.Type;
export type EventingCorrelationId = typeof EventingCorrelationIdSchema.Type;
export type EventingCausationId = typeof EventingCausationIdSchema.Type;
export type EventingRequestId = typeof EventingRequestIdSchema.Type;
export type EventingJournalHash = typeof EventingJournalHashSchema.Type;
export type EventingAggregateKey = typeof EventingAggregateKeySchema.Type;
export type EventingIdempotencyKey = typeof EventingIdempotencyKeySchema.Type;
export type EventingSubscriberId = typeof EventingSubscriberIdSchema.Type;
export type EventingTargetHandler = typeof EventingTargetHandlerSchema.Type;
export type EventingEventCustody = typeof EventingEventCustodySchema.Type;
export type EventingRuntimeRole = typeof EventingRuntimeRoleSchema.Type;
export type EventingSourceService = typeof EventingSourceServiceSchema.Type;
export type EventingSourceComponent = typeof EventingSourceComponentSchema.Type;
export type EventingRuntimeInstanceId = typeof EventingRuntimeInstanceIdSchema.Type;
export type EventingRecordedAt = typeof EventingRecordedAtSchema.Type;
export type EventingRustType = typeof EventingRustTypeSchema.Type;
export type EventingSchemaVersion = typeof EventingSchemaVersionSchema.Type;
export type EventingEventContract = Infer<typeof EventingEventContractSchema>;
export type EventingEventSource = Infer<typeof EventingEventSourceSchema>;
export type EventingEnvelopeMetadata = Infer<typeof EventingEnvelopeMetadataSchema>;
export type EventingStoredEnvelopeHeader = Infer<typeof EventingStoredEnvelopeHeaderSchema>;
export type EventingTopologySubscriberTarget = Infer<typeof EventingTopologySubscriberTargetSchema>;
export type EventingTopologyEntry = Infer<typeof EventingTopologyEntrySchema>;
export type EventingTopologyManifest = Infer<typeof EventingTopologyManifestSchema>;
export type EventingRequestCompletionReport = Infer<typeof EventingRequestCompletionReportSchema>;

function isEventingTaxonomyText(value: string): boolean {
  let previousWasSeparator = false;
  for (let index = 0; index < value.length; index += 1) {
    const character = value.charAt(index);
    const isSeparator = character === '.' || character === '/';
    const isValid = /[A-Za-z0-9_-]/u.test(character) || isSeparator;
    if (!isValid || (isSeparator && (index === 0 || previousWasSeparator))) {
      return false;
    }
    previousWasSeparator = isSeparator;
  }
  return !previousWasSeparator;
}
