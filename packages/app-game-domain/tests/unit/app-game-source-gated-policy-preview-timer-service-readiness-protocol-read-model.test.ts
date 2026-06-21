import { readFileSync } from 'node:fs';
import { describe, expect, it } from 'vitest';
import { AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffSchema } from '../../src/app-game-source-gated-policy-preview-timer-service-readiness-protocol-handoff';
import {
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelSchema,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelState,
  buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModel,
} from '../../src/app-game-source-gated-policy-preview-timer-service-readiness-protocol-read-model';
import { ParentContractSchemaVersion } from '@ocentra-parent/schema-domain/family-reference-primitives';

const UpstreamProtocolHandoffPath = new URL(
  '../../../../test-results/app-game-source-gated-policy-preview-timer-service-readiness-protocol-handoff-proof/timer-service-readiness-protocol-handoff.json',
  import.meta.url
);

const ProtocolReadModelOptions = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  readModelId: 'source-gated-policy-preview-timer-service-readiness-protocol-read-model-proof',
  generatedAt: '2026-06-06T07:23:00Z',
  sourceContractRefs: [
    'app-game-source-gated-policy-preview-timer-service-readiness-protocol-handoff',
    'docs/expectations/app-game-evidence.md',
    'packages/agent-protocol-domain',
    'crates/agent-protocol',
    'crates/agent-service',
  ],
  protocolSummaryRef: 'future-service-readiness-protocol-read-model-summary-proof',
} as const;

describe('app/game source-gated policy preview timer service-readiness protocol read model', () => {
  it('projects protocol handoff rows into protocol read-model proof states', () => {
    const readModel = buildProtocolReadModel();

    expect(readModel.protocolReadModelProofRequiredCount).toBe(1);
    expect(readModel.blockedBySourceFreshnessCount).toBe(1);
    expect(readModel.blockedByCompilerDecisionCount).toBe(1);
    expect(readModel.rows.map((row) => row.protocolReadModelState)).toEqual([
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelState.ProtocolReadModelProofRequired,
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelState.BlockedBySourceFreshness,
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelState.BlockedByCompilerDecision,
    ]);
  });

  it('preserves future protocol refs for eligible rows without emitting service read-model events', () => {
    const readModel = buildProtocolReadModel();

    expect(readModel.rows[0]?.requiredProtocolProofRefs).toEqual([
      'future-agent-protocol-command-contract-proof',
      'future-agent-protocol-event-contract-proof',
      'future-rust-protocol-mirror-proof',
      'future-service-handler-proof',
    ]);
    expect(readModel.rows[0]?.protocolSummaryRef).toBe('future-service-readiness-protocol-read-model-summary-proof');
    expect(readModel.rows[0]?.serviceReadModelEventEmitted).toBe(false);
    expect(readModel.rows[1]?.requiredProtocolProofRefs).toEqual([]);
  });

  it('rejects protocol read-model event, service, UI, adapter, and count overclaims', () => {
    const readModel = buildProtocolReadModel();

    expect(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelSchema.safeParse({
        ...readModel,
        serviceReadModelEventEmitted: true,
      }).success
    ).toBe(false);
    expect(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelSchema.safeParse({
        ...readModel,
        serviceCommandRegistered: true,
      }).success
    ).toBe(false);
    expect(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelSchema.safeParse({
        ...readModel,
        serviceReadApiImplemented: true,
      }).success
    ).toBe(false);
    expect(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelSchema.safeParse({
        ...readModel,
        adapterDispatchClaimed: true,
      }).success
    ).toBe(false);
    expect(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelSchema.safeParse({
        ...readModel,
        protocolReadModelProofRequiredCount: 0,
      }).success
    ).toBe(false);
  });
});

function buildProtocolReadModel() {
  return buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModel(
    ProtocolReadModelOptions,
    readUpstreamProtocolHandoff()
  );
}

function readUpstreamProtocolHandoff() {
  return AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffSchema.parse(
    JSON.parse(readFileSync(UpstreamProtocolHandoffPath, 'utf8'))
  );
}
