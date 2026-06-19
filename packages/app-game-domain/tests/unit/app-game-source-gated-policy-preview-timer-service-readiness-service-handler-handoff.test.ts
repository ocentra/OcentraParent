import { readFileSync } from 'node:fs';
import { describe, expect, it } from 'vitest';
import { AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffSchema } from '../../src/app-game-source-gated-policy-preview-timer-service-readiness-protocol-command-handoff';
import {
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffSchema,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffState,
  buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoff,
} from '../../src/app-game-source-gated-policy-preview-timer-service-readiness-service-handler-handoff';
import { ParentContractSchemaVersion } from '@ocentra-parent/schema-domain/family-reference-primitives';

const UpstreamProtocolCommandHandoffPath = new URL(
  '../../../test-results/app-game-source-gated-policy-preview-timer-service-readiness-protocol-command-handoff-proof/timer-service-readiness-protocol-command-handoff.json',
  import.meta.url
);

const ServiceHandlerHandoffOptions = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  serviceHandlerHandoffId: 'source-gated-policy-preview-timer-service-readiness-service-handler-handoff-proof',
  generatedAt: '2026-06-06T07:58:00Z',
  sourceContractRefs: [
    'app-game-source-gated-policy-preview-timer-service-readiness-protocol-command-handoff',
    'docs/expectations/app-game-evidence.md',
    'crates/agent-service',
  ],
  serviceReadApiProofRefs: ['future-app-game-timer-service-readiness-read-api-proof'],
  serviceHandlerSummaryRef: 'future-service-readiness-service-handler-handoff-summary-proof',
} as const;

describe('app/game source-gated policy preview timer service-readiness service handler handoff', () => {
  it('projects protocol command-handoff rows into service handler proof states', () => {
    const handoff = buildServiceHandlerHandoff();

    expect(handoff.serviceHandlerProofRequiredCount).toBe(1);
    expect(handoff.blockedBySourceFreshnessCount).toBe(1);
    expect(handoff.blockedByCompilerDecisionCount).toBe(1);
    expect(handoff.rows.map((row) => row.serviceHandlerHandoffState)).toEqual([
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffState.ServiceHandlerProofRequired,
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffState.BlockedBySourceFreshness,
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffState.BlockedByCompilerDecision,
    ]);
  });

  it('preserves inherited command/event refs and adds service read API proof refs only for eligible rows', () => {
    const handoff = buildServiceHandlerHandoff();

    expect(handoff.rows[0]?.inheritedAgentProtocolCommandRefs).toEqual([
      'agent.activity.app-game.timer-service-readiness.read-model.get',
    ]);
    expect(handoff.rows[0]?.inheritedAgentProtocolEventRefs).toEqual([
      'agent.activity.app-game.timer-service-readiness.read-model.reported',
    ]);
    expect(handoff.rows[0]?.requiredServiceHandlerRefs).toEqual([
      'future-app-game-timer-service-readiness-command-handler-proof',
    ]);
    expect(handoff.rows[0]?.requiredServiceReadApiProofRefs).toEqual([
      'future-app-game-timer-service-readiness-read-api-proof',
    ]);
    expect(handoff.rows[1]?.requiredServiceHandlerRefs).toEqual([]);
    expect(handoff.rows[1]?.requiredServiceReadApiProofRefs).toEqual([]);
  });

  it('rejects service handler, read API, UI, adapter, and count overclaims', () => {
    const handoff = buildServiceHandlerHandoff();

    expect(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffSchema.safeParse({
        ...handoff,
        serviceHandlerImplemented: true,
      }).success
    ).toBe(false);
    expect(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffSchema.safeParse({
        ...handoff,
        serviceReadApiImplemented: true,
      }).success
    ).toBe(false);
    expect(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffSchema.safeParse({
        ...handoff,
        portalUiRendered: true,
      }).success
    ).toBe(false);
    expect(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffSchema.safeParse({
        ...handoff,
        adapterDispatchClaimed: true,
      }).success
    ).toBe(false);
    expect(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffSchema.safeParse({
        ...handoff,
        serviceHandlerProofRequiredCount: 0,
      }).success
    ).toBe(false);
  });
});

function buildServiceHandlerHandoff() {
  return buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoff(
    ServiceHandlerHandoffOptions,
    readUpstreamProtocolCommandHandoff()
  );
}

function readUpstreamProtocolCommandHandoff() {
  return AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffSchema.parse(
    JSON.parse(readFileSync(UpstreamProtocolCommandHandoffPath, 'utf8'))
  );
}
