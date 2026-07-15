use ocentra_eventing::expect_value::ExpectValue;
use ocentra_tracking_core::local_place_store::{
    build_tracking_local_parent_defined_place_match,
    create_tracking_local_parent_defined_place_store, delete_tracking_local_parent_defined_place,
    export_tracking_local_parent_defined_place_store, import_tracking_local_parent_defined_places,
    upsert_tracking_local_parent_defined_place, TrackingLocalParentDefinedPlace,
    TrackingLocalPlaceKind, TrackingLocalPlaceMutationKind, TrackingLocalPlacePolicySignal,
};

trait TrackingLocalPlaceTestResultExt<T, E> {
    fn value_or_expect(self, context: impl core::fmt::Display) -> T;
}

impl<T, E: core::fmt::Debug> TrackingLocalPlaceTestResultExt<T, E> for Result<T, E> {
    fn value_or_expect(self, context: impl core::fmt::Display) -> T {
        let context = context.to_string();
        self.expect_value(&context)
    }
}

#[test]
fn local_place_store_create_update_import_delete_and_match_stay_parent_local() {
    let imported_store = assert_local_place_store_mutations();
    assert_local_place_store_matches(&imported_store);
}

fn assert_local_place_store_mutations(
) -> ocentra_tracking_core::local_place_store::TrackingLocalParentDefinedPlaceStore {
    let store = create_tracking_local_parent_defined_place_store(
        "parent-local-place-store",
        "2026-06-03T01:00:00.000Z",
        vec!["parent-local-place-store-created".to_owned()],
    );
    let home = place(
        "home",
        "Home",
        TrackingLocalPlaceKind::Home,
        "2026-06-03T01:00:00.000Z",
        "parent-defined-home-created",
    );
    let restricted = place(
        "restricted-lot",
        "Restricted lot",
        TrackingLocalPlaceKind::RestrictedZone,
        "2026-06-03T01:05:00.000Z",
        "parent-defined-restricted-zone-created",
    );
    let safe = place(
        "safe-library",
        "Safe library",
        TrackingLocalPlaceKind::SafeZone,
        "2026-06-03T01:10:00.000Z",
        "parent-defined-safe-zone-imported",
    );

    let created = upsert_tracking_local_parent_defined_place(
        &store,
        home.clone(),
        vec!["parent-defined-home-upserted".to_owned()],
    );
    assert_eq!(created.operation, TrackingLocalPlaceMutationKind::Create);
    assert_eq!(created.after_place_count, 1);
    assert_eq!(created.remote_sync_default, "disabled");
    assert!(!created.ocentra_hosted_default_storage);

    let updated = upsert_tracking_local_parent_defined_place(
        &created.store,
        TrackingLocalParentDefinedPlace {
            updated_at: "2026-06-03T01:15:00.000Z".to_owned(),
            audit_refs: vec!["parent-defined-home-radius-updated".to_owned()],
            ..home
        },
        vec!["parent-defined-home-updated".to_owned()],
    );
    assert_eq!(updated.operation, TrackingLocalPlaceMutationKind::Update);
    assert_eq!(updated.after_place_count, 1);
    assert_eq!(updated.store.storage_boundary, "parent-device-local");

    let imported = import_tracking_local_parent_defined_places(
        &updated.store,
        vec![restricted, safe],
        "2026-06-03T01:20:00.000Z",
        vec!["parent-defined-place-imported".to_owned()],
    );
    assert_eq!(imported.operation, TrackingLocalPlaceMutationKind::Import);
    assert_eq!(imported.after_place_count, 3);

    let exported = export_tracking_local_parent_defined_place_store(
        &imported.store,
        "2026-06-03T01:25:00.000Z",
        vec!["parent-defined-place-exported".to_owned()],
    );
    assert_eq!(exported.custody_label, "parent-owned-export");
    assert_eq!(exported.remote_sync_default, "disabled");
    assert!(!exported.ocentra_hosted_default_storage);
    assert_eq!(
        exported
            .places
            .iter()
            .map(|entry| entry.place_id.as_str())
            .collect::<Vec<_>>(),
        vec!["home", "restricted-lot", "safe-library"]
    );

    let deleted = delete_tracking_local_parent_defined_place(
        &imported.store,
        "restricted-lot",
        "2026-06-03T01:30:00.000Z",
        vec!["parent-requested-place-delete".to_owned()],
        vec!["parent-defined-restricted-zone-deleted".to_owned()],
    );
    assert_eq!(deleted.operation, TrackingLocalPlaceMutationKind::Delete);
    assert_eq!(deleted.before_place_count, 3);
    assert_eq!(deleted.after_place_count, 2);
    assert_eq!(
        deleted.store.tombstones[0].reason_codes,
        vec!["parent-requested-place-delete"]
    );

    imported.store
}

fn assert_local_place_store_matches(
    store: &ocentra_tracking_core::local_place_store::TrackingLocalParentDefinedPlaceStore,
) {
    let restricted_match = build_tracking_local_parent_defined_place_match(
        store,
        "restricted-lot",
        250,
        42,
        0.9,
        vec!["parent-defined-restricted-zone-match".to_owned()],
    )
    .value_or_expect("restricted place match");
    let safe_match = build_tracking_local_parent_defined_place_match(
        store,
        "safe-library",
        250,
        38,
        0.88,
        vec!["parent-defined-safe-zone-match".to_owned()],
    )
    .value_or_expect("safe place match");

    assert_eq!(
        restricted_match.place_match.policy_signal,
        TrackingLocalPlacePolicySignal::RestrictedZoneAttention
    );
    assert_eq!(
        restricted_match.nearby_place_evidence.provider_kind,
        "parent-defined"
    );
    assert_eq!(
        restricted_match.nearby_place_evidence.provider_ref,
        "parent-local-place-store"
    );
    assert_eq!(
        safe_match.place_match.policy_signal,
        TrackingLocalPlacePolicySignal::SafeZoneContext
    );
    assert_eq!(safe_match.nearby_place_evidence.ambiguity_state, "clear");
}

fn place(
    place_id: impl core::fmt::Display,
    label: impl core::fmt::Display,
    place_kind: TrackingLocalPlaceKind,
    updated_at: impl core::fmt::Display,
    audit_ref: impl core::fmt::Display,
) -> TrackingLocalParentDefinedPlace {
    let place_id = place_id.to_string();
    let label = label.to_string();
    let updated_at = updated_at.to_string();
    let audit_ref = audit_ref.to_string();
    TrackingLocalParentDefinedPlace {
        place_id,
        label,
        place_kind,
        updated_at,
        audit_refs: vec![audit_ref],
    }
}
