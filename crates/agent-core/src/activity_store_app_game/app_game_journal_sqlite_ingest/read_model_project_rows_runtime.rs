use ocentra_parent_agent_protocol::app_game::{
    AppGameRuntimeEvidenceRow, APP_GAME_RUNTIME_RUNNING,
};

use crate::ActivityStoreError;

pub(crate) fn project_runtime_row(
    model: &mut ocentra_parent_agent_protocol::app_game::AppGameServiceReadModel,
    row_json: &str,
    seen_runtime_processes: &mut Vec<String>,
) -> Result<(), ActivityStoreError> {
    let runtime = serde_json::from_str::<AppGameRuntimeEvidenceRow>(row_json)?;
    if !seen_runtime_processes
        .iter()
        .any(|candidate| candidate == &runtime.process_identity)
    {
        seen_runtime_processes.push(runtime.process_identity.clone());
    } else {
        return Ok(());
    }
    if runtime.runtime_state == APP_GAME_RUNTIME_RUNNING {
        model.running_now_rows.push(runtime);
    }
    Ok(())
}
