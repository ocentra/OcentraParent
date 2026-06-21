import { readFileSync } from 'node:fs';
import { describe, expect, it } from 'vitest';
import {
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffSchema,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffState,
  buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoff,
} from '../../src/app-game-source-gated-policy-preview-timer-service-readiness-protocol-command-handoff';
import { AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelSchema } from '../../src/app-game-source-gated-policy-preview-timer-service-readiness-protocol-read-model';
import { ParentContractSchemaVersion } from '@ocentra-parent/schema-domain/family-reference-primitives';

const UpstreamProtocolReadModelPath = new URL(
  '../../../../test-results/app-game-source-gated-policy-preview-timer-service-readiness-protocol-read-model-proof/timer-service-readiness-protocol-read-model.json',
  import.meta.url
);

const ProtocolCommandHandoffOptions = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  commandHandoffId: 'source-gated-policy-preview-timer-service-readiness-protocol-command-handoff-proof',
  generatedAt: '2026-06-06T07:45:00Z',
  sourceContractRefs: [
    'app-game-source-gated-policy-preview-timer-service-readiness-protocol-read-model',
    'docs/expectations/app-game-evidence.md',
    'packages/agent-protocol-domain',
    'crates/agent-protocol',
    'crates/agent-service',
  ],
  protocolCommandRefs: ['agent.activity.app-game.timer-service-readiness.read-model.get'],
  protocolEventRefs: ['agent.activity.app-game.timer-service-readiness.read-model.reported'],
  serviceHandlerRefs: ['future-app-game-timer-service-readiness-command-handler-proof'],
  commandSummaryRef: 'future-service-readiness-protocol-command-handoff-summary-proof',
} as const;

describe('app/game source-gated policy preview timer service-readiness protocol command handoff', () => {
  it('projects protocol read-model rows into command handoff proof states', () => {
    const handoff = buildProtocolCommandHandoff();

    expect(handoff.protocolCommandHandoffProofRequiredCount).toBe(1);
    expect(handoff.blockedBySourceFreshnessCount).toBe(1);
    expect(handoff.blockedByCompilerDecisionCount).toBe(1);
    expect(handoff.rows.map((row) => row.protocolCommandHandoffState)).toEqual([
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffState.ProtocolCommandHandoffProofRequired,
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffState.BlockedBySourceFreshness,
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffState.BlockedByCompilerDecision,
    ]);
  });

  it('preserves future command, event, and service-handler refs only for eligible rows', () => {
    const handoff = buildProtocolCommandHandoff();

    expect(handoff.rows[0]?.requiredAgentProtocolCommandRefs).toEqual([
      'agent.activity.app-game.timer-service-readiness.read-model.get',
    ]);
    expect(handoff.rows[0]?.requiredAgentProtocolEventRefs).toEqual([
      'agent.activity.app-game.timer-service-readiness.read-model.reported',
    ]);
    expect(handoff.rows[0]?.requiredServiceHandlerRefs).toEqual([
      'future-app-game-timer-service-readiness-command-handler-proof',
    ]);
    expect(handoff.rows[0]?.commandSummaryRef).toBe('future-service-readiness-protocol-command-handoff-summary-proof');
    expect(handoff.rows[1]?.requiredAgentProtocolCommandRefs).toEqual([]);
    expect(handoff.rows[1]?.requiredAgentProtocolEventRefs).toEqual([]);
    expect(handoff.rows[1]?.requiredServiceHandlerRefs).toEqual([]);
  });

  it('rejects command, event, service, UI, adapter, and count overclaims', () => {
    const handoff = buildProtocolCommandHandoff();

    expect(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffSchema.safeParse({
        ...handoff,
        agentProtocolCommandImplemented: true,
      }).success
    ).toBe(false);
    expect(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffSchema.safeParse({
        ...handoff,
        agentProtocolEventImplemented: true,
      }).success
    ).toBe(false);
    expect(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffSchema.safeParse({
        ...handoff,
        serviceHandlerImplemented: true,
      }).success
    ).toBe(false);
    expect(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffSchema.safeParse({
        ...handoff,
        adapterDispatchClaimed: true,
      }).success
    ).toBe(false);
    expect(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffSchema.safeParse({
        ...handoff,
        protocolCommandHandoffProofRequiredCount: 0,
      }).success
    ).toBe(false);
  });
});

function buildProtocolCommandHandoff() {
  return buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoff(
    ProtocolCommandHandoffOptions,
    readUpstreamProtocolReadModel()
  );
}

function readUpstreamProtocolReadModel() {
  return AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelSchema.parse(
    JSON.parse(readFileSync(UpstreamProtocolReadModelPath, 'utf8'))
  );
}
