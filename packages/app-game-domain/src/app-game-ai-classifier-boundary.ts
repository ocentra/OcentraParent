import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  AppGameAiClassifierCandidateKind,
  AppGameAiClassifierFallbackState,
  AppGameAiClassifierForbiddenOutputKeys,
  AppGameAiClassifierPolicyHandoff,
  AppGameAiClassifierProductKind,
  AppGameAiClassifierSchemaVersion,
  AppGameAiClassifierSourceDigestKind,
  AppGameAiClassifierState,
} from './app-game-ai-classifier-boundary-values';

const NonEmptyAiClassifierText = Schema.String.pipe(Schema.minLength(1));

export const AppGameAiClassifierRunIdSchema = NonEmptyAiClassifierText.pipe(Schema.brand('AppGameAiClassifierRunId'));
export const AppGameAiClassifierDigestRefSchema = NonEmptyAiClassifierText.pipe(
  Schema.brand('AppGameAiClassifierDigestRef')
);
export const AppGameAiClassifierEvidenceRefSchema = NonEmptyAiClassifierText.pipe(
  Schema.brand('AppGameAiClassifierEvidenceRef')
);
export const AppGameAiClassifierSessionRefSchema = NonEmptyAiClassifierText.pipe(
  Schema.brand('AppGameAiClassifierSessionRef')
);
export const AppGameAiClassifierRuntimeRefSchema = NonEmptyAiClassifierText.pipe(
  Schema.brand('AppGameAiClassifierRuntimeRef')
);
export const AppGameAiClassifierPromptRefSchema = NonEmptyAiClassifierText.pipe(
  Schema.brand('AppGameAiClassifierPromptRef')
);
export const AppGameAiClassifierLabelSchema = NonEmptyAiClassifierText.pipe(Schema.brand('AppGameAiClassifierLabel'));
export const AppGameAiClassifierReasonCodeSchema = NonEmptyAiClassifierText.pipe(
  Schema.brand('AppGameAiClassifierReasonCode')
);
export const AppGameAiClassifierTimestampSchema = NonEmptyAiClassifierText.pipe(
  Schema.brand('AppGameAiClassifierTimestamp')
);

const AppGameAiClassifierProductKindSchema = withParser(
  Schema.Literal(...Object.values(AppGameAiClassifierProductKind))
);
const AppGameAiClassifierSourceDigestKindSchema = withParser(
  Schema.Literal(...Object.values(AppGameAiClassifierSourceDigestKind))
);
const AppGameAiClassifierCandidateKindSchema = withParser(
  Schema.Literal(...Object.values(AppGameAiClassifierCandidateKind))
);
const AppGameAiClassifierStateSchema = withParser(Schema.Literal(...Object.values(AppGameAiClassifierState)));
const AppGameAiClassifierPolicyHandoffSchema = withParser(
  Schema.Literal(...Object.values(AppGameAiClassifierPolicyHandoff))
);
const AppGameAiClassifierFallbackStateSchema = withParser(
  Schema.Literal(...Object.values(AppGameAiClassifierFallbackState))
);
const AppGameAiClassifierConfidenceSchema = withParser(Schema.Number.pipe(Schema.between(0, 1)));
const AppGameAiClassifierEvidenceRefsSchema = Schema.Array(AppGameAiClassifierEvidenceRefSchema).pipe(
  Schema.filter((refs) => refs.length > 0 || 'Expected AI classifier output to cite stored evidence refs')
);

const AppGameAiClassifierResultBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(AppGameAiClassifierSchemaVersion),
  classifierRunId: AppGameAiClassifierRunIdSchema,
  productKind: AppGameAiClassifierProductKindSchema,
  digestRef: AppGameAiClassifierDigestRefSchema,
  sourceDigestKind: AppGameAiClassifierSourceDigestKindSchema,
  sourceEvidenceRefs: AppGameAiClassifierEvidenceRefsSchema,
  sourceSessionRefs: Schema.Array(AppGameAiClassifierSessionRefSchema),
  candidateKind: AppGameAiClassifierCandidateKindSchema,
  candidateLabel: AppGameAiClassifierLabelSchema,
  classifierState: AppGameAiClassifierStateSchema,
  confidence: AppGameAiClassifierConfidenceSchema,
  uncertaintyReasonCodes: Schema.Array(AppGameAiClassifierReasonCodeSchema),
  modelRuntimeRef: AppGameAiClassifierRuntimeRefSchema,
  promptTemplateRef: AppGameAiClassifierPromptRefSchema,
  promptVersion: AppGameAiClassifierPromptRefSchema,
  fallbackState: AppGameAiClassifierFallbackStateSchema,
  policyHandoff: AppGameAiClassifierPolicyHandoffSchema,
  generatedAt: AppGameAiClassifierTimestampSchema,
  directActionRequested: Schema.Literal(false),
  rawScanIncluded: Schema.Literal(false),
  contentClaimIncluded: Schema.Literal(false),
});

export const AppGameAiClassifierResultSchema = withParser(
  AppGameAiClassifierResultBaseSchema.pipe(
    Schema.filter(
      (result) =>
        appGameAiClassifierPolicyHandoffIsEvidenceOnly(result) ||
        'Expected AI classifier output to remain evidence-only for policy handoff'
    )
  )
    .pipe(
      Schema.filter(
        (result) =>
          result.classifierState !== AppGameAiClassifierState.ProviderUnavailable ||
          result.fallbackState !== AppGameAiClassifierFallbackState.NotNeeded ||
          'Expected provider-unavailable classifier output to name a fallback state'
      )
    )
    .pipe(
      Schema.filter(
        (result) =>
          result.fallbackState !== AppGameAiClassifierFallbackState.LowConfidence ||
          result.confidence < 0.5 ||
          'Expected low-confidence fallback to keep confidence below 0.5'
      )
    )
);

export type AppGameAiClassifierResult = Infer<typeof AppGameAiClassifierResultSchema>;

export function appGameAiClassifierPolicyHandoffIsEvidenceOnly(
  result: Infer<typeof AppGameAiClassifierResultBaseSchema>
): boolean {
  return (
    result.directActionRequested === false &&
    result.rawScanIncluded === false &&
    result.contentClaimIncluded === false &&
    result.policyHandoff !== AppGameAiClassifierPolicyHandoff.None
  );
}

export function parseAppGameAiClassifierResult(input: unknown): AppGameAiClassifierResult {
  const forbiddenKeys = appGameAiClassifierForbiddenOutputKeyPaths(input);
  if (forbiddenKeys.length > 0) {
    throw new Error(`Forbidden AI classifier output keys: ${forbiddenKeys.join(',')}`);
  }

  return AppGameAiClassifierResultSchema.parse(input);
}

export function safeParseAppGameAiClassifierResult(input: unknown) {
  try {
    return { success: true as const, data: parseAppGameAiClassifierResult(input) };
  } catch (error) {
    return { success: false as const, error };
  }
}

export function appGameAiClassifierForbiddenOutputKeyPaths(input: unknown): readonly string[] {
  return collectForbiddenOutputKeyPaths(input, []);
}

function collectForbiddenOutputKeyPaths(value: unknown, path: readonly PropertyKey[]): readonly string[] {
  if (!appGameAiClassifierIsRecord(value)) {
    return [];
  }

  return Object.entries(value).flatMap(([key, nested]) => {
    const nextPath = [...path, key];
    const currentPath = AppGameAiClassifierForbiddenOutputKeys.includes(
      key as (typeof AppGameAiClassifierForbiddenOutputKeys)[number]
    )
      ? [nextPath.join('.')]
      : [];

    return [...currentPath, ...collectForbiddenOutputKeyPaths(nested, nextPath)];
  });
}

function appGameAiClassifierIsRecord(value: unknown): value is Record<PropertyKey, unknown> {
  return typeof value === 'object' && value !== null;
}
