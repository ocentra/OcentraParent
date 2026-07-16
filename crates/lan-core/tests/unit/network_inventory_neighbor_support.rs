use ocentra_lan_core::network_inventory::neighbor_support::*;

#[test]
fn normalize_neighbor_hostname_rejects_invalid_display_values() {
    assert_eq!(normalize_neighbor_hostname(""), None);
    assert_eq!(normalize_neighbor_hostname(" unknown-host "), None);
    assert_eq!(normalize_neighbor_hostname("bad hostname"), None);
    assert_eq!(normalize_neighbor_hostname("bad\nname"), None);
    assert_eq!(normalize_neighbor_hostname("bad<script>"), None);
    assert_eq!(
        normalize_neighbor_hostname(&"a".repeat(MAX_NEIGHBOR_HOSTNAME_BYTES + 1)),
        None
    );
}

#[test]
fn normalize_neighbor_hostname_accepts_trimmed_dns_style_names() {
    assert_eq!(
        normalize_neighbor_hostname(" printer-1.example.local. "),
        Some("printer-1.example.local".to_string())
    );
    assert_eq!(
        normalize_neighbor_hostname("GAMEDEV"),
        Some("GAMEDEV".to_string())
    );
}

#[test]
fn interface_scope_matching_is_trimmed_case_insensitive_and_explicit() {
    assert!(interface_matches_selected_scope(
        Some(" Ethernet "),
        Some("ethernet")
    ));
    assert!(!interface_matches_selected_scope(
        Some("Wi-Fi"),
        Some("Ethernet")
    ));
    assert!(!interface_matches_selected_scope(None, Some("Ethernet")));
    assert!(interface_matches_selected_scope(Some("Wi-Fi"), None));
}
