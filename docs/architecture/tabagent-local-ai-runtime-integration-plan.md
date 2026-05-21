# TabAgent Local AI Runtime Integration Plan

Status: V0.7 research plan. This plan studies TabAgent and TabAgentServer as
local AI runtime reference systems and maps the useful pieces into Ocentra
Parent-owned contracts. It does not enable model execution, remote AI,
enforcement, portal-side evaluation, or broad subsystem copying.

## Product Boundary

Ocentra Parent's child-device safety path remains local by default:

```text
stored local evidence refs
  -> local evidence context builder
  -> child-device local provider/runtime
  -> schema-valid local AI safety result
  -> deterministic dry-run policy evaluator
  -> later audited enforcement handoff
```

The local provider/runtime may eventually load and run a model on the child
device, but only after contract and Rust protocol boundaries are explicit. It
must not replace stored evidence, parent rules, deterministic policy evaluation,
or future enforcement adapters.

Remote/API AI stays out of the child-device safety decision path. It can be
planned later for parent-facing reports or assistant flows only after explicit
parent action, custody, retention, deletion, and evidence-citation contracts
exist.

## Current Ocentra Baseline

The current repo has a conservative provider/runtime scaffold:

- `packages/parent-domain/src/local-ai-runtime.ts:22-128` defines runtime status,
  adapter probe, provider source, execution state, readiness, and provider
  capability contracts. The only executable-ready probe shape requires
  `adapter-ready`, `local-provider-configured`, `local-adapter-ready`,
  `dry-run-ready`, a non-unavailable provider source, and no unavailable reason.
- `packages/parent-domain/src/local-ai-context-selection.ts:130-180` filters
  evidence by allowed custody, rejects hosted non-activity custody for child
  activity evidence, degrades unallowed custody, and selects only grounded
  memory, graph, and parent-rule context references.
- `docs/architecture/local-ai-provider-runtime-boundary.md` keeps runtime status
  unavailable and local-only by default.
- `docs/architecture/local-ai-evidence-context-builder.md` makes the context
  builder consume stored evidence references only. It does not scan browsers,
  files, processes, network packets, or screenshots directly.

This is the right starting point. The next local runtime slices should harden
this scaffold instead of importing TabAgent runtime code wholesale.

## TabAgent Evidence Reviewed

| Area                         | Source evidence                                                                                                                                                             | What matters for Ocentra Parent                                                                                                                                                                                                                                                    |
| ---------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Browser model owner          | `E:\Desktop\TabAgent\src\backgroundModelManager.ts:303-429`, `563-721`, `756-1124`, `1315-1392`                                                                             | Central model ownership, fetch interception, cache-hit/download progress, load state, generation state, stop-generation, and model restore are useful runtime lifecycle patterns. Ocentra should not copy browser global fetch overrides into the safety path.                     |
| Extension native host bridge | `E:\Desktop\TabAgent\src\Controllers\NativeHostManager.ts:91-145`, `192-237`, `301-365`                                                                                     | Persistent connection, request ids, message timeouts, queued messages, reconnect attempts, and status events are good bridge-state patterns. Ocentra should express them through the existing local Rust service/API contracts, not as a hidden browser-extension control channel. |
| Native model service facade  | `E:\Desktop\TabAgent\src\Controllers\services\NativeModelService.ts:45-170`                                                                                                 | Pull, load, unload, delete, and state queries are the lifecycle command family Ocentra eventually needs, but Ocentra should use opaque local model refs and child-device custody contracts instead of raw model paths in parent-visible surfaces.                                  |
| Native inference facade      | `E:\Desktop\TabAgent\src\Controllers\services\NativeInferenceService.ts:50-128`                                                                                             | Generate, streaming fallback, and stop generation show the lifecycle shape Ocentra needs to model. Ocentra must add timeouts, cancellation, schema parsing, degraded states, and evidence refs before generation affects policy preview.                                           |
| Native messaging protocol    | `E:\Desktop\TabAgent\TabAgentServer\Rust\native-messaging\src\protocol.rs:14-40`, `185-199`; `router.rs:146-185`                                                            | Route/request/response envelopes, request id validation, registered model/generation/resource/hardware routes, and explicit route metadata are useful. Ocentra already has protocol-domain and Rust protocol constants; new routes must go through those layers first.             |
| Model cache                  | `E:\Desktop\TabAgent\TabAgentServer\Rust\model-cache\README.md:14-45`, `79-87`, `156-189`; `src/cache.rs:13-58`, `195-270`, `303-348`, `377-480`                            | Chunked storage, manifests, quant status, progressive download callbacks, cache stats, and streamed model file access are directly useful. Ocentra must keep model cache separate from the encrypted evidence journal and SQLite query store.                                      |
| Execution providers          | `E:\Desktop\TabAgent\TabAgentServer\Rust\execution-providers\README.md:7-18`, `48-64`; `src/lib.rs:169-218`                                                                 | Provider capability, availability checks, priority ordering, and CPU fallback are useful. Ocentra must make fallback explicit and local-only; no silent remote fallback is allowed.                                                                                                |
| Python ML service            | `E:\Desktop\TabAgent\TabAgentServer\PythonML\README.md:3-26`, `199-220`; `services/README.md:31-45`, `84-102`, `210-231`; `Rust\common\src\python_process_manager.rs:1-118` | Rust orchestrates a stateless Python subprocess over localhost gRPC, serves model files, and owns retry/fallback. Ocentra can study this, but should not add Python to the safety path until Rust-owned lifecycle, port, custody, and packaging boundaries are proven.             |
| Native loaders               | `E:\Desktop\TabAgent\TabAgentServer\Rust\onnx-loader\README.md:1-71`, `Rust\gguf-loader\README.md:1-18`, `80-101`, `142-162`                                                | ONNX and GGUF loaders show viable local runtime candidates. Ocentra should start with one narrow, reviewable adapter and leave other formats as unavailable capability states.                                                                                                     |

## What To Reuse As Ideas

### Runtime Owner

Use one child-device runtime owner for provider lifecycle. It should be a Rust
service module or crate that owns:

- runtime status and adapter probe results;
- model cache inventory and health;
- provider availability and fallback order;
- load/unload progress;
- generation running, completed, failed, timed out, or cancelled state;
- output parsing into local AI result contracts.

Capture adapters, SQLite read models, policy evaluation, enforcement adapters,
and portal UI must not load models or call providers directly.

### Model Cache

Use TabAgent's manifest-first cache pattern as the reference shape:

- model artifact refs are opaque ids, not parent-visible filesystem paths;
- manifests track available, downloading, downloaded, failed, corrupted, and
  removed states;
- cache stats report disk size, repo/model count, last checked time, and health;
- progress events are typed and rate-limited;
- cache corruption never deletes or weakens activity evidence;
- model cache storage is separate from encrypted child activity evidence and
  from SQLite query indexes.

The first Ocentra cache slice should inventory and report status only. It should
not download from HuggingFace or another source until a parent-approved model
artifact policy exists.

### Execution Providers

Use a provider registry with explicit local-only fallback:

1. Probe configured local providers without loading a model.
2. Report capability, resource class, provider source, and unavailable reason.
3. Choose a provider only for an approved local model artifact and supported
   task.
4. Fall back only among local providers already represented in capability
   contracts.
5. Degrade to model unavailable instead of calling a remote provider.

CPU fallback is acceptable only as a visible local provider choice. It is not a
reason to hide degraded performance or missing model capability.

### Generation And Halt

Generation needs a first-class lifecycle:

- `queued`: context accepted but no provider slot yet.
- `loading`: model load in progress.
- `running`: generation started.
- `cancelling`: halt requested.
- `completed`: output was produced and parsed.
- `failed`: provider error, invalid output, timeout, or cancellation failure.
- `cancelled`: halt completed before usable output.

Every generation attempt must cite:

- context id;
- evidence refs included by the context builder;
- parent rule refs considered;
- runtime status ref;
- provider id and model artifact ref;
- prompt/template version;
- timeout and max input size;
- output schema version;
- degraded or unknown states.

The local model output is never policy authority by itself. It becomes input to
typed `LocalAiSafetyResult` parsing and then deterministic dry-run policy
evaluation.

### Native Or Python Runtime

Prefer a Rust-native adapter first because Ocentra Parent already centers the
child-device service in Rust and needs packaged Windows-first operation. If a
Python ML bridge is introduced later, follow strict constraints:

- Rust starts, monitors, and stops the subprocess.
- The subprocess binds only to loopback on a managed port.
- Python has no direct child evidence file access.
- Rust serves model artifacts or temp inputs through typed local APIs.
- Python failures are hard failures surfaced as degraded runtime states.
- Packaging and dependency checks are part of the slice, not a later surprise.

Python is not a default dependency for the V0.7 dry-run checkpoint.

## What Not To Copy

- Do not copy TabAgent's assistant UI, persona, browser dashboard, broad agent
  workflows, or remote/provider catalog.
- Do not copy string route ids, provider names, model ids, or field names into
  Ocentra app/runtime code. They belong in domain packages and Rust protocol
  constants.
- Do not copy browser global fetch interception into Ocentra's child safety path.
- Do not copy HuggingFace token handling into child-device safety by default.
- Do not expose raw model paths, raw prompts, screenshots, browser secrets,
  packet data, tokens, or credentials to the parent portal.
- Do not let provider fallback call remote/API models for child-device safety.
- Do not let memory, graph, model output, or cache status replace stored
  evidence refs.
- Do not add enforcement, blocking, timers, or ask-parent actions in the runtime
  adapter slice.

## Integration Slices

### Slice 0: Keep The Current Status Boundary Honest

Owner: existing parent-domain, agent-protocol, agent-service runtime status
contracts.

Goal:

- Keep unconfigured provider status unavailable.
- Keep execution disabled.
- Keep provider source unavailable or local-only.
- Keep portal visibility as status only.

Validation:

- Parent-domain runtime tests.
- Agent protocol runtime tests.
- Agent service runtime status tests.
- Existing portal status rendering tests when touched.

### Slice 1: Local Model Artifact And Cache Contracts

Owner: `@ocentra-parent/parent-domain` first, then Rust protocol only if the
service reports it.

Add contracts for:

- local model artifact ref;
- model source policy: bundled, parent-installed, local-cache, unavailable;
- model cache status and cache health;
- model manifest ref and integrity state;
- download status, but with download disabled by default;
- cache storage error and corruption reason codes.

Acceptance:

- Model artifact refs are opaque and safe to show.
- Model cache status cannot be mistaken for activity evidence.
- No network download is added in this slice.

### Slice 2: Provider Capability And Probe Read Path

Owner: Rust service status module plus matching TypeScript/Rust contracts.

Add a real local probe read path that can report:

- OS and hardware class;
- configured local provider candidates;
- available local provider candidates;
- fallback order;
- why execution remains disabled.

The probe may inspect local runtime configuration and OS capability. It must not
load a model, open a prompt, scan evidence, or call a remote API.

Acceptance:

- Unconfigured systems stay unavailable with explicit reasons.
- Invalid config stays non-executable.
- Local CPU fallback is visible as a fallback, not hidden.

### Slice 3: Load And Unload Lifecycle Skeleton

Owner: Rust service runtime module and protocol contracts.

Add typed request/result/progress events for:

- load requested;
- load progress;
- loaded;
- unload requested;
- unloaded;
- load failed;
- unload failed.

Execution remains disabled until a reviewed adapter is added. This slice can
report that no adapter exists for the requested artifact.

Acceptance:

- Parent portal can eventually explain load state without running generation.
- Load attempts do not touch evidence storage.
- Cancelling or unloading cannot erase evidence refs.

### Slice 4: Generation Contract Skeleton

Owner: TypeScript contracts first, Rust protocol parity second.

Add generation request/result contracts that require:

- a context-builder output ref;
- selected evidence refs;
- parent rule refs;
- runtime status ref;
- local model artifact ref;
- prompt/template version;
- timeout and max input size;
- output schema version.

This slice should still return unavailable or disabled by default.

Acceptance:

- A caller cannot submit raw page text, raw screenshot bytes, raw journals, raw
  SQLite rows, packet dumps, or unbounded OS content.
- Requests without context-builder evidence refs are rejected.
- Remote providers cannot satisfy the contract.

### Slice 5: First Narrow Local Adapter

Owner: a new small Rust runtime module or crate after contract slices are green.

Pick one local path only:

- ONNX classification adapter for a tiny local model, or
- GGUF text adapter for a local model, or
- a Rust-managed Python subprocess only if packaging and port isolation are
  accepted first.

Entry criteria:

- local model artifact is present and integrity-checked;
- provider probe reports ready;
- context builder supplies a schema-valid context;
- generation request includes a timeout;
- output parser rejects invalid output;
- policy evaluator remains dry-run.

Acceptance:

- The adapter returns a schema-valid local AI safety result or an explicit
  unavailable/degraded state.
- Timeouts, invalid output, cancelled generation, missing model, and unavailable
  provider are tested as real code paths.
- Enforcement remains disabled.

## Test And Validation Plan

Docs-only plan changes:

```powershell
cmd /c npm run format:check
git diff --check
cmd /c npm run lanes:guard
cmd /c npm run hub:guard
```

Contract slices:

```powershell
cmd /c npm --workspace @ocentra-parent/parent-domain run test
cmd /c npm --workspace @ocentra-parent/parent-domain run lint:exec
cmd /c npm --workspace @ocentra-parent/agent-protocol-domain run test
cmd /c npm --workspace @ocentra-parent/agent-protocol-domain run lint:exec
cargo test -p ocentra-parent-agent-protocol local_ai
cargo test -p ocentra-parent-agent-service local_ai
node scripts/check-source-shape.mjs
git diff --check
cmd /c npm run lanes:guard
cmd /c npm run hub:guard
```

Runtime adapter slices:

- Use real contracts, parsers, protocol structs, and local service paths.
- Do not use mocks, fakes, stubs, spies, MSW, Nock, or fake-green provider
  behavior.
- If no real model artifact is available, keep the result unavailable and test
  the unavailable path.
- When a real model artifact is introduced, use a small deterministic local
  artifact that can run inside the development and CI constraints, or gate the
  runtime proof with an explicit environment capability check and report the
  unsupported state honestly.

## Risks And Open Questions

- Model downloads can create data, licensing, and supply-chain obligations.
  The first cache slice should report inventory only.
- Python would add packaging, dependency, port, and lifecycle complexity. It is
  useful to study, but not the first Windows local runtime choice.
- Hardware provider detection can be misleading. Provider probes must separate
  compile-time support, OS support, driver availability, model compatibility,
  memory pressure, and execution permission.
- Streaming through browser native messaging is awkward in TabAgent. Ocentra's
  Rust service should prefer its existing local API/WebSocket boundary for
  progress and cancellation events.
- Parent-visible status must never imply that AI is enforcing rules while V0.7
  remains dry-run only.

## Done Signal

This plan is ready for primary review when the repo has a docs-only, evidence
referenced local runtime integration path that:

- keeps local-only child-device safety as the default;
- forbids remote/API fallback in the child safety path;
- keeps model cache separate from evidence storage;
- uses context-builder evidence refs as the only model input source;
- keeps enforcement out of scope;
- names contract-first slices before runtime execution;
- identifies which TabAgent ideas are reusable and which must not be copied.
