# Project Progress Matrix

This is the code-backed execution dashboard for Ocentra Parent. It supplements
`PLAN_INDEX.md`; it does not replace plan-local workpacks, proof roots, or
checklists.

Last broad source inventory: 2026-08-15, on the merged `develop` organization
baseline at `bd27c29498` (tree-equal to `main` at `6728a1d441`). The dated
merged-code delta immediately below overrides older snapshot wording for its
named plans; historical rows remain routing context, not current closure proof.

## Current merged repository and code/test audit baseline - 2026-08-15

Repository organization is complete for this audit baseline. Consolidation PR
`#646` merged normally to `main` as `608ef84fb` after CI run `31862487297`
completed with 61 successful jobs and all three required gates green. `develop`
was created at the same commit and has the same required checks, pull-request,
conversation-resolution, no-force-push, and no-delete protections as `main`.
There are no open pull requests. Remote branches are reduced to `main`,
`develop`, `production`, and the protected
`codex/archive/all-remote-tips-20260815`; the archive contains every deleted
remote tip. The only registered OcentraParent worktree is `E:/OcentraParent`;
short-lived audit branch names are intentionally not status authority here.
AI's 48-workpack audit reached `main` through PRs `#700`, `#701`, and `#702`;
Tracking's 42-workpack audit reached `main` through PRs `#703`, `#704`, and
`#705`.

The executable graph validates at 703 nodes and 705 edges. It imports **23**
actual plan rows and 679 workpacks. The earlier apparent count of 24 included
the Markdown table header in `PLAN_INDEX.md`; no plan directory is missing.
Current derived state is 452 planned, 9 blocked, 0 ready, 1 active, 216 in
validation, and 1 done.

This is the key code-first limitation: live plan roots contain 2,704
implementation files and 1,118 test files, but only **365 of 679 workpacks**
(53.76%) have reviewed, exact code/test ownership maps. Account Identity, AI,
Browser, LAN,
Child Agent Runtime Distribution, Cloudflare Control Plane, Data Custody,
Device Trust, Eventing, Logging Domain Parity, Parent Client Runtime Distribution,
Payment/Subscription, Policy Control Plane, Portal UX/Household Surfaces,
Remote Access, Network, Screen AI, Screen, Setup/Install/Provisioning, Tracking,
and V0.8 Enforcement are fully mapped; 314 workpacks across the repository
remain unmapped. An unmapped workpack is
therefore **unattributed**, not proven absent and not proven implemented. Do not
turn the graph state or a checklist mark into a code-completion percentage.
`npm run graph:matrix -- --json` is the complete 679-row table; the reviewed
coverage below states how much of that table currently has source/test evidence
strong enough for workpack-level decisions.

| Plan | Workpacks | P/B/R/A/V/D | Live implementation/test files | Reviewed workpack maps | Code-first audit state |
| --- | ---: | ---: | ---: | ---: | --- |
| Account identity/family | 8 | 1/0/0/0/7/0 | 159 / 82 | 8 / 8 | Fully code-mapped; all eight remain incomplete for Phase 1 code/expected-test writing. |
| AI | 48 | 46/0/0/0/2/0 | 118 / 39 | 48 / 48 | Fully code-mapped; 11 workpacks are Phase 1 complete for bounded scope and 37 retain concrete production-code or expected-test gaps. |
| App/game | 220 | 132/0/0/0/88/0 | 688 / 436 | 0 / 220 | Unattributed; the large validation set cannot be treated as implemented. |
| App | 95 | 94/0/0/0/1/0 | 670 / 421 | 1 / 95 | Partial; WP01 contract/runtime-decision roots are mapped. |
| Browser | 30 | 30/0/0/0/0/0 | 72 / 13 | 30 / 30 | Fully code-mapped; 14 executable workpacks are Phase 1 complete for bounded scope, 10 retain concrete code/test gaps, and six imported packets are reference-only. |
| Child-agent runtime distribution | 11 | 0/1/0/0/10/0 | 88 / 10 | 11 / 11 | Fully code-mapped; WP01, WP02, WP05, WP06, and WP09 are Phase 1 complete for their bounded scope, while six workpacks retain runtime, lifecycle-test, handoff, or release-gate gaps. |
| Cloudflare control plane | 13 | 13/0/0/0/0/0 | 183 / 63 | 13 / 13 | Fully code-mapped; WP00-WP02 and WP04 are Phase 1 complete, while nine workpacks retain concrete code/test gaps. |
| Data custody/storage | 9 | 1/0/0/1/7/0 | 653 / 410 | 9 / 9 | Fully code-mapped; WP04 and the source-only migrated UI reference are Phase 1 complete. Seven implementation workpacks remain incomplete. |
| Device trust bootstrap | 9 | 1/2/0/0/6/0 | 426 / 131 | 9 / 9 | Fully code-mapped; WP08 is complete for its research/test-only scope, while eight workpacks retain concrete code/test gaps. |
| Eventing | 13 | 1/0/0/0/11/1 | 777 / 492 | 13 / 13 | Fully code-mapped; Phase 1 is complete for 3 workpacks and incomplete for 10. Only WP06 is graph-done. |
| LAN | 25 | 0/0/0/0/25/0 | 308 / 60 | 25 / 25 | Fully code-mapped; 22 workpacks have bounded Phase 1 code/expected tests written, while WP16, WP20, and WP25 retain an integrated delivery-test or missing executable-verifier gap. |
| Logging domain parity | 10 | 5/0/0/0/5/0 | 127 / 49 | 10 / 10 | Fully code-mapped; WP01-WP04 and WP09 are Phase 1 complete, while five workpacks retain concrete expected-test or instrumentation-enforcement gaps. |
| Network | 8 | 7/0/0/0/1/0 | 378 / 99 | 8 / 8 | Fully code-mapped; WP05 and WP08 are Phase 1 complete for their bounded scopes, while WP01-WP04 and WP06-WP07 retain canonical-contract, live-runtime, production-wiring, or executable-harness gaps. |
| Parent desktop/runtime package | 11 | 4/0/0/0/7/0 | 497 / 181 | 11 / 11 | Fully code-mapped; WP01, WP04, WP05, and WP09 are Phase 1 complete for their bounded scope, while seven workpacks retain concrete runtime or expected-test gaps. |
| Payment/subscription | 13 | 8/2/0/0/3/0 | 44 / 39 | 13 / 13 | Fully code-mapped; WP00 is complete for Phase 1 code/expected-test writing, while twelve workpacks retain concrete code/test gaps. |
| Policy control plane | 8 | 0/2/0/0/6/0 | 911 / 481 | 8 / 8 | Fully code-mapped; WP03 is Phase 1 complete and seven workpacks retain concrete code/test gaps. |
| Portal UX/household surfaces | 20 | 15/0/0/0/5/0 | 974 / 531 | 20 / 20 | Fully code-mapped; 9 workpacks have no Phase 1 writing gap in their bounded scope, while 11 retain concrete product-code or expected-test gaps. |
| Remote access | 6 | 4/0/0/0/2/0 | 35 / 19 | 6 / 6 | Fully code-mapped; WP01 and proof-only WP06 have no Phase 1 writing gap, while WP02-WP05 retain concrete runtime, deferred-control, persistence, or relay-security gaps. |
| Screen AI pipeline | 10 | 10/0/0/0/0/0 | 124 / 33 | 10 / 10 | Fully code-mapped; prerequisite routing is the only bounded Phase 1 row without a writing gap. WP02-WP10 retain production-composition, authority, durability, custody-negative, performance-test, or missing executable-harness gaps. |
| Screen | 43 | 25/0/0/0/18/0 | 95 / 26 | 43 / 43 | Fully code-mapped; 9 of 40 executable workpacks are complete for bounded Phase 1 code/expected-test writing, 31 retain concrete gaps, and three imported reference packets own no executable code. |
| Setup/install/provisioning | 7 | 0/1/0/0/6/0 | 529 / 196 | 7 / 7 | Fully code-mapped; six workpacks have no Phase 1 writing gap, while WP07 remains a static unavailable-state panel rather than the required live first-run state machine. |
| Tracking | 42 | 42/0/0/0/0/0 | 94 / 65 | 42 / 42 | Fully code-mapped; 24 bounded packets are Phase 1 complete and 18 retain concrete production-code or expected-test gaps. |
| V0.8 enforcement | 20 | 13/1/0/0/6/0 | 901 / 498 | 20 / 20 | Fully code-mapped; 7 workpacks have no Phase 1 writing gap in their bounded scope, while 13 retain concrete runtime, surface, lifecycle-test, or executable-harness gaps. |

The next organization phase is not feature coding. Audit one plan at a time,
map each workpack to exact implementation and test roots, classify code/test
gaps, then rebuild the graph. Only after a plan's Phase 1 map is complete may
its focused tests and Enforcer checks be used for Phase 2 scheduling. Proof is
the later acceptance phase, not a substitute for missing code or tests.

### Browser plan Phase 1 code/test audit - 2026-08-15

This audit maps the current Rust/service/portal implementation and expected
tests. It does not use historical `output/` proof as implementation evidence
and does not claim that the focused test families have been rerun in Phase 2.

| Workpack | Actual code/test evidence | Phase 1 | Remaining code or expected-test gap |
| --- | --- | --- | --- |
| WP01 Contract Boundary And Effect Schemas | Rust protocol inventory, managed-session, policy, intervention, and read-model contracts have contract/unit tests, including dishonest exact-URL and typed rejection cases. | **Complete for Phase 1** | Generated-edge/proof reconciliation is later; no missing bounded contract family was found. |
| WP02 Source Index And Doc Reconciliation | Coordination-only packet; it owns no product code. | **Complete for Phase 1** | Current source paths still need Phase 3 documentation reconciliation, but no implementation belongs here. |
| WP03 Browser Inventory Model | Typed inventory/read-model contracts and service row conversion have managed/unmanaged and dishonest-claim tests. | **Complete for Phase 1** | Live platform collection belongs to WP04/WP05. |
| WP04 Windows Browser Inventory Adapter | Registry, shortcut, package, known-path, and process inventory code plus service conversion is covered by focused fixture/source tests. | **Complete for Phase 1** | Real-host proof is Phase 3; the bounded Windows adapter and expected tests are written. |
| WP05 Cross-Platform Inventory Matrix | Windows inventory and an Android owned-shell implementation exist. | **Incomplete** | No current macOS/iOS inventory adapter or focused platform-matrix implementation/test family exists; Linux/Android visibility is not represented by one runtime-owned matrix. |
| WP06 Managed Profile Store | Owned-path store code covers create/reload/delete/repair, redacted metadata, and default/unowned path rejection. | **Incomplete** | Corrupt/truncated metadata, atomic-write crash behavior, and concurrent-writer negatives are absent. |
| WP07 Managed Chromium Launcher | Launch planning, executable identity, owned-profile enforcement, loopback port reservation, failed-spawn behavior, and service status tests are written. | **Complete for Phase 1** | Real browser execution is Phase 2/3 validation, not a missing code family here. |
| WP08 Bridge Custody And Security | Loopback HTTP/CDP polling enforces port, process, profile, session, browser identity, timeout/size, and debugger-URL redaction boundaries with negatives. | **Complete for Phase 1** | Runtime proof remains later. |
| WP09 CDP Version And Target Adapter | Version/target parsing rejects malformed JSON, wrong shapes, missing identifiers/URLs, oversized responses, and timeouts. | **Complete for Phase 1** | No active-focus claim is made; that belongs to WP11. |
| WP10 Tab Evidence Mapper | Target observations map stable custody/tab/window/origin fields, strip credentials, normalize URLs, and reject invalid URLs/empty target IDs. | **Complete for Phase 1** | The mapper intentionally produces target-list evidence, not active-tab proof. |
| WP11 Active-Tab Proof Model | Typed active-state/capability/proof-source fields and no-overclaim tests exist. | **Incomplete** | No focus/activation observer feeds the model; `/json/list` remains target-list evidence and cannot establish the active tab. |
| WP12 Journal And SQLite Browser Ingest | Ordered event runtime, encrypted journal replay, SQLite projection, restart recovery, duplicate replay, stale/degraded reconstruction, and empty-state tests are written. | **Complete for Phase 1** | Focused execution remains Phase 2. |
| WP13 Browser Read Models And Service Events | Service inventory and runtime stream APIs project event chains, pending candidates, policy previews, unavailable/stale states, and WebSocket command reports with tests. | **Complete for Phase 1** | Product action execution belongs to WP17/WP19, not this read-model packet. |
| WP14 Portal Browser Status Surfaces | Portal status/intervention renderers consume Rust-owned snapshots and retain explicit empty states. | **Incomplete** | Current tests mainly inspect source shape/intervention separation; dedicated rendered status cases for missing, unmanaged, managed-ready, running, disconnected, stale, and unsupported states are missing. |
| WP15 Browser Policy Authoring Manifest | Generated manifest identifiers feed schema-domain and service patch/store runtime tests, including dishonest update rejection. | **Complete for Phase 1** | Parent-facing authoring UX is outside this bounded manifest packet. |
| WP16 Policy Target Compiler | Compiler code/tests label target requirements, preserve parent authority, keep observe/dry-run non-executing, require adapter proof, and expose manual-required policy-writer state. | **Complete for Phase 1** | Trusted adapter execution is an Enforcement dependency. |
| WP17 Managed Intervention And Block Page | Typed intervention events, a served child page, a portal-domain renderer, and SQLite read-model tests exist. | **Incomplete** | No production policy-decision-to-managed-navigation/block-page delivery owner or receipt/rollback integration test exists. |
| WP18 Unmanaged Browser Detection | Process-only discovery classifies supported, unsupported, and unknown browser-like processes; contracts forbid exact URL claims. | **Complete for Phase 1** | Enforcement/action behavior belongs to WP19. |
| WP19 Unmanaged Fallback UX And Actions | Read-model fallback derivation and portal rendering distinguish warn/terminate/relaunch/manual states without URL overclaim. | **Incomplete** | No trusted terminate/warn/relaunch dispatch owner, durable execution receipt, denial, or rollback test closes the action path. |
| WP20 Windows AppLocker And App Control Proof | Enforcement exposes typed app-control proof states and a tested read model. | **Incomplete** | No AppLocker/WDAC policy application, event capture, rollback, or real-host harness code exists. |
| WP21 Extension And Native Host Boundary | Native-host frame validation covers managed-profile binding, trusted origin, length/schema drift, default/missing binding, and stale heartbeat. | **Incomplete** | A browser extension/runtime connector, installation/registration path, handshake lifecycle, and integration tests are absent. |
| WP22 Performance And Service Health | A browser-core performance-budget model/test and service status serialization tests exist. | **Incomplete** | No executable load/latency/memory/long-run bridge-health harness or recovery-under-load test exists. |
| WP23 E2E And Manual Proof Artifacts | One Playwright browser-AI explanation scenario and failure helper are present. | **Incomplete** | The advertised aggregate browser E2E/manual-artifact verifier is absent; managed launch, custody loss, intervention, unmanaged fallback, and service restart are not covered by one executable suite. |
| WP24 Rollout, Checklist, And PR Gate | Coordination-only final gate; it owns no product implementation. | **Complete for Phase 1** | It remains operationally open until Phase 2/3 inputs are green and merged. |
| Browser Control 1057 Settings Inventory | Imported design/reference packet; no executable code ownership. | **Reference only** | Route implementation through WP15/WP16. |
| Browser Control Coverage Matrix | Imported coverage/reference packet; no executable code ownership. | **Reference only** | Reconcile against the executable workpacks; do not schedule as a product slice. |
| Browser Control Schema Proposal | Imported schema proposal; current schema authority is WP01/WP15. | **Reference only** | Do not restore a parallel contract owner. |
| Browser Policy Questionnaire Forest V1 | Imported authoring reference; generated schema tests exist under WP15 ownership. | **Reference only** | Treat changes as WP15 authoring-manifest work. |
| Browser Policy Settings Catalog | Imported settings reference; current catalog/manifest implementation is WP15. | **Reference only** | Do not count it as another execution packet. |
| Managed Unmanaged Browser | Imported product-boundary reference; executable work is split across WP03-WP21. | **Reference only** | Keep managed exact-URL evidence separate from unmanaged process-only control. |

**Browser Phase 1 result:** all 30 imported packets now have reviewed ownership.
Fourteen of the 24 executable workpacks have their bounded core code and
expected tests written; ten remain incomplete. The first dependency chain is
WP05/WP06 platform and custody hardening, WP11 active-focus evidence, then
WP17/WP19 trusted intervention execution. WP14, WP20-WP23 close the product
surface, platform control, extension, health, and aggregate-test gaps before
any whole-plan Phase 2 or proof claim.

### LAN plan Phase 1 code/test audit - 2026-08-15

This audit maps the current Rust/service/desktop/portal paths and their expected
tests. Historical generated `output/` or `test-results/` files are not counted
as source, and manual physical-device proof is deferred to Phase 3.

| Workpack | Actual code/test evidence | Phase 1 | Remaining code or expected-test gap |
| --- | --- | --- | --- |
| WP01 Contract Boundary And Effect Schemas | Rust protocol pairing, source, route-snapshot, and add-device-state contracts have focused contract tests and fail-closed version handling. | **Complete for Phase 1** | Contract generation/proof reconciliation is later. |
| WP02 Evidence Model And Device Record | Rust read models and service registry projection preserve source evidence and canonical household rows with unit tests. | **Complete for Phase 1** | Physical evidence quality is Phase 3. |
| WP03 Interface Detection | Interface hardware/default-route discovery captures gateway, DNS, DHCP, broadcast, IPv6, and selection reasons with unit tests. | **Complete for Phase 1** | Real-host platform proof remains later. |
| WP04 Neighbor Table Ingestion | Windows, Linux, and macOS parsers normalize IPv4/IPv6, malformed, incomplete, and duplicate rows under focused tests. | **Complete for Phase 1** | Live macOS validation is Phase 3, not missing parser code. |
| WP05 Targeted ARP Checks | Bounded targeted refresh enforces interface/subnet/throttle rules and covers response/no-response packet-IO behavior. | **Complete for Phase 1** | Packet capture is Phase 3. |
| WP06 Bounded ARP Sweep | Scan planning, bounded refresh, command handling, service physical-scan integration, and suppression behavior have Rust tests. | **Complete for Phase 1** | Real-router proof is Phase 3. |
| WP07 Passive Discovery Listeners | Passive ARP, DHCP, mDNS, SSDP, WSD, LLMNR, NetBIOS, beacon, and allowed SNMP observations feed hint-only runtime state with tests. | **Complete for Phase 1** | Long-running listener and packet proof are Phase 3. |
| WP08 mDNS And DNS-SD Discovery | PTR/SRV/TXT/A/AAAA parsing, service enumeration, hostile-name rejection, and hint-only handling have focused tests. | **Complete for Phase 1** | Broader packet/platform proof remains later. |
| WP09 SSDP And UPnP Discovery | Bounded search, safe private descriptor fetch, parsing, unsupported-device handling, timeout, malformed, and oversize negatives are written. | **Complete for Phase 1** | Physical UPnP coverage is Phase 3. |
| WP10 NetBIOS LLMNR Reverse DNS | Weak name evidence and DNS-like passive parsing reject unsafe/malformed values and avoid identity overclaim under tests. | **Complete for Phase 1** | No code gap found in the bounded name-evidence scope. |
| WP11 Light Service Probing | Bounded HTTP/HTTPS/TLS, WSD, and SNMP identity probing plus interface/trusted-device suppression have focused tests. | **Complete for Phase 1** | Optional OS fingerprinting is manual-gated, not required production code. |
| WP12 OUI Vendor Lookup | MAC normalization, private/randomized handling, and vendor lookup behavior have unit tests. | **Complete for Phase 1** | Dataset/provenance refresh is later acceptance work. |
| WP13 Merge And Deduplication Engine | Canonical household merge, strong/weak identity scoring, registry continuity, explicit decisions/reasons, and property tests are written. | **Complete for Phase 1** | Physical collision proof is Phase 3. |
| WP14 Explainable Classification | Weighted classification, explicit reasons/confidence, router/unsupported/unknown fencing, and portal label rendering have tests. | **Complete for Phase 1** | Manual installability proof is later. |
| WP15 Household Device Store | Persistent registry, scan history, replay projection, restart continuity, and stale-address suppression have service tests. | **Complete for Phase 1** | Cross-session physical proof remains later. |
| WP16 Read Models And LAN Events | Service event-chain stream, parent replay validation, desktop delivery decision, and portal replay/state seams each have focused tests. | **Incomplete** | No integrated backend -> real Tauri `AppHandle` emit -> portal listener test proves the complete delivery chain. |
| WP17 Parent Child mDNS Advertisements | Advertisement encoding, opaque metadata, lifecycle evaluation, and service synchronization have focused tests. | **Complete for Phase 1** | Multicast/platform proof is Phase 3. |
| WP18 Signed Child Hello And Heartbeat | Signature/capability/replay rejection, unpaired handling, heartbeat/offline/manual-required state, and service projection have tests. | **Complete for Phase 1** | Second-device and iOS proof are Phase 3. |
| WP19 Assignment Revocation And Audit | Trusted route commands, rename, ignore/restore, select/revoke, audit, persistence/restart, and portal dispatch paths have tests. | **Complete for Phase 1** | Broader physical topology proof remains later. |
| WP20 Proof Gates Fixtures And Rollout | Fixture, property, performance, and portal visual test families exist. | **Incomplete** | Six LAN aggregate verifier commands named by the current plan docs point to absent scripts; two of them are incorrectly checked complete in this workpack. |
| WP21 Contract Boundary And Domain Schemas | Family identity contracts and setup lifecycle records needed by LAN pairing have contract/unit tests. | **Complete for Phase 1** | This is a bounded dependency contract packet. |
| WP22 Current State And Gap Map | Documentation-only reconciliation packet; it owns no product code. | **Complete for Phase 1** | Status must follow this live audit. |
| WP23 Pairing And Route Proof | Browser runtime pairing contract, service runtime command path, route selection, and restart behavior have focused tests. | **Complete for Phase 1** | Real two-device proof is Phase 3. |
| WP24 Portal UX And First Run Handoff | Parent LAN route snapshots, portal-domain device rows, and Devices/command-center E2E surfaces have tests. | **Complete for Phase 1** | Physical first-run proof is later. |
| WP25 Rollout Checklist And PR Gate | Parent replay, desktop host decision, portal state edge, and visual E2E tests exist. | **Incomplete** | The aggregate source-matrix verifier is absent and the integrated backend-to-AppHandle-to-listener regression is still missing. |

**LAN Phase 1 result:** all 25 workpacks now have reviewed code/test ownership.
Twenty-two have their bounded core code and expected tests written. WP16 needs
one real cross-process delivery regression; WP20 needs the six named aggregate
verifier programs restored or replaced; WP25 depends on both gaps.
Manual household/router/platform artifacts remain Phase 3 and do not reopen the
22 code-complete rows.

### Network plan Phase 1 code/test audit - 2026-08-15

This is a source-and-test inspection of the production call paths and their
focused tests. Graph topology satisfaction means the mapped files exist; it
does not mean a fixture/proof builder is a live product path or that tests have
been rerun in Phase 2.

| Workpack | Reviewed live code/test evidence | Phase 1 | Concrete code/test gap |
| --- | --- | --- | --- |
| WP01 Foundation Contracts And Eventing | 60 implementation and 11 test files cover typed network observations, reusable event envelopes, runtime phase records, delivery, queueing, and stream serialization. | **Incomplete** | Canonical evidence/domain truth is duplicated between `agent-protocol` and `ocentra-network-evidence`; the runtime creates an in-memory bus/journal per observation and synthesizes downstream phase records instead of consuming durable owners. Contract parity, durable composition, and publisher-authority negatives remain unwritten. |
| WP02 Passive Capture And Parsing | 71 implementation and 9 test files cover deterministic PCAP/Ethernet/IP/TCP/UDP/DNS/TLS/HTTP parsing, encrypted-visibility limits, flow sessionization, raw-capture custody, Windows `netstat -ano` metadata capture, and service readiness bridges. | **Incomplete** | The parser stack is not fed by the production capture path. Windows production captures connection metadata only; no Npcap/libpcap driver or live packet-to-parser integration exists, and no real live-capture parser test covers that path. |
| WP03 Classification And Correlation | 19 implementation and 11 test files cover normalized domains, categories, classification, tunnel/transfer signals, process, managed/unmanaged browser, app/game, and screen-summary correlation. | **Incomplete** | Production service code does not invoke this classifier/correlation composition; it derives a source kind from whether a process name exists and assigns fixed evidence grades. A live parser-to-correlation-to-read-model path and its negative tests are absent. |
| WP04 Cross-Slice Cascade And Parent Surface | 31 implementation and 13 test files cover cascade/bundle construction, local-AI queue models, policy/notification models, the service read model, and a portal evidence drawer. | **Incomplete** | The service read path calls a proof helper that fabricates AI, policy, adapter, retention, and notification references. No real queued AI job, policy request, provider notification, adapter result, or custody lifecycle is composed into the product path. |
| WP05 Intervention Adapter Proof Gates | 84 implementation and 26 test files cover DNS, Windows Firewall/WFP, Android VPN, Apple Network Extension, Linux nftables, rollback/unavailable/no-overclaim states, protocol status contracts, and service status bridges. | **Complete for Phase 1** | No missing bounded gate code or expected negative-test family was found. Actual OS mutation and physical-platform acceptance remain Phase 2/3 and owning-platform work. |
| WP06 Analyzer, AI Audit, And Risk Budget | 48 implementation and 12 test files cover Zeek/signature ingestion, deterministic AI evaluation/audit models, thresholds, and risk-budget decisions. | **Incomplete** | No live analyzer/model runtime feeds production. The service proof builder compares a hard-coded `VpnProxyTunnel` expectation to the same hard-coded prediction and supplies fixed risk inputs, so drift/precision monitoring and a real analyzer-to-policy handoff are unwritten. |
| WP07 Performance, Security, And Rollout | 20 implementation and 5 test files cover deterministic performance/readiness rows, local platform probes, and adapter-capability status validation. | **Incomplete** | There is no executable concurrency/spike/soak/load harness, live metrics or alert observation, abuse/rate-limit/DoS runner, or rollout/rollback automation. These are missing expected test/harness code, not merely missing proof. |
| WP08 Control Catalog Reference Routing | 16 implementation and 3 test files cover the Rust-owned generated control catalog, generated-drift contract, and exact reference-routing/no-runtime-claim boundary. | **Complete for Phase 1** | No code/test-writing gap exists inside this reference-only workpack. Runtime control authority is explicitly outside its scope. |

**Network Phase 1 result:** 8/8 workpacks inspected and mapped; 2/8 are
complete for code/test-writing scope and 6/8 still require production code or
expected tests. No Network test pass, Enforcer acceptance, proof, or whole-plan
completion is claimed by this Phase 1 audit.

### Screen AI Pipeline Phase 1 code/test audit - 2026-08-15

This audit follows the production startup, capture, queue, local-AI, event,
read-model, and portal paths and compares them with the expected test and
harness code. It does not use the unchecked plan rows as implementation truth,
and it does not treat a generated event chain or proof description as a live
policy/action path.

| Workpack | Reviewed live code/test evidence | Phase 1 | Concrete code/test gap |
| --- | --- | --- | --- |
| WP01 Prerequisite Merge And Branch Gate | This is a coordination-only prerequisite row with no owned production or test code. Current branch promotion is handled by the repository feature-to-`develop`-to-`main` process. | **Complete for Phase 1** | No code or expected-test writing belongs to this workpack. Exact prerequisite commit/PR records and stale-proof reconciliation remain Phase 3 coordination evidence. |
| WP02 Real Trigger To Capture Gate | 31 implementation and 10 test files cover real selected-window capture, scheduler suppression/debounce, native foreground observation, timed cadence, encrypted queue writes, screen settings commands, and focused capture/runtime tests. | **Incomplete** | Production capture reads environment flags rather than the parent screen-settings store. Every foreground change is labeled `NativeAppForegroundStart`; browser URL/video/feed, browser game, native game, launcher, and unknown-process trigger kinds exist only as scheduler vocabulary or isolated tests and are not fed by owning runtimes. Cross-trigger and parent-disable integration tests are missing. |
| WP03 Capture To AI Analysis Gate | 47 implementation and 9 test files cover encrypted queue leasing, a bounded local adapter process, schema-shaped result parsing, OCR redaction, provider unavailable/invalid states, and separate typed router/pipeline libraries. | **Incomplete** | Production never calls the `screen-ai-core` intelligence router, AI-hub router, or pipeline decision. The service sends the frame to one configured adapter process and accepts only `localVision`/`localOCR`; OCR-versus-VLM-versus-text-versus-deterministic routing and typed-context-only model consumption are not composed or tested end to end. |
| WP04 AI Result To Policy Gate | 6 implementation and 4 test files validate local-AI output shape, confidence, provider kind, policy eligibility, and the screen enforcement handoff contract. | **Incomplete** | `service_policy_refs` creates a synthetic decision ID, fixed `allow` action, fixed reason, and fixed parent-rule ref whenever adapter output says `policyEligible`. No policy-control owner is invoked, stricter-parent-rule precedence is absent, and tests currently assert the fabricated metadata instead of proving a trusted AI-result-to-policy boundary. |
| WP05 Policy Action Dry-Run Gate | 5 implementation and 3 test files provide a Rust handoff guard, policy/action phase-state helpers, and event-bridge rejection tests. | **Incomplete** | These are contract and event-shape guards only. There is no production dry-run dispatcher for observe, allow, warn, ask-parent, time-limit, block, or manual-required; no timer/expiry or owned-process adapter handoff is driven from a real policy decision, and no action-matrix integration tests exist. |
| WP06 Journal Read Model And Portal Gate | 31 implementation and 8 test files cover typed screen phases, ActivityStore projection/query behavior, service event bridging, a service-backed portal summary panel, unavailable state, redaction, and UI screenshot test code. | **Incomplete** | The normal chain creates a new in-memory event bus per row; only deletion has a durable hash-chain journal. The portal reads ActivityStore summaries, but there is no durable replay of a real trigger/capture/AI/policy/action chain, and the displayed policy/action refs can originate from WP04's fabricated metadata. Durable full-chain replay and authority-negative tests are missing. |
| WP07 Deletion Retention And Custody Gate | 17 implementation and 5 test files cover encrypted durable queue records, leases, delete-after-analysis, TTL sweeping, durable deletion outbox/restart replay, quarantine, delete-failure projection, and raw-image exclusion from policy/portal event payloads. | **Incomplete** | The core local custody mechanics are substantial, but the explicit no-remote/cloud-upload boundary and retention-only-with-parent-opt-in contract are not enforced by a production capability or dedicated negative tests. The external adapter process boundary also lacks a test proving it cannot select a remote transport while claiming local-only custody. |
| WP08 Live Operator Proof Gate | The two workpack-owned executable test roots are mapped explicitly. Both are absent: `screen-ai-live-operator-proof.mjs` and `screen-ai-live-operator-artifact-gate.mjs`. | **Incomplete** | The plan describes a nine-scenario manifest harness and retained-artifact validator that do not exist in this checkout. This is missing expected harness/test code before it is a missing-proof problem. |
| WP09 Performance Cadence And Backpressure Gate | 13 implementation and 4 test files cover cadence/debounce decisions, queue-size limits, foreground/cadence single-tick capture, analysis `max_jobs`/`max_ticks`, polling, and adapter timeout controls. | **Incomplete** | Existing tests prove isolated decisions or one recorded capture, not the required three-capture cadence, full-queue/no-fourth-row behavior, disable-stops-future-jobs runtime behavior, or repeated-analysis no-flood behavior. The multi-tick/backpressure test harness is unwritten. |
| WP10 Final Rollout And PR Gate | The three named executable rollout test roots are mapped explicitly. `screen-ai-final-product-path-proof.mjs`, `screen-ai-service-winrt-ocr-proof.mjs`, and `screen-ai-household-mesh-proof.mjs` are absent. | **Incomplete** | The aggregate verifier/runner code is missing and every upstream production gap remains open. Final validation, proof, and PR promotion cannot honestly close this workpack until the executable gate exists and WP02-WP09 are complete. |

**Screen AI Pipeline Phase 1 result:** 10/10 workpacks inspected and mapped;
only the no-code prerequisite WP01 has no Phase 1 writing gap. Nine workpacks
still require production composition, authority/custody corrections, expected
integration tests, or executable harnesses. No Screen AI test pass, Enforcer
acceptance, proof, or whole-plan completion is claimed by this audit.

### Screen Plan Phase 1 code/test audit - 2026-08-15

This audit follows the current Rust capture, settings, encrypted queue,
analysis, journal/read-model, deletion, live-view, household-mesh, and portal
paths plus the Android Java capture adapter. It treats cited files that no
longer exist under `packages/*-domain` or `scripts/test/screen-*` as missing
code, not retained implementation, and it does not use the 100/100 legacy
checklist as runtime evidence.

| Workpack | Reviewed live code/test evidence | Phase 1 | Concrete code/test gap |
| --- | --- | --- | --- |
| WP01 Source Index And Doc Reconciliation | Coordination-only packet; no product code is expected. | **Incomplete** | The routed source index still describes removed TypeScript ownership and has not been reconciled to the Rust owners inspected here. |
| WP02 Current Screen Snapshot And Gap Map | Coordination-only packet; no product code is expected. | **Incomplete** | The snapshot still cites removed packages and proof scripts and does not reflect the production-wiring gaps found by this audit. |
| WP03 Contract Boundary And Effect Schemas | Five implementation and three test files provide Rust settings, queue/result, runtime-event, deletion, policy-ref, read-model, and generated parent-screen shapes. | **Incomplete** | Several states remain raw strings rather than closed schema enums; there is no single strict malformed-payload matrix covering settings, capability, queue, analysis result, deletion, and policy evidence together. |
| WP04 Parent Opt-In Settings Contract | Twelve implementation and five test files implement disabled defaults, typed get/replace commands, JSON persistence/reload, stale-version rejection, unsafe-retention rejection, generated bridge state, and portal controls. | **Complete for Phase 1** | Focused execution and acceptance proof remain Phase 2/3. |
| WP05 Capability Status Contract | Five implementation and three test files expose generic capture status, queue health, service screen rows, and capability cards. | **Incomplete** | No unified typed contract covers every required disabled, unsupported, permission, protected, locked, model, queue, degraded, ready, proof-tier, and scope-availability state; portal negative-state tests are incomplete. |
| WP06 Capture Scope Model | Three implementation and three test files model active window, selected window, and primary display. | **Incomplete** | Full-screen/display identity, selected app, managed-browser/window, Android app-window, and explicit unsupported scopes are absent, as are platform gates and complete portal labels. |
| WP07 Capture Trigger Model | Ten implementation and three test files define the required trigger vocabulary, cadence/debounce suppression, and cadence/foreground service loops. | **Incomplete** | Production foreground capture labels every change as native app; browser/game/launcher/unknown/network/policy/manual owners do not feed it, and capability-ready gating is not part of the scheduler decision. |
| WP08 Platform Adapter Abstraction | Seven implementation and two test files provide concrete desktop/Linux capture functions and Android MediaProjection entrypoints. | **Incomplete** | There is no shared adapter trait with adapter identity, capability probe, typed request/result, proof tier, and fake/dev non-proof boundary across Windows, macOS, Linux, Android, and iOS. |
| WP09 Windows Capture Adapter Plan Proof | Five implementation and four test files provide xcap display/window capture plus service cadence/foreground queueing. | **Incomplete** | The adapter lacks a Windows-specific permission/protection capability model and expected tests for protected surfaces, managed-browser scope, and capture-to-queue-to-delete behavior. |
| WP10 macOS Capture Adapter Plan Proof | Three shared xcap implementation files and one generic capture test compile the desktop path for macOS. | **Incomplete** | No macOS-specific capability/permission tests, live Screen Recording path, PPPC/MDM state, OCR/deletion integration, or macOS-host executable test exists. |
| WP11 Linux Capture Adapter Plan Proof | One X11 implementation and two tests cover X11 window-title safety and generic capture behavior. | **Incomplete** | Wayland/PipeWire portal, GNOME/KDE/wlroots state, unsupported-compositor handling, root-display behavior, and OCR/deletion integration tests are absent. |
| WP12 Android MediaProjection Adapter Plan Proof | Four Java production files implement consent activity, foreground capture service, proof metadata, and local proof storage. | **Incomplete** | There is no Android unit/instrumentation test code for consent denial, stop callback, no-silent-background behavior, deletion, physical-device capture, or OCR. |
| WP13 iOS ReplayKit Adapter Plan Proof | The iOS agent application shell exists. | **Incomplete** | No ReplayKit/broadcast-extension capture implementation or test target exists; `OcentraParentAgentTests` is absent. |
| WP14 Protected Surface Detector | Ten router/redaction implementation files and three tests can reject caller-supplied protected/credential flags and redact OCR tokens. | **Incomplete** | No production detector supplies lock-screen, secure-desktop, credential-prompt, password-field, DRM, or OS-protected classifications before capture; capture/portal tests for those states are absent. |
| WP15 Encrypted Temporary Image Queue | Nine implementation and two test files implement encrypted records, digest/source metadata, leases, TTL sweep, retry/outbox, quarantine, atomic mutation, and queue health. | **Complete for Phase 1** | Focused queue/security execution remains Phase 2; custody proof remains Phase 3. |
| WP16 Queue Scheduler And Debouncer | Eight implementation and three test files cover cadence, trigger debounce, queue scan limits, max pending, timeouts, and disabled suppression. | **Incomplete** | Strict-mode composition and end-to-end no-flood/backpressure tests are missing, including repeated-trigger and queue-capacity integration through the service. |
| WP17 Local OCR Vision Runtime Model | Thirty implementation and three test files provide a bounded external local adapter process, structured-output parsing, redaction, timeout/unavailable states, and local-AI contracts. | **Incomplete** | No production OCR or VLM worker is selected/composed; the service executes one configured external command and does not use the canonical screen router/provider scheduler to prove local-only OCR/VLM runtime behavior. |
| WP18 Screen Analysis Result Schema | Five implementation and two test files define the result/event record and serialize category, risk, OCR, confidence, refs, digest, deletion, policy, and explanation fields. | **Incomplete** | Category, risk, status, custody, and deletion values remain open strings and strict invalid-result conversion tests do not cover all required fields. |
| WP19 Sensitive Text And Redaction Model | Five implementation and five test files enforce OCR disablement, snippet limits, credential suppression, PII token redaction, persisted parent policy, and portal rendering without raw text. | **Complete for Phase 1** | Focused security/runtime execution remains Phase 2; real OCR quality and retained proof remain later scopes. |
| WP20 Result Validator And Invalid Output Handling | Two parser/record implementation files and three test files reject invalid JSON, incomplete results, unsupported provider kinds, and out-of-range confidence. | **Incomplete** | Missing refs, unsupported categories/signals, raw-text overflow, malformed deletion/custody state, and policy-driving rejection are not enforced as one fail-closed validator. |
| WP21 Journal And SQLite Ingest | Six implementation and four test files provide screen activity-store projection, runtime phase events, service bridge/subscription, and summary query tests. | **Incomplete** | The normal nine-phase chain uses ephemeral in-memory publication; only deletion has durable journaling. There is no durable full-chain replay/rebuild test proving summary-only SQLite state after restart. |
| WP22 Deletion And Retention Proof | Twelve implementation and five test files cover success/expiry deletion, delete-failed projection, durable outbox replay, quarantine, atomic acknowledgement, bounded reports, and portal custody state. | **Complete for Phase 1** | Focused Rust/portal execution is Phase 2 and the accepted deletion artifact pack is Phase 3. |
| WP23 Policy Compiler For Screen Derived Evidence | Three implementation and two test files expose a guard contract, a policy helper, and service policy-reference construction. | **Incomplete** | Production does not call policy-control authority; the service fabricates an allow decision and parent-rule reference from adapter `policyEligible`, with no real category/risk compiler, confidence/unknown behavior, or raw-input negative path. |
| WP24 Enforcement Handoff Guard | Three implementation and two test files enforce summary/rule/confidence/audit refs and reject raw pixels, raw model text, retained screenshots, and claimed local-AI authority. | **Complete for Phase 1** | This is intentionally a guard/dry-run boundary; adapter execution remains enforcement-plan scope. |
| WP25 Parent Portal Summary UI | Five implementation and four test files render service-backed settings, queue/summary, confidence, refs, deletion, model, policy, audit, and no-enforcement state, including Playwright route coverage. | **Complete for Phase 1** | Focused portal/service execution and screenshots remain Phase 2/3. |
| WP26 Child Disclosure UX | No production or test file exists at the expected child-disclosure UI boundary. | **Incomplete** | Child-visible enabled/paused/active/disabled state, calm copy, runtime delivery, and desktop/mobile UI tests are unwritten. |
| WP27 Screenshot Retention Optional Mode | Eight implementation and two test files provide disabled defaults, parent-approved short-TTL setting validation, persistence, generated UI examples, and portal controls. | **Incomplete** | There is no production raw-retention runtime/export/delete-on-disable composition or end-to-end custody test; settings acceptance alone does not enable the mode safely. |
| WP28 Live View Optional Mode | Seven implementation and three test files provide fail-closed runtime/startup decisions, service readiness mapping, unsafe-retention/control rejection, and portal capability state. | **Incomplete** | The worker `start` function records a decision only; no frame transport, LAN mutual-auth session, relay/cache execution, viewer audit, platform prompt, teardown, or physical-device test is implemented. |
| WP29 Proof Tiers And Proof Packs | This is a no-code proof-routing packet. | **Complete for Phase 1** | Artifact generation and acceptance are Phase 3 and do not substitute for open implementation workpacks. |
| WP30 Test Suite Playwright Rollout PR Gate | Existing Rust and portal tests cover bounded slices, but the two named aggregate executable harnesses are absent. | **Incomplete** | `screen-plan-closure-audit.mjs` and `screen-plan-external-gates-proof.mjs` are missing, along with the final cross-platform/model/live-view rollout acceptance runner. |
| WP31 Screen Intelligence Router | Nine implementation and two test files implement typed route input/output, structured-first selection, protected/manual-required states, capture choices, policy sensitivity, and generated bridge drift coverage. | **Complete for Phase 1** | Production integration belongs to later pipeline work; this bounded router contract has its expected code/tests. |
| WP32 Browser Structured Extraction Before Screenshot | Nine implementation and one test file model bounded structured evidence and `no_screen_needed` selection. | **Incomplete** | No managed-browser DOM/accessibility producer invokes the router, and there is no integration test proving a real browser source skips screenshot capture when structured evidence is sufficient. |
| WP33 Managed Browser CDP Screenshot Capture Path | Both files cited by the workpack are absent. | **Incomplete** | The CDP page/viewport/crop capture harness and its bounds/custody/desktop-exclusion tests must be rewritten against current Rust ownership. |
| WP34 OCR Tesseract Baseline | The named executable evaluation script is absent. | **Incomplete** | Packaging, extraction, resource/failure-mode measurement, and comparison test code must be restored or replaced; prose about historical output is not executable evidence. |
| WP35 OCR PaddleOCR PP-OCR Evaluation | The named executable evaluation script is absent. | **Incomplete** | Local packaging/runtime, quality comparison, resource capture, and no-upload harness code must be restored or replaced. |
| WP36 Small VLM Guided Classifier Evaluation | Twenty-five implementation and two test files provide generic screen pipeline/local-adapter contracts, but the named classifier-readiness harness is absent. | **Incomplete** | No selected VLM worker executes detector-specific bounded crops in production, and model capability/resource/quality/fallback evaluation code is missing. |
| WP37 Household Mesh Screen Analysis Queue | Eight implementation and one test file define redacted-summary custody, provider claim/lease/result validation, ordered events, rejection reasons, and child-owned policy authority. | **Incomplete** | No service startup/composition publishes or consumes this runtime, and there is no real household-provider transfer/restart test. |
| WP38 Local AI Resource Scheduler Priority Queue | Four implementation and two test files implement priorities, singleton heavy-job admission, queue state, timeout/degraded behavior, and provider scheduling. | **Complete for Phase 1** | Screen-analysis production does not yet consume this scheduler, which is a WP17/pipeline composition gap rather than missing scheduler code. |
| WP39 Redacted Summary Only Remote Boundary | Five implementation and three test files enforce redacted-summary mesh payloads, no raw-image/provider authority, and unsafe live-view retention/control rejection. | **Incomplete** | There is no parent-approved remote summary delivery runtime with audit/custody, and no privacy/legal gate or end-to-end raw-upload-denial test at the external transport boundary. |
| WP40 Detector Prompt Packs And Schema Tests | The named detector prompt-pack executable is absent. | **Incomplete** | Detector IDs/prompts, closed output schema, malformed/confidence/uncertainty tests, and privacy-negative tests must be rewritten under Rust-owned contracts. |
| Screen Control Settings Inventory | Imported historical reference packet; no executable code is owned. | **Reference only** | Reconcile its claims through WP01/WP02; do not schedule it as a product implementation packet. |
| Screen Evidence Analysis Capability Guide | Imported historical reference packet; no executable code is owned. | **Reference only** | Reconcile its claims through WP01/WP02; do not schedule it as a product implementation packet. |
| Screen Evidence Analysis Schema Proposal | Imported historical reference packet; no executable code is owned. | **Reference only** | Reconcile its claims through WP01/WP02; do not restore its former TypeScript ownership. |

**Screen Phase 1 result:** all 43 imported packets are mapped. Of the 40
executable workpacks, 9 are complete for bounded code/expected-test writing and
31 still need production code or expected tests; the remaining three packets
are reference-only. No Screen tests, Enforcer acceptance, proof, physical
platform readiness, live-view readiness, or whole-plan completion is claimed.

### V0.8 Enforcement Control Phase 1 code/test audit - 2026-08-15

This audit traces the actual Rust/TypeScript production surfaces and focused
test source. It does not accept the six historically checked workpacks as
complete merely because their Markdown boxes are checked. Graph topology
satisfaction below means the reviewed files exist; the Phase 1 decision also
checks whether the named behavior and expected negative-test families are
written. No test-pass, proof, CI, or whole-plan completion claim is made here.

| Workpack | Reviewed live code/test evidence | Phase 1 | Concrete code/test gap |
| --- | --- | --- | --- |
| WP01 Contract Boundary And Effect Schemas | Four implementation and six test files provide Rust-owned generated Effect Schema contracts, branded non-empty identifiers, parsers, Rust protocol parity, and invalid capability/result/timer/audit cases. | **Complete for Phase 1** | No missing bounded contract or expected negative-test family was found. The workpack's old `packages/enforcement-domain` paths are stale; ownership now lives in schema-domain plus Rust. |
| WP02 Policy Decision Evidence References | Six implementation and three test files bind policy decision, evidence, target, source, schedule, approval, audit, and timer references; validation rejects incoherent/missing action-capable inputs and preserves dry-run, observe-only, ask-parent, report-only, and manual-required outcomes. | **Complete for Phase 1** | No missing bounded decision/evidence contract or expected negative-test family was found. Focused execution is Phase 2. |
| WP03 Adapter Capability Matrix | Six implementation and six test files define supported, unavailable, degraded, report-only, scaffold, manual-required, adapter execution, rollback, proof-level, and claim-state rows, with service read models and no-upgrade assertions. | **Complete for Phase 1** | No missing bounded matrix code/test family was found. Static capability truth does not establish live adapter execution. |
| WP04 Owned-Process Time Limit | Ten implementation and six test files implement PID/process validation, owned-process termination, already-exited/no-op, timer expiry/recovery/cancel, result journaling, and focused proof generation. | **Incomplete** | The executing command still lacks the complete persisted authenticated delivery/consume-once authority and receipt/rollback chain required before adapter dispatch. Only process termination executes; broad app blocking remains manual-required. |
| WP05 App And Game Session Handoff | Four implementation and four test files carry typed app/game timer/session summaries, source-gated preview evidence, persisted timer binding, unknown-session rejection, and parent-facing timer status. | **Incomplete** | The stored session binding is not yet a complete production enforcement handoff from authoritative app/game identity through trusted dispatch to an adapter result and rollback receipt; launcher/game/app foreground-duration coverage is not closed end to end. |
| WP06 Managed Browser Session Control | Five implementation and four test files implement owned profiles, loopback bridge reservation/polling, unmanaged/stale rejection, launch/status delivery, redacted metadata, and failure/degraded states. | **Incomplete** | Managed-browser evidence is not consumed by the enforcement executor: `adapter_outcome` executes only process termination. No managed-session intervention receipt, rollback owner, or exact service-level browser-evidence-to-action audit test exists. |
| WP07 Unmanaged Browser Fallback | Five implementation and two test files keep managed/unmanaged states separate, require process identity, map report/warn/ask-parent/terminate/relaunch/manual/unavailable outcomes, and explicitly reject exact URL/content claims for unmanaged processes. | **Complete for Phase 1** | No missing bounded fallback code/test family was found. The two workpack-named legacy scripts are absent, but their required code/negative behavior is present in Rust; rerunning it is Phase 2 and proof is Phase 3. |
| WP08 Network/Domain Report-Only Boundary | Six implementation and six test files preserve process/domain/IP/tunnel classifications, keep network evidence separate from browser URL evidence, force network blocking to report-only/manual-required, and reject exact-URL or unsupported authority upgrades. | **Complete for Phase 1** | No missing bounded report-only/no-claim code or expected negative-test family was found. A real filtering adapter is deliberately not claimed and would require a separate authorized workpack. |
| WP09 Timer Recovery And Rollback | Five implementation and four test files cover durable timer state, restart recovery, corrupt/unsafe recovery-needed state, expiry, next-check reporting, cancel/override, rollback success, and rollback-unavailable behavior. | **Complete for Phase 1** | No missing bounded timer/recovery code or expected negative-test family was found. The legacy TypeScript/script paths are stale; focused Rust execution remains Phase 2. |
| WP10 Parent Approval And Override | Six implementation and four test files provide typed request approval/resolution, replay/target/authority validation, child-runtime application, parent snapshot projection, and timer cancel override. | **Incomplete** | Approval, denial, expiry, duplicate, and override transitions are split across policy/activity/timer paths rather than one durable enforcement lifecycle. General approval expiry/override issuance-expiry and their enforcement-journal assertions are missing. |
| WP11 Audit And Journal Events | Four implementation and four test files durably append rejected intent, before-dispatch, after-dispatch, adapter result, timer events, and typed recent history over the hash-chained Eventing journal. | **Incomplete** | Approval, denial, approval expiry, and general override transitions are not all produced through this journal, and no single focused matrix proves every adapter-result/timer/rollback family is durably ordered and queryable with all actor/target/policy/evidence/route refs. |
| WP12 Child-Facing Status And Reasons | Four implementation and three test files define typed enforcement results, unavailable/manual states, reason constants, and a child enforcement decision boundary. | **Incomplete** | No production child-facing status/read-model consumer renders time-limit reached, ask-parent, allowed, warning, unavailable, degraded, schedule/target context, and offline behavior. Existing tests prove decision safety, not the required child surface. |
| WP13 Service Read Models And API | Six implementation and three test files expose separate policy-dispatch, capability-proof, integrity, and audit-history reports through service/WebSocket paths. | **Incomplete** | There is no coherent service-backed enforcement state contract/API joining capability row, proof level, evidence/route, active timer, pending approval, and recent action history, nor a consumer-validation test for that aggregate payload. |
| WP14 Portal Control State Consumption | Four portal panels and four tests render adjacent app/game, browser, network, and policy states from typed service data. | **Incomplete** | No dedicated enforcement control-state route consumes the missing WP13 aggregate, sends typed enforcement intents, and renders returned result/audit state. Current panels do not close dry-run/observe/active/degraded/manual/approval behavior as one surface. |
| WP15 Integrity Heartbeat And Permission Loss | Three implementation and three test files define integrity/permission/stale/stopped states and expose a service proof/read model with parent-visible alert rows. | **Incomplete** | The rows are constructed as a static proof report. No live heartbeat source drives fresh-to-stale transitions, no permission-loss runtime updates the model, and no service test advances time through heartbeat/stale/recovery behavior. |
| WP16 Tamper/Uninstall Non-Claim Design | One Rust implementation plus three tests generate and regex-check a typed static tamper/uninstall artifact-status contract with explicit manual-required/no-claim wording. | **Incomplete** | No production uninstall/removal/stop/permission-denied observation feeds parent-visible state, and no runtime lifecycle test proves residual-state visibility or cleanup. Static generated text is not a live integrity boundary. |
| WP17 Cross-Platform Unavailable States | Two implementation and two test files provide honest Windows/Linux/macOS/Android/iOS capability counts, manual requirements, unavailable states, and no-claim serialization. | **Incomplete** | The required parent desktop, parent mobile, child Windows, child Android, and web-authoring/visibility roles are collapsed into a generic platform matrix; web and owner-surface separation plus their negative tests are missing. |
| WP18 Proof Command And Matrix | One production proof-report builder and two test/proof files cover the supported-adapter report and owned-process slice. | **Incomplete** | `scripts/test/v0-8-enforcement-control-plan-proof.mjs` is absent, as are the declared managed/unmanaged browser and timer aggregate runners. The expected deterministic whole-plan matrix/verifier test code has not been written. |
| WP19 Playwright And UI Proof | Three adjacent portal panels and three tests provide browser/network/app-game rendering and one network E2E path. | **Incomplete** | No Playwright spec drives the real enforcement service through dry-run, observe-only, active timer, ask-parent pending/approved/denied/expired, unavailable, degraded, result, and audit states without mocks at desktop/mobile widths. |
| WP20 Rollout Docs And CI/PR Gate | Expected topology is `no-code-required`; this packet coordinates final docs, focused/full validation, PR review, CI, merge, and no-claim reporting. | **Complete for Phase 1** | No product or expected-test code belongs to this packet. It remains operationally open until the 13 incomplete workpacks close and Phase 2/3 promotion evidence exists. |

**V0.8 Enforcement Phase 1 result:** all 20 workpacks now have reviewed
code/test ownership. Six executable workpacks (WP01-WP03, WP07-WP09) and the
coordination-only WP20 have no remaining Phase 1 writing gap; 13 workpacks need
production code or expected tests. The first dependency chain is WP11 durable
transition-family completion -> WP04 trusted dispatch authority -> WP05/WP06
action handoffs -> WP13 aggregate service state -> WP14/WP19 product surfaces.
Proof regeneration remains Phase 3 and cannot close those code gaps.

### Portal UX / Household Surfaces Phase 1 code/test audit - 2026-08-15

This audit follows the current portal, portal-domain, parent-runtime bridge,
mobile shell, package, workflow, and focused test source. It does not treat a
rendered proof panel, fixture, screenshot helper, or checked workpack row as a
finished product path. Graph topology satisfaction means the reviewed roots
exist; the Phase 1 decision also checks whether the workpack's named product
behavior and expected negative-test families are written. No Phase 2 test-pass,
Enforcer, Phase 3 proof, CI, or whole-plan completion claim is made here.

| Workpack | Reviewed live code/test evidence | Phase 1 | Concrete code/test gap |
| --- | --- | --- | --- |
| WP01 Service-Backed Shell And Navigation | Portal shell, HostBridge, Rust-owned route/status snapshots, service-state decoding, and focused bridge/state tests are real. | **Incomplete** | The shell has no focused unauthenticated and no-household tests and derives access from LAN controller authority; it can fall back to proof-missing instead of owning the required authenticated household boundary. |
| WP02 Household First-Run And Profiles | A Rust-backed setup status panel, route snapshot, portal renderer, unit tests, and a screenshot E2E exist. | **Incomplete** | The Rust builder explicitly reports account/provider/trust/custody as not wired. There is no live first-run state machine, profile creation/selection, or transition/action test family. |
| WP03 Device Inventory And Source States | LAN/service read models feed portal device inventory and evidence/source states; focused Rust, portal, fixture, and E2E tests cover real snapshots and degraded inputs. | **Complete for Phase 1** | No missing bounded inventory/source-state writing gap was found. Runtime execution is Phase 2. |
| WP04 Selected Device Context | Normalized device selection persists through session storage without inventing stable IDs; unit and E2E tests cover persistence and wrong-target behavior. | **Complete for Phase 1** | No missing bounded selection-context or expected negative-test family was found. |
| WP05 Policy Authoring Control Center | Policy preview projection, precedence rules, generated cards, parent snapshot loading, and focused Rust/portal tests exist. | **Incomplete** | The portal only refreshes previews. Template/manual-rule authoring, save/confirm/cancel UX, and a trusted one-shot write boundary with replay/tamper/role/cancel tests are missing. |
| WP06 Schedules, Time Budgets, And Templates | Route metadata and narrow app/game schedule/session rows exist in the portal domain and scaffold tests. | **Incomplete** | There is no real schedule/time-budget/template authoring surface, typed save path, state matrix, or focused responsive/negative test family. |
| WP07 Parent Requests And Approvals | Narrow policy-resolution and app/game timer panels/actions have typed Rust and portal tests. | **Incomplete** | There is no general request inbox/detail/history surface covering approve, deny, bonus time, expiry, duplicate, stale, and unauthorized outcomes as one service-backed lifecycle. |
| WP08 Activity, Evidence, And Diagnostics | The network evidence drawer and diagnostics export render references, confidence, custody, degraded states, and live read models with focused tests. | **Incomplete** | Export copies broad live read models without an explicit sanitizer/redaction contract or secret/private-path negative tests, and evidence/custody behavior is not closed across the plan's activity surfaces. |
| WP09 Browser, App, And Network Surfaces | Browser intervention/explanation, app-game session duration, and network evidence panels are backed by stored typed data; tests cover managed/unmanaged, exact-URL boundaries, metadata-only network state, and degraded inputs. | **Complete for Phase 1** | No missing bounded surface/negative-test writing gap was found. |
| WP10 LAN Pairing State Consumption | LAN replay binding/provenance, device/source/diagnostic read models, generated command contracts, Rust projection, portal unit tests, and E2E fixtures exist. | **Incomplete** | The portal mainly displays state; first-class add, route, rename, trust, ignore, and revoke controls do not consume the generated command boundary with result/error tests. |
| WP11 Assistant Action Preview Flow | Assistant chat models, quick actions, scaffold/screenshot coverage, and a narrow policy-preview confirmation overlay exist. | **Incomplete** | No production assistant route consumes service events/citations/provider state and drives typed preview confirmation with authorization, cancellation, replay, stale, and safety tests. |
| WP12 Reports, Notifications, And Custody | App/game notification, social report, tracking export/retention, and custody-oriented proof panels plus focused render tests exist. | **Incomplete** | The surfaces explicitly avoid product-ready export/mutation/delivery claims; no cohesive service-backed report, notification, retention, delivery, and failure lifecycle is written. |
| WP13 Degraded, Empty, Stale, And Error States | Service-state decoding and multiple setup/network/screen/app-game panels have explicit missing, unavailable, degraded, empty, refresh, and retry tests. | **Complete for Phase 1** | No missing bounded honest-state writing gap was found. |
| WP14 Audit History And Copy Debug | Diagnostics panels expose event metadata and copy success/failure UI; export code and a focused export test exist. | **Incomplete** | Copy success/failure/reset has no focused UI test, redaction lacks a secret/private-path matrix, and there is no general ordered audit timeline across the touched routes. |
| WP15 Accessibility, Responsive, Keyboard UX | Responsive styles and selected mobile, keyboard, and accessibility E2E helpers/specs are present. | **Incomplete** | Coverage is limited to a few proof routes; the plan-wide key-flow, focus, long-label, error-overlap, and representative desktop/mobile route matrix is unwritten. |
| WP16 No-Fake-Data Contract Adapter | Strict bridge decoders, generated contracts, host failure behavior, labelled fixture/proof panels, and invalid/missing-boundary tests prevent silent fake-green fallback. | **Complete for Phase 1** | No missing bounded adapter/no-fake-data writing gap was found. |
| WP17 Playwright Screenshot Proof | Browser failure collection and several desktop/mobile screenshot specs cover console/page errors and named routes. | **Complete for Phase 1** | The executable harness is written; retained screenshots and DONE links belong to Phase 3. |
| WP18 Parent Mobile Shell Readiness | Real Android and iOS parent projects, dedicated package scripts/workflow smoke, portal responsive styles, and mobile E2E coverage exist. | **Complete for Phase 1** | No missing bounded mobile-shell/package-smoke writing gap was found; store/signing/runtime-authority closure belongs to owning distribution workpacks. |
| WP19 Product Docs And Checklist Sync | Expected topology is `no-code-required`; this packet reconciles product docs and checklist truth after implementation. | **Complete for Phase 1** | No product/test code belongs here. The packet remains operationally open until the other Phase 1 gaps and later validation/proof state are reconciled. |
| WP20 Manual User Review Gate | Expected topology is `no-code-required`; this is the final user-observation gate. | **Complete for Phase 1** | No product/test code belongs here. It remains open until the product paths are complete and the user performs Phase 3 review. |

**Portal UX Phase 1 result:** all 20 workpacks now have reviewed code/test
ownership. Nine workpacks (WP03, WP04, WP09, WP13, WP16-WP20) have no remaining
Phase 1 writing gap in their bounded scope; 11 need production code or expected
tests. The first dependency chain is WP01/WP02 household authority and first-run
state -> WP05-WP07 authoring/request actions -> WP10/WP11 typed command
consumption -> WP08/WP12/WP14/WP15 cross-surface safety and UX closure. Proof
regeneration remains Phase 3 and cannot close those code gaps.

### Eventing plan Phase 1 code/test audit - 2026-08-15

This is a source-and-test inspection, not a transcription of workpack status.
`code expectation satisfied` means only that the reviewed expected topology is
present; the Phase 1 column below additionally checks whether the workpack's
named behavior and negative tests are actually represented. Tests are not
claimed passing here unless a current run is explicitly named.

| Workpack | Reviewed live code/test evidence | Phase 1 | Concrete code/test gap |
| --- | --- | --- | --- |
| WP01 Source Boundary And Semantics Audit | Expected topology is `no-code-required`; the workpack is a decision and boundary audit. | **Complete for Phase 1** | No implementation is authorized by this workpack. Graph acceptance/proof remains a later gate. |
| WP02 Crate Contract And Type Boundary | 4 implementation and 5 test files cover branded IDs, live/stored envelopes, registry conflicts, serde, and version skew. | **Incomplete** | The plan-named public surface still lacks `SubscriberName`, `HandlerName`, and a production `JournalPath`; no property/fuzz family was found. |
| WP03 Dispatch Runtime And Lifecycle | 5 implementation and 4 test files cover aggregate ordering, cross-aggregate concurrency, nested/detached publish, panic isolation, retry, timeout, trace, and lifecycle disposal. | **Incomplete** | No exact multi-handler registration-order test, true parallel-handler timing/aggregation test, or explicit no-lock-held-await regression was found. |
| WP04 Queue, Idempotency, Dead Letter | 12 implementation and 2 test files cover overflow, TTL, duplicates, rollback, drain preservation, and journal failure. | **Incomplete** | The named retry-storm/DoS guard test is absent. |
| WP05 Request/Response Contracts | 4 implementation and 3 test files cover typed resolution, invalid/late/double response, timeout, abort, release, and durable result separation. | **Incomplete** | No compile-fail/static negative proves event payloads cannot carry deferred, socket, task, or completion handles. |
| WP06 Journal Replay And Lineage | 27 implementation and 16 test files cover NDJSON/hash-chain durability, replay, idempotent recovery, topology, compatibility, and the enforcement audit handoff. | **Complete for Phase 1** | No code/test-writing gap found. Current focused rerun and Enforcer belong to Phase 2; retained acceptance is already graph-done. |
| WP07 Parent Protocol Event Contracts | 8 implementation and 6 test files cover selected parent, child, network, enforcement, and household event contracts. | **Incomplete** | Several chain payloads are typed serde structs but not reusable `DomainEvent`s; no comprehensive namespace registry, publisher-authority negative, or cross-family version-skew suite exists. |
| WP08 Parent Runtime Integration | 7 implementation and 5 test files cover selected tracking, policy, child-domain, and enforcement-journal flows. | **Incomplete** | No generic validated parent-controller intent route, portal-cannot-publish static negative, or one adapter-result-to-audit-to-read-model integration chain was found. |
| WP09 Network Consumer Event Chain | 3 implementation and 2 test files cover observation/runtime event contracts and serialized downstream references. | **Incomplete** | The full classification/AI/policy/enforcement/audit/portal chain is not publishable through reusable Eventing; weak-evidence and AI-cannot-enforce negatives are absent. |
| WP10 LAN Household Mesh Consumer | 2 implementation and 1 test file define transport/event types, fixtures, roundtrip, enum-wire, and a happy-path local-republish assertion. | **Incomplete** | There is no production import validator/republisher and no tests for unauthenticated/unauthorized, direct remote publish, replay/idempotency, stale, family/target mismatch, or provider policy/enforcement rejection. |
| WP11 Type Safety And Ownership Hardening | 4 implementation and 4 test files cover live/stored types, associated responses, Rust fixture parsing, and the naked-domain-string guard. | **Incomplete** | The shared fixture has a Rust consumer but no TypeScript consumer was found; explicit payload-mutation and no-lock-held-await audit tests are not retained as test code. |
| WP12 Rollout Proof And PR Gate | Expected topology is `tests-only`, but `scripts/test/eventing-rollout-proof.mjs` is missing. | **Incomplete** | All five named rollout reconciliation/negative checks lack their declared runner. Proof generation is Phase 3, but the expected test code itself is absent in Phase 1. |
| WP13 Test Folder Layout Regression Audit | Expected topology is `tests-only`; 38 external test files exist under `crates/ocentra-eventing/tests`, and no `src/` test module/entrypoint was found. | **Complete for Phase 1** | The focused Cargo/architecture rerun and fresh generated proof are Phase 2/3 work, not a missing-test-code gap. |

**Eventing Phase 1 result:** 13/13 workpacks inspected and mapped; 3/13
complete for code/test-writing scope, 10/13 still need code or expected tests.
The graph remains 1 planned, 11 validation, and 1 done because code topology
does not override checklist, proof, dependency, or acceptance state.

### Data Custody plan Phase 1 code/test audit - 2026-08-15

This table records current source and test code, not the workpack's checked
status or ignored proof roots. Tests are not claimed passing unless a current
run is named.

| Workpack | Reviewed live code/test evidence | Phase 1 | Concrete code/test gap |
| --- | --- | --- | --- |
| WP01 Custody Source Of Truth | Six Rust source/generator files, one generated TypeScript contract, and one Rust contract-test file implement the 28-class custody matrix and generated-edge drift check. | **Incomplete** | The claimed TypeScript boundary modules, TypeScript contract test, and proof runner do not exist on `main`. The two Rust tests cover serde and generated-file drift, but do not test the advertised class/hosting counts, unique row/class IDs, derived-source validity, redaction/notification rules, or forbidden-hosting invariants. |
| WP02 Encryption Key Custody | Five Rust implementation/contract files and two test files cover platform rows, household/device mismatch, revoked/lost keys, hosted portal, universal-key rejection, mobile proof, and recovery states. | **Incomplete** | The decrypt decision carries `requested_scope` but never checks it against the selected platform row's `decrypt_authority`; a child-service row can therefore authorize a parent-owned-bundle request when caller-supplied match flags are true. No cross-scope authority negative test exists. |
| WP03 Parent Owned Cloud Sync | Rust schema/generator and storage-custody derivation modules plus three Rust test files cover provider modes, ready/revoked refs, manifest integrity, conflict/retry state, tombstone separation, and no-default-Ocentra claims. | **Incomplete** | The claimed TypeScript contract/validation modules, TypeScript contract test, and manifest proof runner do not exist on `main`. Rust tests do not directly cover the declared wrong-account, folder-unavailable, partial-upload, provider-disconnect, and provider-delete visibility matrix. |
| WP04 Retention Delete Tombstone | Rust schema, state-machine, proof-builder, durable typed outbox, and three test files cover all nine required states, wrong-role/expiry negatives, ordering/duplicates, redaction-before-propagation, replay/restore blocking, minimal audit, hard delete, durable reopen/concurrency/migration, corrupt metadata, incoherent actions, and unknown acknowledgement. | **Complete for Phase 1** | No missing shared-boundary code/test-writing gap found. Concrete child-service startup composition remains a WP07 integration gap; provider/device propagation stays an adjacent-owner runtime boundary. |
| WP05 Export Import Backup Recovery | Rust schema, bundle builder, import preflight, restore/apply derivation, and two test files cover encrypted sections, integrity, redacted summaries, non-mutating/partial preview, wrong household/key/corrupt/expired/duplicate/migration negatives, confirmation, tombstone preservation, idempotency, and no support decrypt. | **Incomplete** | The in-scope backup cadence/manual-backup contract is absent. Supported migration is only a preflight label; no migration execution/rollback state machine or named migration-rollback test exists. Actual restore mutation remains outside this derivation-only surface. |
| WP06 Report Query Custody | Rust schema/generator, generated TypeScript contracts, storage-custody derivation/proof modules, and three Rust test files cover all seven required states, stable page ordering, duplicate cursors, disallowed source classes, tombstone-required deletion, conflict metadata, cursor expiry, rate limiting, and generated-contract drift. | **Incomplete** | Runtime derivation trusts request-owned authority and citation fields: it does not reject an unauthorized/non-parent request, raw-child-evidence request, wrong household/child ownership, or unauthorized citation reference. The workpack-named TypeScript adapter/rules modules and TypeScript negative contract test do not exist. |
| WP07 Rollout Proof And Route Gate | Storage-custody durable typed tombstone outbox plus child-runtime event-flow/recovery modules and three test files cover atomic persistence, reopen, concurrent writes, legacy migration, corruption, typed-delete-only enforcement, incoherent action rejection, unknown acknowledgement, journal failure/retry, idempotent restart replay, identity tampering, false-terminal rejection, and explicit terminal acknowledgement. | **Incomplete** | `ChildRuntimeTombstoneEventFlow::recover_pending` is used only inside child-runtime and its tests. No concrete child-service startup owner constructs or invokes it, and no service-owned restart test proves recovery before traffic. The child-runtime crate is library-only, so the existing seam is not live service integration. |
| WP08 Parent Storage Settings Apply Flow | Rust schema/generator, generated TypeScript contracts, storage-custody card/preview/apply/action/proof modules, and two Rust test files cover explicit storage modes, visible manual-required state, restore preview, wrong-household and partial-restore negatives, separate disconnect/delete actions, delete-kind coverage, and generated-contract drift. | **Incomplete** | The apply input has no confirmation receipt or confirmed flag. Every preview sets `confirmation_required = true`, so runtime derivation rejects `Applied` and `Partial` unconditionally and cannot model a completed confirmed apply. The claimed TypeScript adapter/rules modules, TypeScript contract test, and focused proof runner do not exist. |
| Migrated Data And AI UI Plan | The packet is a product/UI reference with proposed pre-contract read-model and intent names. Its explicit non-goals forbid UI implementation, route changes, provider/runtime work, model execution, and behavior changes in this slice. | **Complete for Phase 1** | No implementation or test code is required by this reference packet. Future production Data/AI UI work must be promoted into owning plan workpacks and typed contracts rather than attributed here. |

**Data Custody Phase 1 result:** all 9/9 workpacks now have reviewed code/test
ownership. WP04 and the source-only migrated UI reference are complete for the
code-and-expected-test-writing phase. WP01-WP03 and WP05-WP08 remain incomplete
with concrete gaps recorded above. No Phase 2 passing-test or Phase 3 proof
claim is inferred from this ownership audit.

### Policy Control Plane Phase 1 code/test audit - 2026-08-15

This table records reviewed implementation and expected test code. It does not
promote a workpack from plan/checklist state or infer current passing tests and
proof. The plan is being audited in dependency order and will use one plan PR
after all eight workpacks are classified.

| Workpack | Reviewed live code/test evidence | Phase 1 | Concrete code/test gap |
| --- | --- | --- | --- |
| WP01 Policy Source Of Truth | Rust protocol/source modules, the generated-edge TypeScript policy contracts, one unit test file, and two version-skew test files define the complete 14-state lifecycle vocabulary, household/actor authority, versioned rules and schedules, custody metadata, compiled/enforcement/audit/rollback artifacts, duplicate/stale rejection, delivery-before-active, supersede/rollback, migration boundaries, and source-not-cache/UI/AI negatives. | **Incomplete** | `register_parent_policy_source_document*` has no production caller outside tests and there is no durable/query owner that makes the document the canonical household source or rejects wrong-household reads. The required `PolicyTemplate` entity is absent. The TypeScript `FamilyPolicySet` edge also has no direct TypeScript contract test. |
| WP02 Parent Authoring Preview | Rust preview/request contracts, parent-runtime projection/actions, the generated portal bridge, the rendered portal preview panel, five Rust test files, two parent-runtime integration files, and one portal test cover preview-before-save, conflicts, unsupported/manual/offline/stale visibility, assistant preview-only confirmation, authority rejection, attention precedence, and the no-enforcement claim. | **Incomplete** | No template or manual-rule authoring surface exists. The portal panel is read-only except refresh: there is no confirm/save/cancel control or typed one-shot opaque confirmed-request relay from UI to Rust source mutation. Parent-runtime tests manufacture actions directly, so they do not prove a portal caller. Mobile/accessibility behavior and cancel-no-mutation are not tested. |
| WP03 Domain Policy Compilers | Rust compiler code, Rust-owned generated literal contracts, two compiler test files, and the helper drift test define all eight required domain outputs. Artifacts carry source/consumer versions, deterministic ID, delivery scope, schedules, capability/manual/unsupported state, custody, audit, rollback/supersede refs, and no-claim labels; tests cover the full domain matrix, deterministic output, version mismatch, source/cache rejection, explicit unsupported rows, and serialization. | **Complete for Phase 1** | No missing compiler-contract or expected-test code was found. Domain runtime consumption and effects remain deliberately outside this contract-only workpack; current focused test/Enforcer reruns are Phase 2. |
| WP04 Delivery Ack Audit | Rust delivery state-machine/receipt modules, child-policy and child-runtime handoffs, parent-notification projection, eleven policy delivery test files, and three cross-crate test files cover per-target state, ordering/idempotency, offline/retry/degraded states, audit/rollback/version linkage, redaction, schema hydration, request/artifact identity binding, and parent-visible manual-required fallback. | **Incomplete** | `PolicyDeliveryExecutionReceipt` has public constructible fields, and public `apply_policy_delivery_transition_with_execution_receipt` plus `apply_trusted_adapter_delivery_handoff` accept it without non-forgeable adapter authority or an inspectable execution trace. The only “trusted adapter” test fabricates the receipt in test code and advances `Applied`; no production caller owns that path. This is validated receipt evidence, not trusted execution authority. |
| WP05 Ask Parent Overrides | Rust request/approval/override state machines, child-policy/runtime handoffs, parent-notification projection, durable agent-service confirmation/resolution audit lookup, parent-runtime command actions, and seven focused test files cover parent confirmation, child/observer/revoked/wrong-household rejection, grant/deny/modify/request-expiry, double-submit/replay safety, audit refs, queued delivery binding, parent-visible degradation, and persisted resolution replay. | **Incomplete** | The portal exposes no approval/deny/modify/expire controls, so parent-runtime action tests construct payloads without a real UI caller. Agent-service resolution persists the decision but does not queue the child-runtime delivery path. Notification is a typed projection only, with no outbox/provider transport. Active overrides have no automatic expiry transition/restart test, and real apply/rollback still ends at WP04's untrusted receipt boundary. |
| WP06 Rollout Proof And Route Gate | This is legitimately a proof/routing workpack with no product implementation requirement. Its five named tracked proof artifacts and plan manifest exist; that proof content is intentionally not used to claim Phase 1 code completion. | **Incomplete** | The declared validation route is not executable as written: `packages/policy-domain` is absent, `packages/agent-protocol-domain` is an empty directory without a package, four named agent-protocol-domain tests are absent, and both portal test paths are stale (the live test is under `apps/portal/tests/policy/`). No dedicated rollout verifier checks accepted/missing roots and no-overclaim fields. |
| WP07 Schedule Time Budget Conflict Model | Rust source-time validation, schedule-contract validators, conflict detection, request/override models, generated TypeScript helpers, and five unit-test files cover reset/carryover shape, expiry ordering, explicit DST gap/overlap, clock-source/manual-required classification, deterministic priority conflicts, equal-priority manual review, request expiry, and bonus-time grant shape. | **Incomplete** | There is no runtime schedule evaluator computing window/DST/budget state from a trusted clock; validators accept caller-built boundary/status snapshots. No durable offline timer recovery exists, and `PolicyOverrideState::Expired` has no policy-request transition. Tests classify prepared DST boundaries but do not prove spring-forward cannot overgrant, fall-back cannot double-grant, clock-skew enforcement, restart recovery, or automatic bonus/override expiry. |
| WP08 Policy Event Model | Rust event registry/replay modules and two test files define all 23 required event families plus explicit dead-letter/manual-required events, typed scopes, stable aggregate/idempotency keys, Eventing `DomainEvent` contracts, duplicate/stale/conflicting-sequence handling, rollback scope, registry topology, version lock, and redacted summaries. | **Incomplete** | `PolicyEvent` contains no causation or correlation identifiers despite the workpack requirement. The event type and replay reducer have no production publisher, durable journal, consumer, or dead-letter projection usage outside tests; rollback linkage is typed but never resolves prior history. Tests exercise only constructed in-memory events and do not prove durable replay, parent-visible dead letters, or that serialized/logged paths cannot expose raw policy identifiers. |

**Policy Control Plane Phase 1 result:** all 8/8 workpacks are now inspected
and mapped from live source and test topology. WP03 is complete for the
code-and-expected-test-writing phase. WP01, WP02, and WP04-WP08 remain
incomplete with concrete gaps above. WP06 correctly maps as no-product-code,
but its stale/missing executable validation route prevents Phase 1 closure.
No Phase 2 passing-test or Phase 3 proof claim is inferred from this audit.

### Account Identity Family Phase 1 code/test audit - 2026-08-15

This table records reviewed implementation and expected test code. It does not
promote checked workpacks or historical proof into current completion. The
plan is being audited one workpack at a time before one whole-plan PR.

| Workpack | Reviewed live code/test evidence | Phase 1 | Concrete code/test gap |
| --- | --- | --- | --- |
| WP01 Auth Provider Decision | A narrow Cloudflare D1 adapter and environment ownership declaration store only provider-subject to Ocentra-account mappings. Two unit-test files cover persistence/update, lookup, uniqueness conflict, missing-binding manual-required behavior, malformed inputs, binding ownership, and configuration presence. | **Incomplete** | The adapter and schema initializer have no production caller outside tests. No external token-verification route, account/session runtime wiring, deployed D1 migration, or household/member/role/device authority implementation is connected to this boundary. The workpack itself records the provider decision as partial and forbids treating this storage helper as login or family-authority readiness. |
| WP02 Identity Household Role Model | Rust family records, authority/proof/record-derived handoff modules, three focused owner test files, and the provisioning consumer cover typed household/member/child/device records, role/action decisions, observer read-only behavior, cross-household/device denial, stale/revoked lease and device rejection, signed current-state authority proof, redacted identifier-only handoff, and setup readiness consumption. | **Incomplete** | Every TypeScript owner/test path named by the workpack is absent after the Rust migration. The general evaluator consumes caller-supplied `same_family`, membership, trust, freshness, and capability flags; both production consumers use that flag bundle, while the safer record-derived handoff has no production caller. `SupportAdmin` has no household action/audit-reference contract or dedicated minimized-boundary test, and Pending/Revoked/Disabled membership denial is not directly covered as a full state matrix. |
| WP03 Session Token Lifecycle | Rust lifecycle decisions and focused unit tests cover credential/action separation, session creation and rotation decisions, expiry and clock-skew states, logout/revocation/global-revocation rejection, replay rejection, freshness, scoped issuance, and audit-required/redacted result flags. Provisioning consumes the pairing-token authorization decision and has focused readiness tests. | **Incomplete** | The TypeScript owner package and tests named by the workpack are absent. The Rust API classifies caller-supplied lifecycle flags; it has no credential identifier/secret, issued/expiry time calculation, persistent session or refresh-family store, atomic rotation/replay registry, logout/global-revoke mutation, or real audit emission. Credential issuance has no production caller. Controller-lease and support/admin session classes are missing, and the only live CSRF/origin enforcement is on unrelated Cloudflare billing routes, not an account login/session surface. |
| WP04 Invites And Recovery Lifecycle | Rust invite/recovery decision modules and tests cover co-parent, observer, child-device, and household-transfer scopes; expired/revoked/replayed/non-single-use/wrong-household/wrong-role negatives; five recovery kinds; identity proof, throttling, uniform response timing, support-assisted child-evidence blocking, device-trust projection, and custody-handoff states. Provisioning consumes these decisions for pairing/readiness projections, while record contract tests cover invite, recovery, and setup-audit serialization. | **Incomplete** | The named TypeScript owner packages are absent. There is no durable invite/recovery store, runtime create/revoke/accept/consume flow, clock-derived expiry, atomic single-use transition, recovery transition engine, or audit persistence. The stored invite record omits purpose, inviter authority, device intent, lifecycle/replay state, and token custody. Rate limiting and enumeration resistance are caller-supplied flags. Provisioning fully validates only Pending invites; an Accepted invite bypasses replay, abuse, timing, and inviter checks. SupportAdmin can authorize any recovery kind, including household transfer, from caller flags without a persisted owner-approval/audit record. The custody handoff is only an enum consumed as a local blocker—no typed request/correlation reaches data custody. |
| WP05 Device Ownership Authorization | Rust household authority, signed current-state proof, record-derived handoff, focused authority/contract tests, the provisioning consumer, and the policy authenticated-delivery consumer cover household/account/member/device/trust/freshness checks, observer restrictions, child-agent denial, parent-only billing/export-delete decisions, remote-view/control action separation, controller-lease states, step-up context binding, wrong-household/device negatives, and proof anti-transplant checks. | **Incomplete** | The TypeScript owner package is absent, and four expected actions—create household, invite member, create child profile, and support/admin review—are not represented. The general production API still accepts caller-supplied ownership/trust/freshness state. `capability_granted` is one boolean, so it cannot distinguish a remote-view grant from remote-control authority. Controller lease is only a caller-supplied state with no lease identity, subject/action binding, or computed expiry. Authorization returns audit/elevated-confirmation flags but does not itself require the step-up assertion or emit an audit event. Only ChangePolicy has a downstream signed-proof consumer; remote, export/delete, and billing have no typed handoff/runtime consumer, and export plus delete are collapsed into one action. |
| WP06 Security Proof And Route Gate | This is legitimately an aggregation workpack with no product-code requirement. Its current route names the required security, negative-case, Cloudflare, UI, redaction, manual-gap, and adjacent-plan inputs. | **Incomplete** | The generated `output/` gate root is absent in this checkout; durable tracked proof exists only for the narrow WP01 storage adapter and WP08 authority packet. WP02-WP05 and WP07 have no clean-checkout retained aggregate artifacts. Every preceding workpack still has Phase-1 code/test gaps. The declared commands target absent `packages/family-domain`, a broad portal `account` filter, and an agent-protocol `account` filter with no dedicated aggregate verifier. The graph records no dependencies for this final gate despite its mandatory Account WP01-WP05/WP07/WP08 and Cloudflare WP06/WP08 inputs, so route ordering is not mechanically enforced. |
| WP07 Parent Account And Family Setup UI | A Rust-owned Start-route snapshot contract flows through the generated parent bridge into a portal route component. Rust integration, portal-domain projection, route-rendering unit, generated-bridge, and Playwright tests verify that the panel reaches the UI and honestly displays `unavailable`, `not wired`, route ownership, and withheld completion claims instead of fake readiness. | **Incomplete** | The implemented screen is deliberately a boundary-status placeholder, not the required setup state machine. The Rust snapshot contains no live account, household, member, child-profile, device, invite, recovery, support, security, or custody read model and exposes no setup actions. Tests explicitly assert that welcome and setup-complete states are absent. The portal-domain intent helper is not used by the actual route component, which renders the generated snapshot directly. Required create/join household, role visibility, device lifecycle, invite, recovery, support-access, account-security, manual-required, and full source/custody label states therefore have neither production UI behavior nor acceptance tests. |
| WP08 Rust Schema And Account Authority | Rust-owned family-reference templates/generators, generated TypeScript edge files, family-identity authority/lifecycle modules, two schema drift tests, two Rust contract suites, and three unit suites cover typed accounts/households/members/roles/devices/invites/recovery/sessions, cross-household rejection, observer restrictions, stale/revoked/replayed paths, controller leases, identifier-only handoff redaction, serde round trips, and checked-in edge drift. | **Incomplete** | The generated family-reference edge is copied from hand-authored TypeScript templates rather than projected from the Rust authority structs, and no test compares the TypeScript reference shape to the `family-identity-core` Rust records. The versioned household-authority handoff has no decoder/version-skew test that rejects an unsupported `schema_version` or malformed/unknown fields, so the required schema-incompatible fail-closed case is not written. Cloudflare persistence/runner wiring remains correctly outside this packet. |

**Account Identity Family Phase 1 result:** all 8/8 indexed workpacks are now
inspected and mapped from live source and expected test topology. None is
complete for the code-and-expected-test-writing phase: WP01-WP05, WP07, and
WP08 each have concrete production or test gaps, while the no-product-code
WP06 gate lacks an executable, dependency-enforced clean-checkout aggregation
route. No Phase 2 passing-test or Phase 3 proof claim is inferred from this
audit.

### Cloudflare Control Plane Phase 1 code/test audit - 2026-08-15

This table is based on the current Worker, binding, script, and test source. It
does not inherit the plan's historical dependency blockers or ignored proof
roots as present-day code truth, and it does not claim that any mapped test is
currently passing.

| Workpack | Reviewed live code/test evidence | Phase 1 | Concrete code/test gap |
| --- | --- | --- | --- |
| WP00 Games Infra Parity Extraction | This is a Parent-only keep/adapt/strip decision packet with an explicit no-code boundary; the parity and source-surface documents reject game economy, Solana, matchmaking, social, AI proxy, and asset-storage expansion. | **Complete for Phase 1** | No product implementation or test code is authorized by this workpack. Current docs validation and retained proof are later acceptance gates. |
| WP01 Cloudflare Module Scaffold | Three mapped implementation files and four focused tests sit inside a real separate `infra/cloudflare` package with explicit scripts, dev/prod Wrangler files, generated billing contracts, runner/seed scripts, all seven test-family directories, and honest README-only placeholder subdirectories. | **Complete for Phase 1** | No missing scaffold/package-script test code was found. Placeholder runtime subdirectories remain deliberately non-claims and belong to their functional workpacks. |
| WP02 Wrangler Env Bindings | `env.ts`, separate dev/prod Wrangler configs, `.dev.vars.example`, and the binding test define exact D1/DO/KV/Queue/R2/analytics names, ownership/privacy states, placeholder-only secrets, explicit production origin, optional bindings, and unknown-key rejection. | **Complete for Phase 1** | Real resource IDs, secret custody, and deployed environment verification are Phase 2/3 or manual environment work, not missing config/test code for this boundary. |
| WP03 Worker Entrypoint Runtime Guards | The Worker entrypoint, env/redaction helpers, three unit tests, three security tests, and Worker boot/health tests cover env failure, CORS, content-length framing, request-size limits, kill switch, redacted headers, and dispatch ordering. | **Incomplete** | `scheduled()` is empty and has no reconciliation test. The core Worker has no production structured logger, and both dispatch catch paths return raw `Error.message`; no injected secret/path exception test proves those messages are support-safe. |
| WP04 Route Manifest And Domain Contracts | `routes.ts`, auth-route validation, generated billing contracts, exact-manifest unit tests, fixed-manifest property checks, and the API contract suite define all 25 route rows with method, auth, handler, request/response model, audit event/rule, and proof family. | **Complete for Phase 1** | No missing route-manifest or expected contract-test code was found. Provider semantics, real auth authority, and consumer runtime remain separately owned. |
| WP05 Auth Admin Support Boundary | Auth model/verifier, Worker handlers, unit/integration/security tests cover all seven auth states, no downgrade, parent/trusted-device/admin/support/queue rejection, manual-required adapter modes, Stripe HMAC, role separation, and redacted failures. | **Incomplete** | `local-safe-fixture` accepts arbitrary bearer text plus caller-controlled role/trusted-device headers and production remains manual-required, so no real account/device authority adapter exists. PayPal, Apple, and Google verification use locally invented HMAC or bearer-equality fixtures rather than the providers' authenticated webhook protocols; tests currently enshrine those fixtures instead of a manual-required/real-provider boundary. |
| WP06 Storage DO D1 KV R2 Queue Bindings | Env/config ownership, billing D1/KV/R2 read-model code, the narrow account D1 store, Worker DO/queue paths, and five tests cover binding names, privacy constraints, seed/read behavior, idempotent writes, enqueue failure, and dead-letter capture. | **Incomplete** | Only `ACCOUNT_IDENTITY_D1` exists; the required account DO/KV, isolated `migrations_dir`, migration SQL, canonical Account-WP08 adapter, and real migration integration test are absent. `BillingControlDO`/`ReferralControlDO` keep idempotency in an in-memory `Map` rather than `DurableObjectState.storage`, so restart durability is not implemented or tested. |
| WP07 Local Dev Seeding And Fixtures | Eight scripts/source files and four tests define start/dependency probes, generated fixture families, redacted correlated proof milestones, teardown ownership, Worker boot, webhook fixtures, and replay fixtures. | **Incomplete** | The seed commands only serialize fixture objects to stdout; they do not populate local D1, KV, R2, Queue, or Durable Object state. The workflow can label the seed path runnable without verifying persisted state through a booted Worker, and it probes Wrangler availability rather than launching and smoking the declared local command itself. |
| WP08 Testing Runner And Test Pyramid | The module runner enumerates all seven test families and the complete 30-file test tree; it rejects missing required files and emits family, proof-ID, and assertion-ID manifests. | **Incomplete** | The runner blindly reports declared assertion IDs but does not bind them to executed test cases or verify matrix coverage. The required account-identity migration/adapter integration test is absent, so the storage-facing handoff cannot be represented by the integration family. |
| WP09 Portal To Worker E2E Smoke | The Worker harness and one four-case test exercise `/auth/billing/status` for unauthenticated, active, grace, and manual-review responses with portal-safe fields. | **Incomplete** | This is a direct in-process Worker test, not portal-to-worker E2E: no portal component, portal transport, HostBridge/dev transport, or portal test imports or consumes the billing-status route. The repository search found no billing-status consumer under portal source. |
| WP10 Security Fuzz Property Observability | Worker/auth/redaction source plus 13 security, property, fuzz, and carried integration files cover secret non-disclosure, CORS/CSRF/framing rejection, fixed route invariants, idempotency matrices, malformed webhook bodies, and redacted response boundaries. | **Incomplete** | The “property” tests iterate fixed examples and the “fuzz” test is a deterministic ten-payload smoke rather than generated property/fuzz coverage. The core Worker still lacks correlated structured observability, raw exception messages remain exposed, and the request-smuggling family does not exercise its declared header-injection/newline cases. |
| WP11 Deployment And Environment Promotion | Package deploy commands and separate dev/prod Wrangler configs exist with placeholder IDs and explicit origins. | **Incomplete** | There is no deployment/promotion implementation, CI environment gate, post-deploy smoke runner, version capture, rollback command, or automated rollback test. Both commands still pass `--env` names that have no matching `[env.*]` sections, while all resource IDs and auth authority remain placeholders. |
| WP12 Payment Plan Handoff Gate | This is an aggregation/handoff workpack with no product-code requirement. A tracked blocked-state receipt exists and the payment plan consumes the upstream route as blocked. | **Incomplete** | No executable validator checks the required handoff fields, current accepted/missing roots, downstream acknowledgment, and no-overclaim rules. The workpack still describes ignored `output/` roots as accepted while the tracked receipt says none are accepted, and payment-side wording carries a different missing-root set. |

**Cloudflare Control Plane Phase 1 result:** all 13/13 workpacks are now
mapped from live code and expected tests. WP00-WP02 and WP04 are complete for
the code-and-expected-test-writing phase. WP03 and WP05-WP12 retain concrete
runtime, authority, persistence, local-dev, runner, consumer, security,
deployment, or gate-verifier gaps. No current test-pass, deploy, proof, or
payment-unblock claim is inferred from this audit.

### Payment Subscription Phase 1 code/test audit - 2026-08-15

This table is based on the current Rust billing and entitlement crates,
Rust-owned/generated billing schemas, Cloudflare Worker implementation, and
portal source/tests. A generated contract, fixture, or passing historical proof
claim is not counted as a live provider, durable ledger, or parent product path.
No mapped test is claimed passing until Phase 2 reruns it.

| Workpack | Reviewed live code/test evidence | Phase 1 | Concrete code/test gap |
| --- | --- | --- | --- |
| WP00 Cloudflare Control Plane Handoff | This is an upstream dependency/handoff packet with an explicit no-product-code boundary. The Cloudflare plan and tracked handoff state can describe whether payment work is blocked without duplicating Worker implementation here. | **Complete for Phase 1** | No Payment-owned implementation or expected test code is authorized by this workpack. Its blocked/accepted handoff and retained proof remain dependency and Phase 3 concerns. |
| WP01 Product Pricing Entitlement | Seven mapped schema/generated-edge implementation files and two Rust contract tests define entitlement value shapes and validate proof-witness honesty. | **Incomplete** | The named pricing-matrix and entitlement-proof read-model modules and `packages/billing-domain` tests do not exist. No executable product/seat/referral pricing calculation, proration, or pricing-matrix negative suite was found; generated schemas and witness validation do not implement pricing authority. |
| WP02 Checkout Billing Portal | Eight mapped schema/Worker implementation files and seven contract/integration/property/security tests cover typed requests, auth states, hosted-URL allowlists, idempotency, audit rows, and secret-free responses. | **Incomplete** | The named schema wrapper and `packages/billing-domain` tests/proof runners are absent. Worker handlers synthesize Stripe-looking checkout/portal URLs locally instead of creating provider sessions, and account/device authority remains fixture/manual-required, so the tests do not prove real hosted checkout or billing-portal execution. |
| WP03 Subscription Webhook Lifecycle | Billing-core classifiers, projection/review modules, Cloudflare verifier/handler code, and five mapped tests cover provider enums, semantic decisions, duplicates, replay, out-of-order delivery, retry/dead-letter labels, lifecycle projection, and Worker rejection cases. | **Incomplete** | Billing core trusts a caller-supplied signature-state enum and has no cryptographic verifier. Its event constructors have no production caller outside the crate, no app-owned durable journal append, and no retry/reconciliation queue owner. Cloudflare tests do not close that Rust lifecycle-to-ledger path. |
| WP04 Entitlement Delivery Gates | Entitlement access/snapshot modules, Rust runtime-proof contracts, generated TypeScript edge code, and four mapped tests cover capability decisions, signed-snapshot shapes, limits, expiry/freshness states, and denial reasons. | **Incomplete** | Snapshot derivation copies caller-supplied signature/key text and trusts a caller-supplied verification enum; it does not sign, verify, parse trusted time, or persist snapshots. No production consumer was found outside the crate, and effective device-limit addition is unchecked for overflow. A signed-shape test is not trusted entitlement delivery. |
| WP05 Invoice Tax Refund Dispute | Nine mapped schema/billing/Worker implementation files and six tests cover generated invoice/refund/dispute shapes, fixture-backed read models, admin route responses, reconciliation boundaries, and selected lifecycle projection states. | **Incomplete** | The named billing-domain implementation/tests are absent. Current behavior is generated-contract plus synthetic fixture state, not provider-backed invoice/tax/refund/dispute authority or an app-owned durable ledger. Partial-refund failure, chargeback, cancellation, tax/legal region rules, and complete transition negatives are not implemented as one runtime model. |
| WP06 Security Privacy Observability | Billing review modules plus Worker auth/redaction code and ten mapped security/property/fuzz/unit tests cover secret non-disclosure, CORS/CSRF/framing rejection, idempotency examples, malformed payloads, and redacted boundaries. | **Incomplete** | The dedicated billing security/privacy/observability owner and its expected test are absent. The Worker has no production structured correlated logger, raw caught `Error.message` values can reach responses, and retry/dead-letter/provider-mode state is not durably observable. Fixed examples do not establish the required abuse/rate-limit/property/fuzz matrix. |
| WP08 Provider Adapter Portability | Billing provider enums/classifiers, Cloudflare verifier/dispatch code, and three mapped tests exercise Stripe plus fixture paths for Razorpay, PayPal, Google, Apple, and manual invoice. | **Incomplete** | Only Stripe has an HMAC-shaped verification path. Other providers use invented local HMAC or bearer-equality fixtures rather than official protocols, and there is no normalized adapter interface, server-owned provider selection/configuration policy, store verification, or missing-config fail-closed matrix. |
| WP09 Regional Payment Rollout | Worker fixtures/read model plus three integration tests expose public pricing, payment routes, and booted-worker behavior. | **Incomplete** | Runtime behavior is hard-coded to USD and treats Pakistan/manual invoice as a fixture string. There is no region/provider/currency/tax eligibility matrix, fallback policy, rollout gate, or negative test proving an unsupported or misconfigured region cannot charge. |
| WP10 Referral Growth Entitlement | Entitlement snapshot code, referral seed/fixture/Worker/read-model code, and four mapped tests cover referral-count input, invite responses, idempotency examples, and billing API shapes. | **Incomplete** | Referral abuse rejection is driven by sentinel substrings such as `same-household` and `same-device`. No durable qualification/grant/revoke lifecycle, active-paying-parent check, household/device authority, grace/recalculation engine, or history/audit test exists. Entitlement merely consumes a caller-supplied active-credit count. |
| WP11 Parent Website Billing Dashboard | Rust-owned/generated parent-visible summary contracts plus generic portal manage-route projection/rendering code and four mapped tests prove the route scaffold and summary shape exist. | **Incomplete** | The named parent-domain billing dashboard source/tests are absent. The portal exposes only a generic subscription route scaffold; there is no live billing transport/read model, plan/seat/referral/invoice state, checkout/portal action, manual-required handling, or billing-dashboard interaction/accessibility test. |
| WP12 Support Admin Billing Ops | Support/admin schema values, Worker route/auth/read-model implementation, and five mapped tests cover route shape, auth rejection, synthetic refunds/disputes/reconciliation, and redacted responses. | **Incomplete** | The named parent-domain operations source/tests are absent. Support/admin identity remains fixture/caller-header or manual-required, provider history and mutation authority are not backed by a real ledger/provider adapter, and no trusted portal/admin UI exercises these operations. |
| WP07 Rollout Proof And Route Gate | This is a final aggregation packet with no product-code ownership. Its expected role is to validate the preceding workpack outputs and routing without inventing implementation. | **Incomplete** | `output/payment-subscription-plan-proof/` is absent and the existing generic real-evidence script contains no payment assertions. No payment-specific executable verifier checks accepted/missing roots, assertion IDs, negative/rollback coverage, or no-overclaim behavior; that missing expected test code prevents Phase 1 closure even though proof generation itself belongs to Phase 3. |

**Payment Subscription Phase 1 result:** all 13/13 workpacks now have reviewed
code/test ownership. WP00 is complete for the code-and-expected-test-writing
phase because it legitimately owns only an upstream handoff. WP01-WP12,
including the final WP07 gate, retain concrete runtime or expected-test gaps.
The workpack index's `done` labels for WP01, WP03, and WP04 are contradicted by
the live implementation boundaries above and must not drive scheduling or
release claims. No Phase 2 passing-test or Phase 3 proof claim is inferred.

### Device Trust Bootstrap Phase 1 code/test audit - 2026-08-15

This table follows the live Rust/runtime/test paths, including code outside the
plan's older three-root summary. Plan-local Node tests that only read Markdown
are recorded as document tests, not runtime evidence. No mapped test is claimed
passing until the Phase 2 rerun.

| Workpack | Reviewed live code/test evidence | Phase 1 | Concrete code/test gap |
| --- | --- | --- | --- |
| WP01 Device Trust Source Of Truth | Twenty-eight mapped implementation files and twelve tests cover a typed durable lifecycle repository, platform-authority sidecar generations, pending/trusted/revoked/reset/re-pair transitions, redacted outbox events, explicit-path parent-presence custody, concurrency, restart/replay, schema/path integrity, opaque CSPRNG references, and fail-closed Eventing delivery. | **Incomplete** | The lifecycle and parent-presence APIs have no production caller outside their crates/tests. Production parent-presence custody intentionally returns unavailable on every platform, and no live product composition owns bootstrap-through-revoke/reset/re-pair state. The strong library boundary is not yet the product trust source of truth. |
| WP02 Local Key Sealing | Fourteen mapped implementation files and six tests cover a Windows DPAPI/current-user registry epoch, atomic sealed-record custody, wrong binding/revoked/generation negatives, an opaque one-shot staged-ceremony facade, and unsupported-platform behavior. | **Incomplete** | The parent-runtime facade is not registered in a desktop/native command path and no operational ceremony issuer exists. Android, Linux, iOS, and macOS custody plus encrypted recovery fallback are absent; the current slice is Windows-only and cannot close wrong-user/device/key/reinstall behavior across supported platforms. |
| WP03 Parent Step-Up Auth | Three mapped implementation files and four tests cover signed action/household/device/target-bound receipts, expiry, replay, trust-epoch changes, tampering, and schema round trips. | **Incomplete** | The only production verifier is the fail-closed unavailable verifier. No passkey/WebAuthn, biometric, or OS-native ceremony acquires the proof, no runtime caller consumes it as a live high-risk action boundary, and no durable one-shot replay owner exists. |
| WP04 Phone QR Approval Bridge | The sole mapped test asserts wording in the plan model. | **Incomplete** | There is no typed QR challenge/response contract, phone or desktop runtime bridge, one-shot/expiry store, audit append, or executable wrong-household/action/target/device/replay test. This is document coverage only. |
| WP05 Entitlement Device License | Eight mapped implementation files and six tests cover entitlement access decisions, signed-snapshot/runtime-proof shapes, limits, freshness/expiry/revocation labels, generated contracts, and selected capability denials. | **Incomplete** | Snapshot derivation copies caller-supplied signature/key text and trusts a caller-supplied verification enum; it neither signs nor verifies a device/household binding and has no production consumer. Revocation-over-stale-cache and copied-binary/config rejection are not proved through a trusted device runtime. |
| WP06 Recovery Reset Re-Pair | Sixteen mapped implementation files and five tests cover durable lifecycle reset/revoke/re-pair generations, copied-database authority failure, encrypted versioned bundle construction, wrong household/key/corruption/migration preflight, tombstone preservation, confirmation labels, and idempotent result derivation. | **Incomplete** | `apply_restore` only returns a result object; it does not mutate or roll back product state. The generic export/import bundle has no device-trust runtime caller and is not joined to lifecycle authority, so no encrypted restore followed by explicit audited re-pair exists. |
| WP07 Child Tamper Uninstall | The mapped child-enforcement status contract and three tests expose honest per-platform/manual-required artifact rows and assert the parent-authority/no-fake-anti-root boundary. | **Incomplete** | This is a proof/read-model contract that deliberately reports missing/manual evidence. No child-service tamper signal drives trust revocation, no parent-authorized uninstall command reaches a platform/package adapter, and no test proves revoked cached trust stops unlocking behavior. |
| WP08 Open Source Dependency Adoption | The tests-only packet and retained review matrix classify WebAuthn, passkey, keyring, encrypted-bundle, and RustDesk candidates while keeping every trust root explicit. | **Complete for Phase 1** | No implementation is owned by this research workpack. Any selected dependency still requires a separately authorized runtime adapter slice and Phase 2/3 validation. |
| WP09 Cross Plan Route Gate | Three mapped tests verify the test-category folders and selected plan/index wording. | **Incomplete** | No executable aggregator validates the required route-gate fields, accepted/missing proof roots, adjacent typed handoffs, blockers, manual-required gaps, or allowed/blocked claims. Document/taxonomy assertions cannot authorize readiness. |

**Device Trust Bootstrap Phase 1 result:** all 9/9 workpacks now have reviewed
code/test ownership. WP08 is complete for its bounded research/test-only scope;
WP01-WP07 and WP09 retain concrete runtime or expected-test gaps. The plan's
real Windows custody, lifecycle, step-up-proof, entitlement, recovery-contract,
and tamper-status libraries materially reduce the remaining work, but none is a
live end-to-end trusted-device product path. No Phase 2 passing-test, platform,
or Phase 3 proof claim is inferred.

### Setup Install Provisioning Phase 1 code/test audit - 2026-08-15

This audit separates setup-owned journey contracts from sibling-owned account,
package, runtime, LAN, trust, custody, policy, payment, and portal completion.
Historical workpack completion text is not treated as code evidence: several
named `setup-domain`, `production-domain`, and `parent-domain` source/test files
are absent from the current repository. No mapped test is claimed passing until
Phase 2.

| Workpack | Reviewed live code/test evidence | Phase 1 | Concrete code/test gap |
| --- | --- | --- | --- |
| WP01 Family Web Info Site | Expected topology is `no-code-required`; this workpack owns the public route/data-boundary decision and deployment blocker, not a site implementation. | **Complete for Phase 1** | No setup-owned product code or expected test code is authorized. A deployable family-site package, browser accessibility, preview, and custom-domain proof remain future implementation/adjacent-owner work, not evidence supplied by this packet. |
| WP02 Registration Login Entry | Expected topology is `no-code-required`; the workpack defines route labels and an account-identity handoff while explicitly excluding provider, session, household, invite, and recovery implementation. | **Complete for Phase 1** | No setup-owned implementation is authorized. The historical fill section names `setup-domain` and `production-domain` files that do not exist on current `main`; that stale proof wording must be reconciled in Phase 3 and cannot establish account readiness. |
| WP03 Parent Install Journey | Expected topology is `no-code-required`; the workpack owns visible install/platform/version/integrity labels and a runtime-distribution handoff, not package mechanics. | **Complete for Phase 1** | No setup-owned package code is authorized. The historical fill section names removed/nonexistent setup, production, and parent-domain files, so it cannot prove a live parent install journey; signed package/update/rollback execution remains with the distribution plan. |
| WP04 Child Install Permission Journey | Expected topology is `no-code-required`; the workpack owns the journey vocabulary and sibling-owner handoff, not child runtime, platform permission, package, trust, LAN, or policy code. | **Complete for Phase 1** | No setup-owned implementation is authorized. The old proof narrative references workspace packages no longer present; real installed/running/permissioned/paired/trusted/policy-ready inputs remain sibling-owned and must not be inferred from those claims. |
| WP05 Pairing Readiness Recovery | Nine mapped implementation files and five tests cover typed account/parent/child/permission/pairing/policy/custody/network/recovery states, readiness and action decisions, Eventing contracts, wrong-household/device/anonymous/replay/stale/revoked/offline/permission negatives, redacted audit events, and a child-runtime preflight consumer. | **Complete for Phase 1** | No missing setup-owned code or expected test family was found. The crate/runtime APIs have no concrete service composition caller, but physical LAN, trusted-device authority, and product service startup are explicitly sibling-owner/Phase 2 integration boundaries rather than missing WP05 state-model code. |
| WP07 First Run Setup UI And State Machine | Eight mapped implementation files and five tests cover a Rust-owned Start-route snapshot, generated bridge, portal-domain projection, rendered panel, null-state behavior, and one Playwright route proof. | **Incomplete** | The Rust snapshot hard-codes `not wired`/`unavailable` account, trust, and custody rows. It is a truthful boundary-status panel, not the required first-run state machine or screen map, and it has no live `provisioning-core` input, readiness-matrix-driven completion guard, guided actions, or executable blocked/degraded/manual transition suite. |
| WP06 Rollout Proof And Route Gate | Expected topology is `no-code-required`; this final packet aggregates proof and blockers without owning product implementation. | **Complete for Phase 1** | No product code is authorized. Its ignored/local proof roots, stale package references, route reconciliation, and sibling blocker acceptance remain Phase 3 work and cannot produce PR_READY from the current workpack labels alone. |

**Setup Install Provisioning Phase 1 result:** all 7/7 workpacks now have
reviewed code/test ownership. WP01-WP06 have no remaining Phase 1 writing gap
within their bounded ownership, including the real WP05 readiness model; WP07
still needs the live first-run state machine and expected transition tests.
Thus 6/7 are complete for code/expected-test writing, while the whole product
journey remains blocked by WP07 plus explicitly sibling-owned runtime proof.
No Phase 2 passing-test or Phase 3 proof/PR_READY claim is inferred.

### Parent Client Runtime Distribution Phase 1 code/test audit - 2026-08-15

This audit follows the current parent web, Tauri desktop, Android parent, iOS
parent, route-bridge, workflow, and release-helper source. The 20 historical
desktop-only workpack files that are not selected by `WORKPACK_INDEX.md` remain
legacy reference material and are not silently counted as additional current
workpacks. Enforcer-hosted proof scripts were inspected where package scripts
route to them, but only repository-owned files are recorded as graph roots. No
mapped test is claimed passing until the Phase 2 rerun.

| Workpack | Reviewed live code/test evidence | Phase 1 | Concrete code/test gap |
| --- | --- | --- | --- |
| WP01 Parent Client Scope And Route Boundary | Expected topology is `no-code-required`; this packet owns the canonical parent-client versus setup/child-runtime/portal ownership decision. | **Complete for Phase 1** | No product implementation is owned by this routing packet. Its boundary still has to be obeyed by WP02/WP03/WP06; their current code does not become correct merely because WP01 documents the split. |
| WP02 Parent Web Portal Distribution | Six implementation files and three tests provide pathname-based preview/staging/production rendering, wrong-route rejection, parent-only copy, and deterministic unit/Playwright coverage. | **Incomplete** | Authentication is considered successful unless the URL says `?auth=missing`; cache freshness is likewise a query parameter, and the enabled action button has no production action. There is no hosted deployment configuration or real session/cache input. The generated TypeScript claims a Rust source file that is absent. This is a distribution-status demo, not a real hosted authenticated parent portal boundary. |
| WP03 Parent Desktop Shell Package | Five implementation files and three tests provide a real Tauri shell, built-portal frontend binding, invoke/listen route commands, subscriptions, a TCP reachability probe, and Windows package/build anchors. | **Incomplete** | The platform state hard-codes active controller lease, local-network route/custody, and implemented child-agent/AI-provider roles even when the service is unavailable. Route loading calls `parent-runtime-core` in-process rather than decoding a typed local-service response, and the dedicated CI workflow type-checks/builds but does not package and exercise the Tauri artifact. Expected negative tests for unauthorized/version-mismatch/stale service responses are absent. |
| WP04 Parent Android Package | The parent-only Gradle project, launcher activity, versioned/checksummed APK builder, dedicated CI emulator install/launch/uninstall workflow, repository topology test, and Enforcer parent-package/source-boundary harness are written. | **Complete for Phase 1** | Release signing and Google Play publication remain manual/Phase 3 inputs. Phase 2 must rerun the build and emulator smoke; this row does not claim store readiness or child-agent authority. |
| WP05 Parent iOS Package | The parent-only Xcode project, simulator status app, versioned/checksummed package builder, dedicated macOS CI simulator install/launch workflow, repository topology test, and Enforcer source-boundary harness are written. | **Complete for Phase 1** | Device signing, provisioning profiles, TestFlight, App Store review, and a physical-device run remain manual/Phase 3 inputs. Phase 2 still must rerun the simulator build/smoke. |
| WP06 Parent Local-Service Route Bridge | Twelve implementation files and twelve tests cover the Rust route snapshot schema, route subscription delivery/deduplication, LAN read-model projection, and desktop command surface. | **Incomplete** | The desktop shell only probes whether a TCP socket accepts, then constructs package/authority fields locally. It does not perform a typed health/version/auth handshake or load route state from a service transport; `parent_load_route` directly calls the in-process parent runtime. Hard-coded controller, source, and custody state can therefore disagree with the reachable service. |
| WP07 Parent Client Signing Store Matrix | The desktop state and release-support helper expose manual-required signing/notarization/store labels, with general packaging assertions. | **Incomplete** | There is no parent-client per-artifact authority matrix or focused test. The helper incorrectly labels Android Play and iOS TestFlight rows as `child-mobile`, and it records no parent Android/iOS artifact hash, certificate, provisioning, notarization, store-review, or signing-authority state. |
| WP08 Parent Client Update Rollback | The desktop status shape and release-support helper distinguish scaffold, unavailable, and manual-required labels; selected assertions preserve those labels. | **Incomplete** | There is no parent-client updater: no manifest fetch, signature/checksum verification, apply state machine, durable update journal, rollback executor, failure recovery, or tampered/replayed manifest tests. Static unavailable/manual labels are honest but are not update/rollback implementation. |
| WP09 Parent Client Launch Smoke Matrix | Web Playwright launch coverage, desktop launch/build anchors, Android emulator smoke, iOS simulator smoke, repository packaging assertions, and the Enforcer four-row matrix harness are written. | **Complete for Phase 1** | The matrix harness is Enforcer-owned rather than a repository code root, and actual smoke outcomes remain Phase 2/manual evidence. Launch smoke does not establish auth, setup, signing/store, update/rollback, or child-runtime readiness. |
| WP10 Setup Handoff Contracts | Six nearby implementation files and three tests expose the Start-route setup panel through the Rust bridge and portal projection. | **Incomplete** | This is the setup status panel already audited under Setup WP07, not an explicit setup-to-parent-client-distribution request/response contract. There is no package target/version/integrity/install-precondition handoff, expiry/replay protection, consumer acknowledgement, compatibility contract, or focused boundary test. |
| WP11 Proof CI Release Gate | CI detects parent desktop/mobile changes and has dedicated desktop build, Android emulator, and iOS simulator jobs; generic release/packaging assertions and a release-support helper exist. | **Incomplete** | There is no executable parent-client aggregate gate that validates WP01-WP10 required fields and accepted/missing roots. The production release workflow builds the child Windows agent MSI, not signed parent desktop/mobile artifacts, and no parent-client promotion gate enforces signing/store, setup handoff, negative, teardown, rollback, or allowed/blocked claim truth. |

**Parent Client Runtime Distribution Phase 1 result:** all 11/11 current
workpacks now have reviewed code/test ownership. WP01, WP04, WP05, and WP09 are
complete for code/expected-test writing within their bounded routing/package/
smoke scope. WP02, WP03, WP06, WP07, WP08, WP10, and WP11 retain real runtime
or expected-test gaps. The plan's seven document-claimed closures therefore do
not establish product or release completion. No Phase 2 passing-test, signing,
store, or Phase 3 proof/PR_READY claim is inferred.

### Child Agent Runtime Distribution Phase 1 code/test audit - 2026-08-15

This audit follows the actual Windows MSI/service harness, macOS launchd
package, Linux systemd package, Android child application, iOS capability
application, Rust-owned shared contracts, smoke scripts, and CI/release
workflows. Enforcer-hosted legacy proof runners were inspected where repository
commands route to them, but external Enforcer files are not counted as
repository code roots. No mapped test is claimed passing until Phase 2.

| Workpack | Reviewed live code/test evidence | Phase 1 | Concrete code/test gap |
| --- | --- | --- | --- |
| WP01 Child Agent Scope And Route Boundary | Expected topology is `no-code-required`; this packet owns the child-versus-parent/setup/trust/runtime distribution boundary. | **Complete for Phase 1** | No product implementation is authorized by the routing packet. The remaining workpacks must still obey its package/install/runtime/respawn/uninstall separation. |
| WP02 Child Windows Service Package | The WiX MSI, WinSW/service configuration, signed-update-manifest builder, installer, artifact verifier, elevated lifecycle harness, CI MSI install/uninstall smoke, and repository asset test are written. The lifecycle harness distinguishes install, start, stop, restart, service-manager recovery state, uninstall, and residual authority cleanup. | **Complete for Phase 1** | The existing CI smoke exercises install/start/uninstall only; the fuller harness still needs an elevated Phase 2 run. Crash-loop and reboot recovery remain explicitly unexercised/manual rather than missing core harness code. |
| WP03 Child macOS Service Package | The launchd plist, `pkgbuild` script, pre/post-install launchctl hooks, checksum output, payload smoke, thin package-proof contract/test, and macOS CI package job are written. | **Incomplete** | The smoke only expands and inspects the package payload. No real-host install/bootstrap/health/kill-recovery/disable/uninstall/cleanup harness or corresponding negative tests exist; signing and notarization automation are also absent. |
| WP04 Child Linux Service Package | The baseline-pinned `.deb` builder, systemd unit, package hooks, checksum/baseline test, contract test, CI job, and smoke code cover extraction health plus `dpkg` install/remove/purge. | **Incomplete** | Maintainer scripts ignore `systemctl` failures, and the smoke never asserts the installed service becomes active, survives a crash through `Restart=always`, or cleans runtime/data state after service-managed execution. Those expected lifecycle tests are not written. |
| WP05 Child Android Agent Package | The Gradle child app, declared foreground service, debug-APK/checksum builder, lifecycle/device-proof contracts, CI contract jobs, and emulator install/launch/process/uninstall smoke are written for the explicit debug-sideload mode. | **Complete for Phase 1** | Device-owner, managed-profile, Play Store, reboot recovery, and physical-device authority remain intentionally manual/Phase 2-3 boundaries; this row does not infer them from emulator launch. |
| WP06 Child iOS Capability Package | Rust owns the capability contract and generated TypeScript, with Rust contract tests, a current repository proof harness, the Xcode capability-only app, simulator ZIP builder, simulator install/launch/uninstall smoke, and dedicated CI jobs. | **Complete for Phase 1** | Physical-device provisioning/signing, supervision, store distribution, and background execution remain explicit manual or unsupported boundaries rather than hidden daemon claims. |
| WP07 Child Managed Service Respawn | Windows, macOS, and Linux service-manager declarations plus Android/iOS limitation surfaces are present, with nearby platform contract tests. | **Incomplete** | No repository-owned managed-respawn contract/test exists. The Enforcer legacy runner still imports deleted `schema-domain` source/test files and proves configuration strings rather than executing kill, reboot, service-manager restart, and deliberate-stop behavior. Real host/emulator respawn test code is missing. |
| WP08 Child Parent Authorized Uninstall | `child-enforcement-core` retains a Rust-owned generator for the uninstall/tamper status read model and one contract test; platform package smoke paths provide nearby removal mechanics. | **Incomplete** | No production parent authorization, one-shot/replay guard, trust revocation mutation, audit append, platform adapter dispatch, or teardown workflow consumes this contract. The legacy Enforcer runner targets deleted TypeScript schema/consumer tests, so it cannot establish current expected-test coverage. |
| WP09 Child Signing Store Device Owner Matrix | Rust owns the five-platform matrix and generated TypeScript with a focused Rust contract test; release builders expose the actual unsigned/debug/simulator artifact states. | **Complete for Phase 1** | The legacy Enforcer proof command still expects removed TypeScript adapter/test files and must be rerouted during Phase 2. Actual signing, notarization, repository/store publication, and mobile enrollment artifacts remain later platform/release work, not missing matrix code. |
| WP10 Setup Device Trust Handoff | Eight Rust contract modules and one contract test define branded request/response identities, platform/artifact state, manual-required state, replay-guard reference, route sync, and no-claim boundaries. | **Incomplete** | Repository-wide usage is limited to schema tests. No setup/trust producer constructs the request, no distribution consumer validates/acknowledges it, no durable replay/expiry owner exists, and no end-to-end wrong-household/device/package/replay test is written. |
| WP11 Proof CI Release Gate | CI detects and builds all five child package targets; package-preview runs Windows/Linux/macOS/Android/iOS smoke code, while repository release/version/packaging tests cover the workflow surface. | **Incomplete** | The production release workflow publishes only the Windows child MSI. There is no executable child-plan aggregate gate over WP01-WP10, no signed/notarized/store promotion path for the other platforms, and no release decision that rejects missing lifecycle, respawn, uninstall, handoff, or platform-authority evidence. |

**Child Agent Runtime Distribution Phase 1 result:** all 11/11 current
workpacks now have reviewed code/test ownership. WP01, WP02, WP05, WP06, and
WP09 are complete for code/expected-test writing within their bounded route,
Windows-harness, debug-emulator, capability-only, and matrix scopes. WP03,
WP04, WP07, WP08, WP10, and WP11 retain concrete lifecycle-test, runtime,
handoff, or release-gate gaps. The plan's ten document-claimed closures
therefore do not establish child runtime or release completion. No Phase 2
passing-test or Enforcer claim and no Phase 3 proof/PR_READY claim is inferred.

### Logging Domain Parity Phase 1 code/test audit - 2026-08-15

This audit follows the live TypeScript logging package, Rust logging core,
portal and agent-service routing, deterministic evidence wrappers, MCP query
server, lifecycle controls, and proof-trace test code. Historical proof roots
and checklist marks were not used to decide whether implementation or expected
tests exist. No mapped test is claimed passing until the Phase 2 rerun.

| Workpack | Reviewed live code/test evidence | Phase 1 | Concrete code/test gap |
| --- | --- | --- | --- |
| WP01 Current State And Reference Audit | Expected topology is `no-code-required`; the packet owns the reference/current-state inventory and routes implementation to the remaining workpacks. | **Complete for Phase 1** | No product implementation or test code is authorized by this audit-only packet. Its missing retained proof is Phase 3 work. |
| WP02 TypeScript Logging Package Parity | Thirty-eight implementation files and twelve tests cover explicit parent scopes, core logger/config, bridge transport/server, NDJSON writers, DuckDB ingest/query, app logs, wipe/retention helpers, package exports, scripts, invalid payloads, fixture parity, and database/query behavior. | **Complete for Phase 1** | No missing package-parity code or expected test family was found. Focused execution remains Phase 2; the absent proof root remains Phase 3. |
| WP03 Parent Logging Architecture And Routing | Seven implementation and four test files implement the portal-domain bridge-first logger with compatibility fallback, thin portal wrapper, live bridge receiver, logging-core-backed agent-service writer, structured startup diagnostics, and portal/Rust route tests. | **Complete for Phase 1** | The workpack prose is stale: the Rust-side mapping it calls open is present and test-mounted. A stale compatibility comment in `dev_log.rs` is documentation debt, not a missing runtime path. |
| WP04 Rust Logging Core Crate | Fifty-two Rust/fixture implementation files and seventeen tests cover typed events/levels/sources/fields, NDJSON durability and recovery, artifacts, redaction, path safety, diagnostics/run records, snapshots, concurrency, subprocess recovery, TypeScript fixture parity, and the agent-service consumer. | **Complete for Phase 1** | No missing core implementation or expected test family was found. Cargo, clippy, consumer, and fixture reruns remain Phase 2. |
| WP05 Local Validation Evidence | Eight implementation files and two tests provide `agent:run`, `agent:query`, `codex:evidence`, artifact/NDJSON/DuckDB storage, compact summaries, and parsers for rustc, clippy, Cargo tests, TypeScript, ESLint, npm, architecture, and no-reexport diagnostics. | **Incomplete** | The parser families have no direct fixture-driven unit matrix. Current tests prove run/command identity and a generic controlled failure, but do not exercise each required diagnostic grammar, duplicate aggregation, or malformed-line fallback. |
| WP06 Validation And Enforcement | Four validator scripts and five nearby tests/smokes check package layout, exports, wrapper guidance, portal/service routing, local evidence, and invalid bridge payload handling. The current route checker recognizes the implemented portal-domain bridge-first path and logging-core delegation. | **Incomplete** | No fixture-based negative test invokes the validators against a missing bridge, an unimplemented endpoint, or missing exports/wrappers. Invalid payload rejection is tested, but the required validator failure matrix itself is unwritten. |
| WP07 MCP Query Interface | Four implementation files and one integration suite expose all thirteen current tools, including errors, recent/source/context/query/stats, latest failures, run diagnostics, bounded artifact slices, proof inventory, and proof traces through a shared query service. | **Incomplete** | Integration tests exercise listing, latest failures, proof trace, artifact path/limit safety, and proof inventory, but do not call and assert the general errors/recent/source/context/query/stats/run-diagnostics tool families or the NDJSON fallback when DuckDB is absent/stale. |
| WP08 Logger Instrumentation And Adoption | Twelve implementation and six test files cover registered TypeScript source/context/file metadata, the portal shared logger path, Rust logging-core delegation, structured agent-service startup fields, evidence run/command identities, storage/query preservation, and selected CLI/MCP visibility. | **Incomplete** | No checker or negative test prevents new raw console logging or ad hoc JSON log writers on touched production surfaces, and health/runtime diagnostic adoption remains narrower than the workpack's startup/health/dev target. The bounded instrumentation is real, but the expected enforcement code/tests are missing. |
| WP09 Log Control Retention And Bridge Lifecycle | Ten implementation and seven tests cover separate console/storage decisions, always-stored warning/error levels, source/file/run debug selection, local/tunnel/disabled modes, scoped wipe, configurable retention, bridge health, run-start metadata, stale-run rejection, invalid payload rejection, and script behavior. | **Complete for Phase 1** | No missing lifecycle/control implementation or expected test family was found. Focused test execution and retained proof remain Phase 2/3. |
| WP10 Proof Trace Pipeline | Six implementation and five test files provide proof/correlation fields, bridge run-start and stale wipe, ordered portal trace emission, flush, DuckDB ingest, CLI/MCP queries, missing-step reporting, and cleanup of proof-mode globals. | **Incomplete** | The query service computes `outOfOrderSteps`, but no test creates an out-of-order trace and asserts that failure. The happy path only proves the empty result, leaving one explicit negative behavior untested. |

**Logging Domain Parity Phase 1 result:** all 10/10 workpacks now have
reviewed code/test ownership. WP01-WP04 and WP09 are complete for the
code-and-expected-test-writing phase. WP05-WP08 and WP10 retain concrete
expected-test or instrumentation-enforcement gaps. The plan's source-present
and partial-proof labels therefore do not establish Phase 1 completion, and no
Phase 2 passing-test/Enforcer or Phase 3 proof/PR_READY claim is inferred.

### Remote Access Phase 1 code/test audit - 2026-08-15

This audit follows the live Rust capability, grant, session, child-runtime,
screen-live-view, and agent-service paths. It also searches for production
callers and relay/security behavior, so a type name, environment flag, or
boolean execution record is not counted as a working relay. Historical proof
roots and checklist marks were not used to decide whether implementation or
expected test code exists. No mapped test is claimed passing until Phase 2.

| Workpack | Reviewed live code/test evidence | Phase 1 | Concrete code/test gap |
| --- | --- | --- | --- |
| WP01 Remote Capability Fabric | Three implementation files and two contract test files define a Rust-owned view-only capability grant with explicit household, parent/support actor, child device, route, pairing, grant, session, device-trust, audit, and diagnostic-redaction fields. Tests cover serialization/versioning plus wrong household, actor, device, route, pairing, trust, revoked/removed, missing audit, raw diagnostics, and deferred-control rejection. | **Complete for Phase 1** | No missing contract behavior or named negative-test family was found. Runtime pairing, relay, custody, and rendered disclosure belong to later workpacks; focused execution remains Phase 2. |
| WP02 Live Screen Relay | Nine implementation and seven test files provide typed remote session authorization/effect decisions, child-runtime gating, live-view readiness/worker decisions, agent-service startup wiring, relay/LAN mode selection, bounded duration, replay rejection, no-raw-cache/no-recording/no-input guards, and prerequisite tests. | **Incomplete** | `start_screen_live_view_worker` only returns a `worker_started` record; no capture-to-transport/relay worker, protected-surface handling, degraded/reconnect state machine, child disclosure delivery, parent rendered state, or executable deletion/custody boundary exists. Relay readiness is supplied by environment proof booleans, not verified runtime behavior. |
| WP03 Remote Input Control Authority | The workpack is explicitly deferred and expects no current-pass code. One implementation file and two test files nevertheless expose `InputAllowed` and produce `RemoteAccessInputBridgeState::Start` when the generic session gates pass; a test asserts that start path. | **Incomplete** | Current code contradicts the live-view-only no-code/no-claim boundary. It does not implement a real input bridge, but it models control as startable without the workpack's fresh confirmation, scoped input, blocked-surface, stop/escape, policy, platform-permission, replay-input, or privilege-escalation authority. This legacy surface must be removed or fail-closed until WP03 is explicitly opened. |
| WP04 Session Pairing Grants | Twenty-four implementation and seven test files implement a detailed in-memory/serializable live-view grant lifecycle with parent confirmation, disclosure, paired/active/paused/stopped/reconnect/terminal states, current-authority and device-trust checks, support visibility, revocation/removal precedence, supersession, bounded replay identity, restart-recovery evidence, durable event-shaped audit milestones, and extensive negative/round-trip tests. | **Incomplete** | Production search finds grant construction and transitions only in this crate's tests. There is no persistence adapter/store, composition-root loading, live session/relay consumer, device-trust authority port, child/portal disclosure delivery, or durable audit-journal owner. JSON round trips and replay history are contract code, not a persistence-backed runtime. |
| WP05 Relay Security Abuse Controls | Three implementation and four test files contain relay available/unavailable, request replay, relay-mode/cache prerequisite, transport selection, and unsafe-retention/control blocking states. | **Incomplete** | No authenticated capability-scoped relay token, expiry/replay store, rate limiting, backpressure, per-household/device connection limits, cross-household isolation, origin/host/redirect defense, stale-grant cache control, partial-outage/slow-dependency/reconnect-storm handling, redacted diagnostics, metrics, or abuse alert implementation/test matrix exists. Current relay claims are prerequisite booleans only. |
| WP06 Rollout Proof And Route Gate | Expected topology is `no-code-required`; this packet aggregates the five current-pass workpacks and explicitly keeps WP03 deferred. | **Complete for Phase 1** | No product implementation is authorized by this proof-only packet. Its accepted/missing proof roots, route synchronization, manual gaps, and no-overclaim result remain Phase 3 and cannot close while WP02, WP04, and WP05 are incomplete. |

**Remote Access Phase 1 result:** all 6/6 workpacks now have reviewed
code/test ownership. WP01 and the proof-only WP06 have no Phase 1 writing gap.
WP02-WP05 remain incomplete: the repository has strong capability and grant
contracts, but no real relay/live-view data path, the deferred input surface is
incorrectly startable in legacy effect-plan code, pairing grants are not owned
by a persistence-backed production runtime, and relay abuse controls are
largely absent. No Phase 2 passing-test/Enforcer or Phase 3 proof/PR_READY claim
is inferred.

### Tracking Phase 1 code/test audit - 2026-08-15

The code-first audit maps all 42 imported Tracking packets to precise current
Rust, service, policy, child-runtime, notification, AI, and portal roots. The
full row-by-row result is in
`docs/plans/tracking-plan/CODE_AUDIT.md`; stale references to the absent
`packages/tracking-domain` package and absent `scripts/test/tracking-*.mjs`
verifiers were removed from the plan routing documents.

**Tracking Phase 1 result:** 24 bounded packets have their core production code
and expected test code written; 18 are incomplete. Typed contracts, validation,
device/capability state, geofence and expected-place decisions,
acknowledgement/check-in, policy compilation, alert/notification intents, a
SQLite ActivityStore read model, and portal presentation are real. The live
product chain is not complete: `TrackingRuntimeEventFlow` uses a process-local
event bus, there is no durable cascade-to-journal-to-SQLite replay path, no
production Android/iOS/desktop sensor adapters, no concrete places or AI
provider route, no durable place store, and no durable notification/escalation
outbox with provider receipts. WP37 durable journal/replay/projection is the
first implementation unblocker. No Phase 2 passing-test/Enforcer or Phase 3
proof/PR_READY claim is inferred.

## Consolidated branch code/test inventory - 2026-08-09

This is the recorded **source and test-topology** pass from 2026-08-09 on the
then-consolidated E: integration worktree. Verify the current checkout with
`git rev-parse HEAD`; the executable graph snapshot below is the current
authority. It counted
the plan-owned Rust/worker/UI source and crate-visible test files, then traced
the active policy request path. Counts mean files exist; they do **not** mean
the feature is accepted, a test was run today, or a workpack is complete.

## Executable graph control-plane snapshot - 2026-08-10

The repo-owned graph is now the mechanical status source over these plan rows;
the matrix remains the human-readable code/test audit. `graph.json` imports 23
plan directories and 679 workpack rows (the older 526-row figure above is the
matrix's narrower scheduled-row view). Run `npm run graph:report` for the
joined state/topology view or `npm run graph:report -- --json` for machine
consumption. This snapshot was refreshed from the single E: checkout on
2026-08-10 after the WP07 code/test ownership map and WP04 review-repair slice
were reviewed. Current
derived workpack state is:

| Planned | Blocked | Ready | Active | Validation | Done |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 452 | 9 | 0 | 1 | 216 | 1 |

### Graph-derived plan/workpack matrix

The rows below are the current graph projection for every plan directory. The
`P/B/R/A/V/D` column is the workpack count in `planned/blocked/ready/active/
validation/done` order. Code/test counts are reviewed-root topology for that
plan and may overlap because shared crates are intentionally visible in more
than one plan; they are not completion percentages.

| Plan | Workpacks | P/B/R/A/V/D | Implementation files | Test files |
| --- | ---: | ---: | ---: | ---: |
| Account identity/family | 8 | 1/0/0/0/7/0 | 159 | 82 |
| AI | 48 | 46/0/0/0/2/0 | 118 | 39 |
| App/game | 220 | 132/0/0/0/88/0 | 688 | 436 |
| App | 95 | 94/0/0/0/1/0 | 670 | 421 |
| Browser | 30 | 30/0/0/0/0/0 | 72 | 13 |
| Child-agent runtime distribution | 11 | 0/1/0/0/10/0 | 88 | 10 |
| Cloudflare control plane | 13 | 13/0/0/0/0/0 | 183 | 63 |
| Data custody/storage | 9 | 1/0/0/1/7/0 | 653 | 410 |
| Device trust bootstrap | 9 | 1/2/0/0/6/0 | 426 | 131 |
| Eventing | 13 | 1/0/0/0/11/1 | 777 | 492 |
| LAN | 25 | 0/0/0/0/25/0 | 308 | 60 |
| Logging domain parity | 10 | 5/0/0/0/5/0 | 127 | 49 |
| Network | 8 | 7/0/0/0/1/0 | 378 | 99 |
| Parent desktop/runtime package | 11 | 4/0/0/0/7/0 | 497 | 181 |
| Payment/subscription | 13 | 8/2/0/0/3/0 | 44 | 39 |
| Policy control plane | 8 | 0/2/0/0/6/0 | 911 | 481 |
| Portal UX/household surfaces | 20 | 15/0/0/0/5/0 | 974 | 531 |
| Remote access | 6 | 4/0/0/0/2/0 | 35 | 19 |
| Screen AI pipeline | 10 | 10/0/0/0/0/0 | 124 | 33 |
| Screen | 43 | 25/0/0/0/18/0 | 95 | 26 |
| Setup/install/provisioning | 7 | 0/1/0/0/6/0 | 529 | 196 |
| Tracking | 42 | 42/0/0/0/0/0 | 94 | 65 |
| V0.8 enforcement | 20 | 13/1/0/0/6/0 | 901 | 498 |

The graph validates at 703 nodes and 705 edges, with 34 migration/dependency
review items. The live map now covers 365 of 679 workpacks. Tracking contributes
42 reviewed maps: 24 bounded packets are Phase 1 complete and 18 retain concrete
production-code or expected-test gaps. Graph states remain separate
from that code-first classification. Historical
source/test rows are now classified as `validation` instead of being counted
as unreviewed planned work. Use `npm run graph:status`, `graph:ready`, `graph:blocked`,
`graph:inspect <id>`, and `graph:why <id>` instead of inferring readiness from
checklist prose. A graph `DONE` state requires the referenced implementation,
test, proof, checklist, and any detected ADR paths to exist; it does not claim
that CI or a product path has been merged.

There are **23 plan folders** and **526 scheduled workpack rows** in this
checkout (not 24 plans). The consolidated code train is merged to current
`main` and contains the LAN target repair, enforcement-journal sequence
repairs, parent policy-resolution/delivery binding, remote/tracking contracts,
and app runtime-decision contracts. The live reviewed-root topology is 2,831
implementation files and 1,182 test files. Scoped Enforcer, architecture,
generated-artifact, Rust, TypeScript, portal, and pre-commit validation pass.
Required CI run `31366692141` passed and PR #643 merged this train to `main` as
`47a2ac717`; proof custody and product-workpack acceptance remain separate
gates.

| Plan | Live code/test topology observed | Code-first state and next executable dependency |
| --- | --- | --- |
| Account identity/family | `family-identity-core` 43/23, `provisioning-core` 9/7, `storage-custody-core` 78/16 source/test files. | Core authority/custody exists; Cloudflare binding/migration and real provider/runtime routes remain separate work. |
| AI | `child-ai-core` 3/2 and `screen-ai-core` 20/3, joined through the shared agent/eventing stack. | Foundation only; next is one typed AI-result-to-policy consumer with negative safety coverage. |
| App/game | `app-game-core` 21/15 plus service adapters/read models. | Integration source exists; choose one live Windows capture-to-read-model path before treating any customer control path as ready. |
| App | `app-core` 3/5 plus shared agent-service integration and the versioned runtime-decision contract slice. | WP01 runtime-decision envelope/tuple sub-slice is code/test/proof/CI/main-merge verified; the broader workpack and app identity/evidence/session/service path remain open. |
| Browser | `browser-core` 33/20 plus service policy/adapter surfaces. | Integration source exists; managed command, adapter result, rollback, and parent-visible state are not a closed product path. |
| Child runtime distribution | `child-runtime` 31/9 plus service/protocol sources. | Runtime source exists; Windows service lifecycle/package smoke remains the next physical blocker. |
| Cloudflare control plane | `infra/cloudflare` 20/29 worker source/test files. | Local worker/contract source exists; deployment, account binding/migration, and payment runtime are not proven. |
| Data custody/storage | `storage-custody-core` 78/16, `ocentra-evidence` 1/4, eventing 80/34. | Core custody shapes exist; select retention/delete/export through service and journal. |
| Device trust bootstrap | `family-identity-core` 43/23 and `storage-custody-core` 78/16. | Windows custody/step-up slices exist, but durable trusted-device lifecycle, recovery, and cross-platform proof remain open. |
| Eventing | `ocentra-eventing` 80/34 plus protocol/service consumers. | Journal/replay and WP11 sequence repairs are freshly tested (24/24 journal/replay, 2/2 enforcement-eventing, 43/43 enforcement-runtime filtered); WP10 LAN consumer/product proof remains open. |
| LAN | `lan-core` 241/42 and `parent-runtime-core` 102/18. | Substantial pairing/runtime source exists; a real paired-device lifecycle through service and portal is still required. |
| Logging parity | `logging-core` 19/7 plus agent-service/portal integrations. | Foundation exists; enforce correlated logging on one product path rather than counting instrumentation alone. |
| Network | `network-core` 19/6 and `ocentra-network-evidence` 237/60. | Typed eventing foundation exists; complete parser-to-policy/service runtime before platform claims. |
| Parent desktop/runtime distribution | 497/181 across the reviewed parent runtime, schema, portal, Tauri, Android/iOS, and package-helper roots. | Package and smoke mechanics exist, but hosted authority, typed service truth, signing/store, updater/rollback, setup handoff, and the aggregate release gate remain open. |
| Payment/subscription | `billing-core` 17/4 and `entitlement-core` 8/5. | Core source exists; Cloudflare/account/trust dependency chain blocks a real checkout-to-entitlement path. |
| Policy control plane | `policy-control-core` 126/34, child policy/runtime and service seams. | Parent-resolution contract/service/UI and replay/audit persistence now exist; approved-request → compiled-artifact → child/device/domain binding is now code-backed and focused-tested. Trusted adapter execution and product delivery remain open and unclaimed. |
| Portal UX | `apps/portal` 104/50 and `portal-domain` 112/14. | Real presentation/test topology exists; it needs service-backed actions, not more presentation-only completion claims. |
| Remote access | `remote-access-core` 2/5 with adjacent screen/LAN surfaces plus the Rust-owned schema capability-fabric contract. | WP01 view-only capability authorization contract and focused schema tests now exist; pairing/relay/session runtime, device-trust integration, revoke/remove flow, and proof remain open. |
| Screen AI pipeline | `screen-ai-core` 20/3 and capture/agent/eventing seams. | Foundation; next is a redacted selected-window capture to typed AI-result chain. |
| Screen | `screen-core` 3/3, live view 3/4, capture adapter 7/3. | Foundation; prove custody/delete on one supported capture platform. |
| Setup/install/provisioning | `provisioning-core` 9/7 and `child-runtime` 31/9. | Integration source exists; parent setup-to-child trust/install depends on device-trust lifecycle truth. |
| Tracking | `tracking-core` 39/41 plus agent/eventing/network evidence owners and the Rust-owned WP34 event catalog. | WP34 event identity/required-field validation and focused schema tests now exist; runtime publishers, journal/read-model consumers, provider path, and proof remain open. |
| V0.8 enforcement | `child-enforcement-core` 8/3 plus policy/service/eventing owners. | Journal sequencing, parent-resolution, and request/artifact/target binding slices are code/test verified; trusted adapter execution, child delivery, rollback, and product-visible receipts remain open/unclaimed. |

## Fresh code/test verification on 2026-08-10

This is the current implementation evidence for the consolidated branch, not a
completion certificate. Local code/test/pre-commit gates pass; retained proof
artifacts and product-workpack acceptance remain separate gates. The
consolidated train's required CI and normal merge are recorded above.

| Slice | Code and focused validation observed | Honest state |
| --- | --- | --- |
| Policy WP05 / Enforcement WP10 parent resolution and delivery binding | Protocol 2/2; agent-service parent-resolution 2/2 plus the persistence-failure unit test 1/1 and confirmation target 3/3; parent-runtime UI 1/1; child-policy delivery binding 11/11; child-runtime delivery facade 5/5. Successful parent resolution now fails closed when durable activity-audit persistence cannot open or ingest, with `ACTIVITY_STORE_UNAVAILABLE` returned to the caller. Rust format check, scoped architecture gates, generated-artifact check, and Enforcer guard passed. | Request/artifact/target identity binding and audit-persistence failure propagation are code/test verified; consolidated CI and main merge are complete. Trusted adapter execution, product notification, retained proof, and workpack closure remain open. |
| Eventing WP06 Journal Replay And Lineage | `ocentra-eventing` journal/replay 24/24; topology/lineage and version-skew targets pass; the typed enforcement-audit handoff is retained in four durable proof records. The graph now maps 21 implementation and 13 test files and marks WP06 `done` only after reviewed code/test/proof/checklist evidence. | Generic journal/replay/handoff workpack is code/test/proof complete in the consolidated branch. CI/main merge and the downstream LAN consumer remain separate; Enforcement WP11 still owns its enforcement-specific journal contract. |
| App Plan WP01 Contract Boundary And Effect Schemas | Schema-domain build/type-check and contract tests 11/11; focused Rust app tests pass; scoped architecture and Enforcer guard pass; durable proof retained in `docs/proof/app-plan/slice-01-app-runtime-decision.md`; implementation merge PR #640 `ddec6d6c7d1a15e1a3d56562a3ab22ed2e990a3a`, fresh CI/main-state merge PR #643 `47a2ac717` recorded. | Focused contract sub-slice is code/test/proof/CI/main-merge verified; the workpack remains validation until its broader checklist and product paths are complete. |
| Enforcement WP11 journal consumer slice | Service enforcement-eventing 2/2; enforcement-runtime focused target 43/43 passed (7 filtered). Rust format check, scoped architecture gate, and Enforcer guard passed. | Code/test slice verified; the workpack remains open until its enforcement-specific durable query/audit proof and transition-family coverage are retained. |
| Eventing WP04 queue/idempotency/dead-letter rollback | `cargo test -p ocentra-eventing --tests` passed 31 contract, 2 integration, 24 journal-replay, 57 unit, and 2 version-skew tests, including initial queued-publish rollback and drop-oldest overflow restoration after journal failure. `cargo fmt --all -- --check`, scoped architecture, `hub:guard`, and `git diff --check` passed. | Code/test repair is locally verified and merged with the consolidated CI train; the eventing workpack and plan remain open until retained proof and acceptance agree. |
| Enforcement WP04 owned-process terminate/time-limit adapter | Windows owned-process lookup now reports `NoOp` with `PROCESS_ALREADY_EXITED` when the target is already gone; the time-limit boundary still maps that terminal process result to `Expired`. Agent-core enforcement target: 53 passed; agent-service enforcement target: 40 passed; Rust format, targeted clippy, scoped architecture, `hub:guard`, and `git diff --check` passed. | Code/test correction is locally verified on the consolidated branch. WP04 remains open (0/5 checklist rows closed): trusted persisted dispatch authority, durable WP11 journal handoff, rollback, retained proof, and CI/main merge are still required. |
| Policy WP04 receipt-aware transition precondition | `policy-control-core` unit target: 46/46 focused tests passed; `child-policy-core` delivery-handoff replay target: 12/12 passed. The trusted-receipt transition seam persists a matching adapter receipt and keeps the ordinary receiptless path fail-closed; scoped architecture, Rust format, and Enforcer commit gates passed. | Code/test slice is `validation`, not WP04 closure; consolidated CI and main merge are complete. The parent-runtime-to-agent-service trusted dispatch ledger, Eventing WP06 -> WP11 durable-journal handoff, OS adapter side effect, rollback execution, retained WP04 proof bundle, and workpack acceptance remain open. |
| Remote WP01 capability fabric | `crates/schema` Rust-owned view-only capability/grant/session contract; 4 focused contract tests passed, including exact schema-version, authenticated-parent, requested-child-device, and nonblank-audit checks. Rust format, architecture, generated-artifact, and Enforcer gates passed. Durable manifest: `docs/proof/remote-access-plan/slice-01-capability-fabric.md`. | Contract slice is `validation`; consolidated CI and main merge are complete. Pairing/relay/device-trust/session runtime, revoke/remove flow, custody, abuse proof, and workpack acceptance remain open; remote input/control remains deferred. |
| Device-trust WP08 dependency adoption | Dependency matrix covers WebAuthn, passkey, keyring, encrypted-bundle, and RustDesk decisions; contract test 1/1 passed; scoped architecture and Enforcer guard passed. Durable manifest: `docs/proof/device-trust-bootstrap-plan/slice-08-dependency-adoption.md`. | Review slice is `validation`; consolidated CI and main merge are complete. Runtime adoption, platform ceremony, key sealing, recovery execution, retained proof, and workpack acceptance remain open. |
| Network WP08 control-catalog routing | Focused route-boundary contract test 1/1 passed; exact capability/schema/settings source docs and no-default-read/no-runtime-claim wording are asserted. Scoped architecture and Enforcer guard passed. Durable manifest: `docs/proof/network-plan/slice-08-control-catalog-routing.md`. | Reference-routing slice is `validation`; consolidated CI and main merge are complete. Network runtime/parser/classification/policy/enforcement/platform work and workpack acceptance remain open. |
| Tracking WP34 event contracts | `crates/schema` Rust-owned 19-event catalog with required causation/evidence/policy/live-mode TTL/audit/reason/transition and typed AI payload safety validation; 4 focused contract tests passed as part of the 107-test schema contract target. | Contract slice verified. Runtime event publishers, journal/replay projection, portal read model, and retained proof/audit remain open. |
| App WP01 runtime decision boundary | `ocentra-app-core` contract 4/4 and invariant 2/2 passed; schema-domain build/type-check and `app-runtime-decision.test.ts` passed 11/11; scoped architecture and Enforcer guards passed. | Contract/runtime-decision code/test slice and local proof are current; consolidated CI and main merge are complete. Installed inventory, broader app runtime/service/portal path, retained proof, and workpack acceptance remain open. |
| Other 18 plan folders | This pass rechecked topology and selected dependency rows only; no fresh focused implementation validation was run for those plans. | Not closure. Their existing matrix/workpack rows remain routing context until each code/test slice is audited. |

### Current dependency decision

The artifact/target identity contract is present at the child delivery boundary,
enforced by the child-runtime facade, and now carried through the
parent-resolution transport/service binding with focused mismatch coverage.
The policy owner now exposes a receipt-aware transition seam, but that seam only
accepts adapter evidence; it does not mint authority or perform an OS action.
The next implementation is the enforcement-owned trusted adapter and durable
WP11 journal route. Until that route supplies non-forgeable execution authority,
delivery remains manual-required rather than active. Actor/device/route/version,
expiry, duplicate/replay, audit, and notification-no-claim negatives remain
required for that next slice. This is still the shared unblocker for policy,
enforcement, portal, app/game, browser, network, and screen paths.

## Current merged-code train - 2026-08-06

Live baseline: `origin/main` at `0923976516098d36962566c2eea3933e6d878b00`.
This refresh inspected merged source, retained proof paths, and required CI for
the six PRs below. It records delivered slices only; no row upgrades a plan or
product path to complete.

| Plan / workpack | Merged source, tests, and proof observed on the baseline | Fresh required CI | Honest current state / next unblocker |
| --- | --- | --- | --- |
| Eventing WP06 journal/replay/lineage | [#621](https://github.com/ocentra/OcentraParent/pull/621) merged as `96735c5715cca0a59858febcfe43a2370707d43c`. `agent-protocol`, `agent-service`, and `ocentra-eventing` carry the typed enforcement audit-journal handoff and retry/idempotency coverage; retained evidence is `docs/proof/eventing-plan/wp06-00-enforcement-wp11-handoff.md` and `wp06-16-validation-commands.md`. | [run 31064093081](https://github.com/ocentra/OcentraParent/actions/runs/31064093081): required format/types/Rust, full validation, and package-preview gates passed. | WP06's merged handoff is evidenced. The plan stays open on WP10 LAN consumer proof and its consumer-plan handoff. |
| Account identity/family WP08 authority parity | [#622](https://github.com/ocentra/OcentraParent/pull/622) merged as `36969f92a6a41573284d1ae93e422640bd3d873e`. `family-identity-core` contains the account/household authority handoff and contract/negative tests; durable parity, negative, redaction, Cloudflare-handoff, and validation records are tracked under `docs/proof/account-identity-family-plan/08-rust-schema-workers-d1-runtime-migration/`. | [run 31067638811](https://github.com/ocentra/OcentraParent/actions/runs/31067638811): required format/types/Rust, full validation, and package-preview gates passed. | The Rust authority/generated-edge slice is merged and proven. Cloudflare WP06/WP08 runtime/migration work and Account WP06 aggregation remain open. |
| Account identity/family WP01 D1 storage adapter | [#632](https://github.com/ocentra/OcentraParent/pull/632) merged as `0923976516098d36962566c2eea3933e6d878b00`. `infra/cloudflare` now contains the optional, fail-closed `ACCOUNT_IDENTITY_D1` provider-subject-to-Ocentra-account store, binding declarations, and D1-shaped unit/env coverage. Retained evidence is `docs/proof/account-identity-family-plan/01-auth-provider-decision/06-account-identity-storage-adapter-proof.md`. | [#632 required CI](https://github.com/ocentra/OcentraParent/pull/632/checks): required format/types/Rust, full validation, and package-preview gates passed before merge. | This is a narrow mapping boundary only: it does not prove external token verification, login/session routes, household/device authority, D1 deployment or migration, Worker production readiness, WP01 closure, or plan closure. Next is the separately owned Cloudflare WP06 binding/migration and WP08 runner proof, then Account WP06 aggregation. |
| Policy/enforcement authenticated delivery-grant boundary | [#633](https://github.com/ocentra/OcentraParent/pull/633) merged as `86e0b5d8ea071fe71970b9e231cd4bf92ca257e0`. `agent-protocol`, `policy-control-core`, and `agent-core` now issue, validate, and consume the signed delivery-grant prerequisite with contract/unit coverage. | [run 31096089598](https://github.com/ocentra/OcentraParent/actions/runs/31096089598): all required format/types/Rust, full validation, platform, and package-preview gates passed. | Merged prerequisite only. It does not establish trusted adapter execution, rollback, receipt visibility, broad enforcement completion, or any workpack/checklist closure; those remain owned by the policy and enforcement runtime workpacks. |
| Device trust WP02 Windows local custody | [#623](https://github.com/ocentra/OcentraParent/pull/623) merged as `46bb53da4d0dfbdd8d1b40937abfd67262aac8c3`. `storage-custody-core` has `windows_device_trust_custody` plus Windows/unsupported-platform tests, with the authority path in `family-identity-core` and `parent-runtime-core`. | [run 31057272950](https://github.com/ocentra/OcentraParent/actions/runs/31057272950): required format/types/Rust, full validation, and package-preview gates passed. | Narrow Windows-only custody slice is merged. Cross-platform custody, recovery/reset/re-pair, QR approval, and child tamper/uninstall proof remain open. |
| Device trust WP03 parent step-up | [#627](https://github.com/ocentra/OcentraParent/pull/627) merged as `1ce56056c8c233addafe89feec7008c2bdda7059`. `family-identity-core::parent_step_up_authority` and `schema::parent_step_up_receipt` provide the fail-closed authority/receipt boundary with unit and contract coverage. | [run 31061365961](https://github.com/ocentra/OcentraParent/actions/runs/31061365961): required format/types/Rust, full validation, and package-preview gates passed. | The record-backed parent-step-up boundary is merged; it is not phone-QR, recovery-bundle, entitlement, or child-uninstall runtime closure. |
| Network WP01 typed-eventing handoff | [#617](https://github.com/ocentra/OcentraParent/pull/617) remains the merged bounded `NetworkFlowObservedEvent` to reusable `DomainEvent`/`EventEnvelope` contract handoff. Its retained contract proof is `docs/proof/network-plan/01-network-foundation-eventing-contract.md`. | Historical merged evidence; no fresh network-runtime CI assertion is made by this docs refresh. | WP01 and all seven sibling network workpacks remain open; next is the remaining WP01 contract obligations, then parser-to-policy runtime proof. |

## Merged-baseline evidence delta — 2026-08-05

This narrow refresh inspected `origin/main` at `580fb9ec` plus the live PR and
CI state named below. It updates only the affected routing decisions; it does
not turn a merged CI/tooling repair or an open slice into workpack closure.

| Scope | Current evidence | Status / no-claim boundary | Next unblocker |
| --- | --- | --- | --- |
| CI setup reliability | PR [#619](https://github.com/ocentra/OcentraParent/pull/619) merged as `580fb9ec`; its fresh CI Gate passed. `.github/workflows/dependency-policy.yml` now explicitly skips unused Linux Tauri packages while retaining Rust setup, and `tests/repo-tooling/workflow-ci-trigger.test.mjs` covers that call. | Merged CI reliability/tooling repair only; it is not policy, device-trust, or product-workpack proof. | Treat the repaired Dependency Policy runner as normal CI infrastructure; validate each product slice on its own evidence. |
| Policy WP02 parent authoring/preview | PR [#615](https://github.com/ocentra/OcentraParent/pull/615), head `cc01f9ae`, is an open, reviewed partial slice with CI still active. It projects Rust-owned conflict/manual-required/unsupported attention cards and adds scoped portal/integration tests plus `02-conflict-visible-proof.md`. | WP02 remains open: templates/manual-rule authoring, preview-to-save confirmation, opaque confirmed-request relay, delivery/enforcement, child-device application, and full proof fields are outside #615. | Finish the remaining WP02 authoring and confirmation path with targeted portal/accessibility/mobile proof, then reconcile the workpack from merged evidence. |
| Device Trust WP02 local key sealing | PR [#616](https://github.com/ocentra/OcentraParent/pull/616) is open and unmerged. Current `main` has neither `crates/storage-custody-core/src/windows_dpapi_key_sealing.rs` nor `crates/family-identity-core/src/trust_bootstrap/current_authority.rs`; the required `output/device-trust-bootstrap-plan-proof/02-local-key-sealing/` root is absent. Review findings also reject the proposed lifecycle/custody model as an authority closure. | Blocked. There is no merged production Windows custody coordinator, sealed-key persistence/current-authority owner, or accepted runtime proof; model and narrow parent-presence code are not WP02 closure. | Assign a single production custody owner that durably coordinates Windows sealing with non-restorable current authority, household/device binding, revocation/reset, and Windows proof before reopening merge readiness. |

## Code-backed refresh — 2026-07-31

This refresh is a scheduling override for the older rows below. It was produced
from plan state/workpack indexes plus current source and test surfaces. A
checked plan row is **not** reproducible closure when its named `output/*proof*`
root is absent from the checkout. Use the four states below when assigning work:

- **Source + focused tests**: implementation and relevant test source exist;
  the test was not re-run by this refresh.
- **Locally proven**: a current focused validation/proof artifact was observed.
- **Proof absent**: implementation may exist, but the plan's required proof
  output is unavailable here and must be recreated before closure.
- **Blocked**: one exact missing runtime contract, provider, platform, or
  external prerequisite prevents the next validation.

| Plan | 2026-07-31 code-backed state | Next owning workpack / dependency |
| --- | --- | --- |
| account identity/family | Source + focused tests; named proof roots absent. | Account WP08 Rust-owned schema/authority, then Cloudflare WP06 D1/DO/KV binding/migration and Cloudflare WP08 runner/proof, after the provider/custody decision. |
| AI | Precursor source only; 47 of 48 workpacks remain open. | WP03 Rust contract, then WP07 queue and WP08 routing. |
| app | Source + focused tests; all app workpacks remain open/unknown. | WP06–09 Windows capture/foreground path, then WP12–15 service/journal. |
| app/game | Source + focused tests; customer control path unproven. | WP19 policy compiler, then WP20 budget evaluator. |
| browser | Source + focused tests; plan still has all primary workpacks open. | WP01 Rust-owner contract reconciliation before launcher/custody work. |
| child runtime distribution | Source present; package/service proof roots absent. | WP02 Windows service lifecycle, then recreate package/install proof. |
| Cloudflare control plane | #608 merged to `main` as `5af4a1a92` after fresh full CI; the worker and generated billing contracts are present, and the local WP07 dev/seed/proof boundary is validated. PR #604 is closed without merge: its branch/evidence are preserved, but its overlapping implementation is superseded/conflicting. | Keep WP07 local-only/open. Its successor is proof-only and must start from current source after the real Cloudflare dependency-resolution gap is fixed; then rerun selected gates and retain a tracked bundle. |
| data custody/storage | Source + focused tests; rollout proof absent. | WP07 regenerate current proof manifest and verify runtime gaps. |
| device trust bootstrap | Source + focused tests; platform sealing/product chain absent. | WP02 local key sealing, then step-up/recovery/tamper workpacks. |
| eventing | WP06 Journal Replay And Lineage is graph-complete: reviewed code/test ownership and the durable generic-to-enforcement handoff are present. WP10 LAN household mesh proof and downstream enforcement work remain open. | WP10 LAN household mesh consumer and route proof; separately, Enforcement WP11 consumes the WP06 handoff. |
| LAN | Strong source/test foundation; physical product proof absent. | WP23 two-device proof, then WP25 backend-to-Tauri-to-portal chain. |
| logging parity | Source + focused tests; proof restoration and routing lint remain. | WP03 Rust service-to-logging mapping, then routing-lint repair. |
| network | Source + focused tests; all workpacks open. | WP01 foundation contracts/eventing. |
| parent runtime distribution | Source present; signing/install proof absent. | WP10 setup handoff contracts, then signing/store gates. |
| payment/subscription | Source + focused tests; Cloudflare dependency blocks runtime. | Cloudflare billing boundary, then WP05 tax/refund/dispute policy. |
| policy control plane | Strong focused contracts; adapter execution remains an external enforcement dependency, not a WP04 contract task. | WP02 authoring/preview and WP05 ask-parent/override work; track trusted-adapter execution with the enforcement owner. |
| portal UX | Real shell/UI; most product flows open. | WP05 policy authoring after owning policy read model is ready. |
| remote access | Scaffold/adjacent route only. | WP01 capability fabric, then WP04 pairing/revocation. |
| screen AI pipeline | Source pieces only; no pipeline proof root. | WP01 prerequisite gate then WP02 trigger-to-capture proof. |
| screen | Capture/AI/live-view source exists; custody/platform closure open. | WP03 contract, then WP06–08 capture model/adapter. |
| setup/install/provisioning | Source + focused tests; depends on identity/device/install truth. | Parent setup-to-child trust/install handoff. |
| tracking | Strong source/test foundation; event spine and provider proof open. | WP34 tracking event contracts, then WP35–39 chains. |
| v0.8 enforcement | Source + focused tests; Eventing WP06 generic journal/replay handoff is now graph-complete, while enforcement-specific journal, adapters, and parent-visible receipts remain open. | WP11 durable enforcement-journal handoff -> WP04 trusted dispatch -> adapter/receipt/rollback. WP11 and WP04 remain unscheduled/manual-required until their own evidence exists. |

### Priority order

1. Cloudflare module dependency reconciliation; separately, Account WP08 Rust contract -> Cloudflare WP06 D1/DO/KV binding/migration -> Cloudflare WP08 runner/proof -> Account WP06 aggregation (unblocks accountable Cloudflare verification and payment routing).
2. Eventing WP06 Journal Replay And Lineage: retain the actual journal/replay proof and typed enforcement handoff before WP10 or enforcement WP11/WP04 scheduling.
3. Network WP01 eventing foundation.
4. Tracking WP34 event contracts.
5. Screen-AI WP01 → WP02 trigger/capture chain.
6. AI WP03 → WP07 → WP08; Remote WP01 → WP04.

Proof-root restoration is a verification/reproducibility packet. It must not be
used to claim a product path complete without the selected runtime proof.

## Live integration refresh — 2026-07-28

This is a merge-queue snapshot, separate from the 2026-07-17 whole-plan code
audit below. It records work that has current source, tests, proof, and a
pushed review branch; it is **not** a completion claim until the PR has green
CI, an acceptable review, is merged, and the named checklist row is reconciled.

| Plan / workpack | Current implementation evidence | Validation / integration state | Next action |
| --- | --- | --- | --- |
| Enforcement / WP05 app-game session handoff | Service validates persisted runtime/session evidence, rejects unknown identity, and asserts stored timer-binding fields. Protocol, core, and service focused tests are present. | PR [#584](https://github.com/ocentra/OcentraParent/pull/584), head `098b18acd`; all required jobs except the rerun service Clippy chain are green. The rerun is actively executing, not failed. | Let Clippy complete; then run service-test/aggregate checks, review, reconcile WP05 rows, and merge only if all are green. |
| App / WP01 runtime-decision contract boundary | Rust-owned app runtime contract change and focused branch are pushed. | PR [#577](https://github.com/ocentra/OcentraParent/pull/577) awaits CI/review. | Audit exact checklist/proof rows before treating it as closed. |
| Device trust / WP01 durable registry | Durable family/device trust registry branch is pushed. | PR [#576](https://github.com/ocentra/OcentraParent/pull/576) awaits CI/review. | Validate platform-sealing and parent-presence boundary before merge. |
| Network / WP01 foundation contract/eventing | Runtime contract/eventing repair is pushed. | PR [#573](https://github.com/ocentra/OcentraParent/pull/573) is behind `main`. | Rebase, rerun focused gates/CI, then review. |
| Cloudflare / WP12 handoff | Current generated billing-contract source, local seed/runtime proof, and focused worker tests are in the branch. | PR [#568](https://github.com/ocentra/OcentraParent/pull/568) is behind `main`. | Rebase, rerun worker/billing gates, reconcile stale plan wording. |
| Logging / WP04 parity, WP04/WP05/WP10 proof restoration | Three proof-restoration branches and one parity branch are pushed. | PRs [#572](https://github.com/ocentra/OcentraParent/pull/572), [#580](https://github.com/ocentra/OcentraParent/pull/580), [#581](https://github.com/ocentra/OcentraParent/pull/581), [#582](https://github.com/ocentra/OcentraParent/pull/582); #572 is behind `main`, the others await CI/review. | Regenerate each proof at its exact head; docs-only repair is not plan closure. |
| Portal / LAN result flow and E2E cleanup | Selected LAN-target persistence and E2E process cleanup are pushed. | PRs [#578](https://github.com/ocentra/OcentraParent/pull/578) and [#579](https://github.com/ocentra/OcentraParent/pull/579) await CI/review. | Run real portal/service click-through after CI; map accepted work to LAN/portal rows. |
| Shared tooling / schema build order | Test-build sequencing repair is pushed. | PR [#575](https://github.com/ocentra/OcentraParent/pull/575) is behind `main`. | Rebase and validate before wider test claims. |
| Local proof environment | Local proof isolation from LAN environment is pushed. | PR [#583](https://github.com/ocentra/OcentraParent/pull/583) awaits CI/review. | Treat as test infrastructure, not product-workpack closure. |

Only #584 is actively executing. The queue is parked for integration triage:
rebase-required first, then CI/review, then checklist and post-merge proof
reconciliation. No workpack here is done solely because a branch or PR exists.

## Status vocabulary

- **Foundation**: meaningful source and focused tests exist, but the product
  path is not yet proven end to end.
- **Integration**: multiple runtime layers exist; remaining work is joining,
  proving, or operating the path.
- **Blocked**: code or an external/runtime prerequisite prevents the next
  required proof.
- **Scaffold**: models/contracts or a thin implementation exist, but no
  credible product-runtime closure exists yet.

`Implemented` is not `done`. A workpack is only done after its selected tests,
proof artifact, checklist row, and merge state agree.

## Plan execution matrix

| Plan | Code state | Main runtime owners | Evidence observed | Current blocker / dependency | Next unblocker |
| --- | --- | --- | --- | --- | --- |
| `account-identity-family-plan` | Foundation | `family-identity-core`, `provisioning-core`, `entitlement-core`, `schema`, narrow `infra/cloudflare` D1 adapter | Family identity has 13 source / 7 test files; setup and signed-entitlement paths exist. Merged [#632](https://github.com/ocentra/OcentraParent/pull/632) adds only a retained, optional/manual-required provider-subject mapping adapter with D1-shaped tests; the old #607 TypeScript test-double remains closed-unmerged and is not authority evidence. | Account WP08 Rust schema/account-authority proof is merged, but Cloudflare WP06 D1/DO/KV binding/migration proof, Cloudflare WP08 runner proof, external provider verification, and account runtime routes remain absent; Account WP06 is reopened to aggregate the required handoffs. | Keep #632's narrow custody boundary fail-closed; execute Cloudflare WP06 then WP08 from the Account WP08 handoff, prove deployment/migration and runtime routes, then rerun Account WP06 only after all required inputs are green. Any missing input remains a payment/policy/remote/device-trust scheduling block. |
| `ai-plan` | Fully audited / Phase 1 incomplete | `agent-protocol`, `agent-core`, `agent-service`, `child-ai-core`, `screen-ai-core`, browser/app-game/tracking evidence owners, portal-domain, portal | All 48 workpacks now have reviewed topology: 118 implementation and 39 test files in the narrowed plan roots. Eleven workpacks have bounded Phase 1 code/expected tests written. Real foundations include configured local `llama.cpp` execution, singleton scheduling, household LAN claim/lease/idempotency, Activity memory graph, tracking AI validation, Parent Assistant boundaries, and portal AI runtime cards. | Thirty-seven workpacks retain code/test gaps. Highest-impact missing systems are the canonical general AI contract family, durable general work lifecycle, SQLite-backed context/result journal, semantic memory, verified model artifact installer, owned OCR/VLM workers, unified explanation surface, trusted remote authorization/adapter, and complete security/performance tests. | Finish WP03/WP04, WP07, WP09/WP12, and WP14-WP19 in dependency order before memory/model/screen/feature closeout; run Phase 2 and Phase 3 only after the corresponding Phase 1 rows are complete. |
| `app-plan` | Foundation | `app-core`, `agent-service`, `schema` | `app-core` has 3 source / 5 test files; service owns wider integration. | App-only authority and runtime evidence are incomplete. | Make app identity/evidence flow a single Rust-owned service path. |
| `app-game-plan` | Integration | `app-game-core`, `agent-service`, `schema` | 25 source / 20 test files; inventory, runtime, journal, and policy code exist. | Live platform metadata/crawling and portal product rows are incomplete. | Finish one live Windows app/game capture-to-read-model path. |
| `browser-plan` | Fully audited / Phase 1 incomplete | `agent-protocol`, `agent-core`, `agent-service`, `browser-core`, schema-domain, portal-domain, portal, Android owned shell | All 30 imported packets have reviewed ownership. Fourteen executable workpacks have bounded code/expected tests written; contracts, Windows inventory, managed launch, custody/CDP mapping, durable ingest/read models, policy manifest/compiler, and unmanaged detection are real. | Ten workpacks still lack cross-platform inventory, corrupt/concurrent profile-store negatives, active-focus evidence, complete portal status tests, trusted managed/unmanaged action execution, live AppLocker control, extension integration, health/load harnesses, or aggregate E2E. | Complete WP05/WP06 and WP11 first, then WP17/WP19; finish WP14 and WP20-WP23 before Phase 2 focused execution and Phase 3 proof. |
| `child-agent-runtime-distribution-plan` | Fully audited / Phase 1 incomplete | `child-runtime`, Rust schema/child-enforcement contracts, Android/iOS child apps, desktop package/service scripts, CI/release workflows | All 11 workpacks have reviewed code/test ownership. Windows lifecycle harness, Android emulator lifecycle, iOS capability package, platform matrix, and all five package builders are real. | Six workpacks still lack real-host lifecycle tests, executable respawn, parent-authorized uninstall runtime, a consumed setup/trust handoff, or a complete multi-platform release gate; several migrated Enforcer proof runners target deleted files. | Complete macOS/Linux lifecycle tests first, then respawn/uninstall and setup-handoff runtime paths, and close with the aggregate multi-platform release gate. |
| `cloudflare-control-plane-plan` | Fully audited / Phase 1 incomplete | `infra/cloudflare`, account/billing contracts, portal consumer boundary | All 13 workpacks now have reviewed code/test ownership. WP00-WP02 and WP04 are complete for code/expected-test writing; the Worker, bindings, runner, local-dev, security, and contract source is real. | Nine workpacks still lack required runtime/authority/persistence/consumer/deployment or verifier code. The sharpest gaps are real auth/provider verification, durable DO state, account migration isolation, actual persisted local seeding, a true portal consumer smoke, and deployment/rollback automation. | Complete the nine Phase 1 rows before broad Cloudflare tests or proof regeneration; then run focused module families and Enforcer, followed by retained proof and the payment handoff gate. |
| `data-custody-storage-plan` | Integration | `storage-custody-core`, `ocentra-evidence`, `ocentra-eventing` | Storage core has 63 source / 12 test files; custody/delete/export shapes exist. | Rollout/route-gate aggregation and cross-runtime custody proof remain open. | Prove one retention/delete/export flow through storage, eventing, and service. |
| `device-trust-bootstrap-plan` | Fully audited / Phase 1 incomplete | `family-identity-core`, `storage-custody-core`, `parent-runtime-core`, `entitlement-core`, `child-enforcement-core`, `schema` | All 9 workpacks have reviewed code/test ownership. Durable lifecycle/parent-presence libraries, a Windows DPAPI custody slice, signed step-up proof contracts, entitlement/recovery contracts, and honest tamper-status rows are real; WP08 is complete for its research/test-only scope. | Eight workpacks still lack required production composition, cross-platform custody, live approval/QR authority, trusted entitlement verification, actual restore/re-pair mutation, child tamper/uninstall execution, or a route-gate verifier. | Complete WP01's live trust-owner composition first, then WP02/WP03 platform custody and authority; continue in dependency order through WP04-WP07 and close with WP09 after WP08's bounded research packet. |
| `eventing-plan` | Integration | `ocentra-eventing`, `agent-protocol`, `agent-service` | 76 source / 34 test files; WP06 journal/replay, topology, version-skew, and typed handoff surfaces are retained and graph-mapped. | WP10 LAN consumer proof remains open; downstream Enforcement WP11 owns the enforcement-specific durable journal contract. | Select the WP10 consumer path and prove replay/idempotency end to end; then complete Enforcement WP11 before WP04. |
| `lan-plan` | Fully audited / Phase 1 incomplete | `lan-core`, `agent-protocol`, `agent-service`, `parent-runtime-core`, parent desktop, portal-domain, portal | All 25 workpacks have reviewed ownership. Twenty-two have bounded code/expected tests written across discovery, identity, merge/classification, persistence, pairing, routing, and portal surfaces. | WP16 lacks one integrated backend-to-Tauri-AppHandle-to-portal-listener regression. Six aggregate verifier commands named by current LAN docs point to absent scripts, including two incorrectly checked complete by WP20. WP25 depends on both gaps; physical/manual artifacts remain Phase 3. | Write the WP16 integrated delivery regression and restore or replace the six named executable verifiers; then WP25 can enter Phase 2 focused validation before physical proof. |
| `logging-domain-parity` | Foundation | `logging-core`, `logging-domain`, `agent-service`, portal | Logger, local evidence, MCP/query, and portal paths exist. | Broad adoption and several proof-root closeouts remain. | Make logging/proof correlation mandatory for one high-value product chain. |
| `network-plan` | Fully audited / Phase 1 incomplete | `network-core`, `ocentra-network-evidence`, `agent-protocol`, `agent-core`, `agent-service`, portal | All eight workpacks have reviewed code/test ownership. Deterministic capture parsing, classification/correlation, evidence bundles, platform proof gates, analyzer/AI/risk models, service read models, and the portal drawer are real; WP05 and WP08 have no bounded Phase 1 writing gap. | Six workpacks remain incomplete. Production captures only Windows connection metadata; parser/classifier/AI/policy/notification/adapter/custody owners are not composed into a live path, the product-path helper fabricates downstream proof rows, canonical contract truth is duplicated, and performance/security rollout lacks executable load/abuse/rollback harnesses. | Fix WP01 durable canonical contracts and WP02 live capture-to-parser composition first; then wire WP03/WP04/WP06 real owners and finish WP07's executable harnesses before any whole-plan Phase 2 or proof claim. |
| `parent-client-runtime-distribution-plan` | Fully audited / Phase 1 incomplete | Tauri parent desktop, hosted portal, Android/iOS parent projects, `parent-runtime-core`, parent CI/package helpers | All 11 workpacks have reviewed code/test ownership. WP01 is a bounded no-code route packet; WP04/WP05 have real parent package projects plus dedicated CI smoke; WP09's cross-target smoke harness is written. | Seven workpacks still lack real hosted auth/cache, typed local-service transport, honest service-derived authority state, parent artifact signing/store matrix, updater/rollback runtime, setup-distribution handoff, or a parent-client aggregate release gate. | Fix WP02 hosted authority and WP06/WP03 service truth first, then WP10 setup handoff, WP07 signing matrix, WP08 updater/rollback, and WP11 aggregate gate. Run Phase 2 per target only after those writing gaps close. |
| `payment-subscription-plan` | Fully audited / Phase 1 incomplete | `billing-core`, `entitlement-core`, Rust billing schemas, Cloudflare worker, portal consumer boundary | All 13 workpacks have reviewed code/test ownership. Real schema, webhook classification, entitlement gates, Worker routes, and focused tests exist, but only WP00 has no Phase 1 writing gap. | Twelve workpacks still lack required pricing authority, provider execution/verification, durable ledger/retry state, trusted entitlement signatures, regional/referral lifecycle, parent/admin UI, or the payment-specific rollout verifier. The plan's `done` labels for WP01/WP03/WP04 overstate live runtime closure. | Complete WP01 pricing authority first, then WP02-WP04 checkout/webhook/entitlement runtime foundations; continue in dependency order through WP05/WP06/WP08-WP12 and finish with WP07. Run focused tests/Enforcer only after each Phase 1 slice is written; proof remains last. |
| `policy-control-plane-plan` | Integration | `policy-control-core`, `agent-service`, `schema`, eventing | 126 source / 25 test files; compiler, preview, delivery, conflict, and authority code exist. | Policy-to-enforcement command/rollback product proof is incomplete. | Prove typed policy compile, delivery, execution receipt, and rollback. |
| `portal-ux-household-surfaces-plan` | Fully audited / Phase 1 incomplete | Portal, `portal-domain`, `parent-runtime-core`, service read models, Android/iOS parent shells, package/CI helpers | All 20 workpacks have reviewed code/test ownership. Nine bounded workpacks have their core code/expected tests written; real shell, device targeting, browser/app/network state, honest degraded handling, no-fake-data contracts, screenshot harnesses, and mobile shells exist. | Eleven workpacks still lack household authority/first-run state, authoring/request actions, LAN/assistant command consumption, diagnostics redaction/history, cohesive report/notification custody, or plan-wide accessibility tests. | Complete WP01/WP02 first, then WP05-WP07 and WP10/WP11; finish WP08/WP12/WP14/WP15 before Phase 2 focused tests and Phase 3 proof. |
| `remote-access-plan` | Scaffold | `remote-access-core`, `screen-live-view-core`, LAN, portal | Remote core has 2 source / 5 test files; adjacent live-view pieces exist. | Session grants, relay, revocation, and safety proof are not implemented as a product path. | Build view-only session grant/revoke state before any control feature. |
| `screen-ai-pipeline-plan` | Fully audited / Phase 1 incomplete | `screen-ai-core`, capture adapter, `agent-protocol`, `agent-core`, `agent-service`, portal | All 10 workpacks have reviewed code/test ownership. Real capture, encrypted queueing, local adapter execution, deletion, read models, and portal rendering exist; only WP01 has no bounded Phase 1 writing gap. | Nine workpacks remain incomplete: trigger ownership and parent settings are disconnected, canonical AI routing is not production-wired, policy/action authority is fabricated or absent, the normal event chain is not durably replayable, custody negatives are missing, and live/performance/final harnesses are unwritten. | Wire WP02 parent settings and real trigger owners first, then WP03 canonical AI routing and WP04 trusted policy handoff; complete action, journal, custody, performance, and operator/final gates in dependency order before Phase 2. |
| `screen-plan` | Fully audited / Phase 1 incomplete | `screen-capture-adapter`, `agent-protocol`, `agent-core`, `agent-service`, `screen-ai-core`, `screen-live-view-core`, schema, Android agent, portal | All 43 imported packets have reviewed ownership: 40 executable workpacks plus three reference-only packets. Real parent settings, desktop/Linux capture, Android MediaProjection code, encrypted queueing, redaction, deletion, read models, portal UI, route guards, live-view gates, and local-AI scheduling exist. | Thirty-one executable workpacks retain code/test gaps. The sharpest are stale source/snapshot routing, missing iOS/Android/macOS/Linux platform tests, no protected-surface detector, no durable full-chain replay, fabricated policy refs, gate-only live view, no child disclosure, and missing CDP/OCR/VLM/detector/rollout harnesses. | Reconcile WP01/WP02 first, then close contracts/scope/platform safety (WP03/WP05-WP14) and queue/runtime composition (WP16-WP21) before policy/live/AI/rollout packets. Do not regenerate proof until those Phase 1 gaps close. |
| `setup-install-provisioning-plan` | Fully audited / Phase 1 incomplete | `provisioning-core`, `child-runtime`, `parent-runtime-core`, `schema`, `portal-domain`, portal | All 7 workpacks have reviewed code/test ownership. WP01-WP04 and WP06 are bounded no-code handoff/aggregation packets; WP05's Rust readiness model and negative tests are real; WP07 has a Rust-owned portal boundary-status panel. | WP07 still hard-codes sibling states as unavailable and does not implement the required first-run state machine, screen flow, actions, or readiness-driven completion guard. Historical proof text also cites removed packages and requires Phase 3 reconciliation. | Implement WP07 against typed `provisioning-core` readiness, with blocked/degraded/manual transition tests; then run focused Phase 2 gates before reconciling proof and sibling handoffs in WP06. |
| `tracking-plan` | Fully audited / Phase 1 incomplete | `tracking-core`, `schema`, `agent-protocol`, `child-runtime`, `parent-runtime-core`, `child-policy-core`, `child-notification-core`, `child-ai-core`, `policy-control-core`, `agent-core`, `agent-service`, portal-domain, portal | All 42 packets have reviewed code/test ownership. Twenty-four bounded packets have core code and expected tests across contracts, validation, status, geofence/expected-place decisions, acknowledgement/check-in, policy, alert intents, ActivityStore read models, and portal presentation. | Eighteen packets remain incomplete. The process-local event cascade is not durably journaled/replayed/projected; production platform sensor adapters, concrete places/AI providers, durable local places, notification/escalation delivery state, and an end-to-end restart-safe portal chain are absent. | Complete WP37 durable journal/replay/projection first, then WP38/WP27 delivery and escalation, WP22/WP07 persistence/custody, platform adapters, providers, and final composition/UI packets before Phase 2 or proof. |
| `v0-8-enforcement-control-plan` | Foundation / fully audited, Phase 1 incomplete | `schema-domain`, `schema`, `agent-protocol`, `child-enforcement-core`, `policy-control-core`, `agent-core`, `agent-service`, browser/network owners, portal | All 20 workpacks have reviewed code/test maps. Typed contracts, evidence-bound dispatch, honest capability matrices, unmanaged fallback, report-only network state, and durable timer recovery are written. | Thirteen workpacks still lack trusted delivery/execution authority, complete audit transition families, app/game and managed-browser action receipts, aggregate service state, child/portal surfaces, live integrity/uninstall observation, platform-role separation, or whole-plan verifier/UI tests. | Complete WP11 -> WP04 -> WP05/WP06, then WP10/WP12/WP13, portal/integrity/platform surfaces, and finally WP18/WP19/WP20. Run focused Phase 2 validation only after each Phase 1 slice is written. |

## Workpack execution audit

This table is the current scheduling baseline derived from every routed
`WORKPACK_INDEX.md`, not a completion certificate. `Doc-claimed closed` means
the plan index currently marks the row checked/done. `Freshly reverified` stays
zero until the current branch regenerates the named proof and passes the
focused acceptance gate. Gitignored or absent historical `output/` and
`test-results/` paths cannot be used as retained current proof.

| Plan | Execution rows | Doc-claimed closed | Open / partial / blocked / unknown | Freshly reverified | Scheduling note |
| --- | ---: | ---: | ---: | ---: | --- |
| `account-identity-family-plan` | 8 | 5 | 3 | 0 | Checklist now has 90/103 checked: WP01 remains partial. Merged #632 adds retained D1 storage-adapter proof, but no checklist row changes because token verification, runtime routes, authority, deployment/migration, and WP01 acceptance remain open. Account WP08 is 0/9 open for Rust-schema/account-authority proof, and WP06 is reopened at 14/18 to aggregate Account WP08 plus Cloudflare WP06/WP08 evidence. PR #607's TS adapter/D1-test-double is not a workpack closure. |
| `ai-plan` | 48 | 2 | 46 | 0 | Fully mapped from live source/tests. Eleven workpacks are Phase 1 complete for bounded code/test scope; 37 retain concrete production-code or expected-test gaps. WP01/WP02 now close the source-reconciliation/snapshot documentation work; the remaining checkbox split still does not describe implementation maturity. |
| `app-game-plan` | 88 | 54 | 34 | 0 | The remaining 34 are only `possibly done`; audit before implementation. |
| `app-plan` | 95 | 0 | 95 | 0 | Reconciliation rows overlap app/game heavily; deduplicate before delegation. |
| `browser-plan` | 24 | 0 | 24 | 0 | Fully mapped from live code/tests. Fourteen executable workpacks are Phase 1 complete for bounded scope; ten retain concrete product-code or expected-test gaps. Six additional imported packets are reference-only and excluded from the 24 execution rows. |
| `child-agent-runtime-distribution-plan` | 11 | 10 | 1 | 0 | Fully mapped from live code/tests. Five workpacks are Phase 1 complete for bounded scope; six retain concrete lifecycle/runtime/handoff/release-gate gaps despite ten index-level completion labels. |
| `cloudflare-control-plane-plan` | 13 | 0 | 13 | 0 | Fully mapped from live source/tests. WP00-WP02 and WP04 are Phase 1 complete for code/expected-test writing; no proof row is freshly reverified, and WP03 plus WP05-WP12 have concrete Phase 1 gaps. |
| `data-custody-storage-plan` | 8 | 7 | 1 | 0 | Workpack index and checklist disagree in both directions on several rows. |
| `device-trust-bootstrap-plan` | 9 | 0 | 9 | 0 | Fully mapped from live code/tests. WP08 is Phase 1 complete for its bounded research/test-only scope; WP01-WP07 and WP09 retain concrete production-integration or expected-test gaps, so no adapter-backed product closure is claimed. |
| `eventing-plan` | 5 | 3 | 2 | 0 | Five selectable workpacks: WP06 and WP10 are open; eight historical rows are excluded and must not be rescheduled. |
| `lan-plan` | 25 | 13 | 12 | 0 | Fully mapped from live code/tests. Twenty-two workpacks have no bounded Phase 1 writing gap; WP16, WP20, and WP25 remain incomplete. The 13 document-claimed closures and manual-proof labels are tracked separately from code/test readiness. |
| `logging-domain-parity` | 10 | 0 | 10 | 0 | Five partial-proof, four source-present, one audit-open. |
| `network-plan` | 8 | 0 | 8 | 0 | Fully mapped from live source/tests. WP05 and WP08 are Phase 1 complete for bounded gate/reference scope; WP01-WP04 and WP06-WP07 retain concrete canonical-contract, production-composition, live-runtime, or executable-harness gaps. No proof row is freshly reverified. |
| `parent-client-runtime-distribution-plan` | 11 | 7 | 4 | 0 | Fully mapped from live source/tests. WP01, WP04, WP05, and WP09 are Phase 1 complete for bounded route/package/smoke writing; WP02, WP03, WP06-WP08, WP10, and WP11 retain concrete gaps, so seven document-claimed closures are not product closure. |
| `payment-subscription-plan` | 13 | 3 | 10 | 0 | Fully mapped from live code/tests. WP00 is Phase 1 complete as a no-code handoff; WP01-WP12 retain concrete code or expected-test gaps, and the three doc-claimed closures do not match runtime truth. |
| `policy-control-plane-plan` | 8 | 6 | 2 | 0 | Six checked workpacks are not reflected by the generic checklist status. |
| `portal-ux-household-surfaces-plan` | 20 | 5 | 15 | 0 | Fully mapped from live code/tests. Nine workpacks have no Phase 1 writing gap in their bounded scope; eleven retain concrete product-code or expected-test gaps. The five doc-claimed closures are not used as implementation truth. |
| `remote-access-plan` | 6 | 0 | 6 | 0 | Five planned rows and one deferred control row. |
| `screen-ai-pipeline-plan` | 10 | 0 | 10 | 0 | Fully mapped from live code/tests. WP01 is complete for its no-code prerequisite scope; WP02-WP10 retain concrete production-composition, authority, durability, custody-negative, performance-test, or missing executable-harness gaps. Proof remains deferred until those Phase 1 gaps close. |
| `screen-plan` | 40 | 18 | 22 | 0 | Fully mapped from live code/tests: 9/40 executable workpacks are Phase 1 complete, 31 retain concrete code/test gaps, and three additional imported packets are reference-only. The 100/100 legacy checklist and 18 checked workpack labels overstate current runtime truth. |
| `setup-install-provisioning-plan` | 7 | 6 | 1 | 0 | Fully mapped from live code/tests. Six workpacks have no Phase 1 writing gap within their bounded ownership; WP07 remains incomplete because the current portal panel reports unavailable static state instead of implementing the live first-run state machine. The 93/93 checklist is not product completion. |
| `tracking-plan` | 39 | 0 | 39 | 0 | Internally checked rows were intentionally reopened for audit/proof reruns. |
| `v0-8-enforcement-control-plan` | 20 | 6 | 14 | 0 | Fully mapped from live source/tests. WP01-WP03 and WP07-WP09 are Phase 1 complete for bounded executable scope, and WP20 is coordination-only; 13 workpacks retain trusted-dispatch, lifecycle, aggregate-state, child/portal surface, live-integrity, platform-role, or executable-harness gaps. Historical checked boxes overstate WP18 because its declared umbrella verifier is absent. |
| **Total** | **526** | **143** | **383** | **0** | Plus 145 reference/source-only rows and 8 historical rows excluded from execution scheduling. |

### Acceptance state for each workpack

A workpack advances independently through `implemented -> focused gate green ->
retained proof regenerated -> checklist reconciled -> accepted commit -> merged ->
post-merge reverified`. Only the final state counts toward `Freshly reverified`.

## Crate to plan matrix

| Runtime owner | Plans unblocked or directly served | Priority reason |
| --- | --- | --- |
| `schema` + `agent-protocol` | every plan | Stable shared contracts prevent duplicate ownership and unblock service integration. |
| `ocentra-eventing` | custody, policy, tracking, LAN, enforcement, portal | Replay/idempotency/journal semantics are the common durability spine. |
| `family-identity-core` + device trust | account, setup, payment, remote, enforcement | Parent authority and device presence are required before sensitive actions. |
| `policy-control-core` | app/game, browser, LAN, network, screen, enforcement, portal | Policy is the shared decision producer for product controls. |
| `child-enforcement-core` + adapters | policy, browser, app/game, network, screen, enforcement | Converts approved policy into a reversible platform action. |
| `agent-service` | nearly every product plan | It joins feature crates into commands, read models, proof logs, and HostBridge state. |
| `storage-custody-core` | custody, tracking, screen, AI, account, payment | Retention/delete/export safety is a cross-cutting closure requirement. |
| `lan-core` | LAN, setup, remote, portal, child runtime | Device pairing and household transport unblock multi-device product proof. |
| `infra/cloudflare` + generated billing contracts | payment, account, setup, portal | Current source exists; focused worker/billing gates and stale proof reconciliation decide the next cloud packet. |
| `apps/portal` | portal, setup, policy, tracking, LAN, remote, reports | Presentation closes only after service read models and typed actions are available. |

## Dependency and unblock order

1. **Reconcile Cloudflare module dependencies, then run the proof-only WP07 successor**: resolve the current Wrangler/Workers-types module prerequisite first; from current source, rerun WP07's focused lint, unit, contract, and worker-boot gates and retain its bundle. Separately route Account WP08 Rust contract -> Cloudflare WP06 binding/migration -> Cloudflare WP08 module-runner proof -> Account WP06 aggregation. Only then schedule dependent payment/billing gates. Do not restore removed TypeScript contract ownership or revive #604.
2. **Build device trust runtime**: parent presence and sealed device trust unblock safe account, setup, payment, remote, and enforcement decisions.
3. **Close policy to enforcement in dependency order**: Eventing WP06's generic handoff is now complete; next is WP11's enforcement-specific durable-journal contract, then WP04 trusted dispatch -> adapter -> receipt -> rollback. Until WP11's actual handoff exists, WP04 remains unscheduled/manual-required. This becomes the reusable control path for browser, app/game, network, and screen.
4. **Use LAN/service as the first physical household proof**: pairing -> device state -> portal read model establishes the multi-device integration baseline.
5. **Close custody and observability on that vertical slice**: correlated logs, retention/delete, and replay make later feature work trustworthy.
6. **Scale feature producers**: browser, app/game, network, screen, tracking, and AI can then feed the same decision and evidence spine.
7. **Finish portal and distribution proof**: run real click-through and package/smoke proof only after the underlying product paths exist.

## Checklist synchronization rule

Every plan checklist should retain unchecked rows unless the named workpack has
all of the following: implementation, focused tests, retained proof artifact,
and an accepted merge state. A checklist audit may record current code evidence
and a blocker, but must never turn a code inventory into a completion claim.
