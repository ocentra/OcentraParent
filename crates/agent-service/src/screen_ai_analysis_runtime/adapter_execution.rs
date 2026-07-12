use std::{
    process::Stdio,
    time::{Duration, Instant},
};

use ocentra_parent_agent_protocol::local_ai_runtime::generation::LocalAiChatGenerationResult;
use ocentra_parent_agent_protocol::screen_evidence::ScreenAnalysisResult;
use tokio::{io::AsyncWriteExt, time::timeout};

use super::super::{adapter_process::adapter_process_command, queue::QueuedScreenImage};
use super::{
    adapter_request, failed_generation, generation_from_process_output, timed_out_generation,
    unavailable_generation, ScreenAiAnalysisRuntimeConfig,
};

pub(super) async fn run_adapter(
    config: &ScreenAiAnalysisRuntimeConfig,
    image: &QueuedScreenImage,
    metadata: Option<&ScreenAnalysisResult>,
) -> LocalAiChatGenerationResult {
    let Some(command) = config
        .adapter_command
        .as_ref()
        .filter(|candidate| candidate.is_file())
    else {
        return unavailable_generation(config, image, 0);
    };
    let request = adapter_request(image, metadata);
    let request_bytes = match serde_json::to_vec(&request) {
        Ok(bytes) => bytes,
        Err(_) => return failed_generation(config, image, 0, 0),
    };
    let started = Instant::now();
    let mut process = adapter_process_command(command);
    let mut child = match process
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(child) => child,
        Err(_) => return failed_generation(config, image, request_bytes.len() as u64, 0),
    };
    if let Some(mut stdin) = child.stdin.take() {
        if stdin.write_all(&request_bytes).await.is_err() {
            return failed_generation(config, image, request_bytes.len() as u64, 0);
        }
    }
    match timeout(
        Duration::from_millis(config.adapter_timeout_ms),
        child.wait_with_output(),
    )
    .await
    {
        Ok(Ok(output)) => generation_from_process_output(
            config,
            image,
            request_bytes.len() as u64,
            started.elapsed().as_millis() as u64,
            output,
        ),
        Ok(Err(_)) => failed_generation(
            config,
            image,
            request_bytes.len() as u64,
            started.elapsed().as_millis() as u64,
        ),
        Err(_) => timed_out_generation(config, image, request_bytes.len() as u64),
    }
}
