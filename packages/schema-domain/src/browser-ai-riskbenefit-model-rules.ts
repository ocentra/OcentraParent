import type {
  BrowserAiStructuredContentCategory,
  BrowserAiStructuredRiskSignal,
} from '@ocentra-parent/schema-domain/browser-ai-riskbenefit-model-values';

type BrowserAiRiskBenefitTaxonomyShape = {
  readonly categories: readonly string[];
  readonly benefitSignals: readonly string[];
  readonly riskSignals: readonly string[];
};

type BrowserAiRiskBenefitAssessmentShape = {
  readonly primaryCategory: BrowserAiStructuredContentCategory;
  readonly benefitSignals: readonly string[];
  readonly riskSignals: readonly BrowserAiStructuredRiskSignal[];
  readonly contentModifiers: readonly string[];
  readonly uncertaintyReasons: readonly string[];
  readonly confidence: string;
  readonly sourceSupport: string;
  readonly platformLabelUsedAsAuthority: boolean;
  readonly finalPolicyActionClaimed: boolean;
  readonly enforcementActionClaimed: boolean;
};

const EducationalCategories = new Set<BrowserAiStructuredContentCategory>(['education', 'homework', 'research']);

const RiskSignalByCategory = {
  adult: 'adult',
  violence: 'violence',
  'self-harm': 'self-harm',
  'drugs-alcohol': 'drugs-alcohol',
  gambling: 'gambling',
  'hate-harassment': 'hate-harassment',
  misinformation: 'misinformation',
} as const satisfies Partial<Record<BrowserAiStructuredContentCategory, BrowserAiStructuredRiskSignal>>;

export function browserAiRiskBenefitTaxonomyIsConsistent(value: BrowserAiRiskBenefitTaxonomyShape) {
  return (
    value.categories.includes('unknown') &&
    value.benefitSignals.includes('unknown-benefit') &&
    value.riskSignals.includes('unknown-risk')
  );
}

export function browserAiRiskBenefitAssessmentIsConsistent(value: BrowserAiRiskBenefitAssessmentShape) {
  return (
    !claimsAuthority(value) &&
    lowConfidenceStateIsExplicit(value) &&
    primaryCategoryHasMatchingEvidenceSignal(value) &&
    (value.sourceSupport !== 'platform-label-only' || value.confidence !== 'high')
  );
}

function claimsAuthority(value: BrowserAiRiskBenefitAssessmentShape) {
  return value.platformLabelUsedAsAuthority || value.finalPolicyActionClaimed || value.enforcementActionClaimed;
}

function lowConfidenceStateIsExplicit(value: BrowserAiRiskBenefitAssessmentShape) {
  return !['low', 'unknown'].includes(value.confidence) ||
    (value.contentModifiers.includes('low-confidence') && value.uncertaintyReasons.length > 0);
}

function primaryCategoryHasMatchingEvidenceSignal(value: BrowserAiRiskBenefitAssessmentShape) {
  if (value.primaryCategory === 'unknown') {
    return value.confidence !== 'high' && value.riskSignals.includes('unknown-risk');
  }
  if (EducationalCategories.has(value.primaryCategory)) {
    return value.benefitSignals.some((signal) => signal !== 'neutral' && signal !== 'unknown-benefit');
  }
  const requiredRiskSignal = RiskSignalByCategory[value.primaryCategory];
  return requiredRiskSignal === undefined || value.riskSignals.includes(requiredRiskSignal);
}
