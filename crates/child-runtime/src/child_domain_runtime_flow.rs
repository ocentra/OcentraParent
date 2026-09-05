use ocentra_eventing::bus::reports::handler::EventMetricsSnapshot;
use ocentra_eventing::{
    bus::publisher::RootEventPublisher, bus::subscriber::SubscriptionReport, bus::EventBus,
    error::EventingError,
};
use ocentra_lan_core::lan_pairing;
use ocentra_network_core::network_runtime;
use ocentra_parent_agent_protocol::child_domain_runtime::{
    ChildDomainAiAnalysisCompletedEvent, ChildDomainAiAnalysisRequestedEvent,
    ChildDomainEvidenceRecordedEvent, ChildDomainNotificationRequestedEvent,
    ChildDomainObservedEvent, ChildDomainPolicyEvaluationRequestedEvent,
    ChildDomainPolicyViolationDetectedEvent, ChildRuntimeDomain,
};
use routing::{child_domain_runtime_metadata, ChildDomainRuntimeHop};
use state::ChildDomainRuntimeFlowState;
use subscriptions::{
    subscribe_child_domain_ai, subscribe_child_domain_ai_policy_bridge,
    subscribe_child_domain_evidence, subscribe_child_domain_notification,
    subscribe_child_domain_observer, subscribe_child_domain_policy,
};

mod routing;
mod state;
mod subscriptions;

#[derive(Clone, Debug, PartialEq)]
pub struct ChildDomainRuntimeFlowReport {
    pub domain: ChildRuntimeDomain,
    pub observer_subscription_report: SubscriptionReport,
    pub ai_subscription_report: SubscriptionReport,
    pub ai_policy_subscription_report: SubscriptionReport,
    pub policy_subscription_report: SubscriptionReport,
    pub notification_subscription_report: SubscriptionReport,
    pub evidence_recorded: ChildDomainEvidenceRecordedEvent,
    pub ai_analysis_requested: Option<ChildDomainAiAnalysisRequestedEvent>,
    pub ai_analysis_completed: Option<ChildDomainAiAnalysisCompletedEvent>,
    pub policy_evaluation_requested: Option<ChildDomainPolicyEvaluationRequestedEvent>,
    pub policy_violation_detected: Option<ChildDomainPolicyViolationDetectedEvent>,
    pub notification_requested: Option<ChildDomainNotificationRequestedEvent>,
}

pub struct ChildDomainRuntimeEventFlow {
    bus: RootEventPublisher,
    domain: ChildRuntimeDomain,
    observer_subscription_report: SubscriptionReport,
    ai_subscription_report: SubscriptionReport,
    ai_policy_subscription_report: SubscriptionReport,
    policy_subscription_report: SubscriptionReport,
    notification_subscription_report: SubscriptionReport,
    state: ChildDomainRuntimeFlowState,
}

impl ChildDomainRuntimeEventFlow {
    pub async fn for_domain(domain: ChildRuntimeDomain) -> Result<Self, EventingError> {
        let bus = EventBus::root();
        let state = ChildDomainRuntimeFlowState::default();
        let observer_subscription_report = subscribe_child_domain_observer(&bus, domain).await?;
        let _evidence_subscription_report =
            subscribe_child_domain_evidence(&bus, domain, state.clone()).await?;
        let ai_subscription_report = subscribe_child_domain_ai(&bus, domain, state.clone()).await?;
        let ai_policy_subscription_report =
            subscribe_child_domain_ai_policy_bridge(&bus, state.clone()).await?;
        let policy_subscription_report =
            subscribe_child_domain_policy(&bus, domain, state.clone()).await?;
        let notification_subscription_report =
            subscribe_child_domain_notification(&bus, state.clone()).await?;

        Ok(Self {
            bus,
            domain,
            observer_subscription_report,
            ai_subscription_report,
            ai_policy_subscription_report,
            policy_subscription_report,
            notification_subscription_report,
            state,
        })
    }

    pub async fn for_event(event: &ChildDomainObservedEvent) -> Result<Self, EventingError> {
        Self::for_domain(event.domain).await
    }

    pub fn domain(&self) -> ChildRuntimeDomain {
        self.domain
    }

    pub async fn publish_observed(
        &self,
        event: ChildDomainObservedEvent,
    ) -> Result<ChildDomainRuntimeFlowReport, EventingError> {
        let recorded_at = event.observed_at.clone();
        self.bus
            .publish(
                event.clone(),
                child_domain_runtime_metadata(
                    ChildDomainRuntimeHop::Observed(&event.observation_id),
                    &recorded_at,
                )?,
            )
            .await?;

        self.report()
    }

    pub async fn metrics_snapshot(&self) -> EventMetricsSnapshot {
        self.bus.metrics_snapshot().await
    }

    fn report(&self) -> Result<ChildDomainRuntimeFlowReport, EventingError> {
        Ok(ChildDomainRuntimeFlowReport {
            domain: self.domain,
            observer_subscription_report: self.observer_subscription_report.clone(),
            ai_subscription_report: self.ai_subscription_report.clone(),
            ai_policy_subscription_report: self.ai_policy_subscription_report.clone(),
            policy_subscription_report: self.policy_subscription_report.clone(),
            notification_subscription_report: self.notification_subscription_report.clone(),
            evidence_recorded: self.state.evidence_recorded()?,
            ai_analysis_requested: self.state.ai_analysis_requested(),
            ai_analysis_completed: self.state.ai_analysis_completed(),
            policy_evaluation_requested: self.state.policy_evaluation_requested(),
            policy_violation_detected: self.state.policy_violation_detected(),
            notification_requested: self.state.notification_requested(),
        })
    }
}

pub async fn publish_default_child_domain_runtime_flows(
) -> Result<Vec<ChildDomainRuntimeFlowReport>, EventingError> {
    let events = vec![
        ocentra_app_core::default_app_observed_event(),
        ocentra_app_game_core::default_app_game_observed_event(),
        ocentra_browser_core::default_browser_observed_event(),
        lan_pairing::default_lan_observed_event(),
        network_runtime::default_network_observed_event(),
        ocentra_screen_core::default_screen_observed_event(),
        ocentra_screen_live_view_core::default_screen_live_view_observed_event(),
    ];
    let mut reports = Vec::new();
    for event in events {
        reports.push(publish_child_domain_observed_event(event).await?);
    }
    Ok(reports)
}

pub async fn publish_child_domain_observed_event(
    event: ChildDomainObservedEvent,
) -> Result<ChildDomainRuntimeFlowReport, EventingError> {
    ChildDomainRuntimeEventFlow::for_event(&event)
        .await?
        .publish_observed(event)
        .await
}
