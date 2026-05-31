import { browserControlFullCatalogSettings } from './browser-control-full-catalog';
import {
  BrowserPolicyQuestionIds,
  type BrowserPolicyAnswerMap,
  type BrowserPolicyComputedFlagId,
  type BrowserPolicyCondition,
  type BrowserPolicyQuestion,
  type BrowserPolicyQuestionId,
  type BrowserPolicyQuestionState,
  type BrowserPolicySurface,
} from './browser-policy-questionnaire-forest-contract';
import {
  BrowserPolicyQuestionnaireCompactOrder,
  BrowserPolicyQuestions,
} from './browser-policy-questionnaire-forest-data';

export {
  BrowserPolicyDefaultAnswers,
  BrowserPolicyQuestionIds,
  type BrowserPolicyAnswerMap,
  type BrowserPolicyComputedFlagId,
  type BrowserPolicyCondition,
  type BrowserPolicyOption,
  type BrowserPolicyQuestion,
  type BrowserPolicyQuestionId,
  type BrowserPolicyQuestionState,
  type BrowserPolicySelectionMode,
  type BrowserPolicySurface,
} from './browser-policy-questionnaire-forest-contract';
export {
  BrowserPolicyQuestionnaireCompactOrder,
  BrowserPolicyQuestions,
} from './browser-policy-questionnaire-forest-data';

export const BrowserPolicyQuestionnaireForest = {
  questions: BrowserPolicyQuestions,
  compactOrder: BrowserPolicyQuestionnaireCompactOrder,
} as const;

export function browserPolicyQuestionById(id: BrowserPolicyQuestionId): BrowserPolicyQuestion {
  const question = BrowserPolicyQuestions.find((candidate) => candidate.id === id);
  if (question === undefined) {
    throw new Error(`Unknown browser policy question ${id}`);
  }
  return question;
}

export function browserPolicyQuestionStates(
  answers: BrowserPolicyAnswerMap,
  questions: readonly BrowserPolicyQuestion[] = BrowserPolicyQuestions
): BrowserPolicyQuestionState[] {
  return questions.map((question) => browserPolicyQuestionState(question, answers));
}

export function browserPolicyVisibleQuestions(
  answers: BrowserPolicyAnswerMap,
  surface: BrowserPolicySurface = 'rules'
): BrowserPolicyQuestion[] {
  const questionById = new Map(BrowserPolicyQuestions.map((question) => [question.id, question]));
  const orderedIds =
    surface === 'ai'
      ? BrowserPolicyQuestionIds.filter((id) => id.startsWith('A'))
      : BrowserPolicyQuestionnaireCompactOrder;
  return orderedIds
    .map((id) => questionById.get(id))
    .filter((question): question is BrowserPolicyQuestion => question !== undefined)
    .filter((question) => question.surface === surface)
    .filter((question) => browserPolicyQuestionState(question, answers).visible);
}

export function browserPolicyQuestionState(
  question: BrowserPolicyQuestion,
  answers: BrowserPolicyAnswerMap
): BrowserPolicyQuestionState {
  const neverShow = conditionListMatches(question.neverShowWhen, answers);
  const show = (question.showWhen ?? []).length === 0 || conditionsMatch(question.showWhen ?? [], answers);
  const visible = show && !neverShow;
  return {
    question,
    visible,
    disabled: visible && conditionListMatches(question.disabledWhen, answers),
    readonly: visible && conditionListMatches(question.readonlyWhen, answers),
  };
}

export function browserPolicyConditionMatches(
  condition: BrowserPolicyCondition,
  answers: BrowserPolicyAnswerMap
): boolean {
  const selected = (questionId: BrowserPolicyQuestionId) => answers[questionId] ?? [];
  switch (condition.kind) {
    case 'answer-equals':
      return selected(condition.questionId)[0] === condition.optionId;
    case 'answer-includes':
      return selected(condition.questionId).includes(condition.optionId);
    case 'answer-includes-any':
      return condition.optionIds.some((optionId) => selected(condition.questionId).includes(optionId));
    case 'answer-has-any-selected':
      return selected(condition.questionId).length > 0;
    case 'computed-flag':
      return browserPolicyComputedFlag(condition.flagId, answers);
    case 'all':
      return conditionsMatch(condition.conditions, answers);
    case 'any':
      return condition.conditions.some((candidate) => browserPolicyConditionMatches(candidate, answers));
    case 'not':
      return !browserPolicyConditionMatches(condition.condition, answers);
  }
}

export function browserPolicyComputedFlag(
  flagId: BrowserPolicyComputedFlagId,
  answers: BrowserPolicyAnswerMap
): boolean {
  return BrowserPolicyComputedFlagEvaluators[flagId](answers);
}

function browserPolicyRootAnswer(answers: BrowserPolicyAnswerMap): string {
  return answers['1.1']?.[0] ?? 'off';
}

function browserPolicyHas(
  answers: BrowserPolicyAnswerMap,
  questionId: BrowserPolicyQuestionId,
  optionId: string
): boolean {
  return (answers[questionId] ?? []).includes(optionId);
}

function browserPolicyHasAny(
  answers: BrowserPolicyAnswerMap,
  questionId: BrowserPolicyQuestionId,
  optionIds: readonly string[]
): boolean {
  return optionIds.some((optionId) => browserPolicyHas(answers, questionId, optionId));
}

function browserPolicyCount(answers: BrowserPolicyAnswerMap, questionId: BrowserPolicyQuestionId): number {
  return answers[questionId]?.length ?? 0;
}

function browserPolicyEmergencyOverrideActive(answers: BrowserPolicyAnswerMap): boolean {
  return browserPolicyHasAnyRoot(answers, ['emergency-allow', 'emergency-block']);
}

function browserPolicyHasAnyRoot(answers: BrowserPolicyAnswerMap, rootValues: readonly string[]): boolean {
  return rootValues.includes(browserPolicyRootAnswer(answers));
}

function browserPolicyAskParentExists(answers: BrowserPolicyAnswerMap): boolean {
  return (
    browserPolicyHasAnyAskParentTrigger(answers) ||
    browserPolicyHasAny(answers, '10.2', ['allow-once', 'custom-window']) ||
    browserPolicyHas(answers, '11.2', 'extension') ||
    browserPolicyRootAnswer(answers) === 'emergency-block'
  );
}

function browserPolicyHasAnyAskParentTrigger(answers: BrowserPolicyAnswerMap): boolean {
  return (
    browserPolicyHasAny(answers, '1.2', ['ask-parent']) ||
    browserPolicyHasAny(answers, '2.3', ['ask-parent']) ||
    browserPolicyHasAny(answers, '4.1', ['ask-parent']) ||
    browserPolicyHasAny(answers, '5.2', ['ask-parent']) ||
    browserPolicyHasAny(answers, '5.3', ['ask-parent']) ||
    browserPolicyHasAny(answers, '6.1', ['ask-parent']) ||
    browserPolicyHasAny(answers, '7.1', ['ask-parent']) ||
    browserPolicyHasAny(answers, '8.1', ['ask-parent']) ||
    browserPolicyHasAny(answers, '9.1', ['ask-parent']) ||
    browserPolicyHasAny(answers, '9.3', ['ask-parent']) ||
    browserPolicyHasAny(answers, '15.2', ['ask-parent'])
  );
}

function browserPolicyExactEvidenceSelected(answers: BrowserPolicyAnswerMap): boolean {
  return (
    browserPolicyHasAny(answers, '5.1', ['exact-url', 'search-terms']) ||
    browserPolicyHasAny(answers, '13.1', ['exact-url', 'search-term']) ||
    browserPolicyHas(answers, '3.1', 'managed-exact')
  );
}

function browserPolicyManagedBrowserRequired(answers: BrowserPolicyAnswerMap): boolean {
  return (
    browserPolicyHasAny(answers, '3.1', ['managed-exact', 'managed-all']) ||
    browserPolicyHas(answers, '6.1', 'require-managed')
  );
}

function browserPolicySetupRelevant(answers: BrowserPolicyAnswerMap): boolean {
  return (
    browserPolicyHasAny(answers, '3.1', ['prefer-managed', 'managed-exact', 'managed-all']) ||
    browserPolicyHasAny(answers, '2.2', ['standard', 'strict', 'custom']) ||
    browserPolicyHasAny(answers, '2.3', ['ask-parent', 'block-until-approved']) ||
    browserPolicyComputedFlag('unsupportedCapabilityRelevant', answers)
  );
}

function browserPolicyEvidencePrivacyVisible(answers: BrowserPolicyAnswerMap): boolean {
  return browserPolicyRootAnswer(answers) === 'on' && browserPolicyHasAnyEvidenceTrigger(answers);
}

function browserPolicyHasAnyEvidenceTrigger(answers: BrowserPolicyAnswerMap): boolean {
  return (
    browserPolicyCount(answers, '2.1') > 0 ||
    browserPolicyCount(answers, '5.1') > 0 ||
    browserPolicyCount(answers, '7.1') > 0 ||
    browserPolicyCount(answers, '8.1') > 0 ||
    browserPolicyCount(answers, '9.1') > 0 ||
    browserPolicyComputedFlag('reportsEnabled', answers)
  );
}

function browserPolicyNotificationEventsRelevant(answers: BrowserPolicyAnswerMap): boolean {
  return browserPolicyRootAnswer(answers) !== 'off' && browserPolicyHasAnyNotificationTrigger(answers);
}

function browserPolicyHasAnyNotificationTrigger(answers: BrowserPolicyAnswerMap): boolean {
  return (
    browserPolicyHasAny(answers, '2.3', ['notify-parent', 'ask-parent', 'block-until-approved']) ||
    browserPolicyHasAny(answers, '4.1', [
      'warn',
      'notify-parent',
      'ask-parent',
      'close',
      'close-open-managed',
      'block-launch',
    ]) ||
    browserPolicyHasAny(answers, '6.1', ['warn', 'ask-parent', 'block', 'close-browser']) ||
    browserPolicyHasAny(answers, '9.1', ['notify-parent', 'ask-parent', 'block-risky', 'block-all-approved']) ||
    browserPolicyComputedFlag('limitExists', answers) ||
    browserPolicyHasAnyRoot(answers, ['paused', 'emergency-allow', 'emergency-block'])
  );
}

function browserPolicyUnsupportedCapabilityRelevant(answers: BrowserPolicyAnswerMap): boolean {
  return (
    browserPolicyHasAny(answers, '2.1', ['strict', 'custom']) ||
    browserPolicyComputedFlag('managedBrowserRequired', answers) ||
    browserPolicyHasAny(answers, '5.1', ['exact-url', 'search-terms', 'downloads', 'video']) ||
    browserPolicyHasAny(answers, '6.1', ['block', 'close-browser', 'require-managed'])
  );
}

function browserPolicyStoredBrowserDataExists(answers: BrowserPolicyAnswerMap): boolean {
  return (
    browserPolicyComputedFlag('reportsEnabled', answers) ||
    browserPolicyComputedFlag('auditEnabled', answers) ||
    browserPolicyCount(answers, '13.1') > 0 ||
    browserPolicyCount(answers, '12.1') > 0
  );
}

const BrowserPolicyComputedFlagEvaluators: Record<
  BrowserPolicyComputedFlagId,
  (answers: BrowserPolicyAnswerMap) => boolean
> = {
  policyIsOff: (answers) => browserPolicyRootAnswer(answers) === 'off',
  policyIsOn: (answers) => browserPolicyRootAnswer(answers) === 'on',
  policyPaused: (answers) => browserPolicyRootAnswer(answers) === 'paused',
  emergencyOverrideActive: browserPolicyEmergencyOverrideActive,
  askParentExists: browserPolicyAskParentExists,
  limitExists: (answers) =>
    browserPolicyHas(answers, '1.2', 'limit') ||
    browserPolicyHas(answers, '6.1', 'limit-time') ||
    browserPolicyHas(answers, '8.1', 'limit'),
  downloadsSelected: (answers) => browserPolicyHas(answers, '5.1', 'downloads'),
  searchSelected: (answers) => browserPolicyHasAny(answers, '5.1', ['search-terms', 'safe-search']),
  videoSelected: (answers) => browserPolicyHas(answers, '5.1', 'video'),
  exactEvidenceSelected: browserPolicyExactEvidenceSelected,
  managedBrowserRequired: browserPolicyManagedBrowserRequired,
  reportsEnabled: (answers) => (answers['14.1'] ?? []).some((optionId) => optionId !== 'policy-status'),
  auditEnabled: (answers) => browserPolicyHasAny(answers, '18.1', ['minimal', 'standard', 'detailed', 'custom']),
  setupRelevant: browserPolicySetupRelevant,
  classificationServiceReferenced: (answers) =>
    browserPolicyHas(answers, '5.1', 'category') || browserPolicyHas(answers, '5.2', 'classification-service'),
  multiTargetActionMatrixRelevant: (answers) =>
    browserPolicyCount(answers, '5.1') >= 2 && browserPolicyCount(answers, '6.1') >= 2,
  evidencePrivacyVisible: browserPolicyEvidencePrivacyVisible,
  notificationEventsRelevant: browserPolicyNotificationEventsRelevant,
  unsupportedCapabilityRelevant: browserPolicyUnsupportedCapabilityRelevant,
  storedBrowserDataExists: browserPolicyStoredBrowserDataExists,
};

export function browserPolicyForestSourceSettingIds(): ReadonlyMap<BrowserPolicyQuestionId, readonly string[]> {
  const settingsByQuestion = new Map<BrowserPolicyQuestionId, string[]>();
  BrowserPolicyQuestionIds.forEach((id) => settingsByQuestion.set(id, []));
  browserControlFullCatalogSettings().forEach((setting) => {
    settingsByQuestion
      .get(browserPolicyQuestionIdForSetting(setting.uiTab, setting.sourceText))
      ?.push(setting.settingId);
  });
  return new Map([...settingsByQuestion.entries()].map(([id, settingIds]) => [id, settingIds.sort()]));
}

function conditionsMatch(conditions: readonly BrowserPolicyCondition[], answers: BrowserPolicyAnswerMap): boolean {
  return conditions.every((condition) => browserPolicyConditionMatches(condition, answers));
}

function conditionListMatches(
  conditions: readonly BrowserPolicyCondition[] | undefined,
  answers: BrowserPolicyAnswerMap
): boolean {
  return conditions !== undefined && conditions.length > 0 && conditionsMatch(conditions, answers);
}

function browserPolicyQuestionIdForSetting(uiTab: string, sourceText: string): BrowserPolicyQuestionId {
  const text = sourceText.toLowerCase();
  if (uiTab === 'ai' || browserPolicySourceTextIncludesAny(text, BrowserPolicyAiSourceTerms)) {
    return 'A1';
  }
  if (uiTab === 'schedule') {
    return browserPolicyScheduleQuestionIdForSetting(text);
  }
  return (
    BrowserPolicyQuestionIdByUiTab.get(uiTab) ??
    browserPolicyQuestionIdForSourceText(text) ??
    BrowserPolicyFallbackQuestionId
  );
}

const BrowserPolicyAiSourceTerms = [' ai ', 'classification'] as const;
const BrowserPolicyScheduleBudgetSourceTerms = ['budget', 'quota'] as const;
const BrowserPolicyFallbackQuestionId = '1.2' satisfies BrowserPolicyQuestionId;
const BrowserPolicyQuestionIdByUiTab = new Map<string, BrowserPolicyQuestionId>([
  ['audit', '18.2'],
  ['data', '17.1'],
  ['reports', '14.1'],
  ['approvals', '12.1'],
  ['setup', '16.1'],
  ['platform', '15.2'],
]);
const BrowserPolicyQuestionSourceRules: readonly {
  readonly terms: readonly string[];
  readonly questionId: BrowserPolicyQuestionId;
}[] = [
  { terms: ['download'], questionId: '9.1' },
  { terms: ['search'], questionId: '7.1' },
  { terms: ['video', 'channel'], questionId: '8.1' },
  { terms: ['managed browser', 'profile', 'extension'], questionId: '3.1' },
  { terms: ['unmanaged', 'bypass', 'tor', 'portable'], questionId: '4.1' },
  { terms: ['url', 'domain', 'category', 'rule'], questionId: '5.1' },
  { terms: ['evidence', 'proof', 'privacy'], questionId: '13.1' },
  { terms: ['browser', 'discover', 'coverage'], questionId: '2.1' },
];

function browserPolicyScheduleQuestionIdForSetting(text: string): BrowserPolicyQuestionId {
  return browserPolicySourceTextIncludesAny(text, BrowserPolicyScheduleBudgetSourceTerms) ? '11.2' : '10.1';
}

function browserPolicyQuestionIdForSourceText(text: string): BrowserPolicyQuestionId | undefined {
  return BrowserPolicyQuestionSourceRules.find((rule) => browserPolicySourceTextIncludesAny(text, rule.terms))
    ?.questionId;
}

function browserPolicySourceTextIncludesAny(text: string, terms: readonly string[]): boolean {
  return terms.some((term) => text.includes(term));
}
