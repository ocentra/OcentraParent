use ocentra_browser_core::social_alert_report_scheduler_bridge::social_alert_report_scheduler_bridge_typescript;

#[test]
fn social_alert_report_scheduler_bridge_stays_rust_owned_without_scheduler_owner() {
    let source = social_alert_report_scheduler_bridge_typescript();

    assert_eq!(
        source
            .matches("@ocentra-parent/schema-domain/notification-local-outbox';")
            .count(),
        0
    );
    assert_eq!(
        source
            .matches("} from './social_alert_report_scheduler_bridge_support';")
            .count(),
        1
    );
}
