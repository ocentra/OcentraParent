import { describe, expect, it } from 'vitest';
import {
  ScreenAiBrowserTriggerProofRowSchema,
  ScreenAiBrowserTriggerProofSchema,
  screenAiBrowserTriggerProof,
  screenAiBrowserTriggerProofRows,
  screenAiBrowserTriggerProofSummary,
} from '@ocentra-parent/schema-domain/screen-ai-browser-trigger-proof';

describe('screen AI browser trigger proof contracts', () => {
  it('accepts managed browser URL, video, social, and cloud-game trigger rows', acceptsTriggerRows);
  it('links browser AI screen refs to screen evidence result refs', linksBrowserAndScreenRefs);
  it('keeps manual and unavailable browser-like surfaces out of policy-eligible state', keepsManualStatesExplicit);
  it('rejects unlinked screen evidence and claim upgrades', rejectsClaimUpgrades);
});

function acceptsTriggerRows() {
  const parsed = ScreenAiBrowserTriggerProofSchema.parse(screenAiBrowserTriggerProof);

  expect(parsed.rows.map((row) => row.surface)).toEqual([
    'managed-browser-url',
    'browser-like-video',
    'browser-like-social',
    'browser-like-cloud-game',
  ]);
  expect(parsed.rows.map((row) => row.triggerState)).toEqual(['ready', 'ready', 'manual-required', 'unavailable']);
  expect(screenAiBrowserTriggerProofSummary(parsed.rows)).toEqual({
    totalRows: 4,
    readyRows: 2,
    manualRequiredRows: 1,
    unavailableRows: 1,
    localAiReadyRows: 2,
    localAiPartialRows: 2,
    productClaimed: false,
    enforcementClaimed: false,
    remoteAiRequired: false,
  });
}

function linksBrowserAndScreenRefs() {
  for (const proofRow of screenAiBrowserTriggerProofRows) {
    const screenEvidenceIds = proofRow.screenAnalysis.sourceEvidenceRefs.map((reference) => reference.evidenceId);

    expect(proofRow.browserInput.screenEvidenceRefs).toEqual([screenEvidenceIds[1]]);
    expect(proofRow.browserResult.sourceEvidenceIds).toEqual(proofRow.browserInput.sourceEvidenceIds);
    expect(proofRow.noClaimFlags).toEqual({
      rawBrowserStateIncluded: false,
      rawScreenFrameStored: false,
      remoteAiRequired: false,
      finalPolicyClaimed: false,
      enforcementClaimed: false,
      liveExternalAccountClaimed: false,
      mobileBrowserParityClaimed: false,
      cloudFrameAnalysisClaimed: false,
    });
  }
}

function keepsManualStatesExplicit() {
  const social = screenAiBrowserTriggerProofRows.find((row) => row.surface === 'browser-like-social');
  const cloudGame = screenAiBrowserTriggerProofRows.find((row) => row.surface === 'browser-like-cloud-game');

  expect(social?.triggerState).toBe('manual-required');
  expect(social?.browserResult.degradedState).toBe('manual-required');
  expect(social?.screenAnalysis.policyEligible).toBe(false);
  expect(social?.mobileParityState).toBe('scaffold-only');

  expect(cloudGame?.triggerState).toBe('unavailable');
  expect(cloudGame?.browserResult.degradedState).toBe('unavailable');
  expect(cloudGame?.screenAnalysis.capabilityStatus).toBe('protectedSurface');
  expect(cloudGame?.screenAnalysis.policyEligible).toBe(false);
  expect(cloudGame?.mobileParityState).toBe('scaffold-only');
}

function rejectsClaimUpgrades() {
  const base = screenAiBrowserTriggerProofRows[0];

  expect(
    ScreenAiBrowserTriggerProofRowSchema.safeParse({
      ...base,
      browserInput: {
        ...base.browserInput,
        screenEvidenceRefs: ['unlinked-screen-ref'],
      },
    }).success
  ).toBe(false);

  expect(
    ScreenAiBrowserTriggerProofRowSchema.safeParse({
      ...base,
      browserInput: {
        ...base.browserInput,
        screenEvidenceRefs: [],
      },
    }).success
  ).toBe(false);

  expect(
    ScreenAiBrowserTriggerProofRowSchema.safeParse({
      ...base,
      noClaimFlags: {
        ...base.noClaimFlags,
        enforcementClaimed: true,
      },
    }).success
  ).toBe(false);
}
