#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TrackingLocalPlaceKind {
    Home,
    School,
    Activity,
    SafeZone,
    RestrictedZone,
    Custom,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrackingLocalParentDefinedPlace {
    pub place_id: String,
    pub label: String,
    pub place_kind: TrackingLocalPlaceKind,
    pub updated_at: String,
    pub audit_refs: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrackingLocalParentDefinedPlaceTombstone {
    pub place_id: String,
    pub deleted_at: String,
    pub reason_codes: Vec<String>,
    pub audit_refs: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrackingLocalParentDefinedPlaceStore {
    pub store_id: String,
    pub created_at: String,
    pub updated_at: String,
    pub storage_boundary: String,
    pub remote_sync_default: String,
    pub ocentra_hosted_default_storage: bool,
    pub places: Vec<TrackingLocalParentDefinedPlace>,
    pub tombstones: Vec<TrackingLocalParentDefinedPlaceTombstone>,
    pub audit_refs: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TrackingLocalPlaceMutationKind {
    Create,
    Update,
    Import,
    Delete,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrackingLocalParentDefinedPlaceMutationReceipt {
    pub operation: TrackingLocalPlaceMutationKind,
    pub before_place_count: usize,
    pub after_place_count: usize,
    pub remote_sync_default: String,
    pub ocentra_hosted_default_storage: bool,
    pub audit_refs: Vec<String>,
    pub store: TrackingLocalParentDefinedPlaceStore,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrackingLocalParentDefinedPlaceExportSnapshot {
    pub store_id: String,
    pub exported_at: String,
    pub custody_label: String,
    pub remote_sync_default: String,
    pub ocentra_hosted_default_storage: bool,
    pub places: Vec<TrackingLocalParentDefinedPlace>,
    pub tombstones: Vec<TrackingLocalParentDefinedPlaceTombstone>,
    pub audit_refs: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TrackingLocalPlacePolicySignal {
    SafeZoneContext,
    RestrictedZoneAttention,
    ExpectedPlaceContext,
    CustomPlaceContext,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TrackingPlaceRiskCategory {
    Home,
    School,
    Park,
    Unknown,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrackingLocalParentDefinedPlaceMatch {
    pub place_id: String,
    pub place_kind: TrackingLocalPlaceKind,
    pub distance_meters: u32,
    pub query_radius_meters: u32,
    pub ambiguity_state: String,
    pub policy_signal: TrackingLocalPlacePolicySignal,
    pub reason_codes: Vec<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TrackingNearbyPlaceEvidence {
    pub provider_kind: String,
    pub provider_ref: String,
    pub query_radius_meters: u32,
    pub distance_meters: u32,
    pub category: TrackingPlaceRiskCategory,
    pub confidence: f64,
    pub ambiguity_state: String,
    pub reason_codes: Vec<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TrackingLocalParentDefinedPlaceMatchDecision {
    pub place_match: TrackingLocalParentDefinedPlaceMatch,
    pub nearby_place_evidence: TrackingNearbyPlaceEvidence,
}

pub fn create_tracking_local_parent_defined_place_store(
    store_id: impl Into<String>,
    created_at: impl Into<String>,
    audit_refs: Vec<String>,
) -> TrackingLocalParentDefinedPlaceStore {
    let created_at = created_at.into();
    TrackingLocalParentDefinedPlaceStore {
        store_id: store_id.into(),
        created_at: created_at.clone(),
        updated_at: created_at,
        storage_boundary: "parent-device-local".to_owned(),
        remote_sync_default: "disabled".to_owned(),
        ocentra_hosted_default_storage: false,
        places: Vec::new(),
        tombstones: Vec::new(),
        audit_refs,
    }
}

pub fn upsert_tracking_local_parent_defined_place(
    store: &TrackingLocalParentDefinedPlaceStore,
    place: TrackingLocalParentDefinedPlace,
    audit_refs: Vec<String>,
) -> TrackingLocalParentDefinedPlaceMutationReceipt {
    let operation = if store
        .places
        .iter()
        .any(|entry| entry.place_id == place.place_id)
    {
        TrackingLocalPlaceMutationKind::Update
    } else {
        TrackingLocalPlaceMutationKind::Create
    };
    let updated_at = place.updated_at.clone();
    let mut places = store
        .places
        .iter()
        .filter(|entry| entry.place_id != place.place_id)
        .cloned()
        .collect::<Vec<_>>();
    places.push(place);

    mutation_receipt(store, places, None, updated_at, audit_refs, operation)
}

pub fn import_tracking_local_parent_defined_places(
    store: &TrackingLocalParentDefinedPlaceStore,
    places: Vec<TrackingLocalParentDefinedPlace>,
    imported_at: impl Into<String>,
    audit_refs: Vec<String>,
) -> TrackingLocalParentDefinedPlaceMutationReceipt {
    let place_ids = places
        .iter()
        .map(|place| place.place_id.clone())
        .collect::<std::collections::BTreeSet<_>>();
    let mut merged_places = store
        .places
        .iter()
        .filter(|place| !place_ids.contains(&place.place_id))
        .cloned()
        .collect::<Vec<_>>();
    merged_places.extend(places);

    mutation_receipt(
        store,
        merged_places,
        None,
        imported_at.into(),
        audit_refs,
        TrackingLocalPlaceMutationKind::Import,
    )
}

pub fn delete_tracking_local_parent_defined_place(
    store: &TrackingLocalParentDefinedPlaceStore,
    place_id: impl Into<String>,
    deleted_at: impl Into<String>,
    reason_codes: Vec<String>,
    audit_refs: Vec<String>,
) -> TrackingLocalParentDefinedPlaceMutationReceipt {
    let place_id = place_id.into();
    let deleted_at = deleted_at.into();
    let tombstone = TrackingLocalParentDefinedPlaceTombstone {
        place_id: place_id.clone(),
        deleted_at: deleted_at.clone(),
        reason_codes,
        audit_refs: audit_refs.clone(),
    };
    let tombstones = store
        .tombstones
        .iter()
        .filter(|entry| entry.place_id != place_id)
        .cloned()
        .chain([tombstone])
        .collect::<Vec<_>>();
    let places = store
        .places
        .iter()
        .filter(|place| place.place_id != place_id)
        .cloned()
        .collect::<Vec<_>>();

    mutation_receipt(
        store,
        places,
        Some(tombstones),
        deleted_at,
        audit_refs,
        TrackingLocalPlaceMutationKind::Delete,
    )
}

pub fn export_tracking_local_parent_defined_place_store(
    store: &TrackingLocalParentDefinedPlaceStore,
    exported_at: impl Into<String>,
    audit_refs: Vec<String>,
) -> TrackingLocalParentDefinedPlaceExportSnapshot {
    TrackingLocalParentDefinedPlaceExportSnapshot {
        store_id: store.store_id.clone(),
        exported_at: exported_at.into(),
        custody_label: "parent-owned-export".to_owned(),
        remote_sync_default: "disabled".to_owned(),
        ocentra_hosted_default_storage: false,
        places: store.places.clone(),
        tombstones: store.tombstones.clone(),
        audit_refs,
    }
}

pub fn build_tracking_local_parent_defined_place_match(
    store: &TrackingLocalParentDefinedPlaceStore,
    place_id: &str,
    query_radius_meters: u32,
    distance_meters: u32,
    confidence: f64,
    reason_codes: Vec<String>,
) -> Result<TrackingLocalParentDefinedPlaceMatchDecision, &'static str> {
    let place = store
        .places
        .iter()
        .find(|candidate| candidate.place_id == place_id)
        .ok_or("tracking local parent-defined place match needs an existing place")?;

    Ok(TrackingLocalParentDefinedPlaceMatchDecision {
        place_match: TrackingLocalParentDefinedPlaceMatch {
            place_id: place.place_id.clone(),
            place_kind: place.place_kind.clone(),
            distance_meters,
            query_radius_meters,
            ambiguity_state: "clear".to_owned(),
            policy_signal: policy_signal_for_place(place),
            reason_codes: reason_codes.clone(),
        },
        nearby_place_evidence: TrackingNearbyPlaceEvidence {
            provider_kind: "parent-defined".to_owned(),
            provider_ref: store.store_id.clone(),
            query_radius_meters,
            distance_meters,
            category: category_for_place(place),
            confidence,
            ambiguity_state: "clear".to_owned(),
            reason_codes,
        },
    })
}

fn mutation_receipt(
    store: &TrackingLocalParentDefinedPlaceStore,
    places: Vec<TrackingLocalParentDefinedPlace>,
    tombstones: Option<Vec<TrackingLocalParentDefinedPlaceTombstone>>,
    updated_at: String,
    audit_refs: Vec<String>,
    operation: TrackingLocalPlaceMutationKind,
) -> TrackingLocalParentDefinedPlaceMutationReceipt {
    let next_store = TrackingLocalParentDefinedPlaceStore {
        store_id: store.store_id.clone(),
        created_at: store.created_at.clone(),
        updated_at,
        storage_boundary: store.storage_boundary.clone(),
        remote_sync_default: store.remote_sync_default.clone(),
        ocentra_hosted_default_storage: store.ocentra_hosted_default_storage,
        places,
        tombstones: tombstones.unwrap_or_else(|| store.tombstones.clone()),
        audit_refs: store
            .audit_refs
            .iter()
            .cloned()
            .chain(audit_refs.iter().cloned())
            .collect(),
    };

    TrackingLocalParentDefinedPlaceMutationReceipt {
        operation,
        before_place_count: store.places.len(),
        after_place_count: next_store.places.len(),
        remote_sync_default: "disabled".to_owned(),
        ocentra_hosted_default_storage: false,
        audit_refs,
        store: next_store,
    }
}

fn policy_signal_for_place(
    place: &TrackingLocalParentDefinedPlace,
) -> TrackingLocalPlacePolicySignal {
    match place.place_kind {
        TrackingLocalPlaceKind::SafeZone => TrackingLocalPlacePolicySignal::SafeZoneContext,
        TrackingLocalPlaceKind::RestrictedZone => {
            TrackingLocalPlacePolicySignal::RestrictedZoneAttention
        }
        TrackingLocalPlaceKind::Home
        | TrackingLocalPlaceKind::School
        | TrackingLocalPlaceKind::Activity => TrackingLocalPlacePolicySignal::ExpectedPlaceContext,
        TrackingLocalPlaceKind::Custom => TrackingLocalPlacePolicySignal::CustomPlaceContext,
    }
}

fn category_for_place(place: &TrackingLocalParentDefinedPlace) -> TrackingPlaceRiskCategory {
    match place.place_kind {
        TrackingLocalPlaceKind::Home => TrackingPlaceRiskCategory::Home,
        TrackingLocalPlaceKind::School => TrackingPlaceRiskCategory::School,
        TrackingLocalPlaceKind::RestrictedZone | TrackingLocalPlaceKind::Custom => {
            TrackingPlaceRiskCategory::Unknown
        }
        TrackingLocalPlaceKind::Activity | TrackingLocalPlaceKind::SafeZone => {
            TrackingPlaceRiskCategory::Park
        }
    }
}
