use super::*;

pub(super) fn browser_policy_condition_matches(
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

pub(super) fn browser_policy_computed_flag(
    flag_id: &str,
    answers: &BrowserPolicyAnswerMap,
) -> bool {
    browser_policy_computed_flag_evaluators()
        .iter()
        .find_map(|(candidate_flag_id, evaluator)| {
            (*candidate_flag_id == flag_id).then_some(evaluator)
        })
        .is_some_and(|evaluator| evaluator(answers))
}

const BROWSER_POLICY_COMPUTED_FLAG_EVALUATORS: &[(&str, fn(&BrowserPolicyAnswerMap) -> bool)] = &[
    ("policyIsOff", |answers| {
        browser_policy_root_answer(answers) == "off"
    }),
    ("policyIsOn", |answers| {
        browser_policy_root_answer(answers) == "on"
    }),
    ("policyPaused", |answers| {
        browser_policy_root_answer(answers) == "paused"
    }),
    (
        "emergencyOverrideActive",
        browser_policy_emergency_override_active,
    ),
    ("askParentExists", browser_policy_ask_parent_exists),
    ("limitExists", |answers| {
        browser_policy_has(answers, "1.2", "limit")
            || browser_policy_has(answers, "6.1", "limit-time")
            || browser_policy_has(answers, "8.1", "limit")
    }),
    ("downloadsSelected", |answers| {
        browser_policy_has(answers, "5.1", "downloads")
    }),
    ("searchSelected", |answers| {
        browser_policy_has_any(answers, "5.1", &["search-terms", "safe-search"])
    }),
    ("videoSelected", |answers| {
        browser_policy_has(answers, "5.1", "video")
    }),
    (
        "exactEvidenceSelected",
        browser_policy_exact_evidence_selected,
    ),
    (
        "managedBrowserRequired",
        browser_policy_managed_browser_required,
    ),
    ("reportsEnabled", |answers| {
        selected(answers, "14.1")
            .iter()
            .any(|option_id| option_id != "policy-status")
    }),
    ("auditEnabled", |answers| {
        browser_policy_has_any(
            answers,
            "18.1",
            &["minimal", "standard", "detailed", "custom"],
        )
    }),
    ("setupRelevant", browser_policy_setup_relevant),
    ("classificationServiceReferenced", |answers| {
        browser_policy_has(answers, "5.1", "category")
            || browser_policy_has(answers, "5.2", "classification-service")
    }),
    ("multiTargetActionMatrixRelevant", |answers| {
        browser_policy_count(answers, "5.1") >= 2 && browser_policy_count(answers, "6.1") >= 2
    }),
    (
        "evidencePrivacyVisible",
        browser_policy_evidence_privacy_visible,
    ),
    (
        "notificationEventsRelevant",
        browser_policy_notification_events_relevant,
    ),
    (
        "unsupportedCapabilityRelevant",
        browser_policy_unsupported_capability_relevant,
    ),
    (
        "storedBrowserDataExists",
        browser_policy_stored_browser_data_exists,
    ),
    (
        "browserGamesRelevant",
        browser_policy_browser_games_relevant,
    ),
];

fn browser_policy_computed_flag_evaluators(
) -> &'static [(&'static str, fn(&BrowserPolicyAnswerMap) -> bool)] {
    BROWSER_POLICY_COMPUTED_FLAG_EVALUATORS
}
