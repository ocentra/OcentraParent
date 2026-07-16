use ocentra_lan_core::network_inventory_command::{
    normalize_mac_address, record_text, record_u64, targeted_arp_probe_commands, value_text,
};
use ocentra_parent_agent_protocol::constants;

#[test]
fn targeted_arp_probe_commands_follow_platform_and_interface_rules() {
    let commands = targeted_arp_probe_commands("192.168.2.20", Some("  Ethernet 2  "));

    if cfg!(target_os = "windows") {
        assert_eq!(commands.len(), 1);
        assert_eq!(commands[0].program, constants::lan_pairing::PING_EXE);
        assert_eq!(
            commands[0].args,
            vec![
                constants::lan_pairing::PING_WINDOWS_COUNT_ARG.to_string(),
                "1".to_string(),
                constants::lan_pairing::PING_WINDOWS_TIMEOUT_ARG.to_string(),
                "200".to_string(),
                "192.168.2.20".to_string(),
            ]
        );
    } else if cfg!(target_os = "linux") {
        assert_eq!(commands.len(), 2);
        assert_eq!(commands[0].program, "arping");
        assert_eq!(
            commands[0].args,
            vec![
                "-I".to_string(),
                "Ethernet 2".to_string(),
                "-c".to_string(),
                "1".to_string(),
                "-w".to_string(),
                "1".to_string(),
                "192.168.2.20".to_string(),
            ]
        );
        assert_eq!(commands[1].program, constants::lan_pairing::PING_EXE);
    } else {
        assert!(commands.is_empty());
    }
}

#[test]
fn command_helpers_trim_and_parse_scalar_values() {
    let record = serde_json::json!({
        "name": "  printer  ",
        "count": " 7 ",
        "flag": true
    });

    assert_eq!(record_text(&record, "name"), Some("printer".to_string()));
    assert_eq!(record_text(&record, "missing"), None);
    assert_eq!(
        value_text(&serde_json::Value::String("  hello  ".to_string())),
        Some("hello".to_string())
    );
    assert_eq!(
        value_text(&serde_json::Value::Number(42_u64.into())),
        Some("42".to_string())
    );
    assert_eq!(record_u64(&record, "count"), Some(7));
    assert_eq!(record_u64(&record, "flag"), None);
    assert_eq!(
        normalize_mac_address(" 54:27:1E:97:C3:31 "),
        Some(constants::lan_pairing::TEST_LAN_MAC.to_string())
    );
}
