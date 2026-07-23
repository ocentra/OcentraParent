use std::path::Path;

use ocentra_parent_agent_protocol::activity::policy_context::LocalAiParentRuleContextRef;
use ocentra_parent_agent_protocol::activity::policy_preview::PolicyPreviewReadModel;
use ocentra_parent_agent_protocol::activity::ActivityEvent;
use ocentra_parent_agent_protocol::activity_memory_graph::ActivityMemoryGraphReadModel;
use ocentra_parent_agent_protocol::activity_query::{
    ActivityIngestStatus, ActivityRecentSummary, ACTIVITY_QUERY_SCHEMA_VERSION,
};
use ocentra_parent_agent_protocol::app_game::{AppGameServiceReadModel, AppGameSessionReport};
use ocentra_parent_agent_protocol::browser_read_model::BrowserEvidenceReadModel;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::network_flow::ActivityNetworkFlowReadModel;
use ocentra_parent_agent_protocol::screen_evidence::{
    ScreenAnalysisResult, ScreenEvidenceRecentSummary,
};
use rusqlite::{params, Connection};

use crate::{
    activity_store_app_game::{app_game_service_read_model, app_game_session_report},
    activity_store_browser::browser_evidence_read_model,
    activity_store_connection::initialize_connection,
    activity_store_memory_graph::activity_memory_graph_read_model,
    activity_store_network_flow::network_flow_read_model,
    activity_store_parent_rule_context::replace_parent_rule_contexts,
    activity_store_policy_preview::policy_preview_read_model,
    activity_store_rows::{row_from_sqlite, summary_from_rows},
    activity_store_screen_evidence::{
        screen_evidence_recent_summary, screen_evidence_result_for_queue_job,
    },
    ActivityJournal, ActivityStoreError,
};

mod internals;

pub struct ActivityStore {
    pub(crate) connection: Connection,
}

impl ActivityStore {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, ActivityStoreError> {
        let connection = Connection::open(path)?;
        initialize_connection(&connection)?;
        Ok(Self { connection })
    }

    pub fn open_in_memory() -> Result<Self, ActivityStoreError> {
        let connection = Connection::open_in_memory()?;
        initialize_connection(&connection)?;
        Ok(Self { connection })
    }

    pub fn ingest_events(
        &self,
        events: &[ActivityEvent],
    ) -> Result<ActivityIngestStatus, ActivityStoreError> {
        let mut ingested = 0;
        let mut duplicate_events = 0;
        for event in events {
            if internals::has_event_id(&self.connection, &event.event_id)? {
                duplicate_events += 1;
                continue;
            }
            internals::insert_event(&self.connection, event)?;
            ingested += 1;
        }

        self.status_with_counts(ingested, duplicate_events)
    }

    pub fn ingest_journal(
        &self,
        journal: &ActivityJournal,
    ) -> Result<ActivityIngestStatus, ActivityStoreError> {
        let mut events = Vec::new();
        for line in journal.lines()? {
            events.push(journal.decrypt_line(&line)?);
        }
        self.ingest_events(&events)
    }

    pub fn status(&self) -> Result<ActivityIngestStatus, ActivityStoreError> {
        self.status_with_counts(0, 0)
    }

    pub fn recent_summary(&self, limit: u64) -> Result<ActivityRecentSummary, ActivityStoreError> {
        let mut statement = self
            .connection
            .prepare(constants::sqlite::SELECT_RECENT_ACTIVITY)?;
        let mapped_rows = statement.query_map(params![limit as i64], row_from_sqlite)?;
        let mut rows = Vec::new();
        for row in mapped_rows {
            rows.push(row?);
        }
        Ok(summary_from_rows(limit, &rows))
    }

    pub fn browser_evidence_read_model(
        &self,
        limit: u64,
        generated_at: &str,
    ) -> Result<BrowserEvidenceReadModel, ActivityStoreError> {
        browser_evidence_read_model(&self.connection, limit, generated_at)
    }

    pub fn app_game_session_report(
        &self,
        limit: u64,
    ) -> Result<AppGameSessionReport, ActivityStoreError> {
        app_game_session_report(&self.connection, limit)
    }

    pub fn app_game_service_read_model(
        &self,
        limit: u64,
        generated_at: &str,
    ) -> Result<AppGameServiceReadModel, ActivityStoreError> {
        app_game_service_read_model(&self.connection, limit, generated_at)
    }

    pub fn activity_memory_graph_read_model(
        &self,
        limit: u64,
        generated_at: &str,
    ) -> Result<ActivityMemoryGraphReadModel, ActivityStoreError> {
        activity_memory_graph_read_model(&self.connection, limit, generated_at)
    }

    pub fn network_flow_read_model(
        &self,
        limit: u64,
        generated_at: &str,
    ) -> Result<ActivityNetworkFlowReadModel, ActivityStoreError> {
        network_flow_read_model(&self.connection, limit, generated_at)
    }

    pub fn screen_evidence_recent_summary(
        &self,
        limit: u64,
        generated_at: &str,
    ) -> Result<ScreenEvidenceRecentSummary, ActivityStoreError> {
        screen_evidence_recent_summary(&self.connection, limit, generated_at)
    }

    pub fn screen_evidence_result_for_queue_job(
        &self,
        queue_job_id: &str,
    ) -> Result<Option<ScreenAnalysisResult>, ActivityStoreError> {
        screen_evidence_result_for_queue_job(&self.connection, queue_job_id)
    }

    pub fn policy_preview_read_model(
        &self,
        limit: u64,
        generated_at: &str,
    ) -> Result<PolicyPreviewReadModel, ActivityStoreError> {
        policy_preview_read_model(&self.connection, limit, generated_at)
    }

    pub fn replace_parent_rule_contexts(
        &self,
        contexts: &[LocalAiParentRuleContextRef],
    ) -> Result<(), ActivityStoreError> {
        replace_parent_rule_contexts(&self.connection, contexts)
    }

    pub fn connection_for_test(&self) -> &Connection {
        &self.connection
    }

    fn status_with_counts(
        &self,
        events_ingested: u64,
        duplicate_events: u64,
    ) -> Result<ActivityIngestStatus, ActivityStoreError> {
        Ok(ActivityIngestStatus {
            schema_version: ACTIVITY_QUERY_SCHEMA_VERSION,
            database_ready: true,
            events_ingested,
            events_stored: internals::event_count(&self.connection)?,
            duplicate_events,
            last_event_id: internals::last_event_id(&self.connection)?,
        })
    }
}
