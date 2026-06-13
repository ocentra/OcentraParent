import { describe, expect, it } from 'vitest';
import { ActivityEventSchema, ActivitySchemaVersion } from '../../src/contracts';
import { ActivityEventKind, ActivityEvidenceKind, ActivityObserver, ActivitySubjectKind } from '../../src/kinds';
import { decodeActivityDeviceId, decodeActivityEventId, decodeActivityTimestamp } from '../../src/primitives';

const ProcessObservationSample = {
  schemaVersion: ActivitySchemaVersion,
  eventId: 'activity-event-1',
  observedAt: '2026-05-20T00:00:00Z',
  source: {
    deviceId: 'child-device-1',
    platform: 'windows',
    observer: ActivityObserver.WindowsProcess,
    sourceId: 'windows-process-adapter',
  },
  kind: ActivityEventKind.ProcessObserved,
  subject: {
    kind: ActivitySubjectKind.Process,
    subjectId: 'process-4242',
    displayName: 'chrome.exe',
  },
  fields: {
    pid: 4242,
    foreground: true,
  },
  evidence: [
    {
      evidenceId: 'journal-entry-1',
      kind: ActivityEvidenceKind.JournalEntry,
      digest: 'sha256:process-event-digest',
      uri: null,
    },
  ],
} as const;

describe('activity event contracts', () => {
  it('ActivityEventSchema: accepts a Windows process observation with evidence references', () => {
    const parsed = ActivityEventSchema.safeParse(ProcessObservationSample);

    expect(parsed.success).toBe(true);
    if (parsed.success) {
      expect(parsed.data.source.observer).toBe('windows-process');
      expect(parsed.data.subject.displayName).toBe('chrome.exe');
      expect(parsed.data.evidence[0]?.digest).toBe('sha256:process-event-digest');
    }
  });

  it('ActivityEventSchema: rejects unknown activity event kinds', () => {
    const parsed = ActivityEventSchema.safeParse({
      schemaVersion: ActivitySchemaVersion,
      eventId: 'activity-event-2',
      observedAt: '2026-05-20T00:00:00Z',
      source: {
        deviceId: 'child-device-1',
        platform: 'windows',
        observer: ActivityObserver.WindowsWindow,
        sourceId: 'windows-window-adapter',
      },
      kind: 'activity.keystrokes.captured',
      subject: {
        kind: ActivitySubjectKind.Window,
        subjectId: 'window-1',
        displayName: null,
      },
      fields: {},
      evidence: [],
    });

    expect(parsed.success).toBe(false);
  });

  it('ActivityEventSchema: accepts an enforcement audit journal event', () => {
    const parsed = ActivityEventSchema.safeParse({
      schemaVersion: ActivitySchemaVersion,
      eventId: 'enforcement-audit-1',
      observedAt: '2026-05-20T00:00:00Z',
      source: {
        deviceId: 'child-device-1',
        platform: 'windows',
        observer: ActivityObserver.AgentService,
        sourceId: 'enforcement-service',
      },
      kind: ActivityEventKind.EnforcementAuditRecorded,
      subject: {
        kind: ActivitySubjectKind.Intervention,
        subjectId: 'action-1',
        displayName: 'terminate-process',
      },
      fields: {
        policyDecisionId: 'decision-1',
        enforcementStatus: 'actually-enforced',
      },
      evidence: [],
    });

    expect(parsed.success).toBe(true);
  });

  it('activity brand decoders: reject empty ids and timestamps', () => {
    expect(decodeActivityEventId('activity-event-1')).toBe('activity-event-1');
    expect(decodeActivityDeviceId('child-device-1')).toBe('child-device-1');
    expect(decodeActivityTimestamp('2026-05-20T00:00:00Z')).toBe('2026-05-20T00:00:00Z');
    expect(() => decodeActivityEventId('')).toThrow();
  });
});
