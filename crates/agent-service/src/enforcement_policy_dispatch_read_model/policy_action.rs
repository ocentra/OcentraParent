use ocentra_parent_agent_protocol::activity::policy::PolicyAction;
use ocentra_parent_agent_protocol::enforcement_product_control_spine::V08EnforcementProductControlParentAction;

pub(super) fn policy_action_for(action: V08EnforcementProductControlParentAction) -> PolicyAction {
    match action {
        V08EnforcementProductControlParentAction::Warn => PolicyAction::Warn,
        V08EnforcementProductControlParentAction::TimeLimit => PolicyAction::TimeLimit,
        V08EnforcementProductControlParentAction::BlockScopedProcess => PolicyAction::Block,
        V08EnforcementProductControlParentAction::AskParent => PolicyAction::AskParent,
        V08EnforcementProductControlParentAction::Observe
        | V08EnforcementProductControlParentAction::DryRunPreview
        | V08EnforcementProductControlParentAction::ReportOnly => PolicyAction::Allow,
    }
}
