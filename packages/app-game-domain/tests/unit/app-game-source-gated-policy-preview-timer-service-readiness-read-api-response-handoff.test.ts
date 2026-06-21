import { readFileSync } from 'node:fs';
import { describe, expect, it } from 'vitest';
import { AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffSchema } from '../../src/app-game-source-gated-policy-preview-timer-service-readiness-read-api-handoff';
import {
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffSchema,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffState,
  buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoff,
} from '../../src/app-game-source-gated-policy-preview-timer-service-readiness-read-api-response-handoff';
import { ParentContractSchemaVersion } from '@ocentra-parent/schema-domain/family-reference-primitives';

const UpstreamReadApiHandoffPath = new URL(
  '../../../../test-results/app-game-source-gated-policy-preview-timer-service-readiness-read-api-handoff-proof/timer-service-readiness-read-api-handoff.json',
  import.meta.url
);

const ReadApiResponseHandoffOptions = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  readApiResponseHandoffId: 'source-gated-policy-preview-timer-service-readiness-read-api-response-handoff-proof',
  generatedAt: '2026-06-06T08:36:00Z',
  sourceContractRefs: [
    'app-game-source-gated-policy-preview-timer-service-readiness-read-api-handoff',
    'docs/expectations/app-game-evidence.md',
    'crates/agent-service',
  ],
  readApiResponseProofRefs: ['future-app-game-timer-service-readiness-read-api-response-proof'],
  readApiResponseSummaryRef: 'future-service-readiness-read-api-response-handoff-summary-proof',
} as const;

describe('app/game source-gated policy preview timer service-readiness read API response handoff', () => {
  it('projects read API handoff rows into response proof states', () => {
    const handoff = buildReadApiResponseHandoff();

    expect(handoff.readApiResponseProofRequiredCount).toBe(1);
    expect(handoff.blockedBySourceFreshnessCount).toBe(1);
    expect(handoff.blockedByCompilerDecisionCount).toBe(1);
    expect(handoff.rows.map((row) => row.readApiResponseHandoffState)).toEqual([
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffState.ReadApiResponseProofRequired,
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffState.BlockedBySourceFreshness,
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffState.BlockedByCompilerDecision,
    ]);
  });

  it('preserves read API proof refs and adds response proof refs only for eligible rows', () => {
    const handoff = buildReadApiResponseHandoff();

    expect(handoff.rows[0]?.inheritedServiceReadApiProofRefs).toEqual([
      'future-app-game-timer-service-readiness-read-api-proof',
    ]);
    expect(handoff.rows[0]?.requiredReadApiResponseProofRefs).toEqual([
      'future-app-game-timer-service-readiness-read-api-response-proof',
    ]);
    expect(handoff.rows[1]?.inheritedServiceReadApiProofRefs).toEqual([]);
    expect(handoff.rows[1]?.requiredReadApiResponseProofRefs).toEqual([]);
  });

  it('rejects service read API response, service, UI, adapter, and count overclaims', () => {
    const handoff = buildReadApiResponseHandoff();

    expect(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffSchema.safeParse({
        ...handoff,
        serviceReadApiResponseImplemented: true,
      }).success
    ).toBe(false);
    expect(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffSchema.safeParse({
        ...handoff,
        serviceReadApiImplemented: true,
      }).success
    ).toBe(false);
    expect(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffSchema.safeParse({
        ...handoff,
        portalUiRendered: true,
      }).success
    ).toBe(false);
    expect(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffSchema.safeParse({
        ...handoff,
        adapterDispatchClaimed: true,
      }).success
    ).toBe(false);
    expect(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffSchema.safeParse({
        ...handoff,
        readApiResponseProofRequiredCount: 0,
      }).success
    ).toBe(false);
  });
});

function buildReadApiResponseHandoff() {
  return buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoff(
    ReadApiResponseHandoffOptions,
    readUpstreamReadApiHandoff()
  );
}

function readUpstreamReadApiHandoff() {
  return AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffSchema.parse(
    JSON.parse(readFileSync(UpstreamReadApiHandoffPath, 'utf8'))
  );
}
