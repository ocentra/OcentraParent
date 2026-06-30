use ocentra_parent_agent_maintenance::hash::sha256_file;

#[test]
fn sha256_file_matches_known_payload() {
    let dir_result = tempfile::tempdir();
    assert!(dir_result.is_ok(), "temp dir failed: {dir_result:?}");
    let Ok(dir) = dir_result else {
        return;
    };
    let path = dir.path().join("payload.bin");
    let write_result = std::fs::write(&path, b"ocentra");
    assert!(
        write_result.is_ok(),
        "payload write failed: {write_result:?}"
    );
    if write_result.is_err() {
        return;
    }

    let hash_result = sha256_file(&path);
    assert!(hash_result.is_ok(), "payload hash failed: {hash_result:?}");
    let Ok(hash) = hash_result else {
        return;
    };

    assert_eq!(
        hash,
        "B099331FE5A04DD0C031B0C6747E4A2AAD74FA87F6145F3F351FA48CC29A94BE"
    );
}
