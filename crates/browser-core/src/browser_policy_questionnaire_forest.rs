use std::collections::{BTreeMap, HashMap};
#[path = "browser_policy_questionnaire_forest_answers.rs"]
mod browser_policy_questionnaire_forest_answers;
#[path = "browser_policy_questionnaire_forest_logic.rs"]
mod browser_policy_questionnaire_forest_logic;
#[path = "browser_policy_questionnaire_forest_question_ids.rs"]
mod browser_policy_questionnaire_forest_question_ids;

use self::browser_policy_questionnaire_forest_answers::*;
use self::browser_policy_questionnaire_forest_logic::*;
use self::browser_policy_questionnaire_forest_question_ids::*;
use serde::{Deserialize, Serialize};

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

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct BrowserPolicyAnswerMap(BTreeMap<String, Vec<String>>);

impl BrowserPolicyAnswerMap {
    pub fn new(value: BTreeMap<String, Vec<String>>) -> Self {
        Self(value)
    }

    pub fn get(&self, key: &str) -> Option<&Vec<String>> {
        self.0.get(key)
    }

    pub fn into_inner(self) -> BTreeMap<String, Vec<String>> {
        self.0
    }
}

impl From<BTreeMap<String, Vec<String>>> for BrowserPolicyAnswerMap {
    fn from(value: BTreeMap<String, Vec<String>>) -> Self {
        Self::new(value)
    }
}

impl From<BrowserPolicyAnswerMap> for BTreeMap<String, Vec<String>> {
    fn from(value: BrowserPolicyAnswerMap) -> Self {
        value.0
    }
}

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
    include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../browser-core-generated/browser_policy_questionnaire_forest.ts"
    ))
}

pub fn browser_policy_questionnaire_forest_contract_typescript() -> &'static str {
    include_str!("browser_policy_questionnaire_forest_contract.template.txt")
}
