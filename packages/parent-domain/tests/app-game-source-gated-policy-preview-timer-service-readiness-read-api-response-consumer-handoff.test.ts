import { readFileSync } from 'node:fs';
import { describe, expect, it } from 'vitest';
import { AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffSchema } from '../src/app-game-source-gated-policy-preview-timer-service-readiness-read-api-response-handoff';
import {
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffSchema,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffState,
  buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoff,
} from '../src/app-game-source-gated-policy-preview-timer-service-readiness-read-api-response-consumer-handoff';
import { ParentContractSchemaVersion } from '../src/reference-primitives';

const UpstreamResponseHandoffPath = new URL(
  '../../../test-results/app-game-source-gated-policy-preview-timer-service-readiness-read-api-response-handoff-proof/timer-service-readiness-read-api-response-handoff.json',
  import.meta.url
);

const ResponseConsumerHandoffOptions = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  readApiResponseConsumerHandoffId:
    'source-gated-policy-preview-timer-service-readiness-read-api-response-consumer-handoff-proof',
  generatedAt: '2026-06-06T08:56:00Z',
  sourceContractRefs: [
    'app-game-source-gated-policy-preview-timer-service-readiness-read-api-response-handoff',
    'docs/expectations/app-game-evidence.md',
    'docs/expectations/enforcement.md',
  ],
  readApiResponseConsumerProofRefs: ['future-app-game-timer-service-readiness-read-api-response-consumer-proof'],
  readApiResponseConsumerSummaryRef: 'future-service-readiness-read-api-response-consumer-handoff-summary-proof',
} as const;

describe('app/game source-gated policy preview timer service-readiness read API response consumer handoff', () => {
  it('projects response handoff rows into response consumer proof states', () => {
    const handoff = buildResponseConsumerHandoff();

    expect(handoff.readApiResponseConsumerProofRequiredCount).toBe(1);
    expect(handoff.blockedBySourceFreshnessCount).toBe(1);
    expect(handoff.blockedByCompilerDecisionCount).toBe(1);
    expect(handoff.rows.map((row) => row.readApiResponseConsumerHandoffState)).toEqual([
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffState.ReadApiResponseConsumerProofRequired,
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffState.BlockedBySourceFreshness,
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffState.BlockedByCompilerDecision,
    ]);
  });

  it('preserves response proof refs and adds consumer proof refs only for eligible rows', () => {
    const handoff = buildResponseConsumerHandoff();

    expect(handoff.rows[0]?.inheritedReadApiResponseProofRefs).toEqual([
      'future-app-game-timer-service-readiness-read-api-response-proof',
    ]);
    expect(handoff.rows[0]?.requiredReadApiResponseConsumerProofRefs).toEqual([
      'future-app-game-timer-service-readiness-read-api-response-consumer-proof',
    ]);
    expect(handoff.rows[1]?.inheritedReadApiResponseProofRefs).toEqual([]);
    expect(handoff.rows[1]?.requiredReadApiResponseConsumerProofRefs).toEqual([]);
  });

  it('rejects response consumer, service, portal, adapter, and count overclaims', () => {
    const handoff = buildResponseConsumerHandoff();

    expect(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffSchema.safeParse({
        ...handoff,
        serviceReadApiResponseConsumerImplemented: true,
      }).success
    ).toBe(false);
    expect(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffSchema.safeParse({
        ...handoff,
        serviceReadApiResponseImplemented: true,
      }).success
    ).toBe(false);
    expect(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffSchema.safeParse({
        ...handoff,
        portalResponseConsumerRendered: true,
      }).success
    ).toBe(false);
    expect(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffSchema.safeParse({
        ...handoff,
        adapterDispatchClaimed: true,
      }).success
    ).toBe(false);
    expect(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffSchema.safeParse({
        ...handoff,
        readApiResponseConsumerProofRequiredCount: 0,
      }).success
    ).toBe(false);
  });
});

function buildResponseConsumerHandoff() {
  return buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoff(
    ResponseConsumerHandoffOptions,
    readUpstreamResponseHandoff()
  );
}

function readUpstreamResponseHandoff() {
  return AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffSchema.parse(
    JSON.parse(readFileSync(UpstreamResponseHandoffPath, 'utf8'))
  );
}
