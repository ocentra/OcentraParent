use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::AppGameTimerParentPreferenceSetupRequest;

#[derive(Default)]
pub(super) struct SetupRequestRefs {
    pub(super) parent_preference_mutation_receipt_id: String,
    pub(super) child_runtime_delivery_handoff_id: String,
    pub(super) child_runtime_delivery_queue_id: String,
    pub(super) child_runtime_delivery_dispatch_id: String,
    pub(super) child_runtime_delivery_receipt_requirement_id: String,
    pub(super) child_runtime_delivery_receipt_pending_id: String,
    pub(super) child_runtime_delivery_receipt_ingested_id: String,
    pub(super) durable_outbox_record_id: String,
    pub(super) provider_delivery_readiness_id: String,
    pub(super) provider_delivery_attempt_id: String,
    pub(super) provider_delivery_adapter_requirement_id: String,
    pub(super) provider_delivery_credential_requirement_id: String,
    pub(super) provider_delivery_queue_id: String,
    pub(super) provider_delivery_receipt_requirement_id: String,
    pub(super) provider_delivery_receipt_pending_id: String,
    pub(super) provider_delivery_receipt_ingested_id: String,
    pub(super) action_result_reference_ids: Vec<String>,
    pub(super) parent_preference_mutation_receipt_ids: Vec<String>,
    pub(super) child_runtime_delivery_handoff_ids: Vec<String>,
    pub(super) child_runtime_delivery_queue_ids: Vec<String>,
    pub(super) child_runtime_delivery_dispatch_ids: Vec<String>,
    pub(super) child_runtime_delivery_receipt_requirement_ids: Vec<String>,
    pub(super) child_runtime_delivery_receipt_pending_ids: Vec<String>,
    pub(super) child_runtime_delivery_receipt_ingested_ids: Vec<String>,
    pub(super) durable_outbox_record_ids: Vec<String>,
    pub(super) provider_delivery_readiness_ids: Vec<String>,
    pub(super) provider_delivery_attempt_ids: Vec<String>,
    pub(super) provider_delivery_adapter_requirement_ids: Vec<String>,
    pub(super) provider_delivery_credential_requirement_ids: Vec<String>,
    pub(super) provider_delivery_queue_ids: Vec<String>,
    pub(super) provider_delivery_receipt_requirement_ids: Vec<String>,
    pub(super) provider_delivery_receipt_pending_ids: Vec<String>,
    pub(super) provider_delivery_receipt_ingested_ids: Vec<String>,
}

struct SetupReferenceId(String);

struct SetupReferenceIds(Vec<String>);

struct SetupSuffix(&'static str);

struct SetupRequestIds {
    parent_preference_mutation_receipt_id: SetupReferenceId,
    child_runtime_delivery_handoff_id: SetupReferenceId,
    child_runtime_delivery_queue_id: SetupReferenceId,
    child_runtime_delivery_dispatch_id: SetupReferenceId,
    child_runtime_delivery_receipt_requirement_id: SetupReferenceId,
    child_runtime_delivery_receipt_pending_id: SetupReferenceId,
    child_runtime_delivery_receipt_ingested_id: SetupReferenceId,
    durable_outbox_record_id: SetupReferenceId,
    provider_delivery_readiness_id: SetupReferenceId,
    provider_delivery_attempt_id: SetupReferenceId,
    provider_delivery_adapter_requirement_id: SetupReferenceId,
    provider_delivery_credential_requirement_id: SetupReferenceId,
    provider_delivery_queue_id: SetupReferenceId,
    provider_delivery_receipt_requirement_id: SetupReferenceId,
    provider_delivery_receipt_pending_id: SetupReferenceId,
    provider_delivery_receipt_ingested_id: SetupReferenceId,
}

pub(super) fn setup_request_refs(
    request: &AppGameTimerParentPreferenceSetupRequest,
) -> SetupRequestRefs {
    let ids = setup_request_ids(request);
    let mut refs = SetupRequestRefs::default();
    assign_setup_scalar_ids(&ids, &mut refs);
    assign_setup_action_refs(request, &ids, &mut refs);
    assign_setup_provider_refs(&ids, &mut refs);
    refs
}

fn assign_setup_scalar_ids(ids: &SetupRequestIds, refs: &mut SetupRequestRefs) {
    refs.parent_preference_mutation_receipt_id =
        ids.parent_preference_mutation_receipt_id.0.clone();
    refs.child_runtime_delivery_handoff_id = ids.child_runtime_delivery_handoff_id.0.clone();
    refs.child_runtime_delivery_queue_id = ids.child_runtime_delivery_queue_id.0.clone();
    refs.child_runtime_delivery_dispatch_id = ids.child_runtime_delivery_dispatch_id.0.clone();
    refs.child_runtime_delivery_receipt_requirement_id =
        ids.child_runtime_delivery_receipt_requirement_id.0.clone();
    refs.child_runtime_delivery_receipt_pending_id =
        ids.child_runtime_delivery_receipt_pending_id.0.clone();
    refs.child_runtime_delivery_receipt_ingested_id =
        ids.child_runtime_delivery_receipt_ingested_id.0.clone();
    refs.durable_outbox_record_id = ids.durable_outbox_record_id.0.clone();
    refs.provider_delivery_readiness_id = ids.provider_delivery_readiness_id.0.clone();
    refs.provider_delivery_attempt_id = ids.provider_delivery_attempt_id.0.clone();
    refs.provider_delivery_adapter_requirement_id =
        ids.provider_delivery_adapter_requirement_id.0.clone();
    refs.provider_delivery_credential_requirement_id =
        ids.provider_delivery_credential_requirement_id.0.clone();
    refs.provider_delivery_queue_id = ids.provider_delivery_queue_id.0.clone();
    refs.provider_delivery_receipt_requirement_id =
        ids.provider_delivery_receipt_requirement_id.0.clone();
    refs.provider_delivery_receipt_pending_id = ids.provider_delivery_receipt_pending_id.0.clone();
    refs.provider_delivery_receipt_ingested_id =
        ids.provider_delivery_receipt_ingested_id.0.clone();
}

fn assign_setup_action_refs(
    request: &AppGameTimerParentPreferenceSetupRequest,
    ids: &SetupRequestIds,
    refs: &mut SetupRequestRefs,
) {
    refs.action_result_reference_ids = action_result_reference_ids(request).0;
    refs.parent_preference_mutation_receipt_ids =
        parent_preference_mutation_receipt_ids(request, &ids.parent_preference_mutation_receipt_id)
            .0;
    refs.child_runtime_delivery_handoff_ids = child_runtime_delivery_handoff_ids(
        request,
        &ids.parent_preference_mutation_receipt_id,
        &ids.child_runtime_delivery_handoff_id,
    )
    .0;
    refs.child_runtime_delivery_queue_ids = child_runtime_delivery_queue_ids(
        request,
        &ids.parent_preference_mutation_receipt_id,
        &ids.child_runtime_delivery_handoff_id,
        &ids.child_runtime_delivery_queue_id,
    )
    .0;
    refs.child_runtime_delivery_dispatch_ids = child_runtime_delivery_dispatch_ids(
        request,
        &ids.parent_preference_mutation_receipt_id,
        &ids.child_runtime_delivery_handoff_id,
        &ids.child_runtime_delivery_queue_id,
        &ids.child_runtime_delivery_dispatch_id,
    )
    .0;
    refs.child_runtime_delivery_receipt_requirement_ids =
        child_runtime_delivery_receipt_requirement_ids(
            request,
            &ids.parent_preference_mutation_receipt_id,
            &ids.child_runtime_delivery_handoff_id,
            &ids.child_runtime_delivery_queue_id,
            &ids.child_runtime_delivery_dispatch_id,
            &ids.child_runtime_delivery_receipt_requirement_id,
        )
        .0;
    refs.child_runtime_delivery_receipt_pending_ids = child_runtime_delivery_receipt_pending_ids(
        request,
        &ids.parent_preference_mutation_receipt_id,
        &ids.child_runtime_delivery_handoff_id,
        &ids.child_runtime_delivery_queue_id,
        &ids.child_runtime_delivery_dispatch_id,
        &ids.child_runtime_delivery_receipt_requirement_id,
        &ids.child_runtime_delivery_receipt_pending_id,
    )
    .0;
    refs.child_runtime_delivery_receipt_ingested_ids =
        child_runtime_delivery_receipt_ingested_ids(request, ids).0;
}

fn assign_setup_provider_refs(ids: &SetupRequestIds, refs: &mut SetupRequestRefs) {
    refs.durable_outbox_record_ids = unique_refs(SetupReferenceIds(vec![
        ids.durable_outbox_record_id.0.clone(),
        ids.child_runtime_delivery_receipt_ingested_id.0.clone(),
    ]))
    .0;
    refs.provider_delivery_readiness_ids = unique_refs(SetupReferenceIds(vec![
        ids.provider_delivery_readiness_id.0.clone(),
        ids.durable_outbox_record_id.0.clone(),
    ]))
    .0;
    refs.provider_delivery_attempt_ids = unique_refs(SetupReferenceIds(vec![
        ids.provider_delivery_attempt_id.0.clone(),
        ids.provider_delivery_readiness_id.0.clone(),
    ]))
    .0;
    refs.provider_delivery_adapter_requirement_ids = unique_refs(SetupReferenceIds(vec![
        ids.provider_delivery_adapter_requirement_id.0.clone(),
        ids.provider_delivery_attempt_id.0.clone(),
    ]))
    .0;
    refs.provider_delivery_credential_requirement_ids = unique_refs(SetupReferenceIds(vec![
        ids.provider_delivery_credential_requirement_id.0.clone(),
        ids.provider_delivery_adapter_requirement_id.0.clone(),
    ]))
    .0;
    refs.provider_delivery_queue_ids = unique_refs(SetupReferenceIds(vec![
        ids.provider_delivery_queue_id.0.clone(),
        ids.provider_delivery_credential_requirement_id.0.clone(),
    ]))
    .0;
    refs.provider_delivery_receipt_requirement_ids = unique_refs(SetupReferenceIds(vec![
        ids.provider_delivery_receipt_requirement_id.0.clone(),
        ids.provider_delivery_queue_id.0.clone(),
    ]))
    .0;
    refs.provider_delivery_receipt_pending_ids = unique_refs(SetupReferenceIds(vec![
        ids.provider_delivery_receipt_pending_id.0.clone(),
        ids.provider_delivery_receipt_requirement_id.0.clone(),
    ]))
    .0;
    refs.provider_delivery_receipt_ingested_ids = unique_refs(SetupReferenceIds(vec![
        ids.provider_delivery_receipt_ingested_id.0.clone(),
        ids.provider_delivery_receipt_pending_id.0.clone(),
    ]))
    .0;
}

fn action_result_reference_ids(
    request: &AppGameTimerParentPreferenceSetupRequest,
) -> SetupReferenceIds {
    let mut refs = vec![request.parent_preference_setup_reference_id.clone()];
    refs.extend(request.request_reference_ids.clone());
    unique_refs(SetupReferenceIds(refs))
}

fn child_runtime_delivery_receipt_ingested_ids(
    request: &AppGameTimerParentPreferenceSetupRequest,
    ids: &SetupRequestIds,
) -> SetupReferenceIds {
    let mut refs = vec![
        ids.child_runtime_delivery_receipt_ingested_id.0.clone(),
        ids.child_runtime_delivery_receipt_pending_id.0.clone(),
        ids.child_runtime_delivery_receipt_requirement_id.0.clone(),
        ids.child_runtime_delivery_dispatch_id.0.clone(),
        ids.child_runtime_delivery_queue_id.0.clone(),
        ids.child_runtime_delivery_handoff_id.0.clone(),
        ids.parent_preference_mutation_receipt_id.0.clone(),
        request.parent_preference_setup_reference_id.clone(),
    ];
    refs.extend(request.request_reference_ids.clone());
    unique_refs(SetupReferenceIds(refs))
}

fn setup_request_ids(request: &AppGameTimerParentPreferenceSetupRequest) -> SetupRequestIds {
    let request_id = |suffix| parent_preference_setup_suffixed_id(request, &SetupSuffix(suffix));
    SetupRequestIds {
        parent_preference_mutation_receipt_id: request_id(
            constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_MUTATION_RECEIPT_SUFFIX,
        ),
        child_runtime_delivery_handoff_id: request_id(
            constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_HANDOFF_SUFFIX,
        ),
        child_runtime_delivery_queue_id: request_id(
            constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_QUEUE_SUFFIX,
        ),
        child_runtime_delivery_dispatch_id: request_id(
            constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_DISPATCH_SUFFIX,
        ),
        child_runtime_delivery_receipt_requirement_id: request_id(
            constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_RECEIPT_REQUIREMENT_SUFFIX,
        ),
        child_runtime_delivery_receipt_pending_id: request_id(
            constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_RECEIPT_PENDING_SUFFIX,
        ),
        child_runtime_delivery_receipt_ingested_id: request_id(
            constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_RECEIPT_INGESTED_SUFFIX,
        ),
        durable_outbox_record_id: request_id(
            constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_DURABLE_OUTBOX_SUFFIX,
        ),
        provider_delivery_readiness_id: request_id(
            constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_PROVIDER_DELIVERY_READINESS_SUFFIX,
        ),
        provider_delivery_attempt_id: request_id(
            constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_PROVIDER_DELIVERY_ATTEMPT_SUFFIX,
        ),
        provider_delivery_adapter_requirement_id: request_id(
            constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_PROVIDER_DELIVERY_ADAPTER_REQUIREMENT_SUFFIX,
        ),
        provider_delivery_credential_requirement_id: request_id(
            constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_PROVIDER_DELIVERY_CREDENTIAL_REQUIREMENT_SUFFIX,
        ),
        provider_delivery_queue_id: request_id(
            constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_PROVIDER_DELIVERY_QUEUE_SUFFIX,
        ),
        provider_delivery_receipt_requirement_id: request_id(
            constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_PROVIDER_DELIVERY_RECEIPT_REQUIREMENT_SUFFIX,
        ),
        provider_delivery_receipt_pending_id: request_id(
            constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_PROVIDER_DELIVERY_RECEIPT_PENDING_SUFFIX,
        ),
        provider_delivery_receipt_ingested_id: request_id(
            constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_PROVIDER_DELIVERY_RECEIPT_INGESTED_SUFFIX,
        ),
    }
}

fn parent_preference_setup_suffixed_id(
    request: &AppGameTimerParentPreferenceSetupRequest,
    suffix: &SetupSuffix,
) -> SetupReferenceId {
    let mut reference_id = request.parent_preference_setup_reference_id.clone();
    reference_id.push(constants::delimiter::HYPHEN);
    reference_id.push_str(suffix.0);
    SetupReferenceId(reference_id)
}

fn parent_preference_mutation_receipt_ids(
    request: &AppGameTimerParentPreferenceSetupRequest,
    receipt_id: &SetupReferenceId,
) -> SetupReferenceIds {
    let mut refs = vec![
        receipt_id.0.clone(),
        request.parent_preference_setup_reference_id.clone(),
    ];
    refs.extend(request.request_reference_ids.clone());
    unique_refs(SetupReferenceIds(refs))
}

fn child_runtime_delivery_handoff_ids(
    request: &AppGameTimerParentPreferenceSetupRequest,
    receipt_id: &SetupReferenceId,
    handoff_id: &SetupReferenceId,
) -> SetupReferenceIds {
    let mut refs = vec![
        handoff_id.0.clone(),
        receipt_id.0.clone(),
        request.parent_preference_setup_reference_id.clone(),
    ];
    refs.extend(request.request_reference_ids.clone());
    unique_refs(SetupReferenceIds(refs))
}

fn child_runtime_delivery_queue_ids(
    request: &AppGameTimerParentPreferenceSetupRequest,
    receipt_id: &SetupReferenceId,
    handoff_id: &SetupReferenceId,
    queue_id: &SetupReferenceId,
) -> SetupReferenceIds {
    let mut refs = vec![
        queue_id.0.clone(),
        handoff_id.0.clone(),
        receipt_id.0.clone(),
        request.parent_preference_setup_reference_id.clone(),
    ];
    refs.extend(request.request_reference_ids.clone());
    unique_refs(SetupReferenceIds(refs))
}

fn child_runtime_delivery_dispatch_ids(
    request: &AppGameTimerParentPreferenceSetupRequest,
    receipt_id: &SetupReferenceId,
    handoff_id: &SetupReferenceId,
    queue_id: &SetupReferenceId,
    dispatch_id: &SetupReferenceId,
) -> SetupReferenceIds {
    let mut refs = vec![
        dispatch_id.0.clone(),
        queue_id.0.clone(),
        handoff_id.0.clone(),
        receipt_id.0.clone(),
        request.parent_preference_setup_reference_id.clone(),
    ];
    refs.extend(request.request_reference_ids.clone());
    unique_refs(SetupReferenceIds(refs))
}

fn child_runtime_delivery_receipt_requirement_ids(
    request: &AppGameTimerParentPreferenceSetupRequest,
    receipt_id: &SetupReferenceId,
    handoff_id: &SetupReferenceId,
    queue_id: &SetupReferenceId,
    dispatch_id: &SetupReferenceId,
    receipt_requirement_id: &SetupReferenceId,
) -> SetupReferenceIds {
    let mut refs = vec![
        receipt_requirement_id.0.clone(),
        dispatch_id.0.clone(),
        queue_id.0.clone(),
        handoff_id.0.clone(),
        receipt_id.0.clone(),
        request.parent_preference_setup_reference_id.clone(),
    ];
    refs.extend(request.request_reference_ids.clone());
    unique_refs(SetupReferenceIds(refs))
}

fn child_runtime_delivery_receipt_pending_ids(
    request: &AppGameTimerParentPreferenceSetupRequest,
    receipt_id: &SetupReferenceId,
    handoff_id: &SetupReferenceId,
    queue_id: &SetupReferenceId,
    dispatch_id: &SetupReferenceId,
    receipt_requirement_id: &SetupReferenceId,
    receipt_pending_id: &SetupReferenceId,
) -> SetupReferenceIds {
    let mut refs = vec![
        receipt_pending_id.0.clone(),
        receipt_requirement_id.0.clone(),
        dispatch_id.0.clone(),
        queue_id.0.clone(),
        handoff_id.0.clone(),
        receipt_id.0.clone(),
        request.parent_preference_setup_reference_id.clone(),
    ];
    refs.extend(request.request_reference_ids.clone());
    unique_refs(SetupReferenceIds(refs))
}

fn unique_refs(reference_ids: SetupReferenceIds) -> SetupReferenceIds {
    let mut unique = Vec::new();
    for reference_id in reference_ids.0 {
        if reference_id.is_empty() || unique.iter().any(|existing| existing == &reference_id) {
            continue;
        }
        unique.push(reference_id);
    }
    SetupReferenceIds(unique)
}
