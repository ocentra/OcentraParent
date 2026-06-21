import { readFileSync } from 'node:fs';
import { describe, expect, it } from 'vitest';
import { AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffSchema } from '../../src/app-game-source-gated-policy-preview-timer-service-readiness-service-handler-handoff';
import {
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffSchema,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffState,
  buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoff,
} from '../../src/app-game-source-gated-policy-preview-timer-service-readiness-read-api-handoff';
import { ParentContractSchemaVersion } from '@ocentra-parent/schema-domain/family-reference-primitives';

const UpstreamServiceHandlerHandoffPath = new URL(
  '../../../../test-results/app-game-source-gated-policy-preview-timer-service-readiness-service-handler-handoff-proof/timer-service-readiness-service-handler-handoff.json',
  import.meta.url
);

const ServiceReadApiHandoffOptions = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  serviceReadApiHandoffId: 'source-gated-policy-preview-timer-service-readiness-read-api-handoff-proof',
  generatedAt: '2026-06-06T07:58:00Z',
  sourceContractRefs: [
    'app-game-source-gated-policy-preview-timer-service-readiness-service-handler-handoff',
    'docs/expectations/app-game-evidence.md',
    'crates/agent-service',
  ],
  serviceReadApiProofRefs: ['future-app-game-timer-service-readiness-read-api-proof'],
  serviceReadApiSummaryRef: 'future-service-readiness-read-api-handoff-summary-proof',
} as const;

describe('app/game source-gated policy preview timer service-readiness service read API handoff', () => {
  it('projects service handler-handoff rows into service read API proof states', () => {
    const handoff = buildServiceReadApiHandoff();

    expect(handoff.serviceReadApiProofRequiredCount).toBe(1);
    expect(handoff.blockedBySourceFreshnessCount).toBe(1);
    expect(handoff.blockedByCompilerDecisionCount).toBe(1);
    expect(handoff.rows.map((row) => row.serviceReadApiHandoffState)).toEqual([
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffState.ServiceReadApiProofRequired,
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffState.BlockedBySourceFreshness,
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffState.BlockedByCompilerDecision,
    ]);
  });

  it('preserves inherited command/event refs and adds service read API proof refs only for eligible rows', () => {
    const handoff = buildServiceReadApiHandoff();

    expect(handoff.rows[0]?.inheritedAgentProtocolCommandRefs).toEqual([
      'agent.activity.app-game.timer-service-readiness.read-model.get',
    ]);
    expect(handoff.rows[0]?.inheritedAgentProtocolEventRefs).toEqual([
      'agent.activity.app-game.timer-service-readiness.read-model.reported',
    ]);
    expect(handoff.rows[0]?.inheritedServiceHandlerRefs).toEqual([
      'future-app-game-timer-service-readiness-command-handler-proof',
    ]);
    expect(handoff.rows[0]?.requiredServiceReadApiProofRefs).toEqual([
      'future-app-game-timer-service-readiness-read-api-proof',
    ]);
    expect(handoff.rows[1]?.inheritedServiceHandlerRefs).toEqual([]);
    expect(handoff.rows[1]?.requiredServiceReadApiProofRefs).toEqual([]);
  });

  it('rejects service handler, read API, UI, adapter, and count overclaims', () => {
    const handoff = buildServiceReadApiHandoff();

    expect(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffSchema.safeParse({
        ...handoff,
        serviceHandlerImplemented: true,
      }).success
    ).toBe(false);
    expect(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffSchema.safeParse({
        ...handoff,
        serviceReadApiImplemented: true,
      }).success
    ).toBe(false);
    expect(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffSchema.safeParse({
        ...handoff,
        portalUiRendered: true,
      }).success
    ).toBe(false);
    expect(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffSchema.safeParse({
        ...handoff,
        adapterDispatchClaimed: true,
      }).success
    ).toBe(false);
    expect(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffSchema.safeParse({
        ...handoff,
        serviceReadApiProofRequiredCount: 0,
      }).success
    ).toBe(false);
  });
});

function buildServiceReadApiHandoff() {
  return buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoff(
    ServiceReadApiHandoffOptions,
    readUpstreamServiceHandlerHandoff()
  );
}

function readUpstreamServiceHandlerHandoff() {
  return AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffSchema.parse(
    JSON.parse(readFileSync(UpstreamServiceHandlerHandoffPath, 'utf8'))
  );
}
