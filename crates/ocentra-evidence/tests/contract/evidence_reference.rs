use ocentra_evidence::EvidenceCustodyScope;

#[test]
fn custody_scope_serializes_to_canonical_schema_literals() {
    let local_only = serde_json::to_string(&EvidenceCustodyScope::LocalOnly).ok();
    let family_shared = serde_json::to_string(&EvidenceCustodyScope::FamilyShared).ok();
    let exportable = serde_json::to_string(&EvidenceCustodyScope::Exportable).ok();

    assert_eq!(local_only.as_deref(), Some("\"local-only\""));
    assert_eq!(family_shared.as_deref(), Some("\"family-shared\""));
    assert_eq!(exportable.as_deref(), Some("\"exportable\""));
}

#[test]
fn custody_scope_deserializes_from_canonical_schema_literals() {
    let local_only = serde_json::from_str::<EvidenceCustodyScope>("\"local-only\"").ok();
    let family_shared = serde_json::from_str::<EvidenceCustodyScope>("\"family-shared\"").ok();
    let exportable = serde_json::from_str::<EvidenceCustodyScope>("\"exportable\"").ok();

    assert_eq!(local_only, Some(EvidenceCustodyScope::LocalOnly));
    assert_eq!(family_shared, Some(EvidenceCustodyScope::FamilyShared));
    assert_eq!(exportable, Some(EvidenceCustodyScope::Exportable));
}
