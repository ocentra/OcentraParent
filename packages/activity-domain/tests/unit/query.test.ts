import { describe, expect, it } from 'vitest';
import { ActivityEventKind, ActivityObserver, ActivitySubjectKind } from '@ocentra-parent/schema-domain/evidence-kinds';
import {
  ActivityIngestStatusSchema,
  ActivityQuerySchemaVersion,
  ActivityRecentQuerySchema,
  ActivityRecentSummarySchema,
} from '@ocentra-parent/schema-domain/activity-query';

describe('activity query contracts', () => {
  it('parses query store ingest status with a nullable last event', () => {
    const status = ActivityIngestStatusSchema.parse({
      schemaVersion: ActivityQuerySchemaVersion,
      databaseReady: true,
      eventsIngested: 2,
      eventsStored: 2,
      duplicateEvents: 0,
      lastEventId: 'activity-event-2',
    });

    expect(status.databaseReady).toBe(true);
    expect(status.eventsIngested).toBe(2);
    expect(status.lastEventId).toBe('activity-event-2');
  });

  it('parses recent activity query limits and summary fields', () => {
    const query = ActivityRecentQuerySchema.parse({
      schemaVersion: ActivityQuerySchemaVersion,
      limit: 10,
    });
    const summary = ActivityRecentSummarySchema.parse({
      schemaVersion: ActivityQuerySchemaVersion,
      limit: 10,
      returned: 1,
      firstObservedAt: '2026-05-20T00:00:00Z',
      lastObservedAt: '2026-05-20T00:00:00Z',
      lastEventId: 'activity-event-1',
      mostRecentKind: ActivityEventKind.ProcessObserved,
      mostRecentObserver: ActivityObserver.WindowsProcess,
      mostRecentSubjectKind: ActivitySubjectKind.Process,
      mostRecentSubjectId: 'process-4242',
      mostRecentSubjectName: 'chrome.exe',
    });

    expect(query.limit).toBe(10);
    expect(summary.returned).toBe(1);
    expect(summary.mostRecentKind).toBe(ActivityEventKind.ProcessObserved);
    expect(summary.mostRecentSubjectName).toBe('chrome.exe');
  });

  it('rejects summaries with unknown event kinds', () => {
    const result = ActivityRecentSummarySchema.safeParse({
      schemaVersion: ActivityQuerySchemaVersion,
      limit: 10,
      returned: 1,
      firstObservedAt: '2026-05-20T00:00:00Z',
      lastObservedAt: '2026-05-20T00:00:00Z',
      lastEventId: 'activity-event-1',
      mostRecentKind: 'activity.unknown',
      mostRecentObserver: ActivityObserver.WindowsProcess,
      mostRecentSubjectKind: ActivitySubjectKind.Process,
      mostRecentSubjectId: 'process-4242',
      mostRecentSubjectName: null,
    });

    expect(result.success).toBe(false);
  });
});
