import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentEvidenceReferenceIdSchema } from './reference-primitives';

const NonEmptyBrowserGameSignalText = Schema.String.pipe(Schema.minLength(1));

export const BrowserGameRiskBenefitSignalSchemaVersionSchema = withParser(
  Schema.Literal('browser-game-risk-benefit-signal-contract')
);

export const BrowserGameRiskBenefitSignalSetIdSchema = withParser(
  NonEmptyBrowserGameSignalText.pipe(Schema.brand('BrowserGameRiskBenefitSignalSetId'))
);

export const BrowserGameRiskSignalIdSchema = withParser(
  NonEmptyBrowserGameSignalText.pipe(Schema.brand('BrowserGameRiskSignalId'))
);

export const BrowserGameBenefitSignalIdSchema = withParser(
  NonEmptyBrowserGameSignalText.pipe(Schema.brand('BrowserGameBenefitSignalId'))
);

export const BrowserGameRiskSignalKindSchema = withParser(
  Schema.Literal(
    'violence',
    'horror',
    'adult-theme',
    'addiction-loop',
    'multiplayer-contact',
    'chat-risk',
    'purchase-risk',
    'loot-box-risk',
    'user-generated-content-risk',
    'privacy-risk',
    'unblocked-bypass-risk',
    'unknown-risk'
  )
);

export const BrowserGameBenefitSignalKindSchema = withParser(
  Schema.Literal(
    'educational-value',
    'homework-relevance',
    'skill-building',
    'creativity',
    'problem-solving',
    'parent-approved-game',
    'neutral-benefit',
    'unknown-benefit'
  )
);

export const BrowserGameSignalSeveritySchema = withParser(Schema.Literal('none', 'low', 'medium', 'high', 'unknown'));
export const BrowserGameSignalStateSchema = withParser(Schema.Literal('candidate', 'manual-required', 'unavailable'));
export const BrowserGameSignalConfidenceSchema = withParser(Schema.Literal('high', 'medium', 'low', 'unknown'));

export const BrowserGameSignalSourceKindSchema = withParser(
  Schema.Literal('game-ai-analysis', 'game-metadata', 'parent-rule', 'manual-required')
);

export const BrowserGameRecommendedPolicyInputSchema = withParser(
  Schema.Literal(
    'allow-candidate',
    'warn-candidate',
    'ask-parent-candidate',
    'block-candidate',
    'time-limit-candidate',
    'manual-review-candidate',
    'unknown-candidate'
  )
);

export const BrowserGameSignalSetDegradedStateSchema = withParser(
  Schema.Literal('none', 'degraded', 'manual-required', 'unavailable')
);

export const BrowserGameSignalUncertaintyReasonSchema = withParser(
  Schema.Literal('low-confidence', 'missing-analysis', 'missing-metadata', 'conflicting-evidence', 'manual-required')
);

export const BrowserGameSignalEvidenceRefsSchema = Schema.Array(ParentEvidenceReferenceIdSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected browser-game signal evidence refs')
);

export type BrowserGameBenefitSignalKind = Infer<typeof BrowserGameBenefitSignalKindSchema>;
export type BrowserGameRiskSignalKind = Infer<typeof BrowserGameRiskSignalKindSchema>;
export type BrowserGameSignalConfidence = Infer<typeof BrowserGameSignalConfidenceSchema>;
export type BrowserGameSignalSetDegradedState = Infer<typeof BrowserGameSignalSetDegradedStateSchema>;
export type BrowserGameSignalSeverity = Infer<typeof BrowserGameSignalSeveritySchema>;
export type BrowserGameSignalState = Infer<typeof BrowserGameSignalStateSchema>;
