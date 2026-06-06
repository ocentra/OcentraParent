import { readFileSync } from 'node:fs';
import { describe, expect, it } from 'vitest';
import {
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffSchema,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffState,
  buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoff,
} from '../src/app-game-source-gated-policy-preview-timer-service-readiness-protocol-handoff';
import { AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelSchema } from '../src/app-game-source-gated-policy-preview-timer-service-readiness-read-model';
import { ParentContractSchemaVersion } from '../src/reference-primitives';

const UpstreamReadModelPath = new URL(
  '../../../test-results/app-game-source-gated-policy-preview-timer-service-readiness-read-model-proof/timer-service-readiness-read-model.json',
  import.meta.url
);

const ProtocolHandoffOptions = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  handoffId: 'source-gated-policy-preview-timer-service-readiness-protocol-handoff-proof',
  generatedAt: '2026-06-06T07:12:00Z',
  sourceContractRefs: [
    'app-game-source-gated-policy-preview-timer-service-readiness-read-model',
    'docs/expectations/app-game-evidence.md',
    'packages/agent-protocol-domain',
    'crates/agent-protocol',
    'crates/agent-service',
  ],
  protocolCommandContractProofRef: 'future-agent-protocol-command-contract-proof',
  protocolEventContractProofRef: 'future-agent-protocol-event-contract-proof',
  rustProtocolMirrorProofRef: 'future-rust-protocol-mirror-proof',
  serviceHandlerProofRef: 'future-service-handler-proof',
} as const;

describe('app/game source-gated policy preview timer service-readiness protocol handoff', () => {
  it('projects WP87 rows into protocol handoff requirements without implementing protocol or service runtime', () => {
    const handoff = buildProtocolHandoff();

    expect(handoff.protocolProofRequiredCount).toBe(1);
    expect(handoff.blockedBySourceFreshnessCount).toBe(1);
    expect(handoff.blockedByCompilerDecisionCount).toBe(1);
    expect(handoff.rows.map((row) => row.protocolHandoffState)).toEqual([
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffState.ProtocolProofRequired,
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffState.BlockedBySourceFreshness,
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffState.BlockedByCompilerDecision,
    ]);
  });

  it('preserves service-readiness refs and adds only future protocol proof refs for eligible rows', () => {
    const handoff = buildProtocolHandoff();

    expect(handoff.rows[0]?.requiredProtocolProofRefs).toEqual([
      'future-agent-protocol-command-contract-proof',
      'future-agent-protocol-event-contract-proof',
      'future-rust-protocol-mirror-proof',
      'future-service-handler-proof',
    ]);
    expect(handoff.rows[0]?.inheritedServiceReadinessProofRefs).toEqual([
      'future-service-timer-runtime-proof',
      'future-scheduler-persistence-proof',
      'future-scheduler-state-store-proof',
      'future-timer-audit-trail-proof',
      'future-timer-rollback-plan-proof',
      'future-timer-audit-rollback-read-model-proof',
      'future-parent-surface-audit-rollback-intent-proof',
      'future-service-readiness-proof',
      'future-service-read-api-proof',
    ]);
    expect(handoff.rows[1]?.requiredProtocolProofRefs).toEqual([]);
    expect(handoff.rows[2]?.requiredProtocolProofRefs).toEqual([]);
  });

  it('rejects protocol, service, UI, adapter, and count overclaims', () => {
    const handoff = buildProtocolHandoff();

    expect(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffSchema.safeParse({
        ...handoff,
        agentProtocolContractImplemented: true,
      }).success
    ).toBe(false);
    expect(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffSchema.safeParse({
        ...handoff,
        serviceEventEmitted: true,
      }).success
    ).toBe(false);
    expect(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffSchema.safeParse({
        ...handoff,
        serviceReadApiImplemented: true,
      }).success
    ).toBe(false);
    expect(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffSchema.safeParse({
        ...handoff,
        adapterDispatchClaimed: true,
      }).success
    ).toBe(false);
    expect(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffSchema.safeParse({
        ...handoff,
        protocolProofRequiredCount: 0,
      }).success
    ).toBe(false);
  });
});

function buildProtocolHandoff() {
  return buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoff(
    ProtocolHandoffOptions,
    readUpstreamReadModel()
  );
}

function readUpstreamReadModel() {
  return AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelSchema.parse(
    JSON.parse(readFileSync(UpstreamReadModelPath, 'utf8'))
  );
}
