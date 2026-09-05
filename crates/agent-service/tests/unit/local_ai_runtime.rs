#![forbid(unsafe_code)]

#[path = "../../src/event_builder/build.rs"]
mod event_builder;
#[path = "../../src/fields.rs"]
mod fields;
#[path = "../../src/local_ai_cache_root.rs"]
mod local_ai_cache_root;
#[path = "../../src/local_ai_chat_generation.rs"]
mod local_ai_chat_generation;
#[path = "../../src/local_ai_chat_generation_args.rs"]
mod local_ai_chat_generation_args;
#[path = "../../src/local_ai_chat_generation_request.rs"]
mod local_ai_chat_generation_request;
#[path = "../../src/local_ai_chat_generation_request/input.rs"]
mod local_ai_chat_generation_request_input;
#[path = "local_ai_chat_generation_request_tests.rs"]
mod local_ai_chat_generation_request_tests;
#[path = "../../src/local_ai_chat_generation_result.rs"]
mod local_ai_chat_generation_result;
#[path = "../../src/local_ai_chat_generation_runner.rs"]
mod local_ai_chat_generation_runner;
#[path = "local_ai_chat_generation_tests.rs"]
mod local_ai_chat_generation_tests;
#[path = "../../src/local_ai_generation_payload.rs"]
mod local_ai_generation_payload;
#[path = "../../src/local_ai_model_registry.rs"]
mod local_ai_model_registry;
#[path = "local_ai_model_registry_tests.rs"]
mod local_ai_model_registry_tests;
#[path = "local_ai_model_request_status_tests.rs"]
mod local_ai_model_request_status_tests;
#[path = "../../src/local_ai_provider_scheduler.rs"]
mod local_ai_provider_scheduler;
#[path = "../../src/local_ai_provider_scheduler_queue.rs"]
mod local_ai_provider_scheduler_queue;
#[path = "../../src/local_ai_provider_scheduler_state.rs"]
mod local_ai_provider_scheduler_state;
#[path = "local_ai_provider_scheduler_tests.rs"]
mod local_ai_provider_scheduler_tests;
#[path = "../../src/local_ai_runtime_acceleration_config.rs"]
mod local_ai_runtime_acceleration_config;
#[path = "local_ai_runtime_acceleration_tests.rs"]
mod local_ai_runtime_acceleration_tests;
#[path = "../../src/local_ai_runtime_cache_status.rs"]
mod local_ai_runtime_cache_status;
#[path = "../../src/local_ai_runtime_config.rs"]
mod local_ai_runtime_config;
#[path = "../../src/local_ai_runtime_config_environment.rs"]
mod local_ai_runtime_config_environment;
#[path = "../../src/local_ai_runtime_config_parts.rs"]
mod local_ai_runtime_config_parts;
#[path = "../../src/local_ai_runtime_config_path.rs"]
mod local_ai_runtime_config_path;
#[path = "../../src/local_ai_runtime_config_values.rs"]
mod local_ai_runtime_config_values;
#[path = "../../src/local_ai_runtime_configured_status.rs"]
mod local_ai_runtime_configured_status;
#[path = "../../src/local_ai_runtime_distribution.rs"]
mod local_ai_runtime_distribution;
#[path = "../../src/local_ai_runtime_distribution_assets.rs"]
mod local_ai_runtime_distribution_assets;
#[path = "local_ai_runtime_distribution_tests.rs"]
mod local_ai_runtime_distribution_tests;
#[path = "../../src/local_ai_runtime_install_plan.rs"]
mod local_ai_runtime_install_plan;
#[path = "local_ai_runtime_install_plan_tests.rs"]
mod local_ai_runtime_install_plan_tests;
#[path = "../../src/local_ai_runtime_model_selection.rs"]
mod local_ai_runtime_model_selection;
#[path = "../../src/local_ai_runtime_payload.rs"]
mod local_ai_runtime_payload;
#[path = "local_ai_runtime_payload_tests.rs"]
mod local_ai_runtime_payload_tests;
#[path = "../../src/local_ai_runtime_provider_proof_read_model.rs"]
mod local_ai_runtime_provider_proof_read_model;
#[path = "local_ai_runtime_provider_proof_read_model_tests.rs"]
mod local_ai_runtime_provider_proof_read_model_tests;
#[path = "../../src/local_ai_runtime_readiness.rs"]
mod local_ai_runtime_readiness;
#[path = "../../src/local_ai_runtime_status.rs"]
mod local_ai_runtime_status;
#[path = "local_ai_runtime_status_tests.rs"]
mod local_ai_runtime_status_tests;
#[path = "../../src/local_ai_runtime_status_unavailable.rs"]
mod local_ai_runtime_status_unavailable;
#[path = "../support/test_invariants/require_ok.rs"]
mod test_require_ok;
#[path = "../support/test_invariants/require_some.rs"]
mod test_require_some;
#[path = "../../src/time/now.rs"]
mod time;
