/* thin adapter helpers for Rust-seeded screen control catalog metadata */

import { ScreenControlRuleIdSchema } from './screen-control-catalog-schema';
import type {
  ScreenControlCatalogEffectStatus,
  ScreenControlCatalogRule,
  ScreenControlCatalogSourceKind,
} from './screen-control-catalog-schema';
import { slugToken } from './catalog-metadata-text';

export function screenVisibilityConditionsFor(): ScreenControlCatalogRule[] {
  return [screenRule('Visible when the Screen side-panel category is selected.')];
}

export function screenEnabledConditionsFor(
  effectStatus: ScreenControlCatalogEffectStatus
): ScreenControlCatalogRule[] {
  return [
    screenRule('A family, child, or device target must be selected before writing Screen policy intent.'),
    screenRule(`Capability state must allow ${effectStatus} presentation.`),
  ];
}

export function screenValidationRulesFor(
  effectStatus: ScreenControlCatalogEffectStatus,
  proofRequirement: string | null
): ScreenControlCatalogRule[] {
  const rules = [
    screenRule('Selected option ids must belong to this setting acceptedOptions list.'),
    screenRule('Raw screen capture stays local-only by default and is deleted after queue processing or TTL expiry.'),
    screenRule(
      'Portal writes authoring intent only; child runtime owns capture, analysis, compile, policy handoff, and audit.'
    ),
  ];
  if (proofRequirement !== null) {
    rules.push(screenRule(`Strict behavior requires proof: ${proofRequirement}.`));
  }
  if (effectStatus === 'unavailable') {
    rules.push(screenRule('Unavailable states must fail closed and must not be promoted to enforcement support.'));
  }
  return rules;
}

export function screenSourceEffectStatus(
  sourceKind: ScreenControlCatalogSourceKind,
  sectionTitle: string,
  sourceText: string,
  effectStatusFor: (
    sourceKind: ScreenControlCatalogSourceKind,
    sectionTitle: string,
    sourceText: string
  ) => ScreenControlCatalogEffectStatus
): ScreenControlCatalogEffectStatus {
  return effectStatusFor(sourceKind, sectionTitle, sourceText);
}

function screenRule(description: string): ScreenControlCatalogRule {
  return {
    ruleId: ScreenControlRuleIdSchema.parse(`screen-catalog-rule-${slugToken(description, 'rule')}`),
    description,
  };
}
