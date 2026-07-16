use ocentra_parent_agent_protocol::activity::policy::PolicyTargetType;
use ocentra_parent_agent_protocol::enforcement::EnforcementAdapterKind;
use ocentra_parent_agent_protocol::enforcement::EnforcementMode;

pub(super) fn adapter_kind(
    mode: EnforcementMode,
    target_type: PolicyTargetType,
) -> EnforcementAdapterKind {
    match (mode, target_type) {
        (
            EnforcementMode::TemporaryBlock,
            PolicyTargetType::Domain | PolicyTargetType::Category,
        ) => EnforcementAdapterKind::NetworkControl,
        (
            EnforcementMode::TemporaryBlock,
            PolicyTargetType::Site | PolicyTargetType::Video | PolicyTargetType::Channel,
        ) => EnforcementAdapterKind::ManagedBrowserControl,
        (mode, _) => match mode {
            EnforcementMode::TerminateProcess
            | EnforcementMode::BlockProcess
            | EnforcementMode::TemporaryBlock
            | EnforcementMode::TimeLimit => EnforcementAdapterKind::ProcessControl,
            EnforcementMode::AskParent | EnforcementMode::ObserveOnly => {
                EnforcementAdapterKind::TimerControl
            }
        },
    }
}
