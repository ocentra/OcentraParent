use ocentra_parent_agent_protocol::constants;

pub(crate) fn screen_correlation_id(queue_job_id: &str) -> String {
    let mut value = String::from(constants::screen_flow::CORRELATION_SCREEN_RUNTIME_PREFIX);
    value.push_str(queue_job_id);
    value
}
