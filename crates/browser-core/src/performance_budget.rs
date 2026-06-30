use ocentra_parent_agent_protocol::constants;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BrowserPerformanceBudgetCheck {
    pub budget_id: &'static str,
    pub observed_ms: u64,
    pub budget_ms: u64,
    pub sample_size: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BrowserPerformanceBudgetState {
    WithinBudget,
    Degraded,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BrowserPerformanceBudgetError {
    MissingBudget,
    MissingSample,
}

pub fn evaluate_browser_performance_budget(
    check: BrowserPerformanceBudgetCheck,
) -> Result<BrowserPerformanceBudgetState, BrowserPerformanceBudgetError> {
    if check.budget_ms == 0 {
        return Err(BrowserPerformanceBudgetError::MissingBudget);
    }
    if check.sample_size == 0 {
        return Err(BrowserPerformanceBudgetError::MissingSample);
    }
    if check.observed_ms <= check.budget_ms {
        Ok(BrowserPerformanceBudgetState::WithinBudget)
    } else {
        Ok(BrowserPerformanceBudgetState::Degraded)
    }
}

pub fn browser_performance_fixture_budget_matrix() -> [BrowserPerformanceBudgetCheck; 8] {
    [
        BrowserPerformanceBudgetCheck {
            budget_id: constants::browser::PERFORMANCE_BUDGET_INVENTORY_SCAN,
            observed_ms: 120,
            budget_ms: constants::browser::PERFORMANCE_BUDGET_MS_INVENTORY_SCAN,
            sample_size: constants::browser::PERFORMANCE_SAMPLE_SIZE_1,
        },
        BrowserPerformanceBudgetCheck {
            budget_id: constants::browser::PERFORMANCE_BUDGET_SUPPORT_MATRIX_DERIVATION,
            observed_ms: 20,
            budget_ms: constants::browser::PERFORMANCE_BUDGET_MS_SUPPORT_MATRIX_DERIVATION,
            sample_size: constants::browser::PERFORMANCE_SAMPLE_SIZE_1,
        },
        BrowserPerformanceBudgetCheck {
            budget_id: constants::browser::PERFORMANCE_BUDGET_CDP_TARGET_MAPPING_100_TABS,
            observed_ms: 80,
            budget_ms: constants::browser::PERFORMANCE_BUDGET_MS_CDP_TARGET_MAPPING_100_TABS,
            sample_size: constants::browser::PERFORMANCE_SAMPLE_SIZE_100,
        },
        BrowserPerformanceBudgetCheck {
            budget_id: constants::browser::PERFORMANCE_BUDGET_JOURNAL_WRITE_PER_EVENT,
            observed_ms: 5,
            budget_ms: constants::browser::PERFORMANCE_BUDGET_MS_JOURNAL_WRITE_PER_EVENT,
            sample_size: constants::browser::PERFORMANCE_SAMPLE_SIZE_1,
        },
        BrowserPerformanceBudgetCheck {
            budget_id: constants::browser::PERFORMANCE_BUDGET_SQLITE_REPLAY_10000_EVENTS,
            observed_ms: 1_200,
            budget_ms: constants::browser::PERFORMANCE_BUDGET_MS_SQLITE_REPLAY_10000_EVENTS,
            sample_size: constants::browser::PERFORMANCE_SAMPLE_SIZE_10000,
        },
        BrowserPerformanceBudgetCheck {
            budget_id: constants::browser::PERFORMANCE_BUDGET_UNMANAGED_PROCESS_SCAN,
            observed_ms: 100,
            budget_ms: constants::browser::PERFORMANCE_BUDGET_MS_UNMANAGED_PROCESS_SCAN,
            sample_size: constants::browser::PERFORMANCE_SAMPLE_SIZE_1,
        },
        BrowserPerformanceBudgetCheck {
            budget_id: constants::browser::PERFORMANCE_BUDGET_RAPID_BRIDGE_RECONNECT,
            observed_ms: 200,
            budget_ms: constants::browser::PERFORMANCE_BUDGET_MS_RAPID_BRIDGE_RECONNECT,
            sample_size: 2,
        },
        BrowserPerformanceBudgetCheck {
            budget_id: constants::browser::PERFORMANCE_BUDGET_MEMORY_CACHE_LOOKUP_INVALIDATION,
            observed_ms: 8,
            budget_ms: constants::browser::PERFORMANCE_BUDGET_MS_MEMORY_CACHE_LOOKUP_INVALIDATION,
            sample_size: constants::browser::PERFORMANCE_SAMPLE_SIZE_1,
        },
    ]
}
