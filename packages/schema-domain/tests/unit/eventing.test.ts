import { describe, expect, it } from 'vitest';

import {
  EventingEventContractSchema,
  EventingEventPriority,
  EventingEventTypeSchema,
  EventingRequestCompletionOutcome,
  EventingRequestCompletionReportSchema,
  EventingStoredEnvelopeHeaderSchema,
  EventingTopologyManifestSchema,
  EventingTopologyStatus,
} from '../../src/eventing';
import { TrackingEventName } from '../../src/agent-tracking-retention-settings-write-command';

describe('event-domain eventing taxonomy and envelopes', () => {
  it('rejects malformed eventing taxonomy values through the shared schema owner', () => {
    expect(EventingEventTypeSchema.safeParse(TrackingEventName.LocationObserved).success).toBe(true);
    expect(EventingEventTypeSchema.safeParse(`.${TrackingEventName.LocationObserved}`).success).toBe(false);
    expect(
      EventingEventTypeSchema.safeParse(TrackingEventName.LocationObserved.replace('.location.', '..location.')).success
    ).toBe(false);
    expect(EventingEventTypeSchema.safeParse(`${TrackingEventName.LocationObserved}/`).success).toBe(false);
  });

  it('parses eventing contract and stored envelope header without a feature-local schema clone', () => {
    const contract = EventingEventContractSchema.parse({
      eventType: TrackingEventName.LocationObserved,
      schemaVersion: 2,
    });
    const header = EventingStoredEnvelopeHeaderSchema.parse({
      contract,
      metadata: {
        eventId: 'event-1',
        correlationId: 'correlation-1',
        causationId: null,
        aggregateKey: 'child-device-1',
        idempotencyKey: 'tracking-location-1',
        source: {
          custody: 'local-child-runtime',
          role: 'child-runtime',
          service: 'tracking-core',
          component: 'tracking-runtime-flow',
          instanceId: 'child-runtime-1',
        },
        observedAt: '2026-06-12T10:00:00.000Z',
        targetHandler: null,
        priority: EventingEventPriority.Normal,
        deadline: null,
      },
      journalHash: 'journal-hash-1',
    });

    expect(header.contract.eventType).toBe(TrackingEventName.LocationObserved);
    expect(header.contract.schemaVersion).toBe(2);
    expect(header.metadata.source.component).toBe('tracking-runtime-flow');
    expect(header.journalHash).toBe('journal-hash-1');
  });

  it('rejects zero schema versions at the shared stored-envelope boundary', () => {
    const result = EventingStoredEnvelopeHeaderSchema.safeParse({
      contract: {
        eventType: TrackingEventName.LocationObserved,
        schemaVersion: 0,
      },
      metadata: {
        eventId: 'event-1',
        correlationId: 'correlation-1',
        causationId: null,
        aggregateKey: 'child-device-1',
        idempotencyKey: 'tracking-location-1',
        source: {
          custody: 'local-child-runtime',
          role: 'child-runtime',
          service: 'tracking-core',
          component: 'tracking-runtime-flow',
          instanceId: 'child-runtime-1',
        },
        observedAt: '2026-06-12T10:00:00.000Z',
        targetHandler: null,
        priority: EventingEventPriority.Normal,
        deadline: null,
      },
      journalHash: 'journal-hash-1',
    });

    expect(result.success).toBe(false);
  });
});

describe('event-domain eventing manifests and reports', () => {
  it('parses event topology manifests with subscriber targets from the common schema', () => {
    const manifest = EventingTopologyManifestSchema.parse({
      entries: [
        {
          contract: {
            eventType: TrackingEventName.AiAnalysisRequested,
            schemaVersion: 1,
          },
          rustType: 'tracking_core::ai_boundary::TrackingAiAnalysisRequested',
          publishers: ['tracking-core'],
          subscribers: [
            {
              subscriberId: 'child-ai-core',
              targetHandler: 'child_ai_core::handle_tracking_ai_request',
            },
          ],
          families: ['tracking'],
          status: EventingTopologyStatus.Covered,
        },
      ],
    });

    expect(manifest.entries[0]?.subscribers[0]?.subscriberId).toBe('child-ai-core');
    expect(manifest.entries[0]?.status).toBe(EventingTopologyStatus.Covered);
  });

  it('parses request completion reports with the Rust eventing outcome vocabulary', () => {
    const report = EventingRequestCompletionReportSchema.parse({
      requestId: 'request-1',
      outcome: EventingRequestCompletionOutcome.Completed,
    });

    expect(report.requestId).toBe('request-1');
    expect(report.outcome).toBe(EventingRequestCompletionOutcome.Completed);
  });
});
