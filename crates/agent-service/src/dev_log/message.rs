use super::{AgentLogMessageRef, AgentLogMessageSource};

impl AgentLogMessageSource for AgentLogMessageRef<'_> {
    fn as_agent_log_message_ref(&self) -> AgentLogMessageRef<'_> {
        *self
    }
}

impl AgentLogMessageSource for String {
    fn as_agent_log_message_ref(&self) -> AgentLogMessageRef<'_> {
        AgentLogMessageRef(self.as_str())
    }
}

impl AgentLogMessageSource for &String {
    fn as_agent_log_message_ref(&self) -> AgentLogMessageRef<'_> {
        AgentLogMessageRef(self.as_str())
    }
}

impl AgentLogMessageSource for &str {
    fn as_agent_log_message_ref(&self) -> AgentLogMessageRef<'_> {
        AgentLogMessageRef(self)
    }
}
