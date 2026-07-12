import type {
  BrowserPolicyAnswerMap,
  BrowserPolicyQuestionId,
} from '@ocentra-parent/schema-domain/browser-policy-questionnaire-forest-contract';

export function browserPolicyRootAnswer(answers: BrowserPolicyAnswerMap): string {
  return answers['1.1']?.[0] ?? 'off';
}

export function browserPolicyHas(
  answers: BrowserPolicyAnswerMap,
  questionId: BrowserPolicyQuestionId,
  optionId: string
): boolean {
  return (answers[questionId] ?? []).includes(optionId);
}

export function browserPolicyHasAny(
  answers: BrowserPolicyAnswerMap,
  questionId: BrowserPolicyQuestionId,
  optionIds: readonly string[]
): boolean {
  return optionIds.some((optionId) => browserPolicyHas(answers, questionId, optionId));
}

export function browserPolicyCount(answers: BrowserPolicyAnswerMap, questionId: BrowserPolicyQuestionId): number {
  return answers[questionId]?.length ?? 0;
}

export function browserPolicyEmergencyOverrideActive(answers: BrowserPolicyAnswerMap): boolean {
  return browserPolicyHasAnyRoot(answers, ['emergency-allow', 'emergency-block']);
}

export function browserPolicyHasAnyRoot(answers: BrowserPolicyAnswerMap, rootValues: readonly string[]): boolean {
  return rootValues.includes(browserPolicyRootAnswer(answers));
}

export function browserPolicyAskParentExists(answers: BrowserPolicyAnswerMap): boolean {
  return (
    browserPolicyHasAnyAskParentTrigger(answers) ||
    browserPolicyHasAny(answers, '10.2', ['allow-once', 'custom-window']) ||
    browserPolicyHas(answers, '11.2', 'extension') ||
    browserPolicyRootAnswer(answers) === 'emergency-block'
  );
}

function browserPolicyHasAnyAskParentTrigger(answers: BrowserPolicyAnswerMap): boolean {
  return (
    browserPolicyHasAny(answers, '1.2', ['parent-review']) ||
    browserPolicyHasAny(answers, '2.3', ['parent-review']) ||
    browserPolicyHasAny(answers, '4.1', ['parent-review']) ||
    browserPolicyHasAny(answers, '5.2', ['parent-review']) ||
    browserPolicyHasAny(answers, '5.3', ['parent-review']) ||
    browserPolicyHasAny(answers, '6.1', ['parent-review']) ||
    browserPolicyHasAny(answers, '7.1', ['parent-review']) ||
    browserPolicyHasAny(answers, '8.1', ['parent-review']) ||
    browserPolicyHasAny(answers, '9.1', ['parent-review']) ||
    browserPolicyHasAny(answers, '9.3', ['parent-review']) ||
    browserPolicyHasAny(answers, '15.2', ['parent-review'])
  );
}

export function browserPolicyExactEvidenceSelected(answers: BrowserPolicyAnswerMap): boolean {
  return (
    browserPolicyHasAny(answers, '5.1', ['exact-url', 'search-terms']) ||
    browserPolicyHasAny(answers, '13.1', ['exact-url', 'search-term']) ||
    browserPolicyHas(answers, '3.1', 'managed-exact')
  );
}

export function browserPolicyManagedBrowserRequired(answers: BrowserPolicyAnswerMap): boolean {
  return (
    browserPolicyHasAny(answers, '3.1', ['managed-exact', 'managed-all']) ||
    browserPolicyHas(answers, '6.1', 'require-managed')
  );
}

export function browserPolicyBrowserGamesRelevant(answers: BrowserPolicyAnswerMap): boolean {
  return browserPolicyRootAnswer(answers) === 'on' && browserPolicyHas(answers, '5.1', 'browser-games');
}
