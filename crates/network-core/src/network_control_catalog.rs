#[path = "network_control_catalog_text.rs"]
mod network_control_catalog_text;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NetworkControlKind {
    Toggle,
    SingleChoice,
    MultiChoice,
    Number,
    Duration,
    Schedule,
    RuleList,
    TargetList,
    Retention,
    ActionList,
    ReadOnlyStatus,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NetworkControlSelectionMode {
    Single,
    Multi,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NetworkControlCardKind {
    SingleChoiceCompact,
    SingleChoiceMany,
    MultiChoiceNormal,
    MultiChoiceMany,
    Toggle,
    ScheduleCard,
    RuleListCard,
    TargetListCard,
    RetentionCard,
    StatusCard,
    NumberCard,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NetworkControlEffectStatus {
    AlreadyRepresented,
    NeedsEffectWiring,
    ManualRequired,
    Unavailable,
    FutureGap,
    Degraded,
    PermissionRequired,
    PermissionLimited,
    ProofRequired,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NetworkControlRuntimeOwner {
    PortalOnly,
    RustParentRuntime,
    AgentProtocol,
    RustService,
    ChildAgent,
    OsAdapter,
    ManualProof,
    ParentOwnedStorage,
    LocalAiRuntime,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NetworkControlCapabilityState {
    Available,
    Disabled,
    Unsupported,
    PermissionRequired,
    PermissionLimited,
    Protected,
    Degraded,
    ManualRequired,
    FutureGap,
    Unavailable,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NetworkControlLayoutHints {
    pub preferred_column_span: usize,
    pub collapsible: bool,
    pub searchable_options: bool,
    pub option_group_count: usize,
    pub show_as_matrix_when_large: bool,
    pub show_selected_count: bool,
}

pub mod capability_state;
pub mod card_kind;
pub mod control_kind;
pub mod effect_status;
pub mod fallback;
pub mod layout_hints;
pub mod policy;
pub mod proof_requirement;
pub mod question;
pub mod requirements;
pub mod runtime_owner;
pub mod selection_mode;
pub mod source_state;
pub mod tokens;
