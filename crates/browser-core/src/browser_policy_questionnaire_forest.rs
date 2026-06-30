use std::collections::{BTreeMap, HashMap};
#[path = "browser_policy_questionnaire_forest_answers.rs"]
mod browser_policy_questionnaire_forest_answers;
#[path = "browser_policy_questionnaire_forest_question_ids.rs"]
mod browser_policy_questionnaire_forest_question_ids;

use self::browser_policy_questionnaire_forest_answers::*;
use self::browser_policy_questionnaire_forest_question_ids::*;


#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BrowserPolicyCondition {
    AnswerEquals {
        question_id: String,
        option_id: String,
    },
    AnswerIncludes {
        question_id: String,
        option_id: String,
    },
    AnswerIncludesAny {
        question_id: String,
        option_ids: Vec<String>,
    },
    AnswerHasAnySelected {
        question_id: String,
    },
    ComputedFlag {
        flag_id: String,
    },
    All {
        conditions: Vec<BrowserPolicyCondition>,
    },
    Any {
        conditions: Vec<BrowserPolicyCondition>,
    },
    Not {
        condition: Box<BrowserPolicyCondition>,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BrowserPolicyQuestionTemplate {
    pub id: String,
    pub surface: String,
    pub show_when: Vec<BrowserPolicyCondition>,
    pub never_show_when: Vec<BrowserPolicyCondition>,
    pub disabled_when: Vec<BrowserPolicyCondition>,
    pub readonly_when: Vec<BrowserPolicyCondition>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BrowserPolicyQuestionStateTemplate {
    pub visible: bool,
    pub disabled: bool,
    pub readonly: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BrowserPolicyCatalogSettingTemplate {
    pub setting_id: String,
    pub ui_tab: String,
    pub source_text: String,
}

pub type BrowserPolicyAnswerMap = BTreeMap<String, Vec<String>>;

pub fn browser_policy_question_state(
    question: &BrowserPolicyQuestionTemplate,
    answers: &BrowserPolicyAnswerMap,
) -> BrowserPolicyQuestionStateTemplate {
    let never_show = condition_list_matches(&question.never_show_when, answers);
    let show = question.show_when.is_empty() || conditions_match(&question.show_when, answers);
    let visible = show && !never_show;

    BrowserPolicyQuestionStateTemplate {
        visible,
        disabled: visible && condition_list_matches(&question.disabled_when, answers),
        readonly: visible && condition_list_matches(&question.readonly_when, answers),
    }
}

pub fn browser_policy_visible_question_ids(
    questions: &[BrowserPolicyQuestionTemplate],
    question_ids: &[String],
    compact_order: &[String],
    answers: &BrowserPolicyAnswerMap,
    surface: &str,
) -> Vec<String> {
    let question_by_id: HashMap<&str, &BrowserPolicyQuestionTemplate> = questions
        .iter()
        .map(|question| (question.id.as_str(), question))
        .collect();
    let ordered_ids: Vec<&String> = if surface == "ai" {
        question_ids
            .iter()
            .filter(|question_id| question_id.starts_with('A'))
            .collect()
    } else {
        compact_order.iter().collect()
    };

    ordered_ids
        .into_iter()
        .filter_map(|question_id| question_by_id.get(question_id.as_str()).copied())
        .filter(|question| question.surface == surface)
        .filter(|question| browser_policy_question_state(question, answers).visible)
        .map(|question| question.id.clone())
        .collect()
}

pub fn browser_policy_condition_matches(
    condition: &BrowserPolicyCondition,
    answers: &BrowserPolicyAnswerMap,
) -> bool {
    match condition {
        BrowserPolicyCondition::AnswerEquals {
            question_id,
            option_id,
        } => selected(answers, question_id).first() == Some(option_id),
        BrowserPolicyCondition::AnswerIncludes {
            question_id,
            option_id,
        } => selected(answers, question_id).contains(option_id),
        BrowserPolicyCondition::AnswerIncludesAny {
            question_id,
            option_ids,
        } => option_ids
            .iter()
            .any(|option_id| selected(answers, question_id).contains(option_id)),
        BrowserPolicyCondition::AnswerHasAnySelected { question_id } => {
            !selected(answers, question_id).is_empty()
        }
        BrowserPolicyCondition::ComputedFlag { flag_id } => {
            browser_policy_computed_flag(flag_id, answers)
        }
        BrowserPolicyCondition::All { conditions } => conditions_match(conditions, answers),
        BrowserPolicyCondition::Any { conditions } => conditions
            .iter()
            .any(|candidate| browser_policy_condition_matches(candidate, answers)),
        BrowserPolicyCondition::Not { condition } => {
            !browser_policy_condition_matches(condition, answers)
        }
    }
}

pub fn browser_policy_computed_flag(flag_id: &str, answers: &BrowserPolicyAnswerMap) -> bool {
    match flag_id {
        "policyIsOff" => browser_policy_root_answer(answers) == "off",
        "policyIsOn" => browser_policy_root_answer(answers) == "on",
        "policyPaused" => browser_policy_root_answer(answers) == "paused",
        "emergencyOverrideActive" => browser_policy_emergency_override_active(answers),
        "askParentExists" => browser_policy_ask_parent_exists(answers),
        "limitExists" => {
            browser_policy_has(answers, "1.2", "limit")
                || browser_policy_has(answers, "6.1", "limit-time")
                || browser_policy_has(answers, "8.1", "limit")
        }
        "downloadsSelected" => browser_policy_has(answers, "5.1", "downloads"),
        "searchSelected" => {
            browser_policy_has_any(answers, "5.1", &["search-terms", "safe-search"])
        }
        "videoSelected" => browser_policy_has(answers, "5.1", "video"),
        "exactEvidenceSelected" => browser_policy_exact_evidence_selected(answers),
        "managedBrowserRequired" => browser_policy_managed_browser_required(answers),
        "reportsEnabled" => selected(answers, "14.1")
            .iter()
            .any(|option_id| option_id != "policy-status"),
        "auditEnabled" => browser_policy_has_any(
            answers,
            "18.1",
            &["minimal", "standard", "detailed", "custom"],
        ),
        "setupRelevant" => browser_policy_setup_relevant(answers),
        "classificationServiceReferenced" => {
            browser_policy_has(answers, "5.1", "category")
                || browser_policy_has(answers, "5.2", "classification-service")
        }
        "multiTargetActionMatrixRelevant" => {
            browser_policy_count(answers, "5.1") >= 2 && browser_policy_count(answers, "6.1") >= 2
        }
        "evidencePrivacyVisible" => browser_policy_evidence_privacy_visible(answers),
        "notificationEventsRelevant" => browser_policy_notification_events_relevant(answers),
        "unsupportedCapabilityRelevant" => browser_policy_unsupported_capability_relevant(answers),
        "storedBrowserDataExists" => browser_policy_stored_browser_data_exists(answers),
        "browserGamesRelevant" => browser_policy_browser_games_relevant(answers),
        _ => false,
    }
}

pub fn browser_policy_forest_source_setting_ids(
    question_ids: &[String],
    settings: &[BrowserPolicyCatalogSettingTemplate],
) -> BTreeMap<String, Vec<String>> {
    let mut settings_by_question: BTreeMap<String, Vec<String>> = question_ids
        .iter()
        .map(|question_id| (question_id.clone(), Vec::new()))
        .collect();

    for setting in settings {
        let question_id =
            browser_policy_question_id_for_setting(&setting.ui_tab, &setting.source_text);
        if let Some(setting_ids) = settings_by_question.get_mut(&question_id) {
            setting_ids.push(setting.setting_id.clone());
        }
    }

    for setting_ids in settings_by_question.values_mut() {
        setting_ids.sort();
    }

    settings_by_question
}

pub fn browser_policy_questionnaire_forest_typescript() -> &'static str {
    BROWSER_POLICY_QUESTIONNAIRE_FOREST_TYPESCRIPT
}

const BROWSER_POLICY_QUESTIONNAIRE_FOREST_TYPESCRIPT: &str = r#"/* generated from crates/browser-core/src/browser_policy_questionnaire_forest.rs */

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
"#;
