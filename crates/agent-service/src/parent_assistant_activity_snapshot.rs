#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ParentAssistantActivitySnapshot {
    pub(crate) device_id: String,
    pub(crate) recent_returned: u64,
    pub(crate) last_event_id: Option<String>,
    pub(crate) last_observed_at: Option<String>,
    pub(crate) browser_returned: u64,
    pub(crate) network_returned: u64,
    pub(crate) games_returned: u64,
    pub(crate) screen_returned: u64,
}
