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
        .find_map(|entry| (entry.flag_id == flag_id).then_some(entry.evaluator))
        .is_some_and(|evaluator| evaluator(answers))
}

struct BrowserPolicyComputedFlagEvaluatorEntry {
    flag_id: &'static str,
    evaluator: fn(&BrowserPolicyAnswerMap) -> bool,
}

const BROWSER_POLICY_COMPUTED_FLAG_EVALUATORS: &[BrowserPolicyComputedFlagEvaluatorEntry] = &[
    BrowserPolicyComputedFlagEvaluatorEntry {
        flag_id: "policyIsOff",
        evaluator: |answers| browser_policy_root_answer(answers) == "off",
    },
    BrowserPolicyComputedFlagEvaluatorEntry {
        flag_id: "policyIsOn",
        evaluator: |answers| browser_policy_root_answer(answers) == "on",
    },
    BrowserPolicyComputedFlagEvaluatorEntry {
        flag_id: "policyPaused",
        evaluator: |answers| browser_policy_root_answer(answers) == "paused",
    },
    BrowserPolicyComputedFlagEvaluatorEntry {
        flag_id: "emergencyOverrideActive",
        evaluator: browser_policy_emergency_override_active,
    },
    BrowserPolicyComputedFlagEvaluatorEntry {
        flag_id: "askParentExists",
        evaluator: browser_policy_ask_parent_exists,
    },
    BrowserPolicyComputedFlagEvaluatorEntry {
        flag_id: "limitExists",
        evaluator: |answers| {
            browser_policy_has(answers, "1.2", "limit")
                || browser_policy_has(answers, "6.1", "limit-time")
                || browser_policy_has(answers, "8.1", "limit")
        },
    },
    BrowserPolicyComputedFlagEvaluatorEntry {
        flag_id: "downloadsSelected",
        evaluator: |answers| browser_policy_has(answers, "5.1", "downloads"),
    },
    BrowserPolicyComputedFlagEvaluatorEntry {
        flag_id: "searchSelected",
        evaluator: |answers| {
            browser_policy_has_any(answers, "5.1", &["search-terms", "safe-search"])
        },
    },
    BrowserPolicyComputedFlagEvaluatorEntry {
        flag_id: "videoSelected",
        evaluator: |answers| browser_policy_has(answers, "5.1", "video"),
    },
    BrowserPolicyComputedFlagEvaluatorEntry {
        flag_id: "exactEvidenceSelected",
        evaluator: browser_policy_exact_evidence_selected,
    },
    BrowserPolicyComputedFlagEvaluatorEntry {
        flag_id: "managedBrowserRequired",
        evaluator: browser_policy_managed_browser_required,
    },
    BrowserPolicyComputedFlagEvaluatorEntry {
        flag_id: "reportsEnabled",
        evaluator: |answers| {
            selected(answers, "14.1")
                .iter()
                .any(|option_id| option_id != "policy-status")
        },
    },
    BrowserPolicyComputedFlagEvaluatorEntry {
        flag_id: "auditEnabled",
        evaluator: |answers| {
            browser_policy_has_any(
                answers,
                "18.1",
                &["minimal", "standard", "detailed", "custom"],
            )
        },
    },
    BrowserPolicyComputedFlagEvaluatorEntry {
        flag_id: "setupRelevant",
        evaluator: browser_policy_setup_relevant,
    },
    BrowserPolicyComputedFlagEvaluatorEntry {
        flag_id: "classificationServiceReferenced",
        evaluator: |answers| {
            browser_policy_has(answers, "5.1", "category")
                || browser_policy_has(answers, "5.2", "classification-service")
        },
    },
    BrowserPolicyComputedFlagEvaluatorEntry {
        flag_id: "multiTargetActionMatrixRelevant",
        evaluator: |answers| {
            browser_policy_count(answers, "5.1") >= 2 && browser_policy_count(answers, "6.1") >= 2
        },
    },
    BrowserPolicyComputedFlagEvaluatorEntry {
        flag_id: "evidencePrivacyVisible",
        evaluator: browser_policy_evidence_privacy_visible,
    },
    BrowserPolicyComputedFlagEvaluatorEntry {
        flag_id: "notificationEventsRelevant",
        evaluator: browser_policy_notification_events_relevant,
    },
    BrowserPolicyComputedFlagEvaluatorEntry {
        flag_id: "unsupportedCapabilityRelevant",
        evaluator: browser_policy_unsupported_capability_relevant,
    },
    BrowserPolicyComputedFlagEvaluatorEntry {
        flag_id: "storedBrowserDataExists",
        evaluator: browser_policy_stored_browser_data_exists,
    },
    BrowserPolicyComputedFlagEvaluatorEntry {
        flag_id: "browserGamesRelevant",
        evaluator: browser_policy_browser_games_relevant,
    },
];

fn browser_policy_computed_flag_evaluators() -> &'static [BrowserPolicyComputedFlagEvaluatorEntry] {
    BROWSER_POLICY_COMPUTED_FLAG_EVALUATORS
}
