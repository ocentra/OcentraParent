use super::{ok, some, TestResult};
use ocentra_eventing::topology::EventTopologyStatus;
use ocentra_parent_agent_core::browser_event_runtime::browser_runtime_stream_report_topology_manifest;
use ocentra_parent_agent_protocol::constants;

#[test]
fn browser_runtime_stream_report_topology_covers_named_event_and_subscriber() -> TestResult {
    let manifest = ok(
        browser_runtime_stream_report_topology_manifest(),
        "browser runtime stream report topology manifest",
    )?;
    assert_eq!(manifest.unready_entries().len(), 0);
    assert_eq!(manifest.entries().len(), 1);

    let entry = some(
        manifest.entries().first(),
        "browser runtime stream report entry missing",
    )?;
    assert_eq!(entry.status, EventTopologyStatus::Covered);
    assert_eq!(
        entry.contract.event_type.as_str(),
        constants::browser::EVENT_BROWSER_RUNTIME_STREAM_REPORT_REQUESTED
    );
    assert_eq!(
        some(
            entry.publishers.first(),
            "browser runtime stream report publisher missing",
        )?
        .as_str(),
        constants::browser::RUNTIME_COMPONENT_BROWSER_SPINE
    );
    let subscriber = some(
        entry.subscribers.first(),
        "browser runtime stream report subscriber missing",
    )?;
    assert_eq!(
        subscriber.subscriber_id.as_str(),
        constants::browser::SUBSCRIBER_BROWSER_RUNTIME_STREAM_REPORT
    );
    assert_eq!(
        subscriber.target_handler.as_str(),
        constants::browser::TARGET_BROWSER_RUNTIME_STREAM_REPORT
    );
    Ok(())
}
