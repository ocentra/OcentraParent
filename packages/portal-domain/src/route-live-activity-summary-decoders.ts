import { GeneratedPortalAgentProtocolRuntime } from './generated-portal-contracts';
import type { PortalLiveActivityState } from './live-activity-state';

const RECENT_SUMMARY_FIELDS = [
  'schemaVersion',
  'limit',
  'returned',
  'firstObservedAt',
  'lastObservedAt',
  'lastEventId',
  'mostRecentKind',
  'mostRecentObserver',
  'mostRecentSubjectKind',
  'mostRecentSubjectId',
  'mostRecentSubjectName',
] as const;

const INGEST_STATUS_FIELDS = [
  'schemaVersion',
  'databaseReady',
  'eventsIngested',
  'eventsStored',
  'duplicateEvents',
  'lastEventId',
] as const;

export function decodeActivityRecentSummary(value: unknown): PortalLiveActivityState['recentSummary'] {
  if (!isExactRecord(value, RECENT_SUMMARY_FIELDS)) return null;
  try {
    const limit = readNonNegativeInteger(value, 'limit');
    const returned = readNonNegativeInteger(value, 'returned');
    if (returned > limit) return null;
    return {
      schemaVersion: readProtocolSchemaVersion(value),
      limit,
      returned,
      firstObservedAt: readOptionalNullableText(value, 'firstObservedAt'),
      lastObservedAt: readOptionalNullableText(value, 'lastObservedAt'),
      lastEventId: readOptionalNullableText(value, 'lastEventId'),
      mostRecentKind: readOptionalNullableText(value, 'mostRecentKind'),
      mostRecentObserver: readOptionalNullableText(value, 'mostRecentObserver'),
      mostRecentSubjectKind: readOptionalNullableText(value, 'mostRecentSubjectKind'),
      mostRecentSubjectId: readOptionalNullableText(value, 'mostRecentSubjectId'),
      mostRecentSubjectName: readOptionalNullableText(value, 'mostRecentSubjectName'),
    };
  } catch {
    return null;
  }
}

export function decodeActivityIngestStatus(value: unknown): PortalLiveActivityState['ingestStatus'] {
  if (!isExactRecord(value, INGEST_STATUS_FIELDS)) return null;
  try {
    return {
      schemaVersion: readProtocolSchemaVersion(value),
      databaseReady: readBoolean(value, 'databaseReady'),
      eventsIngested: readNonNegativeInteger(value, 'eventsIngested'),
      eventsStored: readNonNegativeInteger(value, 'eventsStored'),
      duplicateEvents: readNonNegativeInteger(value, 'duplicateEvents'),
      lastEventId: readOptionalNullableText(value, 'lastEventId'),
    };
  } catch {
    return null;
  }
}

function isExactRecord<const TField extends string>(
  value: unknown,
  fields: readonly TField[]
): value is Readonly<Record<TField, unknown>> {
  return isRecord(value) && Object.keys(value).every((field) => fields.some((allowedField) => allowedField === field));
}

function isRecord(value: unknown): value is Readonly<Record<string, unknown>> {
  return typeof value === 'object' && value !== null && !Array.isArray(value);
}

function readProtocolSchemaVersion(record: Readonly<Record<string, unknown>>): number {
  if (record['schemaVersion'] !== GeneratedPortalAgentProtocolRuntime.SchemaVersion) {
    throw new TypeError('invalid activity protocol schema version');
  }
  return GeneratedPortalAgentProtocolRuntime.SchemaVersion;
}

function readNonNegativeInteger(record: Readonly<Record<string, unknown>>, field: string): number {
  const value = record[field];
  if (typeof value !== 'number' || !Number.isSafeInteger(value) || value < 0) {
    throw new TypeError('invalid activity count');
  }
  return value;
}

function readBoolean(record: Readonly<Record<string, unknown>>, field: string): boolean {
  const value = record[field];
  if (typeof value !== 'boolean') throw new TypeError('invalid activity boolean');
  return value;
}

function readOptionalNullableText(record: Readonly<Record<string, unknown>>, field: string): string | null {
  const value = record[field];
  if (value === undefined || value === null) return null;
  if (typeof value !== 'string' || value.length === 0) throw new TypeError('invalid activity text');
  return value;
}
