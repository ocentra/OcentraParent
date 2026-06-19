use ocentra_browser_core::performance_budget::{
    browser_performance_fixture_budget_matrix, evaluate_browser_performance_budget,
    BrowserPerformanceBudgetCheck, BrowserPerformanceBudgetError, BrowserPerformanceBudgetState,
};
use ocentra_parent_agent_protocol::constants;

#[test]
fn fixture_budget_matrix_stays_within_budget() {
    let budget_matrix = browser_performance_fixture_budget_matrix();

    assert_eq!(budget_matrix.len(), 8);
    assert_eq!(
        budget_matrix[0].budget_id,
        constants::browser::PERFORMANCE_BUDGET_INVENTORY_SCAN
    );
    for budget_check in budget_matrix {
        assert_eq!(
            evaluate_browser_performance_budget(budget_check),
            Ok(BrowserPerformanceBudgetState::WithinBudget)
        );
    }
}

#[test]
fn budget_evaluator_marks_over_budget_rows_degraded() {
    let budget_check = BrowserPerformanceBudgetCheck {
        budget_id: constants::browser::PERFORMANCE_BUDGET_INVENTORY_SCAN,
        observed_ms: constants::browser::PERFORMANCE_BUDGET_MS_INVENTORY_SCAN + 1,
        budget_ms: constants::browser::PERFORMANCE_BUDGET_MS_INVENTORY_SCAN,
        sample_size: constants::browser::PERFORMANCE_SAMPLE_SIZE_1,
    };

    assert_eq!(
        evaluate_browser_performance_budget(budget_check),
        Ok(BrowserPerformanceBudgetState::Degraded)
    );
}

#[test]
fn budget_evaluator_rejects_missing_budget_or_sample() {
    let missing_budget = BrowserPerformanceBudgetCheck {
        budget_id: constants::browser::PERFORMANCE_BUDGET_INVENTORY_SCAN,
        observed_ms: constants::browser::PERFORMANCE_BUDGET_MS_INVENTORY_SCAN,
        budget_ms: 0,
        sample_size: constants::browser::PERFORMANCE_SAMPLE_SIZE_1,
    };
    let missing_sample = BrowserPerformanceBudgetCheck {
        budget_ms: constants::browser::PERFORMANCE_BUDGET_MS_INVENTORY_SCAN,
        sample_size: 0,
        ..missing_budget
    };

    assert_eq!(
        evaluate_browser_performance_budget(missing_budget),
        Err(BrowserPerformanceBudgetError::MissingBudget)
    );
    assert_eq!(
        evaluate_browser_performance_budget(missing_sample),
        Err(BrowserPerformanceBudgetError::MissingSample)
    );
}
