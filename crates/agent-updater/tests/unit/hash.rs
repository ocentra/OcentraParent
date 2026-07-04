use ocentra_parent_agent_maintenance::hash::sha256_file;

#[test]
fn sha256_file_matches_known_payload() {
    let dir = tempfile::tempdir().expect("temp dir failed");
    let path = dir.path().join("payload.bin");
    std::fs::write(&path, b"ocentra").expect("payload write failed");

    let hash = sha256_file(&path).expect("payload hash failed");

    assert_eq!(
        hash,
        "B099331FE5A04DD0C031B0C6747E4A2AAD74FA87F6145F3F351FA48CC29A94BE"
    );
}
