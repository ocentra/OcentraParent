use super::{ChildAgentService, ChildAgentServiceError};

pub(super) async fn run_until_shutdown(
    mut service: ChildAgentService,
) -> Result<(), ChildAgentServiceError> {
    let shutdown = tokio::signal::ctrl_c();
    tokio::pin!(shutdown);
    loop {
        let keep_running = tokio::select! {
            signal = &mut shutdown => {
                signal.map_err(ChildAgentServiceError::Shutdown)?;
                false
            }
            queued = service.commands.recv() => {
                handle_queued_command(&service, queued).await
            }
        };
        if !keep_running {
            return Ok(());
        }
    }
}

async fn handle_queued_command(
    service: &ChildAgentService,
    queued: Option<super::QueuedCommand>,
) -> bool {
    let Some(super::QueuedCommand { command, response }) = queued else {
        return false;
    };
    let result = service.dispatch(command).await;
    let _ = response.send(result);
    true
}
