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

describe('event-domain eventing contracts', () => {
  it('rejects malformed eventing taxonomy values through the shared event domain owner', () => {
    expect(EventingEventTypeSchema.safeParse('tracking.location.observed').success).toBe(true);
    expect(EventingEventTypeSchema.safeParse('.tracking.location.observed').success).toBe(false);
    expect(EventingEventTypeSchema.safeParse('tracking..location.observed').success).toBe(false);
    expect(EventingEventTypeSchema.safeParse('tracking.location.observed/').success).toBe(false);
  });

  it('parses eventing contract and stored envelope header without a feature-local schema clone', () => {
    const contract = EventingEventContractSchema.parse({
      eventType: 'tracking.location.observed',
      schemaVersion: 1,
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

    expect(header.contract.eventType).toBe('tracking.location.observed');
    expect(header.metadata.source.component).toBe('tracking-runtime-flow');
    expect(header.journalHash).toBe('journal-hash-1');
  });

  it('parses event topology manifests with subscriber targets from the common schema', () => {
    const manifest = EventingTopologyManifestSchema.parse({
      entries: [
        {
          contract: {
            eventType: 'tracking.ai.analysis.requested',
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
