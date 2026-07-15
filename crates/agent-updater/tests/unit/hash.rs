use ocentra_parent_agent_maintenance::hash::sha256_file;

#[test]
fn sha256_file_matches_known_payload() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempfile::tempdir()?;
    let path = dir.path().join("payload.bin");
    std::fs::write(&path, b"ocentra")?;

    let hash = sha256_file(&path)?;

    assert_eq!(
        hash,
        "B099331FE5A04DD0C031B0C6747E4A2AAD74FA87F6145F3F351FA48CC29A94BE"
    );
    Ok(())
}
