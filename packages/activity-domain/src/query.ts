import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ActivityEventKindSchema, ActivityObserverSchema, ActivitySubjectKindSchema } from './kinds';
import {
  ActivityEventIdSchema,
  ActivitySubjectIdSchema,
  ActivitySubjectNameSchema,
  ActivityTimestampSchema,
} from './primitives';

export const ActivityQuerySchemaVersion = 1;

export const ActivityIngestStatusSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(ActivityQuerySchemaVersion),
    databaseReady: Schema.Boolean,
    eventsIngested: Schema.Number,
    eventsStored: Schema.Number,
    duplicateEvents: Schema.Number,
    lastEventId: Schema.Union(ActivityEventIdSchema, Schema.Null),
  })
);

export const ActivityRecentQuerySchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(ActivityQuerySchemaVersion),
    limit: Schema.Number,
  })
);

export const ActivityRecentSummarySchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(ActivityQuerySchemaVersion),
    limit: Schema.Number,
    returned: Schema.Number,
    firstObservedAt: Schema.Union(ActivityTimestampSchema, Schema.Null),
    lastObservedAt: Schema.Union(ActivityTimestampSchema, Schema.Null),
    lastEventId: Schema.Union(ActivityEventIdSchema, Schema.Null),
    mostRecentKind: Schema.Union(ActivityEventKindSchema, Schema.Null),
    mostRecentObserver: Schema.Union(ActivityObserverSchema, Schema.Null),
    mostRecentSubjectKind: Schema.Union(ActivitySubjectKindSchema, Schema.Null),
    mostRecentSubjectId: Schema.Union(ActivitySubjectIdSchema, Schema.Null),
    mostRecentSubjectName: Schema.Union(ActivitySubjectNameSchema, Schema.Null),
  })
);

export type ActivityIngestStatus = Infer<typeof ActivityIngestStatusSchema>;
export type ActivityRecentQuery = Infer<typeof ActivityRecentQuerySchema>;
export type ActivityRecentSummary = Infer<typeof ActivityRecentSummarySchema>;
