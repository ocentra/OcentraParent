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
  switch (flagId) {
    case 'policyIsOff':
      return browserPolicyRootAnswer(answers) === 'off';
    case 'policyIsOn':
      return browserPolicyRootAnswer(answers) === 'on';
    case 'policyPaused':
      return browserPolicyRootAnswer(answers) === 'paused';
    case 'emergencyOverrideActive':
      return browserPolicyEmergencyOverrideActive(answers);
    case 'askParentExists':
      return browserPolicyAskParentExists(answers);
    case 'limitExists':
      return (
        browserPolicyHas(answers, '1.2', 'limit') ||
        browserPolicyHas(answers, '6.1', 'limit-time') ||
        browserPolicyHas(answers, '8.1', 'limit')
      );
    case 'downloadsSelected':
      return browserPolicyHas(answers, '5.1', 'downloads');
    case 'searchSelected':
      return browserPolicyHasAny(answers, '5.1', ['search-terms', 'safe-search']);
    case 'videoSelected':
      return browserPolicyHas(answers, '5.1', 'video');
    case 'exactEvidenceSelected':
      return browserPolicyExactEvidenceSelected(answers);
    case 'managedBrowserRequired':
      return browserPolicyManagedBrowserRequired(answers);
    case 'reportsEnabled':
      return (answers['14.1'] ?? []).some((optionId) => optionId !== 'policy-status');
    case 'auditEnabled':
      return browserPolicyHasAny(answers, '18.1', ['minimal', 'standard', 'detailed', 'custom']);
    case 'setupRelevant':
      return browserPolicySetupRelevant(answers);
    case 'classificationServiceReferenced':
      return browserPolicyHas(answers, '5.1', 'category') || browserPolicyHas(answers, '5.2', 'classification-service');
    case 'multiTargetActionMatrixRelevant':
      return browserPolicyCount(answers, '5.1') >= 2 && browserPolicyCount(answers, '6.1') >= 2;
    case 'evidencePrivacyVisible':
      return browserPolicyEvidencePrivacyVisible(answers);
    case 'notificationEventsRelevant':
      return browserPolicyNotificationEventsRelevant(answers);
    case 'unsupportedCapabilityRelevant':
      return browserPolicyUnsupportedCapabilityRelevant(answers);
    case 'storedBrowserDataExists':
      return browserPolicyStoredBrowserDataExists(answers);
  }
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
  if (uiTab === 'ai' || text.includes(' ai ') || text.includes('classification')) return 'A1';
  if (uiTab === 'audit') return '18.2';
  if (uiTab === 'data') return '17.1';
  if (uiTab === 'reports') return '14.1';
  if (uiTab === 'approvals') return '12.1';
  if (uiTab === 'schedule') return text.includes('budget') || text.includes('quota') ? '11.2' : '10.1';
  if (uiTab === 'setup') return '16.1';
  if (uiTab === 'platform') return '15.2';
  if (text.includes('download')) return '9.1';
  if (text.includes('search')) return '7.1';
  if (text.includes('video') || text.includes('channel')) return '8.1';
  if (text.includes('managed browser') || text.includes('profile') || text.includes('extension')) return '3.1';
  if (text.includes('unmanaged') || text.includes('bypass') || text.includes('tor') || text.includes('portable'))
    return '4.1';
  if (text.includes('url') || text.includes('domain') || text.includes('category') || text.includes('rule'))
    return '5.1';
  if (text.includes('evidence') || text.includes('proof') || text.includes('privacy')) return '13.1';
  if (text.includes('browser') || text.includes('discover') || text.includes('coverage')) return '2.1';
  return '1.2';
}
