use ocentra_parent_agent_protocol::activity_surface::ActivityHistoricalReportList;
use ocentra_parent_agent_protocol::transport::AgentCommandEnvelope;

use crate::{
    activity_surface_report_store::history_list,
    activity_surface_request::surface_request_from_command,
};

pub(crate) async fn activity_report_history_from_command(
    command: &AgentCommandEnvelope,
) -> Option<ActivityHistoricalReportList> {
    let command = command.clone();
    tokio::task::spawn_blocking(move || history_list(surface_request_from_command(&command)))
        .await
        .ok()
}
