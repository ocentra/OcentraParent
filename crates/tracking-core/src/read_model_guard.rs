use ocentra_parent_agent_protocol::constants::tracking_runtime::{
    READ_MODEL_DIFFERENTIAL_CONSISTENT, READ_MODEL_DIFFERENTIAL_REJECTED_COUNT_DRIFT,
    READ_MODEL_DIFFERENTIAL_REJECTED_TOMBSTONE_DRIFT, READ_MODEL_SCHEMA_COMPATIBLE,
    READ_MODEL_SCHEMA_MIGRATION_REQUIRED,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrackingReadModelMigrationState {
    Compatible,
    MigrationRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrackingReadModelDifferentialState {
    Accepted,
    Rejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TrackingReadModelSchemaDecision {
    pub decision_state: &'static str,
    pub migration_state: TrackingReadModelMigrationState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TrackingReadModelDifferentialDecision {
    pub decision_state: &'static str,
    pub differential_state: TrackingReadModelDifferentialState,
}

pub fn evaluate_tracking_read_model_schema(
    stored_schema_version: u16,
    current_schema_version: u16,
) -> TrackingReadModelSchemaDecision {
    if stored_schema_version == current_schema_version {
        return TrackingReadModelSchemaDecision {
            decision_state: READ_MODEL_SCHEMA_COMPATIBLE,
            migration_state: TrackingReadModelMigrationState::Compatible,
        };
    }

    TrackingReadModelSchemaDecision {
        decision_state: READ_MODEL_SCHEMA_MIGRATION_REQUIRED,
        migration_state: TrackingReadModelMigrationState::MigrationRequired,
    }
}

pub fn evaluate_tracking_read_model_differential(
    returned_rows: u64,
    active_rows: u64,
    tombstone_rows: u64,
    deleted_evidence_refs: u64,
) -> TrackingReadModelDifferentialDecision {
    if returned_rows != active_rows.saturating_add(tombstone_rows) {
        return TrackingReadModelDifferentialDecision {
            decision_state: READ_MODEL_DIFFERENTIAL_REJECTED_COUNT_DRIFT,
            differential_state: TrackingReadModelDifferentialState::Rejected,
        };
    }

    if deleted_evidence_refs > 0 && tombstone_rows == 0 {
        return TrackingReadModelDifferentialDecision {
            decision_state: READ_MODEL_DIFFERENTIAL_REJECTED_TOMBSTONE_DRIFT,
            differential_state: TrackingReadModelDifferentialState::Rejected,
        };
    }

    TrackingReadModelDifferentialDecision {
        decision_state: READ_MODEL_DIFFERENTIAL_CONSISTENT,
        differential_state: TrackingReadModelDifferentialState::Accepted,
    }
}
