import { describe, expect, it } from 'vitest';
import {
  BrowserAiRiskBenefitAssessmentSchema,
  BrowserAiRiskBenefitModelSchemaVersion,
  BrowserAiRiskBenefitTaxonomySchema,
} from '../src/browser-ai-risk-benefit-model-schemas';

describe('browser AI structured category risk benefit model contract', () => {
  it('accepts the structured taxonomy with unknown fallbacks', acceptsStructuredTaxonomy);
  it('accepts evidence-backed educational assessment with benefit and risk signals', acceptsEducationalAssessment);
  it('rejects platform labels used as authority or direct policy claims', rejectsAuthorityCreep);
  it('rejects adult category without matching adult risk signal', rejectsAdultCategoryWithoutRiskSignal);
  it('rejects low confidence without modifier and uncertainty evidence', rejectsHiddenLowConfidence);
  it('accepts high-risk categories without claiming enforcement', acceptsHighRiskCandidateOnlyAssessment);
  it('rejects unknown category with high confidence', rejectsHighConfidenceUnknownCategory);
});

function acceptsStructuredTaxonomy() {
  const parsed = BrowserAiRiskBenefitTaxonomySchema.safeParse(structuredTaxonomy());

  expect(parsed.success).toBe(true);
  if (parsed.success) {
    expect(parsed.data.categories.includes('unknown')).toBe(true);
    expect(parsed.data.benefitSignals.includes('unknown-benefit')).toBe(true);
    expect(parsed.data.riskSignals.includes('unknown-risk')).toBe(true);
  }
}

function acceptsEducationalAssessment() {
  const parsed = BrowserAiRiskBenefitAssessmentSchema.safeParse(educationalAssessment());

  expect(parsed.success).toBe(true);
  if (parsed.success) {
    expect(parsed.data.primaryCategory).toBe('education');
    expect(parsed.data.benefitSignals).toEqual(['educational-value', 'skill-building']);
    expect(parsed.data.finalPolicyActionClaimed).toBe(false);
  }
}

function rejectsAuthorityCreep() {
  const parsed = BrowserAiRiskBenefitAssessmentSchema.safeParse({
    ...educationalAssessment(),
    platformLabelUsedAsAuthority: true,
    finalPolicyActionClaimed: true,
    enforcementActionClaimed: true,
  });

  expect(parsed.success).toBe(false);
}

function rejectsAdultCategoryWithoutRiskSignal() {
  const parsed = BrowserAiRiskBenefitAssessmentSchema.safeParse({
    ...adultAssessment(),
    riskSignals: ['privacy-risk'],
  });

  expect(parsed.success).toBe(false);
}

function rejectsHiddenLowConfidence() {
  const parsed = BrowserAiRiskBenefitAssessmentSchema.safeParse({
    ...educationalAssessment(),
    confidence: 'low',
    contentModifiers: ['video'],
    uncertaintyReasons: [],
  });

  expect(parsed.success).toBe(false);
}

function acceptsHighRiskCandidateOnlyAssessment() {
  const parsed = BrowserAiRiskBenefitAssessmentSchema.safeParse(adultAssessment());

  expect(parsed.success).toBe(true);
  if (parsed.success) {
    expect(parsed.data.primaryCategory).toBe('adult');
    expect(parsed.data.riskSignals).toEqual(['adult', 'privacy-risk']);
    expect(parsed.data.enforcementActionClaimed).toBe(false);
  }
}

function rejectsHighConfidenceUnknownCategory() {
  const parsed = BrowserAiRiskBenefitAssessmentSchema.safeParse({
    ...educationalAssessment(),
    primaryCategory: 'unknown',
    benefitSignals: ['unknown-benefit'],
    riskSignals: ['unknown-risk'],
    confidence: 'high',
  });

  expect(parsed.success).toBe(false);
}

function structuredTaxonomy() {
  return {
    schemaVersion: BrowserAiRiskBenefitModelSchemaVersion,
    taxonomyVersionRef: 'browser-ai-risk-benefit-taxonomy-v1',
    publishedAt: '2026-06-03T03:40:00.000Z',
    categories: [
      'education',
      'homework',
      'research',
      'news',
      'entertainment',
      'gaming',
      'music',
      'social',
      'shopping',
      'communication',
      'adult',
      'violence',
      'self-harm',
      'drugs-alcohol',
      'gambling',
      'hate-harassment',
      'weapons',
      'misinformation',
      'unknown',
    ],
    modifiers: ['video', 'short-video', 'livestream', 'comments-heavy', 'low-confidence'],
    benefitSignals: ['educational-value', 'homework-relevance', 'skill-building', 'neutral', 'unknown-benefit'],
    riskSignals: ['adult', 'violence', 'self-harm', 'privacy-risk', 'unknown-risk'],
  };
}

function educationalAssessment() {
  return {
    schemaVersion: BrowserAiRiskBenefitModelSchemaVersion,
    assessmentId: 'browser-ai-risk-benefit-assessment-math-video',
    taxonomyVersionRef: 'browser-ai-risk-benefit-taxonomy-v1',
    assessedAt: '2026-06-03T03:41:00.000Z',
    sourceAnalysisId: 'browser-ai-analysis-result-youtube-video',
    sourceEvidenceIds: ['browser-evidence-youtube-video'],
    primaryCategory: 'education',
    secondaryCategories: ['homework'],
    contentModifiers: ['video', 'comments-heavy'],
    benefitSignals: ['educational-value', 'skill-building'],
    riskSignals: ['privacy-risk'],
    confidence: 'medium',
    uncertaintyReasons: [],
    sourceSupport: 'evidence-backed',
    platformLabelUsedAsAuthority: false,
    finalPolicyActionClaimed: false,
    enforcementActionClaimed: false,
  };
}

function adultAssessment() {
  return {
    ...educationalAssessment(),
    assessmentId: 'browser-ai-risk-benefit-assessment-adult-video',
    primaryCategory: 'adult',
    secondaryCategories: ['unknown'],
    contentModifiers: ['video', 'platform-restricted'],
    benefitSignals: ['neutral'],
    riskSignals: ['adult', 'privacy-risk'],
    confidence: 'high',
  };
}
