use super::common::{parse_identifier, parse_optional_identifier};
use super::*;

pub(super) fn parent_lan_canonical_household_device_snapshot(
    device: &LanCanonicalHouseholdDevice,
) -> ParentLanCanonicalHouseholdDeviceSnapshot {
    ParentLanCanonicalHouseholdDeviceSnapshot {
        schema_version: device.schema_version,
        canonical_device_id: parse_identifier(
            device.canonical_device_id.clone(),
            "lan canonical_device_id",
            ParentLanCanonicalDeviceId::parse,
        ),
        display_name: device.display_name.clone(),
        classification: serialized_enum_label(&device.classification),
        role_badges: device
            .role_badges
            .iter()
            .map(serialized_enum_label)
            .collect(),
        enrollable: device.enrollable,
        discovery_state: serialized_enum_label(&device.discovery_state),
        trust_state: serialized_enum_label(&device.trust_state),
        route_id: parse_optional_identifier(device.route_id.clone(), |value| {
            ParentLanRouteId::parse(value)
        }),
        route_state: serialized_enum_label(&device.route_state),
        network_mode: serialized_enum_label(&device.network_mode),
        source_labels: device
            .source_labels
            .iter()
            .map(serialized_enum_label)
            .collect(),
        network_identity: parent_lan_canonical_household_network_identity_snapshot(
            &device.network_identity,
        ),
        child_agent_inventory: device
            .child_agent_inventory
            .as_ref()
            .map(parent_lan_child_agent_inventory_packet_snapshot),
        policy_target_surfaces: device
            .policy_target_surfaces
            .iter()
            .map(serialized_enum_label)
            .collect(),
    }
}

pub(super) fn parent_lan_trusted_device_registry_entry_snapshot(
    entry: &LanTrustedDeviceRegistryEntry,
) -> ParentLanTrustedDeviceRegistryEntrySnapshot {
    ParentLanTrustedDeviceRegistryEntrySnapshot {
        schema_version: entry.schema_version,
        pairing_id: parse_identifier(entry.pairing_id.clone(), "lan pairing_id", |value| {
            ParentLanPairingId::parse(value)
        }),
        child_device: parent_lan_pairing_device_ref_snapshot(&entry.child_device),
        parent_device: parent_lan_pairing_device_ref_snapshot(&entry.parent_device),
        route_id: parse_identifier(entry.route_id.clone(), "lan route_id", |value| {
            ParentLanRouteId::parse(value)
        }),
        origin: entry.origin.clone(),
        proof_digest: entry.proof_digest.clone(),
        trust_state: serialized_enum_label(&entry.trust_state),
        trusted_at: entry.trusted_at.clone(),
        expires_at: entry.expires_at.clone(),
        revoked_at: entry.revoked_at.clone(),
    }
}

pub(super) fn parent_lan_household_device_decision_snapshot(
    decision: &LanHouseholdDeviceDecision,
) -> ParentLanHouseholdDeviceDecisionSnapshot {
    ParentLanHouseholdDeviceDecisionSnapshot {
        schema_version: decision.schema_version,
        action_id: parse_identifier(decision.action_id.clone(), "lan action_id", |value| {
            ParentLanActionId::parse(value)
        }),
        action_kind: serialized_enum_label(&decision.action_kind),
        canonical_device_id: parse_identifier(
            decision.canonical_device_id.clone(),
            "lan canonical_device_id",
            ParentLanCanonicalDeviceId::parse,
        ),
        child_profile_id: parse_optional_identifier(decision.child_profile_id.clone(), |value| {
            ParentContractReferenceId::parse(value)
        }),
        display_name: decision.display_name.clone(),
        device_kind: decision.device_kind.clone(),
        parent_actor_id: parse_identifier(
            decision.parent_actor_id.clone(),
            "parent_actor_id",
            ParentParentActorId::parse,
        ),
        decided_at: decision.decided_at.clone(),
        revoked_at: decision.revoked_at.clone(),
    }
}

pub(super) fn parent_lan_selected_device_readiness_snapshot(
    readiness: &LanSelectedDeviceReadiness,
) -> ParentLanSelectedDeviceReadinessSnapshot {
    ParentLanSelectedDeviceReadinessSnapshot {
        schema_version: readiness.schema_version,
        selected_child_device_id: parse_optional_identifier(
            readiness.selected_child_device_id.clone(),
            ParentChildDeviceId::parse,
        ),
        route_id: parse_optional_identifier(readiness.route_id.clone(), |value| {
            ParentLanRouteId::parse(value)
        }),
        pairing_id: parse_optional_identifier(readiness.pairing_id.clone(), |value| {
            ParentLanPairingId::parse(value)
        }),
        trust_state: serialized_enum_label(&readiness.trust_state),
        reachability: serialized_enum_label(&readiness.reachability),
        ready_for_control: readiness.ready_for_control,
        stale_at: readiness.stale_at.clone(),
        offline_at: readiness.offline_at.clone(),
    }
}

pub(super) fn parent_lan_signed_discovery_relay_adapter_row_snapshot(
    row: &LanSignedDiscoveryRelayAdapterRow,
) -> ParentLanSignedDiscoveryRelayAdapterRowSnapshot {
    ParentLanSignedDiscoveryRelayAdapterRowSnapshot {
        schema_version: row.schema_version,
        adapter: serialized_enum_label(&row.adapter),
        discovery_state: serialized_enum_label(&row.discovery_state),
        proof_state: serialized_enum_label(&row.proof_state),
        source_confidence: serialized_enum_label(&row.source_confidence),
        custody_label: serialized_enum_label(&row.custody_label),
        runtime_owner: serialized_enum_label(&row.runtime_owner),
        evidence_label: row.evidence_label.clone(),
        required_artifact_summary: row.required_artifact_summary.clone(),
    }
}

pub(super) fn parent_lan_signed_discovery_relay_signed_proof_row_snapshot(
    row: &LanSignedDiscoveryRelaySignedProofRow,
) -> ParentLanSignedDiscoveryRelaySignedProofRowSnapshot {
    ParentLanSignedDiscoveryRelaySignedProofRowSnapshot {
        schema_version: row.schema_version,
        check: serialized_enum_label(&row.check),
        discovery_state: serialized_enum_label(&row.discovery_state),
        response_state: serialized_enum_label(&row.response_state),
        rejection_reason: row.rejection_reason.as_ref().map(serialized_enum_label),
        proof_state: serialized_enum_label(&row.proof_state),
        runtime_owner: serialized_enum_label(&row.runtime_owner),
        evidence_label: row.evidence_label.clone(),
    }
}

pub(super) fn parent_lan_signed_discovery_relay_route_safety_row_snapshot(
    row: &LanSignedDiscoveryRelayRouteSafetyRow,
) -> ParentLanSignedDiscoveryRelayRouteSafetyRowSnapshot {
    ParentLanSignedDiscoveryRelayRouteSafetyRowSnapshot {
        schema_version: row.schema_version,
        check: serialized_enum_label(&row.check),
        route_id: parse_optional_identifier(row.route_id.clone(), |value| {
            ParentLanRouteId::parse(value)
        }),
        discovery_state: serialized_enum_label(&row.discovery_state),
        response_state: serialized_enum_label(&row.response_state),
        rejection_reason: row.rejection_reason.as_ref().map(serialized_enum_label),
        proof_state: serialized_enum_label(&row.proof_state),
        runtime_owner: serialized_enum_label(&row.runtime_owner),
        custody_label: serialized_enum_label(&row.custody_label),
        evidence_label: row.evidence_label.clone(),
    }
}

pub(super) fn parent_lan_signed_discovery_relay_cache_row_snapshot(
    row: &LanSignedDiscoveryRelayCacheRow,
) -> ParentLanSignedDiscoveryRelayCacheRowSnapshot {
    ParentLanSignedDiscoveryRelayCacheRowSnapshot {
        schema_version: row.schema_version,
        check: serialized_enum_label(&row.check),
        decision_state: serialized_enum_label(&row.decision_state),
        discovery_state: serialized_enum_label(&row.discovery_state),
        proof_state: serialized_enum_label(&row.proof_state),
        runtime_owner: serialized_enum_label(&row.runtime_owner),
        custody_label: serialized_enum_label(&row.custody_label),
        evidence_label: row.evidence_label.clone(),
    }
}

pub(super) fn parent_lan_signed_discovery_relay_spine_summary_snapshot(
    summary: &LanSignedDiscoveryRelaySpineSummary,
) -> ParentLanSignedDiscoveryRelaySpineSummarySnapshot {
    ParentLanSignedDiscoveryRelaySpineSummarySnapshot {
        schema_version: summary.schema_version,
        generated_at: summary.generated_at.clone(),
        adapter_rows: summary
            .adapter_rows
            .iter()
            .map(parent_lan_signed_discovery_relay_adapter_row_snapshot)
            .collect(),
        signed_proof_rows: summary
            .signed_proof_rows
            .iter()
            .map(parent_lan_signed_discovery_relay_signed_proof_row_snapshot)
            .collect(),
        route_safety_rows: summary
            .route_safety_rows
            .iter()
            .map(parent_lan_signed_discovery_relay_route_safety_row_snapshot)
            .collect(),
        relay_cache_rows: summary
            .relay_cache_rows
            .iter()
            .map(parent_lan_signed_discovery_relay_cache_row_snapshot)
            .collect(),
        manual_proof_required: summary
            .manual_proof_required
            .iter()
            .map(serialized_enum_label)
            .collect(),
        not_implemented: summary
            .not_implemented
            .iter()
            .map(serialized_enum_label)
            .collect(),
        claims_proved: summary.claims_proved.clone(),
        claims_not_proved: summary.claims_not_proved.clone(),
    }
}

pub(super) fn parent_lan_discovery_source_matrix_snapshot(
    matrix: &LanDiscoverySourceMatrix,
) -> ParentLanDiscoverySourceMatrixSnapshot {
    ParentLanDiscoverySourceMatrixSnapshot {
        schema_version: matrix.schema_version,
        generated_at: matrix.generated_at.clone(),
        workpack_rows: matrix
            .workpack_rows
            .iter()
            .map(parent_lan_discovery_source_matrix_workpack_row_snapshot)
            .collect(),
        source_rows: matrix
            .source_rows
            .iter()
            .map(parent_lan_discovery_source_matrix_source_row_snapshot)
            .collect(),
        claims_proved: matrix.claims_proved.clone(),
        claims_not_proved: matrix.claims_not_proved.clone(),
    }
}

pub(super) fn parent_lan_discovery_source_matrix_workpack_row_snapshot(
    row: &LanPlanWorkpackStatusRow,
) -> ParentLanDiscoverySourceMatrixWorkpackRowSnapshot {
    ParentLanDiscoverySourceMatrixWorkpackRowSnapshot {
        workpack_id: parse_identifier(
            serialized_enum_label(&row.workpack_id),
            "lan workpack_id",
            ParentLanWorkpackId::parse,
        ),
        title: row.title.clone(),
        discovery_state: serialized_enum_label(&row.discovery_state),
        proof_state: serialized_enum_label(&row.proof_state),
        runtime_owner: serialized_enum_label(&row.runtime_owner),
        status: serialized_enum_label(&row.status),
        read_model_visible: row.read_model_visible,
        required_artifact_summary: row.required_artifact_summary.clone(),
    }
}

pub(super) fn parent_lan_discovery_source_matrix_source_row_snapshot(
    row: &LanDiscoverySourceRow,
) -> ParentLanDiscoverySourceMatrixSourceRowSnapshot {
    ParentLanDiscoverySourceMatrixSourceRowSnapshot {
        source: serialized_enum_label(&row.source),
        workpack_id: parse_identifier(
            serialized_enum_label(&row.workpack_id),
            "lan workpack_id",
            ParentLanWorkpackId::parse,
        ),
        status: serialized_enum_label(&row.status),
        authority: serialized_enum_label(&row.authority),
        runtime_path: serialized_enum_label(&row.runtime_path),
        ui_surface: serialized_enum_label(&row.ui_surface),
        can_confirm_child_agent: row.can_confirm_child_agent,
        can_assign_child_profile: row.can_assign_child_profile,
        can_control_route: row.can_control_route,
        requires_selected_interface: row.requires_selected_interface,
        persists_across_restart: row.persists_across_restart,
        evidence_label: row.evidence_label.clone(),
        required_artifact_summary: row.required_artifact_summary.clone(),
    }
}
