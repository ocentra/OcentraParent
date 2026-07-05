#![forbid(unsafe_code)]

use crate::policy_event::PolicyEventScope;

pub(crate) fn policy_event_scope_family_label(scope: &PolicyEventScope) -> &'static str {
    match scope {
        PolicyEventScope::SourceDocument { .. } => "source-document",
        PolicyEventScope::Request { .. } => "request",
        PolicyEventScope::Approval { .. } => "approval",
        PolicyEventScope::Override { .. } => "override",
        PolicyEventScope::Delivery { .. } => "delivery",
        PolicyEventScope::Rollback { .. } => "rollback",
        PolicyEventScope::Audit { .. } => "audit",
    }
}
