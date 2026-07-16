use ocentra_browser_core::social_alert_report_local_outbox_bridge::social_alert_report_local_outbox_bridge_typescript;

#[test]
fn social_alert_report_local_outbox_bridge_stays_rust_owned_without_intent_owner() {
    let source = social_alert_report_local_outbox_bridge_typescript();

    assert_eq!(
        source
            .matches("@ocentra-parent/schema-domain/social-alert-report-intent';")
            .count(),
        0
    );
    assert_eq!(
        source
            .matches("@ocentra-parent/schema-domain/notification-local-outbox';")
            .count(),
        0
    );
    assert_eq!(
        source
            .matches("} from './social_alert_report_local_outbox_bridge_support';")
            .count(),
        1
    );
}
