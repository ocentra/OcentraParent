import { readFileSync } from 'node:fs';
import { describe, expect, it } from 'vitest';

import {
  AiActorRole,
  AiAuthorityBoundary,
  AiContractSchemaVersion,
  AiContextBuildState,
  AiCustodyState,
  AiDegradedState,
  AiDurabilityState,
  AiExplanationState,
  AiExplanationSurface,
  AiGraphEdgeKind,
  AiGraphNodeKind,
  AiJournalEntryKind,
  AiJournalPayloadKind,
  AiMemoryReferenceKind,
  AiOutputValidationState,
  AiProvenanceKind,
  AiReferenceValidationState,
  AiRedactionState,
  AiRemoteAssistantRedactionPolicy,
  AiRemoteAssistantSafetyBoundary,
  AiRemoteAssistantState,
  AiResultKind,
  AiRetentionState,
  AiValidationState,
  AiWorkKind,
  AiWorkState,
} from '../../src/generated-ai-contracts';

describe('Rust-owned AI contract generated edge', () => {
  it('keeps the generated schema version synchronized with the Rust leaf', () => {
    const rustSource = readFileSync(
      new URL('../../../../crates/ai-contracts/src/ai_contracts.rs', import.meta.url),
      'utf8'
    );
    const match = rustSource.match(/pub const AI_CONTRACT_SCHEMA_VERSION: &str = "([^"]+)";/);
    if (match === null) {
      throw new TypeError('Rust AI contract schema version declaration is missing');
    }

    expect(AiContractSchemaVersion).toBe(match[1]);
  });

  it('preserves every Rust enum literal at the generated TypeScript edge', () => {
    const enums = {
      authorityBoundary: [
        'evidence-only',
        'deterministic-policy-required',
        'manual-review-required',
      ] as const satisfies readonly AiAuthorityBoundary[],
      custodyState: [
        'child-local-encrypted',
        'parent-local-encrypted',
        'parent-authorized-redacted',
        'ephemeral-local',
        'deleted',
        'unavailable',
      ] as const satisfies readonly AiCustodyState[],
      contextBuildState: [
        'ready',
        'partial',
        'rejected',
        'manual-required',
      ] as const satisfies readonly AiContextBuildState[],
      degradedState: [
        'none',
        'missing-evidence',
        'invalid-output',
        'timeout',
        'model-unavailable',
        'provider-unavailable',
        'custody-unavailable',
        'manual-required',
      ] as const satisfies readonly AiDegradedState[],
      durabilityState: [
        'durable',
        'append-pending',
        'replay-only',
        'not-durable',
        'manual-required',
      ] as const satisfies readonly AiDurabilityState[],
      explanationState: [
        'ready',
        'degraded',
        'unavailable',
        'manual-required',
      ] as const satisfies readonly AiExplanationState[],
      explanationSurface: [
        'parent-read-model',
        'child-safety-internal',
        'audit-record',
      ] as const satisfies readonly AiExplanationSurface[],
      graphEdgeKind: [
        'supports',
        'derived-from',
        'related-to',
        'governed-by',
      ] as const satisfies readonly AiGraphEdgeKind[],
      graphNodeKind: [
        'evidence',
        'activity',
        'result',
        'memory',
        'policy-rule',
      ] as const satisfies readonly AiGraphNodeKind[],
      journalEntryKind: [
        'work-lifecycle',
        'context-built',
        'result-validated',
        'explanation-published',
        'remote-assistant',
      ] as const satisfies readonly AiJournalEntryKind[],
      journalPayloadKind: [
        'work-item',
        'evidence-context',
        'result',
        'explanation',
        'remote-assistant',
      ] as const satisfies readonly AiJournalPayloadKind[],
      memoryReferenceKind: [
        'recent-activity',
        'evidence-memory',
        'semantic-memory',
        'policy-memory',
      ] as const satisfies readonly AiMemoryReferenceKind[],
      outputValidationState: [
        'schema-valid',
        'schema-invalid',
        'evidence-missing',
        'confidence-invalid',
        'policy-handoff-required',
        'manual-required',
      ] as const satisfies readonly AiOutputValidationState[],
      provenanceKind: [
        'direct-observation',
        'derived-from-evidence',
        'derived-from-result',
        'parent-authored-rule',
      ] as const satisfies readonly AiProvenanceKind[],
      referenceValidationState: [
        'validated',
        'missing-source',
        'custody-blocked',
        'stale',
        'rejected',
      ] as const satisfies readonly AiReferenceValidationState[],
      redactionState: [
        'not-applicable',
        'redacted',
        'fully-redacted',
        'rejected-private-payload',
      ] as const satisfies readonly AiRedactionState[],
      remoteAssistantRedactionPolicy: [
        'references-only',
        'redacted-summaries',
        'no-child-payload',
      ] as const satisfies readonly AiRemoteAssistantRedactionPolicy[],
      remoteAssistantSafetyBoundary: [
        'parent-report-only',
        'outside-child-safety-blocking-path',
      ] as const satisfies readonly AiRemoteAssistantSafetyBoundary[],
      remoteAssistantState: [
        'disabled',
        'awaiting-parent-authorization',
        'authorized',
        'submitted',
        'succeeded',
        'degraded',
        'manual-required',
      ] as const satisfies readonly AiRemoteAssistantState[],
      resultKind: [
        'observation',
        'classification',
        'summary',
        'explanation',
        'no-claim',
      ] as const satisfies readonly AiResultKind[],
      retentionState: [
        'active',
        'expired',
        'tombstoned',
        'deleted',
        'manual-required',
      ] as const satisfies readonly AiRetentionState[],
      validationState: ['accepted', 'rejected', 'manual-required'] as const satisfies readonly AiValidationState[],
      actorRole: [
        'parent',
        'child-agent',
        'local-runtime',
        'parent-assistant',
        'remote-assistant',
        'system',
      ] as const satisfies readonly AiActorRole[],
      workKind: [
        'context-build',
        'classification',
        'explanation',
        'memory-derivation',
        'graph-derivation',
        'parent-assistant',
        'remote-assistant',
      ] as const satisfies readonly AiWorkKind[],
      workState: [
        'queued',
        'claimed',
        'running',
        'succeeded',
        'failed',
        'cancelled',
        'timed-out',
        'manual-required',
      ] as const satisfies readonly AiWorkState[],
    };

    expect(enums.authorityBoundary).toEqual([
      'evidence-only',
      'deterministic-policy-required',
      'manual-review-required',
    ]);
    expect(enums.remoteAssistantSafetyBoundary).toEqual(['parent-report-only', 'outside-child-safety-blocking-path']);
    expect(enums.workState).toEqual([
      'queued',
      'claimed',
      'running',
      'succeeded',
      'failed',
      'cancelled',
      'timed-out',
      'manual-required',
    ]);
  });

  it('keeps generated edge field names aligned for public request and owner-boundary declarations', () => {
    const generatedSource = readFileSync(new URL('../../src/generated-ai-contracts.ts', import.meta.url), 'utf8');
    const expectedDeclarations = [
      'export interface AiSchemaIdentity {',
      '  schemaVersion: AiSchemaVersion;',
      '  requestId: AiRequestId;',
      'export interface AiEvidenceContext {',
      '  authorityBoundary: AiAuthorityBoundary;',
      'export interface AiResult {',
      '  digest: AiDigest;',
      'export interface AiJournalEntry {',
      '  durability: AiDurabilityState;',
      'export interface AiRemoteAssistantWireRequest {',
      '  authorizationReferenceId: AiAuthorizationReferenceId;',
    ];

    expectedDeclarations.forEach((declaration) => {
      expect(generatedSource).toContain(declaration);
    });
  });
});
