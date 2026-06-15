import { describe, expect, it } from 'vitest';
import {
  appGameAiClassifierForbiddenOutputKeyPaths,
  AppGameAiClassifierResultSchema,
  parseAppGameAiClassifierResult,
  safeParseAppGameAiClassifierResult,
} from '../../src/app-game-ai-classifier-boundary';
import { AppGameAiClassifierBoundaryProofMatrix } from '../../src/app-game-ai-classifier-boundary-data';
import {
  AppGameAiClassifierFallbackState,
  AppGameAiClassifierPolicyHandoff,
  AppGameAiClassifierState,
} from '../../src/app-game-ai-classifier-boundary-values';

const firstResult = AppGameAiClassifierBoundaryProofMatrix.results[0];

describe('app game AI classifier digest boundary', () => {
  it('accepts app and game classifier candidates that cite stored evidence, runtime, and prompt refs', () => {
    const parsed = AppGameAiClassifierBoundaryProofMatrix.results.map((result) =>
      parseAppGameAiClassifierResult(result)
    );

    expect(parsed).toHaveLength(3);
    expect(parsed.map((result) => result.productKind)).toEqual(['unknownApp', 'unknownGame', 'nativeApp']);
    expect(parsed.every((result) => result.sourceEvidenceRefs.length > 0)).toBe(true);
    expect(parsed.every((result) => result.modelRuntimeRef.length > 0 && result.promptVersion.length > 0)).toBe(true);
  });

  it('rejects missing evidence refs and confidence outside the bounded range', () => {
    expect(AppGameAiClassifierResultSchema.safeParse({ ...firstResult, sourceEvidenceRefs: [] }).success).toBe(false);
    expect(AppGameAiClassifierResultSchema.safeParse({ ...firstResult, confidence: 1.1 }).success).toBe(false);
    expect(AppGameAiClassifierResultSchema.safeParse({ ...firstResult, confidence: -0.1 }).success).toBe(false);
  });

  it('rejects direct action authority in classifier output', () => {
    expect(
      AppGameAiClassifierResultSchema.safeParse({
        ...firstResult,
        directActionRequested: true,
      }).success
    ).toBe(false);
    expect(
      AppGameAiClassifierResultSchema.safeParse({
        ...firstResult,
        policyHandoff: AppGameAiClassifierPolicyHandoff.None,
      }).success
    ).toBe(false);
  });

  it('rejects forbidden action, duration, and raw scan fields before policy sees them', () => {
    const forbiddenOutput = {
      ...firstResult,
      durationMs: 60000,
      rawOsScanResult: { processScanRows: ['private-process.exe'] },
      modelDecision: { block: true },
    };

    expect(safeParseAppGameAiClassifierResult(forbiddenOutput).success).toBe(false);
    expect(appGameAiClassifierForbiddenOutputKeyPaths(forbiddenOutput)).toEqual([
      'durationMs',
      'rawOsScanResult',
      'rawOsScanResult.processScanRows',
      'modelDecision.block',
    ]);
  });

  it('requires provider unavailable results to carry a fallback state', () => {
    const invalidUnavailable = AppGameAiClassifierResultSchema.safeParse({
      ...firstResult,
      classifierState: AppGameAiClassifierState.ProviderUnavailable,
      fallbackState: AppGameAiClassifierFallbackState.NotNeeded,
      confidence: 0,
    });
    const validUnavailable = AppGameAiClassifierResultSchema.safeParse(
      AppGameAiClassifierBoundaryProofMatrix.results[2]
    );

    expect(invalidUnavailable.success).toBe(false);
    expect(validUnavailable.success).toBe(true);
  });

  it('requires low confidence fallback to stay low confidence', () => {
    expect(
      AppGameAiClassifierResultSchema.safeParse({
        ...firstResult,
        fallbackState: AppGameAiClassifierFallbackState.LowConfidence,
        confidence: 0.72,
      }).success
    ).toBe(false);
  });
});
