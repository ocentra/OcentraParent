use super::*;

pub(super) fn selected<'a>(answers: &'a BrowserPolicyAnswerMap, question_id: &str) -> &'a [String] {
    answers.get(question_id).map(Vec::as_slice).unwrap_or(&[])
}

pub(super) fn browser_policy_root_answer(answers: &BrowserPolicyAnswerMap) -> &str {
    selected(answers, "1.1")
        .first()
        .map(String::as_str)
        .unwrap_or("off")
}

pub(super) fn browser_policy_has(
    answers: &BrowserPolicyAnswerMap,
    question_id: &str,
    option_id: &str,
) -> bool {
    selected(answers, question_id)
        .iter()
        .any(|value| value == option_id)
}

pub(super) fn browser_policy_has_any(
    answers: &BrowserPolicyAnswerMap,
    question_id: &str,
    option_ids: &[&str],
) -> bool {
    option_ids
        .iter()
        .any(|option_id| browser_policy_has(answers, question_id, option_id))
}

pub(super) fn browser_policy_count(answers: &BrowserPolicyAnswerMap, question_id: &str) -> usize {
    selected(answers, question_id).len()
}

pub(super) fn browser_policy_emergency_override_active(answers: &BrowserPolicyAnswerMap) -> bool {
    browser_policy_has_any_root(answers, &["emergency-allow", "emergency-block"])
}

pub(super) fn browser_policy_has_any_root(
    answers: &BrowserPolicyAnswerMap,
    root_values: &[&str],
) -> bool {
    root_values.contains(&browser_policy_root_answer(answers))
}

pub(super) fn browser_policy_ask_parent_exists(answers: &BrowserPolicyAnswerMap) -> bool {
    browser_policy_has_any_ask_parent_trigger(answers)
        || browser_policy_has_any(answers, "10.2", &["allow-once", "custom-window"])
        || browser_policy_has(answers, "11.2", "extension")
        || browser_policy_root_answer(answers) == "emergency-block"
}

fn browser_policy_has_any_ask_parent_trigger(answers: &BrowserPolicyAnswerMap) -> bool {
    browser_policy_has_any(answers, "1.2", &["parent-review"])
        || browser_policy_has_any(answers, "2.3", &["parent-review"])
        || browser_policy_has_any(answers, "4.1", &["parent-review"])
        || browser_policy_has_any(answers, "5.2", &["parent-review"])
        || browser_policy_has_any(answers, "5.3", &["parent-review"])
        || browser_policy_has_any(answers, "6.1", &["parent-review"])
        || browser_policy_has_any(answers, "7.1", &["parent-review"])
        || browser_policy_has_any(answers, "8.1", &["parent-review"])
        || browser_policy_has_any(answers, "9.1", &["parent-review"])
        || browser_policy_has_any(answers, "9.3", &["parent-review"])
        || browser_policy_has_any(answers, "15.2", &["parent-review"])
}

pub(super) fn browser_policy_exact_evidence_selected(answers: &BrowserPolicyAnswerMap) -> bool {
    browser_policy_has_any(answers, "5.1", &["exact-url", "search-terms"])
        || browser_policy_has_any(answers, "13.1", &["exact-url", "search-term"])
        || browser_policy_has(answers, "3.1", "managed-exact")
}

pub(super) fn browser_policy_managed_browser_required(answers: &BrowserPolicyAnswerMap) -> bool {
    browser_policy_has_any(answers, "3.1", &["managed-exact", "managed-all"])
        || browser_policy_has(answers, "6.1", "require-managed")
}

pub(super) fn browser_policy_setup_relevant(answers: &BrowserPolicyAnswerMap) -> bool {
    browser_policy_has_any(
        answers,
        "3.1",
        &["prefer-managed", "managed-exact", "managed-all"],
    ) || browser_policy_has_any(answers, "2.2", &["standard", "strict", "custom"])
        || browser_policy_has_any(answers, "2.3", &["parent-review", "block-until-approved"])
        || browser_policy_computed_flag("unsupportedCapabilityRelevant", answers)
}
