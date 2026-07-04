use ocentra_browser_core::social_alert_report_provider_dispatch_execution::social_alert_report_provider_dispatch_execution_typescript;

#[test]
fn social_alert_report_provider_dispatch_execution_stays_rust_owned_without_intent_values_owner() {
    let source = social_alert_report_provider_dispatch_execution_typescript();

    assert_eq!(
        source
            .matches("@ocentra-parent/schema-domain/social-alert-report-intent-values';")
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
            .matches("@ocentra-parent/schema-domain/social-alert-report-provider-receipt-boundary-proof';")
            .count(),
        0
    );
    assert_eq!(
        source
            .matches(
                "import { SocialAlertReportReferenceSchema } from './social_alert_report_provider_dispatch_execution_support';"
            )
            .count(),
        1
    );
    assert_eq!(
        source
            .matches("} from './social_alert_report_local_outbox_bridge_support';")
            .count(),
        1
    );
    assert_eq!(
        source
            .matches("} from './social_alert_report_provider_receipt_boundary_support';")
            .count(),
        1
    );
    assert_eq!(
        source
            .matches("} from './social_alert_report_provider_dispatch_execution_helpers';")
            .count(),
        1
    );
}
