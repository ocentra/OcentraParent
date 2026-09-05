#![forbid(unsafe_code)]

extern crate ocentra_parent_agent_service as agent_service_lib;
extern crate self as ocentra_parent_agent_service;

#[path = "../../src/browser_policy_compiler.rs"]
mod browser_policy_compiler;
#[path = "../../src/browser_policy_compiler_assessment.rs"]
mod browser_policy_compiler_assessment;
#[path = "../../src/browser_policy_request.rs"]
mod browser_policy_request;
#[path = "../../src/browser_policy_runtime_support.rs"]
mod browser_policy_runtime_support;
#[path = "../../src/browser_policy_store.rs"]
mod browser_policy_store;
#[path = "../support/test_invariants/require_ok.rs"]
mod test_require_ok;
#[path = "../support/browser_policy_test_support.rs"]
mod test_support;

#[path = "browser_policy_manifest_patch_tests.rs"]
mod browser_policy_manifest_patch_tests;
