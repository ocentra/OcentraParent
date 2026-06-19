# crates/agent-service
- target_kind: Rust service/orchestration crate.
- owned_paths: crates/agent-service/Cargo.toml; crates/agent-service/README.md; crates/agent-service/src/lib.rs; crates/agent-service/src/websocket.rs; crates/agent-service/src/service_runtime.rs; crates/agent-service/src/network_remote_delivery_status_cross_process.rs; crates/agent-service/src/lan_pairing_browser_add_device_state.rs
- declared_responsibility: Local Rust service that exposes the child-device agent over loopback/LAN development paths and orchestrates runtime commands.
- observed_responsibility: Service-level HTTP/WebSocket endpoints, command validation/dispatch, read models, LAN bind/origin rules, and many bridge/payload/report layers across activity, network, enforcement, screen, and LAN pairing flows.
- should_own: Endpoints, command handlers, service-backed read models, runtime orchestration, and API-facing payload/bridge code.
- should_not_own: Low-level runtime mechanics already in agent-core or wire-shape ownership already in agent-protocol.
- allowed_dependencies: crates/agent-core, crates/agent-protocol, logging-core, child-runtime, eventing, policy-control-core, parent-runtime-core, screen-capture-adapter, screen-live-view-core, network-evidence, tokio, axum, tower-http.
- suspicious_dependencies: The crate mirrors core/protocol shape families almost one-for-one through many `*_payload`, `*_read_model`, and nested bridge modules; several of those look mechanically generated rather than service-specific.
- expected_dependents: parent portal UI, dev tooling, and downstream runtime integration tests.
- shared_contract_schema_usage: Very high; service modules consume protocol/core shapes for websocket commands, payloads, status reports, and read models.
- duplicate_or_near_duplicate_shapes: `network_remote_delivery_status_cross_process.rs` maps core reports into protocol status fields; `lan_pairing_browser_add_device_state.rs` repeats LAN read-model assembly across discovery, proof, and readiness families; `screen_settings_*` modules repeat the same request/result shape across API/runtime/store layers.
- id_name_status_drift: Medium; filenames are descriptive, but many are thin wrappers around identical state tables or field mappings.
- direct_import_vs_event_boundary: Event/command translation belongs here, but repeated direct field mapping across bridge modules suggests some seams should be centralized in shared helpers instead of being reimplemented per feature.
- event_bus_usage: Yes; the crate owns websocket command/event handling and service-backed read-model emission.
- logging_and_proof_chain_expectations: High; runtime, websocket, and bridge code is part of the logged proof chain and should preserve redaction plus correlated milestone reporting.
- boundary_violations: No crate-root barrel issue, but the service repeats domain-shape translation in many tiny bridge modules, which is a structural DRY smell.
- dry_common_core_candidates: Shared mappers for network remote-delivery status assembly, LAN add-device read-model assembly, screen settings request/result handling, and enforcement read-model/report shaping.
- dead_frontage_or_shims: Several payload/report/read-model companions appear to be frontage around a single domain table rather than distinct service behavior.
- proposed_fix_packets: Factor repeated field assembly into common internal mappers, trim feature-specific wrapper churn, and keep the service focused on orchestration rather than shape replication.
- severity: medium-high.
- confidence: medium-high.
- evidence_paths: crates/agent-service/README.md; crates/agent-service/src/lib.rs; crates/agent-service/src/websocket.rs; crates/agent-service/src/service_runtime.rs; crates/agent-service/src/network_remote_delivery_status_cross_process.rs; crates/agent-service/src/lan_pairing_browser_add_device_state.rs; crates/agent-service/src/screen_settings_api.rs; crates/agent-service/src/enforcement_api.rs

## Current Refresh Audit - 2026-06-19

- Current responsibility: local Rust service orchestration for websocket command
  routing, service-backed read models, runtime startup, and proof/log emission
  for activity, browser, enforcement, LAN pairing, network, screen settings,
  and parent-assistant flows.
- Dependencies: heavy fan-in on `agent-protocol`, `agent-core`,
  `ocentra-eventing`, `axum`, `tokio`, and many narrow bridge/read-model
  helpers; `src/websocket.rs` remains the main dispatcher over most of the
  crate surface.
- Boundary violations: service modules still own a lot of protocol-shape
  translation that looks like shared mapper work; the narrowest current
  examples are `src/network_remote_delivery_status_cross_process.rs`,
  `src/lan_pairing_browser_add_device_state.rs`, `src/screen_settings_api.rs`,
  and `src/enforcement_api.rs`, each repeating shape assembly around protocol
  contracts rather than service-only orchestration.
- Duplicated shapes: repeated field-table and read-model assembly appears across
  network remote-delivery status, LAN add-device state, screen settings,
  enforcement reports, and browser social notification delivery payloads; the
  router in `src/websocket.rs` fans the same command-family pattern into many
  near-identical report builders.
- Barrel/reexport/shim debt: no crate-root barrel was added, but there is still
  local reexport/shim debt, including `pub use errors::ActivityCaptureError` in
  `src/activity_capture.rs` and the `pub use ...` wrapper in
  `src/activity_api/social_parent_notification_delivery_read_model_payload.rs`.
  Those are small, but they violate the repo's no-reexport direction and hide
  ownership seams.
- Schema/contract drift: the service still mirrors protocol-owned response and
  status shapes closely, especially for enforcement, network delivery, LAN
  discovery, and screen settings. The risk is not immediate breakage, but the
  service can drift from the canonical protocol contract if field names,
  payload nesting, or status enums change independently.
- Event bus/log/proof misuse: `src/websocket.rs` uses the websocket as a
  general command multiplexer and falls back to log-snapshot style responses for
  unsupported branches; `src/service_runtime.rs` starts several background
  runtimes without a corresponding durable proof artifact, and
  `src/activity_capture.rs` logs failures only through dev log output. The proof
  chain is present, but it is distributed unevenly across logs, event payloads,
  and test helpers rather than a single service-owned proof boundary.
- Test/proof structure issues: tests are split between inline `#[cfg(test)]`
  modules, `tests/unit.rs`, and many path-routed unit files; several
  `tests/*` directories are just `.gitkeep` placeholders, so proof coverage is
  structurally broad but operationally fragmented.
- Current DRY score: 4/10. The crate is coherent, but too much of it is still
  family-specific mapping code with repeated assembly logic.
- Fix-pass recommendation: first collapse repeated field/read-model assembly
  into small internal helpers for network delivery, LAN add-device, screen
  settings, and enforcement reporting; then remove the obvious reexport/shim
  seams and keep `websocket.rs` as a thinner dispatcher.
- Decouple-pass recommendation: move protocol-shape construction toward shared
  family mappers or owning contract crates, so the service owns transport and
  orchestration while shared schema shaping lives in one place.
- Sequencing/blockers: fix-pass is blocked mainly by the breadth of command
  families flowing through `src/websocket.rs`; decoupling should be done in
  slices, not as a single crate-wide rewrite. The safest order is
  `websocket.rs` dispatch cleanup, then network/LAN/enforcement helper
  consolidation, then proof/test consolidation.
- Exact likely file paths: `crates/agent-service/src/lib.rs`,
  `crates/agent-service/src/websocket.rs`,
  `crates/agent-service/src/service_runtime.rs`,
  `crates/agent-service/src/activity_capture.rs`,
  `crates/agent-service/src/network_remote_delivery_status_cross_process.rs`,
  `crates/agent-service/src/lan_pairing_browser_add_device_state.rs`,
  `crates/agent-service/src/screen_settings_api.rs`,
  `crates/agent-service/src/enforcement_api.rs`,
  `crates/agent-service/src/activity_api/social_parent_notification_delivery_read_model_payload.rs`,
  `crates/agent-service/tests/unit.rs`.

## Current Refresh Audit - 2026-06-19

- Responsibility: `crates/agent-service` is the Rust orchestration boundary for
  websocket command handling, service-backed read models, runtime startup, and
  proof/log emission. The live dispatcher in `crates/agent-service/src/websocket.rs`
  still fans many command families into service-owned report builders, while
  `crates/agent-service/src/service_runtime.rs` bootstraps the runtime stack
  and log startup.
- Dependencies: this crate depends heavily on `agent-protocol`, `agent-core`,
  `ocentra-eventing`, `axum`, `tokio`, and many narrow bridge/helper modules.
  Routing pressure remains concentrated in `src/websocket.rs`, with
  feature-specific helper modules hanging off the same dispatcher.
- Violations: the boundary does not show a crate-root barrel in `src/lib.rs`,
  but it still contains local re-export/shim debt. The clearest current
  examples are `crates/agent-service/src/activity_capture.rs` with `pub use
  errors::ActivityCaptureError` and
  `crates/agent-service/src/activity_api/social_parent_notification_delivery_read_model_payload.rs`
  with a `pub use` wrapper over the report-writer handoff types. Those seams
  obscure ownership and conflict with the repo's no-reexport rule.
- Duplicated shapes: repeated field-table and read-model assembly still shows
  up across `network_remote_delivery_status_cross_process.rs`,
  `lan_pairing_browser_add_device_state.rs`, `screen_settings_api.rs`,
  `enforcement_api.rs`, and the social notification delivery payload path.
  `src/websocket.rs` also repeats the same command-family dispatch shape in
  several nested match blocks.
- Barrel/re-export/shim debt: `src/lib.rs` is still direct `mod` wiring rather
  than a barrel, which is good, but the local `pub use` seams above are the
  remaining debt. `crates/agent-service/src/activity_api.rs` is also a heavy
  frontage module that routes many payload/report files, so it behaves like a
  manual shim layer even without a formal re-export barrel.
- Schema drift: the service mirrors protocol-owned response and status shapes
  closely, especially for screen settings, enforcement, network remote
  delivery, and LAN add-device. That is acceptable for now, but it creates
  drift risk if protocol field names or nesting change without a matching
  service update.
- Event/log/proof misuse: `src/websocket.rs` still falls back to log-snapshot
  style responses for unsupported command branches, which is a valid safety
  net but also a sign that the dispatcher is carrying too much breadth.
  `src/service_runtime.rs` starts several background runtimes and logs startup,
  but durable proof artifacts are still spread across runtime logs, event
  payloads, and test helpers instead of a single service-owned proof boundary.
  `src/activity_capture.rs` also logs capture failures only through dev-log
  output.
- Test/proof structure issues: the crate has real unit coverage in
  `crates/agent-service/tests/unit.rs` and many path-routed test modules, but
  the surrounding `tests/*` tree is still dominated by `.gitkeep` placeholders.
  That makes the proof surface wide in naming, but sparse in executable
  structure.
- DRY score: 42/100. The crate is not a trivial wrapper, because it does own
  real orchestration and validation, but it is still held back by repeated
  field assembly, repeated router shapes, and several thin bridge modules that
  mirror protocol/core contracts instead of centralizing them.
- Fix recommendation: collapse repeated field/read-model assembly into smaller
  internal helpers around network remote delivery, LAN add-device, screen
  settings, enforcement, and social notification delivery. Keep
  `src/websocket.rs` thinner by pushing more family-specific construction into
  owning helpers.
- Decouple recommendation: move shape construction toward shared family mapper
  modules or the owning contract crates so `crates/agent-service` stays focused
  on transport, orchestration, and policy-gated response selection rather than
  repeating schema assembly.
- Blockers: no hard blocker to refreshing the audit itself. For actual cleanup,
  the main constraint is breadth in `src/websocket.rs` and the fact that the
  crate still serves many command families from one dispatcher, so any refactor
  should be sliced.
- Exact likely paths:
  `crates/agent-service/src/lib.rs`,
  `crates/agent-service/src/websocket.rs`,
  `crates/agent-service/src/service_runtime.rs`,
  `crates/agent-service/src/activity_capture.rs`,
  `crates/agent-service/src/network_remote_delivery_status_cross_process.rs`,
  `crates/agent-service/src/lan_pairing_browser_add_device_state.rs`,
  `crates/agent-service/src/screen_settings_api.rs`,
  `crates/agent-service/src/enforcement_api.rs`,
  `crates/agent-service/src/activity_api.rs`,
  `crates/agent-service/src/activity_api/social_parent_notification_delivery_read_model_payload.rs`,
  `crates/agent-service/tests/unit.rs`.
