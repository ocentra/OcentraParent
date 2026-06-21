import { describe, expect, it } from 'vitest';
import {
  buildLocalAiRecentMemoryWindowReadModel,
  LocalAiRecentMemoryWindowReadModelSchema,
  LocalAiRecentMemoryWindowSchema,
} from '@ocentra-parent/schema-domain/local-ai-recent-memory-window-proof';

const observedAt = '2026-06-06T05:20:00.000Z';
const asOf = '2026-06-06T05:30:00.000Z';
const childProfile = { childProfileId: 'child:maya', displayName: 'Maya' };
const device = {
  deviceId: 'device:maya-windows',
  childProfileId: childProfile.childProfileId,
  label: 'Maya Windows laptop',
  platform: 'windows',
};
const sourceEvidence = {
  evidenceReferenceId: 'journal:recent-browser-game',
  kind: 'journal-event',
  observedAt,
};
const outsideSourceEvidence = {
  evidenceReferenceId: 'journal:outside-window',
  kind: 'journal-event',
  observedAt: '2026-06-06T04:10:00.000Z',
};
const parentRuleContextReference = {
  parentRuleRefId: 'parent-rule-context:recent-activity',
  policyVersion: 'policy:v1',
  family: { familyId: 'family:maya' },
  childProfile,
  device,
  rule: {
    ruleId: 'rule:recent-games',
    target: { targetId: 'target:games', targetType: 'category', targetValue: 'games' },
    action: 'warn',
    scheduleId: null,
    priority: 10,
    reasonCode: 'recent-activity-games',
    createdBy: { actorId: 'parent:maya', role: 'parent' },
    enabled: true,
    effectiveFrom: null,
    effectiveUntil: null,
  },
  targetEvidenceRefs: ['recent-activity:browser-game'],
  custody: 'parent-device-cache',
  updatedAt: observedAt,
  expiresAt: null,
};
const contextInput = {
  contextId: 'context:recent-memory-window',
  request: {
    schemaVersion: 'v0.6',
    requestId: 'request:recent-memory-window',
    requestedAt: asOf,
    childProfile,
    device,
    requestedEvaluationKind: 'recent-activity',
    requiredEvidenceKinds: ['recent-activity'],
    parentRuleContextReferences: [parentRuleContextReference],
    modelTaskRequirements: [],
    allowedCustody: ['child-device-query-store'],
    promptVersion: 'prompt:recent-memory-v1',
  },
  evidenceReferences: [
    {
      evidenceRefId: 'recent-activity:browser-game',
      evidence: sourceEvidence,
      evidenceKind: 'recent-activity',
      sourceSchemaVersion: 'v0.6',
      observedAt,
      ingestedAt: '2026-06-06T05:20:05.000Z',
      freshUntil: '2026-06-06T05:45:00.000Z',
      sourceId: 'source:recent-activity',
      adapterId: 'adapter:local-window',
      device,
      childProfile,
      custody: 'child-device-query-store',
      retentionState: 'local',
      confidence: 0.88,
      confidenceKind: 'memory-match',
      capabilityStatus: 'available',
      degradedReasons: [],
      unknownReasons: [],
      sourceEvidenceReferences: [sourceEvidence],
    },
    {
      evidenceRefId: 'recent-activity:outside-window',
      evidence: outsideSourceEvidence,
      evidenceKind: 'recent-activity',
      sourceSchemaVersion: 'v0.6',
      observedAt: '2026-06-06T04:10:00.000Z',
      ingestedAt: '2026-06-06T04:10:05.000Z',
      freshUntil: '2026-06-06T05:45:00.000Z',
      sourceId: 'source:outside-window',
      adapterId: 'adapter:local-window',
      device,
      childProfile,
      custody: 'child-device-query-store',
      retentionState: 'local',
      confidence: 0.66,
      confidenceKind: 'memory-match',
      capabilityStatus: 'available',
      degradedReasons: [],
      unknownReasons: [],
      sourceEvidenceReferences: [outsideSourceEvidence],
    },
  ],
  runtimeReferences: [],
  memoryReferences: [
    {
      memoryReferenceId: 'memory:recent-browser-game',
      kind: 'recent-activity',
      sourceEvidenceReferences: [sourceEvidence],
      sourcePolicyVersion: null,
      generatedAt: '2026-06-06T05:21:00.000Z',
      confidence: 0.83,
      derivedIndexVersion: 'recent-memory:v1',
    },
    {
      memoryReferenceId: 'memory:outside-window',
      kind: 'recent-activity',
      sourceEvidenceReferences: [outsideSourceEvidence],
      sourcePolicyVersion: null,
      generatedAt: '2026-06-06T04:11:00.000Z',
      confidence: 0.72,
      derivedIndexVersion: 'recent-memory:v1',
    },
  ],
  graphReferences: [],
};
const readInput = {
  contextInput,
  window: {
    observedFrom: '2026-06-06T05:00:00.000Z',
    observedUntil: asOf,
    asOf,
  },
  limit: 5,
};

describe('local AI recent memory window proof contracts', () => {
  it('buildLocalAiRecentMemoryWindowReadModel: returns only source-grounded recent activity inside the short window', () => {
    const result = buildLocalAiRecentMemoryWindowReadModel(readInput);

    expect(result.state).toBe('partial');
    expect(result.recentActivityEvidenceRefs).toEqual(['recent-activity:browser-game']);
    expect(result.recentActivitySourceEvidenceReferences.map((reference) => reference.evidenceReferenceId)).toEqual([
      'journal:recent-browser-game',
    ]);
    expect(result.recentMemoryReferences.map((reference) => reference.memoryReferenceId)).toEqual([
      'memory:recent-browser-game',
    ]);
    expect(result.returnedRecentActivityCount).toBe(1);
    expect(result.returnedRecentMemoryCount).toBe(1);
    expect(result.omittedRecentActivityCount).toBe(1);
    expect(result.omittedRecentMemoryCount).toBe(1);
    expect(result.degradedReasons).toEqual(['stale-evidence', 'memory-ungrounded']);
    expect(result.rawEvidenceRetained).toBe(false);
    expect(result.remoteAiUsed).toBe(false);
    expect(result.policyAuthorityClaimed).toBe(false);
    expect(result.enforcementClaimed).toBe(false);
  });

  it('buildLocalAiRecentMemoryWindowReadModel: degrades to insufficient when no recent activity is in the read window', () => {
    const result = buildLocalAiRecentMemoryWindowReadModel({
      ...readInput,
      window: {
        observedFrom: '2026-06-06T06:00:00.000Z',
        observedUntil: '2026-06-06T06:10:00.000Z',
        asOf: '2026-06-06T06:10:00.000Z',
      },
    });

    expect(result.state).toBe('insufficient');
    expect(result.recentActivityEvidenceRefs).toEqual([]);
    expect(result.recentMemoryReferences).toEqual([]);
    expect(result.degradedReasons).toEqual(['missing-evidence', 'stale-evidence', 'memory-ungrounded']);
  });

  it('LocalAiRecentMemoryWindowSchema: rejects inverted windows and future reads', () => {
    const inverted = LocalAiRecentMemoryWindowSchema.safeParse({
      observedFrom: asOf,
      observedUntil: observedAt,
      asOf,
    });
    const futureRead = LocalAiRecentMemoryWindowSchema.safeParse({
      observedFrom: observedAt,
      observedUntil: asOf,
      asOf: observedAt,
    });

    expect(inverted.success).toBe(false);
    expect(futureRead.success).toBe(false);
  });

  it('LocalAiRecentMemoryWindowReadModelSchema: rejects raw retention, remote AI, policy authority, and enforcement claims', () => {
    const result = buildLocalAiRecentMemoryWindowReadModel(readInput);
    const rejected = LocalAiRecentMemoryWindowReadModelSchema.safeParse({
      ...result,
      rawEvidenceRetained: true,
      remoteAiUsed: true,
      policyAuthorityClaimed: true,
      enforcementClaimed: true,
    });

    expect(rejected.success).toBe(false);
  });
});
