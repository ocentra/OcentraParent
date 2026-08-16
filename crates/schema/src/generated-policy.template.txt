export const GeneratedPolicyActionValues = ['allow', 'warn', 'block', 'time-limit', 'ask-parent', 'unknown'] as const;

export type GeneratedPolicyAction = (typeof GeneratedPolicyActionValues)[number];

export const GeneratedPolicyActionStrictnessRank = {
  allow: 0,
  warn: 10,
  unknown: 20,
  'ask-parent': 30,
  'time-limit': 40,
  block: 50,
} as const satisfies Readonly<Record<GeneratedPolicyAction, number>>;

export function compareGeneratedPolicyActionStrictness(
  left: GeneratedPolicyAction,
  right: GeneratedPolicyAction
): number {
  return GeneratedPolicyActionStrictnessRank[left] - GeneratedPolicyActionStrictnessRank[right];
}

export function selectGeneratedStricterPolicyAction(
  parentRuleAction: GeneratedPolicyAction,
  localAiAction: GeneratedPolicyAction
): GeneratedPolicyAction {
  return compareGeneratedPolicyActionStrictness(parentRuleAction, localAiAction) >= 0
    ? parentRuleAction
    : localAiAction;
}
