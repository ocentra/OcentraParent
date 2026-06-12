import { describe, expect, it } from 'vitest';
import {
  EnforcementPolicyDispatchCapabilityMatrixRowSchema,
  EnforcementPolicyDispatchIntentSchema,
  EnforcementPolicyDispatchReadModel,
  EnforcementPolicyDispatchReadModelSchema,
} from '../../src/enforcement-policy-dispatch';

describe('enforcement policy dispatch contracts', () => {
  it('keeps the V0.8 dispatch proof matrix schema-backed and parent-visible', () => {
    const parsed = EnforcementPolicyDispatchReadModelSchema.parse(EnforcementPolicyDispatchReadModel);

    expect(parsed.readModelId).toBe('v0-8-enforcement-policy-dispatch');
    expect(parsed.entries.map((entry) => entry.matrixRow.proofLevel)).toEqual([
      'implemented',
      'implemented',
      'report-only',
      'manual-required',
      'scaffold',
    ]);
    expect(parsed.entries[0]?.matrixRow.outcomeState).toBe('dispatch-ready');
    expect(parsed.entries[0]?.intent.evidenceReferences[0]?.evidenceReferenceId).toBe(
      'evidence-app-session-owned-process'
    );
    expect(parsed.entries[1]?.timerState).toBe('restart-recovered');
    expect(parsed.entries[2]?.intent.dryRun).toBe(true);
    expect(parsed.entries[3]?.matrixRow.rejectionReason).toBe('adapter-manual-required');
    expect(parsed.entries[4]?.childReasonCode).toBe('child-reason-integrity-proof-required');
  });

  it('rejects dispatch intents without evidence references', () => {
    const validIntent = EnforcementPolicyDispatchReadModel.entries[0]!.intent;
    const parsed = EnforcementPolicyDispatchIntentSchema.safeParse({
      ...validIntent,
      evidenceReferences: [],
    });

    expect(parsed.success).toBe(false);
  });

  it('rejects accidental implemented claim upgrades without dispatch-ready support', () => {
    const validMatrixRow = EnforcementPolicyDispatchReadModel.entries[0]!.matrixRow;
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
    const validEntry = EnforcementPolicyDispatchReadModel.entries[0]!;
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
