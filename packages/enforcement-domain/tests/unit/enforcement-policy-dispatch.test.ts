import { describe, expect, it } from 'vitest';
import {
  EnforcementPolicyDispatchCapabilityMatrixRowSchema,
  EnforcementPolicyDispatchIntentSchema,
  EnforcementPolicyDispatchReadModel,
  EnforcementPolicyDispatchReadModelSchema,
} from '@ocentra-parent/schema-domain/enforcement-policy-dispatch';

describe('enforcement policy dispatch contracts', () => {
  it('keeps the V0.8 dispatch proof matrix schema-backed and parent-visible', () => {
    const parsed = EnforcementPolicyDispatchReadModelSchema.parse(EnforcementPolicyDispatchReadModel);

    expectParsedDispatchReadModel(parsed);
    expectOwnedProcessEntry();
    expectAskParentEntry();
    expectAppGameEntry();
    expectRejectedEntries();
    expectTamperEntry();
  });

  it('rejects dispatch intents without evidence references', () => {
    const validIntent = entryForIntent('dispatch-owned-process-time-limit').intent;
    const parsed = EnforcementPolicyDispatchIntentSchema.safeParse({
      ...validIntent,
      evidenceReferences: [],
    });

    expect(parsed.success).toBe(false);
  });

  it('rejects missing or malformed decision references', () => {
    const validIntent = entryForIntent('dispatch-owned-process-time-limit').intent;

    const missingDecisionRef = EnforcementPolicyDispatchIntentSchema.safeParse({
      ...validIntent,
      policyDecisionRef: '',
    });
    const malformedDecisionRef = EnforcementPolicyDispatchIntentSchema.safeParse({
      ...validIntent,
      policyDecisionRef: 'malformed-dispatch-ref',
    });

    expect(missingDecisionRef.success).toBe(false);
    expect(malformedDecisionRef.success).toBe(false);
  });

  it('rejects ask-parent intents that claim live execution', () => {
    const askParentIntent = entryForIntent('dispatch-ask-parent-dry-run').intent;
    const parsed = EnforcementPolicyDispatchIntentSchema.safeParse({
      ...askParentIntent,
      dryRun: false,
    });

    expect(parsed.success).toBe(false);
  });

  it('rejects accidental implemented claim upgrades without dispatch-ready support', () => {
    const validMatrixRow = entryForIntent('dispatch-owned-process-time-limit').matrixRow;
    const parsed = EnforcementPolicyDispatchCapabilityMatrixRowSchema.safeParse({
      ...validMatrixRow,
      proofLevel: 'implemented',
      capabilityState: 'manual-required',
      outcomeState: 'manual-required',
      rejectionReason: 'adapter-manual-required',
    });

    expect(parsed.success).toBe(false);
  });

  it('requires child reason codes to match the capability matrix reason', () => {
    const validEntry = entryForIntent('dispatch-owned-process-time-limit');
    const parsed = EnforcementPolicyDispatchReadModelSchema.safeParse({
      ...EnforcementPolicyDispatchReadModel,
      entries: [
        {
          ...validEntry,
          childReasonCode: 'child-reason-mismatched',
        },
      ],
    });

    expect(parsed.success).toBe(false);
  });
});

function entryForIntent(intentId: string) {
  const entry = EnforcementPolicyDispatchReadModel.entries.find((candidate) => candidate.intent.intentId === intentId);
  if (entry === undefined) {
    throw new Error(`missing dispatch entry ${intentId}`);
  }
  return entry;
}

function countBy(values: readonly string[]) {
  return values.reduce<Record<string, number>>((counts, value) => {
    counts[value] = (counts[value] ?? 0) + 1;
    return counts;
  }, {});
}

function expectParsedDispatchReadModel(parsed: typeof EnforcementPolicyDispatchReadModel) {
  expect(parsed.readModelId).toBe('v0-8-enforcement-policy-dispatch');
  expect(parsed.entries).toHaveLength(8);
  expect(countBy(parsed.entries.map((entry) => entry.matrixRow.proofLevel))).toEqual({
    implemented: 2,
    scaffold: 4,
    'report-only': 1,
    'manual-required': 1,
  });
  expect(countBy(parsed.entries.map((entry) => entry.matrixRow.outcomeState))).toEqual({
    'dispatch-ready': 2,
    'dry-run-only': 1,
    'report-only': 1,
    'manual-required': 1,
    rejected: 3,
  });
}

function expectOwnedProcessEntry() {
  const ownedProcessEntry = entryForIntent('dispatch-owned-process-time-limit');

  expect(ownedProcessEntry.matrixRow.outcomeState).toBe('dispatch-ready');
  expect(ownedProcessEntry.intent.evidenceReferences[0]?.evidenceReferenceId).toBe(
    'evidence-app-session-owned-process'
  );
}

function expectAskParentEntry() {
  const askParentEntry = entryForIntent('dispatch-ask-parent-dry-run');

  expect(askParentEntry.intent.requestedParentAction).toBe('ask-parent');
  expect(askParentEntry.intent.requestedPolicyAction).toBe('ask-parent');
  expect(askParentEntry.intent.dryRun).toBe(true);
  expect(askParentEntry.matrixRow.outcomeState).toBe('dry-run-only');
  expect(askParentEntry.approvalState).toBe('pending');
}

function expectAppGameEntry() {
  const appGameEntry = entryForIntent('dispatch-app-game-session-handoff');

  expect(appGameEntry.timerState).toBe('restart-recovered');
}

function expectRejectedEntries() {
  const stalePolicyEntry = entryForIntent('dispatch-stale-policy-version-rejected');
  const missingSourceEntry = entryForIntent('dispatch-missing-source-rejected');

  expect(stalePolicyEntry.matrixRow.rejectionReason).toBe('stale-policy-version');
  expect(stalePolicyEntry.intent.sourceState).toBe('stale');
  expect(missingSourceEntry.matrixRow.rejectionReason).toBe('source-not-ready');
  expect(missingSourceEntry.intent.sourceState).toBe('missing');
}

function expectTamperEntry() {
  const tamperEntry = entryForIntent('dispatch-tamper-alert-scaffold');

  expect(tamperEntry.childReasonCode).toBe('child-reason-integrity-proof-required');
}
