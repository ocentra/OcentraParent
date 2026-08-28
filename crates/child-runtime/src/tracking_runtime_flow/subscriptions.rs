use super::check_in_requests::tracking_child_check_in_request_receipt;
use super::metadata::{tracking_runtime_metadata, TrackingRuntimeHop};
use super::state::TrackingRuntimeEventState;
use ocentra_eventing::{
    bus::publisher::{EventContext, EventPublisher, RootEventPublisher},
    bus::subscriber::EventSubscriber,
    bus::subscriber::SubscriptionReport,
    envelope::EventMetadata,
    error::EventingError,
    ids::EventType,
    ids::SubscriberId,
    ids::TargetHandler,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::tracking::runtime_event::{
    TrackingAiAnalysisRequestedEvent, TrackingChildCheckInDeliveryState,
    TrackingChildCheckInRecordedEvent, TrackingChildCheckInRequestedEvent,
    TrackingEvidenceRecordedEvent, TrackingExpectedPlaceStateEvaluatedEvent,
    TrackingGeofenceTransitionDetectedEvent, TrackingLocationObservedEvent,
    TrackingNearbyPlaceClassifiedEvent, TrackingNotificationMode,
    TrackingPolicyViolationDetectedEvent,
};
use ocentra_tracking_core::ai_boundary::validate_tracking_ai_result_as_evidence;
use ocentra_tracking_core::alerting::{
    evaluate_tracking_alert, TrackingParentNotificationDecisionState,
};

pub(super) async fn subscribe_tracking_location_observed_events(
    bus: &RootEventPublisher,
    state: TrackingRuntimeEventState,
) -> Result<SubscriptionReport, EventingError> {
    bus.subscribe::<TrackingLocationObservedEvent, _, _>(
        EventSubscriber::new(
            SubscriberId::parse(constants::tracking_runtime::SUBSCRIBER_CHILD_TRACKING_OBSERVER)?,
            EventType::parse(constants::tracking_runtime::TRACKING_LOCATION_OBSERVED_EVENT_TYPE)?,
            TargetHandler::parse(
                constants::tracking_runtime::TARGET_HANDLER_CHILD_TRACKING_OBSERVER,
            )?,
        ),
        move |context| {
            let state = state.clone();
            async move { handle_tracking_location_observed_event(context, state).await }
        },
    )
    .await
}

pub(super) async fn subscribe_tracking_evidence_recorded_events(
    bus: &RootEventPublisher,
    state: TrackingRuntimeEventState,
) -> Result<SubscriptionReport, EventingError> {
    bus.subscribe::<TrackingEvidenceRecordedEvent, _, _>(
        EventSubscriber::new(
            SubscriberId::parse(constants::tracking_runtime::SUBSCRIBER_CHILD_TRACKING_OBSERVER)?,
            EventType::parse(constants::tracking_runtime::TRACKING_EVIDENCE_RECORDED_EVENT_TYPE)?,
            TargetHandler::parse(
                constants::tracking_runtime::TARGET_HANDLER_CHILD_TRACKING_OBSERVER,
            )?,
        ),
        move |context| {
            let state = state.clone();
            async move {
                state.record_evidence(context.payload().clone());
                Ok(())
            }
        },
    )
    .await
}

pub(super) async fn subscribe_tracking_geofence_transition_events(
    bus: &RootEventPublisher,
    state: TrackingRuntimeEventState,
) -> Result<SubscriptionReport, EventingError> {
    bus.subscribe::<TrackingGeofenceTransitionDetectedEvent, _, _>(
        EventSubscriber::new(
            SubscriberId::parse(constants::tracking_runtime::SUBSCRIBER_CHILD_TRACKING_OBSERVER)?,
            EventType::parse(
                constants::tracking_runtime::TRACKING_GEOFENCE_TRANSITION_DETECTED_EVENT_TYPE,
            )?,
            TargetHandler::parse(
                constants::tracking_runtime::TARGET_HANDLER_CHILD_TRACKING_OBSERVER,
            )?,
        ),
        move |context| {
            let state = state.clone();
            async move {
                state.record_geofence_transition(context.payload().clone());
                Ok(())
            }
        },
    )
    .await
}

pub(super) async fn subscribe_tracking_child_check_in_recorded_events(
    bus: &RootEventPublisher,
    state: TrackingRuntimeEventState,
) -> Result<SubscriptionReport, EventingError> {
    bus.subscribe::<TrackingChildCheckInRecordedEvent, _, _>(
        EventSubscriber::new(
            SubscriberId::parse(constants::tracking_runtime::SUBSCRIBER_CHILD_TRACKING_OBSERVER)?,
            EventType::parse(
                constants::tracking_runtime::TRACKING_CHILD_CHECK_IN_RECORDED_EVENT_TYPE,
            )?,
            TargetHandler::parse(
                constants::tracking_runtime::TARGET_HANDLER_CHILD_TRACKING_OBSERVER,
            )?,
        ),
        move |context| {
            let state = state.clone();
            async move {
                state.record_child_check_in(context.payload().clone());
                Ok(())
            }
        },
    )
    .await
}

async fn handle_tracking_location_observed_event(
    context: EventContext<TrackingLocationObservedEvent>,
    state: TrackingRuntimeEventState,
) -> Result<(), EventingError> {
    validate_tracking_location_observed_event(context.payload())?;
    state.record_location_observed(context.payload().clone());
    let observation_report =
        ocentra_tracking_core::runtime_flow::observe_tracking_location(context.payload().clone());
    let evidence = publish_tracking_evidence(context.publisher(), &observation_report).await?;
    publish_tracking_geofence(context.publisher(), &evidence).await?;
    publish_tracking_expected_place(context.publisher(), &evidence).await?;
    publish_tracking_check_in(context.publisher(), &observation_report, &evidence).await?;
    publish_tracking_ai_request(
        context.publisher(),
        observation_report.ai_analysis_requested,
    )
    .await
}

fn validate_tracking_location_observed_event(
    event: &TrackingLocationObservedEvent,
) -> Result<(), EventingError> {
    let validation =
        ocentra_tracking_core::location_validation::validate_tracking_location_observation(event);
    if validation.result_state
        == ocentra_tracking_core::location_validation::TrackingLocationValidationResultState::Rejected
    {
        return Err(EventingError::InvalidValue {
            field: constants::tracking_runtime::FIELD_LOCATION_VALIDATION,
            value: validation.validation_state.to_string(),
        });
    }
    Ok(())
}

async fn publish_tracking_evidence(
    publisher: &EventPublisher,
    observation_report: &ocentra_tracking_core::runtime_flow::TrackingRuntimeObservationReport,
) -> Result<TrackingEvidenceRecordedEvent, EventingError> {
    let evidence = observation_report.evidence_recorded.clone();
    publisher
        .publish(
            evidence.clone(),
            tracking_runtime_metadata(
                TrackingRuntimeHop::EvidenceRecorded,
                evidence.evidence_ref.as_str(),
                &evidence.source_observed_at,
            )?,
        )
        .await?;
    Ok(evidence)
}

async fn publish_tracking_geofence(
    publisher: &EventPublisher,
    evidence: &TrackingEvidenceRecordedEvent,
) -> Result<TrackingGeofenceTransitionDetectedEvent, EventingError> {
    let geofence =
        ocentra_tracking_core::runtime_flow::tracking_geofence_transition_from_evidence(evidence);
    publisher
        .publish(
            geofence.clone(),
            tracking_runtime_metadata(
                TrackingRuntimeHop::GeofenceTransitionDetected,
                geofence.transition_id.as_str(),
                &geofence.source_observed_at,
            )?,
        )
        .await?;
    Ok(geofence)
}

async fn publish_tracking_expected_place(
    publisher: &EventPublisher,
    evidence: &TrackingEvidenceRecordedEvent,
) -> Result<TrackingExpectedPlaceStateEvaluatedEvent, EventingError> {
    let expected_place =
        ocentra_tracking_core::runtime_flow::tracking_expected_place_state_from_evidence(evidence);
    publisher
        .publish(
            expected_place.clone(),
            tracking_runtime_metadata(
                TrackingRuntimeHop::ExpectedPlaceStateEvaluated,
                expected_place.evaluation_id.as_str(),
                &expected_place.source_observed_at,
            )?,
        )
        .await?;
    Ok(expected_place)
}

async fn publish_tracking_check_in(
    publisher: &EventPublisher,
    observation_report: &ocentra_tracking_core::runtime_flow::TrackingRuntimeObservationReport,
    evidence: &TrackingEvidenceRecordedEvent,
) -> Result<TrackingChildCheckInRecordedEvent, EventingError> {
    let check_in = ocentra_tracking_core::runtime_flow::tracking_child_check_in_from_location(
        &observation_report.location_observed,
        vec![evidence.evidence_ref.clone()],
    );
    publisher
        .publish(
            check_in.clone(),
            tracking_runtime_metadata(
                TrackingRuntimeHop::ChildCheckInRecorded,
                check_in.check_in_id.as_str(),
                &check_in.checked_in_at,
            )?,
        )
        .await?;
    Ok(check_in)
}

async fn publish_tracking_ai_request(
    publisher: &EventPublisher,
    ai_analysis_requested: Option<TrackingAiAnalysisRequestedEvent>,
) -> Result<(), EventingError> {
    let Some(ai_request) = ai_analysis_requested else {
        return Ok(());
    };
    publisher
        .publish(
            ai_request.clone(),
            tracking_runtime_metadata(
                TrackingRuntimeHop::AiAnalysisRequested,
                ai_request.ai_request_id.as_str(),
                &ai_request.source_observed_at,
            )?,
        )
        .await?;
    Ok(())
}

pub(super) async fn subscribe_child_tracking_check_in_request_events(
    bus: &RootEventPublisher,
    state: TrackingRuntimeEventState,
) -> Result<SubscriptionReport, EventingError> {
    bus.subscribe::<TrackingChildCheckInRequestedEvent, _, _>(
        EventSubscriber::new(
            SubscriberId::parse(
                constants::tracking_runtime::SUBSCRIBER_CHILD_TRACKING_CHECK_IN_REQUESTER,
            )?,
            EventType::parse(
                constants::tracking_runtime::TRACKING_CHILD_CHECK_IN_REQUESTED_EVENT_TYPE,
            )?,
            TargetHandler::parse(
                constants::tracking_runtime::TARGET_HANDLER_CHILD_TRACKING_CHECK_IN_REQUESTER,
            )?,
        ),
        move |context| {
            let state = state.clone();
            async move {
                let request = context.payload().clone();
                let envelope = context.envelope();
                let metadata = EventMetadata {
                    event_id: envelope.event_id().clone(),
                    correlation_id: envelope.correlation_id().clone(),
                    causation_id: envelope.causation_id().cloned(),
                    source: envelope.source().clone(),
                    observed_at: envelope.observed_at().clone(),
                    target_handler: envelope.target_handler().cloned(),
                    priority: envelope.priority(),
                    deadline: envelope.deadline(),
                };
                state.record_parent_requested_check_in(request.clone(), metadata.clone());
                let receipt = tracking_child_check_in_request_receipt(
                    &request,
                    &metadata,
                    state.has_seen_parent_requested_check_in(&request.check_in_id),
                )?;
                if receipt.delivery_state == TrackingChildCheckInDeliveryState::Requested {
                    state.mark_parent_requested_check_in_seen(request.check_in_id.clone());
                }
                let completion = context.complete_request(receipt.clone()).await?;
                state.record_parent_requested_check_in_receipt(receipt, completion);
                Ok(())
            }
        },
    )
    .await
}

pub(super) async fn subscribe_child_ai_tracking_analysis_events(
    bus: &RootEventPublisher,
    state: TrackingRuntimeEventState,
) -> Result<SubscriptionReport, EventingError> {
    bus.subscribe::<TrackingAiAnalysisRequestedEvent, _, _>(
        EventSubscriber::new(
            SubscriberId::parse(
                constants::tracking_runtime::SUBSCRIBER_CHILD_AI_TRACKING_ANALYZER,
            )?,
            EventType::parse(
                constants::tracking_runtime::TRACKING_AI_ANALYSIS_REQUESTED_EVENT_TYPE,
            )?,
            TargetHandler::parse(
                constants::tracking_runtime::TARGET_HANDLER_CHILD_AI_TRACKING_ANALYZER,
            )?,
        ),
        move |context| {
            let state = state.clone();
            async move {
                state.record_ai_analysis_request(context.payload().clone());
                let classified =
                    ocentra_child_ai_core::tracking_boundary::classify_tracking_nearby_place(
                        context.payload(),
                    )?;
                state.record_nearby_place_classified(classified.clone());
                context
                    .publisher()
                    .publish(
                        classified.clone(),
                        tracking_runtime_metadata(
                            TrackingRuntimeHop::NearbyPlaceClassified,
                            classified.source_ai_request_id.as_str(),
                            &classified.source_observed_at,
                        )?,
                    )
                    .await?;
                Ok(())
            }
        },
    )
    .await
}

pub(super) async fn subscribe_child_policy_tracking_analysis_events(
    bus: &RootEventPublisher,
    state: TrackingRuntimeEventState,
) -> Result<SubscriptionReport, EventingError> {
    bus.subscribe::<TrackingNearbyPlaceClassifiedEvent, _, _>(
        EventSubscriber::new(
            SubscriberId::parse(
                constants::tracking_runtime::SUBSCRIBER_CHILD_POLICY_TRACKING_ANALYZER,
            )?,
            EventType::parse(
                constants::tracking_runtime::TRACKING_NEARBY_PLACE_CLASSIFIED_EVENT_TYPE,
            )?,
            TargetHandler::parse(
                constants::tracking_runtime::TARGET_HANDLER_CHILD_POLICY_TRACKING_ANALYZER,
            )?,
        ),
        move |context| {
            let state = state.clone();
            async move {
                let Some(ai_request) = state.ai_analysis_requested() else {
                    return Ok(());
                };
                let ai_boundary_decision =
                    validate_tracking_ai_result_as_evidence(&ai_request, context.payload());
                state.record_ai_boundary_decision(ai_boundary_decision.clone());
                if ai_boundary_decision.decision_state
                    != constants::tracking_runtime::AI_RESULT_ACCEPTED_AS_EVIDENCE
                {
                    return Ok(());
                }
                let policy_decision =
                    ocentra_child_policy_core::tracking_policy::evaluate_tracking_nearby_place_policy(
                        context.payload(),
                    );
                if let Some(violation) = policy_decision.policy_violation_detected {
                    state.record_policy_violation_detected(violation.clone());
                    context
                        .publisher()
                        .publish(
                            violation.clone(),
                            tracking_runtime_metadata(
                                TrackingRuntimeHop::PolicyViolationDetected,
                                violation.violation_id.as_str(),
                                &violation.detected_at,
                            )?,
                        )
                        .await?;
                }
                Ok(())
            }
        },
    )
    .await
}

pub(super) async fn subscribe_child_policy_tracking_expected_place_events(
    bus: &RootEventPublisher,
    state: TrackingRuntimeEventState,
) -> Result<SubscriptionReport, EventingError> {
    bus.subscribe::<TrackingExpectedPlaceStateEvaluatedEvent, _, _>(
        EventSubscriber::new(
            SubscriberId::parse(
                constants::tracking_runtime::SUBSCRIBER_CHILD_POLICY_EXPECTED_PLACE_EVALUATOR,
            )?,
            EventType::parse(
                constants::tracking_runtime::TRACKING_EXPECTED_PLACE_STATE_EVALUATED_EVENT_TYPE,
            )?,
            TargetHandler::parse(
                constants::tracking_runtime::TARGET_HANDLER_CHILD_POLICY_EXPECTED_PLACE_EVALUATOR,
            )?,
        ),
        move |context| {
            let state = state.clone();
            async move {
                state.record_expected_place_state(context.payload().clone());
                let policy_decision =
                    ocentra_child_policy_core::tracking_policy::evaluate_tracking_expected_place_policy(
                        context.payload(),
                    );
                if let Some(violation) = policy_decision.policy_violation_detected {
                    state.record_policy_violation_detected(violation.clone());
                    context
                        .publisher()
                        .publish(
                            violation.clone(),
                            tracking_runtime_metadata(
                                TrackingRuntimeHop::PolicyViolationDetected,
                                violation.violation_id.as_str(),
                                &violation.detected_at,
                            )?,
                        )
                        .await?;
                }
                Ok(())
            }
        },
    )
    .await
}

pub(super) async fn subscribe_child_notification_policy_events(
    bus: &RootEventPublisher,
    state: TrackingRuntimeEventState,
) -> Result<SubscriptionReport, EventingError> {
    bus.subscribe::<TrackingPolicyViolationDetectedEvent, _, _>(
        EventSubscriber::new(
            SubscriberId::parse(
                constants::tracking_runtime::SUBSCRIBER_CHILD_NOTIFICATION_POLICY_BRIDGE,
            )?,
            EventType::parse(
                constants::tracking_runtime::TRACKING_POLICY_VIOLATION_DETECTED_EVENT_TYPE,
            )?,
            TargetHandler::parse(
                constants::tracking_runtime::TARGET_HANDLER_CHILD_NOTIFICATION_POLICY_BRIDGE,
            )?,
        ),
        move |context| {
            let state = state.clone();
            async move {
                let Some(location_observed) = state.location_observed() else {
                    return Ok(());
                };
                if location_observed.config.notification_mode == TrackingNotificationMode::Disabled {
                    return Ok(());
                }
                let recent_duplicate_count =
                    state.recent_policy_violation_duplicate_count(context.payload());
                let alert_decision =
                    evaluate_tracking_alert(context.payload(), recent_duplicate_count);
                state.record_alert_decision(alert_decision.clone());
                state.record_policy_violation_history(context.payload().clone());
                if alert_decision.parent_notification_state
                    != TrackingParentNotificationDecisionState::Allowed
                {
                    return Ok(());
                }

                let notification =
                    ocentra_child_notification_core::tracking_notification::request_parent_notification_from_policy_violation(
                        context.payload(),
                    );
                state.record_parent_notification_requested(notification.clone());
                context
                    .publisher()
                    .publish(
                        notification.clone(),
                        tracking_runtime_metadata(
                            TrackingRuntimeHop::ParentNotificationRequested,
                            notification.notification_id.as_str(),
                            &notification.requested_at,
                        )?,
                    )
                    .await?;
                Ok(())
            }
        },
    )
    .await
}
