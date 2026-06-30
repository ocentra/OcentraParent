use super::*;

fn browser_policy_evidence_privacy_visible(answers: &BrowserPolicyAnswerMap) -> bool {
    browser_policy_root_answer(answers) == "on" && browser_policy_has_any_evidence_trigger(answers)
}

fn browser_policy_has_any_evidence_trigger(answers: &BrowserPolicyAnswerMap) -> bool {
    browser_policy_count(answers, "2.1") > 0
        || browser_policy_count(answers, "5.1") > 0
        || browser_policy_count(answers, "7.1") > 0
        || browser_policy_count(answers, "8.1") > 0
        || browser_policy_count(answers, "9.1") > 0
        || browser_policy_computed_flag("reportsEnabled", answers)
}

fn browser_policy_notification_events_relevant(answers: &BrowserPolicyAnswerMap) -> bool {
    browser_policy_root_answer(answers) != "off"
        && browser_policy_has_any_notification_trigger(answers)
}

fn browser_policy_has_any_notification_trigger(answers: &BrowserPolicyAnswerMap) -> bool {
    browser_policy_has_any(
        answers,
        "2.3",
        &["notify-parent", "parent-review", "block-until-approved"],
    ) || browser_policy_has_any(
        answers,
        "4.1",
        &[
            "warn",
            "notify-parent",
            "parent-review",
            "close",
            "close-open-managed",
            "block-launch",
        ],
    ) || browser_policy_has_any(
        answers,
        "6.1",
        &["warn", "parent-review", "block", "close-browser"],
    ) || browser_policy_has_any(
        answers,
        "9.1",
        &[
            "notify-parent",
            "parent-review",
            "block-risky",
            "block-all-approved",
        ],
    ) || browser_policy_computed_flag("limitExists", answers)
        || browser_policy_has_any_root(answers, &["paused", "emergency-allow", "emergency-block"])
}

fn browser_policy_unsupported_capability_relevant(answers: &BrowserPolicyAnswerMap) -> bool {
    browser_policy_has_any(answers, "2.1", &["strict", "custom"])
        || browser_policy_computed_flag("managedBrowserRequired", answers)
        || browser_policy_has_any(
            answers,
            "5.1",
            &["exact-url", "search-terms", "downloads", "video"],
        )
        || browser_policy_has_any(
            answers,
            "6.1",
            &["block", "close-browser", "require-managed"],
        )
}

fn browser_policy_stored_browser_data_exists(answers: &BrowserPolicyAnswerMap) -> bool {
    browser_policy_computed_flag("reportsEnabled", answers)
        || browser_policy_computed_flag("auditEnabled", answers)
        || browser_policy_count(answers, "13.1") > 0
        || browser_policy_count(answers, "12.1") > 0
}

fn browser_policy_browser_games_relevant(answers: &BrowserPolicyAnswerMap) -> bool {
    browser_policy_root_answer(answers) == "on"
        && browser_policy_has(answers, "5.1", "browser-games")
}

fn conditions_match(
    conditions: &[BrowserPolicyCondition],
    answers: &BrowserPolicyAnswerMap,
) -> bool {
    conditions
        .iter()
        .all(|condition| browser_policy_condition_matches(condition, answers))
}

fn condition_list_matches(
    conditions: &[BrowserPolicyCondition],
    answers: &BrowserPolicyAnswerMap,
) -> bool {
    !conditions.is_empty() && conditions_match(conditions, answers)
}

fn browser_policy_question_id_for_setting(ui_tab: &str, source_text: &str) -> String {
    let text = source_text.to_ascii_lowercase();
    if ui_tab == "ai" || browser_policy_source_text_includes_any(&text, &[" ai ", "classification"])
    {
        return "A1".to_string();
    }
    if ui_tab == "schedule" {
        return browser_policy_schedule_question_id_for_setting(&text);
    }

    if let Some(question_id) = browser_policy_question_id_by_ui_tab(ui_tab) {
        return question_id.to_string();
    }

    browser_policy_question_id_for_source_text(&text)
        .unwrap_or("1.2")
        .to_string()
}

fn browser_policy_schedule_question_id_for_setting(text: &str) -> String {
    if browser_policy_source_text_includes_any(text, &["budget", "quota"]) {
        "11.2".to_string()
    } else {
        "10.1".to_string()
    }
}

fn browser_policy_question_id_by_ui_tab(ui_tab: &str) -> Option<&'static str> {
    match ui_tab {
        "audit" => Some("18.2"),
        "data" => Some("17.1"),
        "reports" => Some("14.1"),
        "approvals" => Some("12.1"),
        "setup" => Some("16.1"),
        "platform" => Some("15.2"),
        _ => None,
    }
}

fn browser_policy_question_id_for_source_text(text: &str) -> Option<&'static str> {
    for (terms, question_id) in [
        (&["download"][..], "9.1"),
        (&["search"][..], "7.1"),
        (&["video", "channel"][..], "8.1"),
        (&["managed browser", "profile", "extension"][..], "3.1"),
        (&["unmanaged", "bypass", "tor", "portable"][..], "4.1"),
        (&["url", "domain", "category", "rule"][..], "5.1"),
        (&["evidence", "proof", "privacy"][..], "13.1"),
        (&["browser", "discover", "coverage"][..], "2.1"),
    ] {
        if browser_policy_source_text_includes_any(text, terms) {
            return Some(question_id);
        }
    }
    None
}

fn browser_policy_source_text_includes_any(text: &str, terms: &[&str]) -> bool {
    terms.iter().any(|term| text.contains(term))
}


