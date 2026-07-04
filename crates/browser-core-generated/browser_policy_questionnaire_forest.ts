/* generated from crates/browser-core/src/browser_policy_questionnaire_forest.rs */

import type { BrowserControlFullCatalogSetting } from '@ocentra-parent/schema-domain/browser-control-full-catalog-schema';
import type {
  BrowserPolicyAnswerMap,
  BrowserPolicyComputedFlagId,
  BrowserPolicyCondition,
  BrowserPolicyQuestion,
  BrowserPolicyQuestionId,
  BrowserPolicyQuestionState,
  BrowserPolicySurface,
} from '@ocentra-parent/schema-domain/browser-policy-questionnaire-forest-contract';

type BrowserPolicyQuestionStateTemplate = Omit<BrowserPolicyQuestionState, 'question'>;

export function browserPolicyVisibleQuestionIdsTemplate(
  questions: readonly BrowserPolicyQuestion[],
  questionIds: readonly BrowserPolicyQuestionId[],
  compactOrder: readonly BrowserPolicyQuestionId[],
  answers: BrowserPolicyAnswerMap,
  surface: BrowserPolicySurface = 'rules'
): readonly BrowserPolicyQuestionId[] {
  const questionById = new Map(questions.map((question) => [question.id, question]));
  const orderedIds =
    surface === 'ai' ? questionIds.filter((questionId) => questionId.startsWith('A')) : compactOrder;

  return orderedIds
    .map((questionId) => questionById.get(questionId))
    .filter((question): question is BrowserPolicyQuestion => question !== undefined)
    .filter((question) => question.surface === surface)
    .filter((question) => browserPolicyQuestionStateTemplate(question, answers).visible)
    .map((question) => question.id);
}

export function browserPolicyQuestionByIdTemplate(
  questions: readonly BrowserPolicyQuestion[],
  id: BrowserPolicyQuestionId
): BrowserPolicyQuestion {
  const question = questions.find((candidate) => candidate.id === id);
  if (question === undefined) {
    throw new Error(`Unknown browser policy question ${id}`);
  }
  return question;
}

export function browserPolicyQuestionStatesTemplate(
  questions: readonly BrowserPolicyQuestion[],
  answers: BrowserPolicyAnswerMap
): readonly BrowserPolicyQuestionState[] {
  return questions.map((question) => browserPolicyQuestionState(question, answers));
}

export function browserPolicyVisibleQuestionsTemplate(
  questions: readonly BrowserPolicyQuestion[],
  questionIds: readonly BrowserPolicyQuestionId[],
  compactOrder: readonly BrowserPolicyQuestionId[],
  answers: BrowserPolicyAnswerMap,
  surface: BrowserPolicySurface = 'rules'
): readonly BrowserPolicyQuestion[] {
  const questionById = new Map(questions.map((question) => [question.id, question]));
  return browserPolicyVisibleQuestionIdsTemplate(questions, questionIds, compactOrder, answers, surface)
    .map((questionId) => questionById.get(questionId))
    .filter((question): question is BrowserPolicyQuestion => question !== undefined);
}

export function browserPolicyQuestionStateTemplate(
  question: BrowserPolicyQuestion,
  answers: BrowserPolicyAnswerMap
): BrowserPolicyQuestionStateTemplate {
  const neverShow = conditionListMatches(question.neverShowWhen, answers);
  const show = (question.showWhen ?? []).length === 0 || conditionsMatch(question.showWhen ?? [], answers);
  const visible = show && !neverShow;

  return {
    visible,
    disabled: visible && conditionListMatches(question.disabledWhen, answers),
    readonly: visible && conditionListMatches(question.readonlyWhen, answers),
  };
}

export function browserPolicyQuestionState(
  question: BrowserPolicyQuestion,
  answers: BrowserPolicyAnswerMap
): BrowserPolicyQuestionState {
  return {
    question,
    ...browserPolicyQuestionStateTemplate(question, answers),
  };
}

export function browserPolicyConditionMatchesTemplate(
  condition: BrowserPolicyCondition,
  answers: BrowserPolicyAnswerMap
): boolean {
  const chosen = (questionId: BrowserPolicyQuestionId) => answers[questionId] ?? [];

  switch (condition.kind) {
    case 'answer-equals':
      return chosen(condition.questionId)[0] === condition.optionId;
    case 'answer-includes':
      return chosen(condition.questionId).includes(condition.optionId);
    case 'answer-includes-any':
      return condition.optionIds.some((optionId) => chosen(condition.questionId).includes(optionId));
    case 'answer-has-any-selected':
      return chosen(condition.questionId).length > 0;
    case 'computed-flag':
      return browserPolicyComputedFlagTemplate(condition.flagId, answers);
    case 'all':
      return conditionsMatch(condition.conditions, answers);
    case 'any':
      return condition.conditions.some((candidate) => browserPolicyConditionMatchesTemplate(candidate, answers));
    case 'not':
      return !browserPolicyConditionMatchesTemplate(condition.condition, answers);
  }
}

export function browserPolicyComputedFlagTemplate(
  flagId: BrowserPolicyComputedFlagId,
  answers: BrowserPolicyAnswerMap
): boolean {
  return browserPolicyComputedFlagEvaluators[flagId](answers);
}

export function browserPolicyForestSourceSettingIdsTemplate(
  questionIds: readonly BrowserPolicyQuestionId[],
  settings: readonly BrowserControlFullCatalogSetting[]
): ReadonlyMap<BrowserPolicyQuestionId, readonly string[]> {
  const settingsByQuestion = new Map<BrowserPolicyQuestionId, string[]>();
  questionIds.forEach((questionId) => settingsByQuestion.set(questionId, []));
  settings.forEach((setting) => {
    settingsByQuestion
      .get(browserPolicyQuestionIdForSetting(setting.uiTab, setting.sourceText))
      ?.push(setting.settingId);
  });
  return new Map([...settingsByQuestion.entries()].map(([questionId, settingIds]) => [questionId, settingIds.sort()]));
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
    browserPolicyHasAny(answers, '2.3', ['parent-review', 'block-until-approved']) ||
    browserPolicyComputedFlagTemplate('unsupportedCapabilityRelevant', answers)
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
    browserPolicyComputedFlagTemplate('reportsEnabled', answers)
  );
}

function browserPolicyNotificationEventsRelevant(answers: BrowserPolicyAnswerMap): boolean {
  return browserPolicyRootAnswer(answers) !== 'off' && browserPolicyHasAnyNotificationTrigger(answers);
}

function browserPolicyHasAnyNotificationTrigger(answers: BrowserPolicyAnswerMap): boolean {
  return (
    browserPolicyHasAny(answers, '2.3', ['notify-parent', 'parent-review', 'block-until-approved']) ||
    browserPolicyHasAny(answers, '4.1', [
      'warn',
      'notify-parent',
      'parent-review',
      'close',
      'close-open-managed',
      'block-launch',
    ]) ||
    browserPolicyHasAny(answers, '6.1', ['warn', 'parent-review', 'block', 'close-browser']) ||
    browserPolicyHasAny(answers, '9.1', ['notify-parent', 'parent-review', 'block-risky', 'block-all-approved']) ||
    browserPolicyComputedFlagTemplate('limitExists', answers) ||
    browserPolicyHasAnyRoot(answers, ['paused', 'emergency-allow', 'emergency-block'])
  );
}

function browserPolicyUnsupportedCapabilityRelevant(answers: BrowserPolicyAnswerMap): boolean {
  return (
    browserPolicyHasAny(answers, '2.1', ['strict', 'custom']) ||
    browserPolicyComputedFlagTemplate('managedBrowserRequired', answers) ||
    browserPolicyHasAny(answers, '5.1', ['exact-url', 'search-terms', 'downloads', 'video']) ||
    browserPolicyHasAny(answers, '6.1', ['block', 'close-browser', 'require-managed'])
  );
}

function browserPolicyStoredBrowserDataExists(answers: BrowserPolicyAnswerMap): boolean {
  return (
    browserPolicyComputedFlagTemplate('reportsEnabled', answers) ||
    browserPolicyComputedFlagTemplate('auditEnabled', answers) ||
    browserPolicyCount(answers, '13.1') > 0 ||
    browserPolicyCount(answers, '12.1') > 0
  );
}

function browserPolicyBrowserGamesRelevant(answers: BrowserPolicyAnswerMap): boolean {
  return browserPolicyRootAnswer(answers) === 'on' && browserPolicyHas(answers, '5.1', 'browser-games');
}

const browserPolicyComputedFlagEvaluators: Record<
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
  browserGamesRelevant: browserPolicyBrowserGamesRelevant,
};

function conditionsMatch(conditions: readonly BrowserPolicyCondition[], answers: BrowserPolicyAnswerMap): boolean {
  return conditions.every((condition) => browserPolicyConditionMatchesTemplate(condition, answers));
}

function conditionListMatches(
  conditions: readonly BrowserPolicyCondition[] | undefined,
  answers: BrowserPolicyAnswerMap
): boolean {
  return conditions !== undefined && conditions.length > 0 && conditionsMatch(conditions, answers);
}

function browserPolicyQuestionIdForSetting(uiTab: string, sourceText: string): BrowserPolicyQuestionId {
  const text = sourceText.toLowerCase();
  if (uiTab === 'ai' || browserPolicySourceTextIncludesAny(text, [' ai ', 'classification'])) {
    return 'A1';
  }
  if (uiTab === 'schedule') {
    return browserPolicyScheduleQuestionIdForSetting(text);
  }
  return (
    browserPolicyQuestionIdByUiTab.get(uiTab) ??
    browserPolicyQuestionIdForSourceText(text) ??
    '1.2'
  );
}

const browserPolicyQuestionIdByUiTab = new Map<string, BrowserPolicyQuestionId>([
  ['audit', '18.2'],
  ['data', '17.1'],
  ['reports', '14.1'],
  ['approvals', '12.1'],
  ['setup', '16.1'],
  ['platform', '15.2'],
]);

const browserPolicyQuestionSourceRules: readonly {
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
  return browserPolicySourceTextIncludesAny(text, ['budget', 'quota']) ? '11.2' : '10.1';
}

function browserPolicyQuestionIdForSourceText(text: string): BrowserPolicyQuestionId | undefined {
  return browserPolicyQuestionSourceRules.find((rule) => browserPolicySourceTextIncludesAny(text, rule.terms))
    ?.questionId;
}

function browserPolicySourceTextIncludesAny(text: string, terms: readonly string[]): boolean {
  return terms.some((term) => text.includes(term));
}
