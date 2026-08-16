<!-- agent-capsule -->

> Agent Capsule
> Doc: Local And LAN Manual Proof Runbook
> Kind: architecture/reference documentation; read only when selected by plan route, source router, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

# Local And LAN Manual Proof Runbook

This runbook turns the cross-platform checkpoint plan into the concrete
local-device and LAN proof pass that should run before more AI or enforcement
work. It is a manual checkpoint, not an implementation plan. The goal is to
record what the current product path can prove, what still needs real hardware
or permissions, and what must stay labeled as scaffold-only or manual-required.

## Scope

Use this runbook for the V0.7 pre-AI review on a current `main` or reviewed
checkpoint branch. It covers:

- one local Windows child-device proof run;
- one local parent portal to child-agent proof run on loopback;
- one LAN parent-to-child smoke run across two devices or a clearly recorded
  substitute;
- evidence-backed dry-run preview checks that route through the Rust service,
  query store, local transport, and parent surface;
- installer, autostart, reboot, uninstall, and degraded-state notes where an
  installable artifact exists.

It does not prove V0.8 enforcement, blocking, timers, device-owner policy,
notification delivery, cloud relay, production signing, store distribution, or
real model execution. Those states must remain `scaffold-only`,
`manual-required`, `permission-required`, `not-yet-proven`, or `unavailable`
until the real product path exists and is exercised.

## Required Inputs

Read these before starting the run:

- `docs/architecture/cross-platform-deliverables-checkpoint.md`
- `docs/expectations/real-evidence-proof.md`
- `docs/expectations/pre-ai-proof-matrix.json`
- `docs/product-roadmap.md`

Record the run against the exact commit under test:

```powershell
git status --short --branch
git rev-parse HEAD
cmd /c npm run format:check
cmd /c npm run test:pre-ai-proof
```

Run `cmd /c npm run validate` before PR-ready or checkpoint-ready handoff. If a
manual device pass omits full validation, record the reason and require a clean
full gate before accepting the checkpoint.

## Run Metadata

Create one record per local or LAN run with:

- commit SHA and branch;
- package or app version;
- child-device OS, version, architecture, and device model;
- parent-device OS, version, architecture, and device model;
- install path and data/cache path;
- Rust service command, process id, and port;
- parent portal URL and WebSocket URL;
- loopback or LAN mode;
- local network IP range and firewall state for LAN mode;
- permission, signing, entitlement, autostart, and update state;
- screenshots or copied diagnostics with sensitive child details minimized.

Use synthetic family, parent, child, and activity names when possible. Do not
paste private browser history, screenshots, raw child activity, tokens, or
machine secrets into the proof record.

## Proof Labels

Use the same labels as the checkpoint plan:

| Label                 | Use when                                                                                                        |
| --------------------- | --------------------------------------------------------------------------------------------------------------- |
| `implemented`         | The current product path works on the named platform with direct evidence.                                      |
| `ci-mechanical-proof` | CI or local automation proves build, launch, package, transport, or storage mechanics only.                     |
| `manual-required`     | A real host, device, permission, signing credential, network, or store capability is required.                  |
| `permission-required` | The OS permission, entitlement, elevation, or firewall approval is missing or denied.                           |
| `degraded`            | The product reports a reduced fidelity state and names what is missing.                                         |
| `unavailable`         | The capability is not available on this platform or environment.                                                |
| `scaffold-only`       | The repo has package, app, API, or contract shape, but the real product behavior is not proved.                 |
| `not-yet-proven`      | Implementation may exist, but this run did not produce the required CI or manual proof.                         |
| `blocked`             | A named external dependency, device, credential, entitlement, permission, or implementation gap prevents proof. |
| `not-applicable`      | The platform or role should not support the deliverable.                                                        |

Do not upgrade a claim because the portal renders a success-shaped label. The
record must identify the service command, query, event, log, or persisted
evidence that backs the visible result.

## Baseline Local Gate

Run these from a clean checkout:

```powershell
git status --short --branch
cmd /c npm run format:check
cmd /c npm run test:pre-ai-proof
cmd /c npm run validate
```

Expected observations:

- the checkout is clean before manual evidence is captured;
- the pre-AI proof matrix accepts all required completed claim ids;
- validation launches the real Rust service for integration, LAN bind, and
  portal smoke checks;
- any failed command is copied exactly into the proof record.

Pass evidence:

- command output showing success;
- commit SHA for the successful run;
- note whether the run was local, CI, WSL, Docker, emulator, simulator, or
  physical hardware.

Fail or gap evidence:

- command and exact failure;
- whether the failure is local setup, product regression, missing device,
  missing permission, stale process, or unsupported platform;
- follow-up owner.

## Loopback Local Proof

Use loopback first to prove the parent surface can reach the child-agent service
without LAN or firewall variables.

Open two terminals in the same checkout. For worker lanes, use lane-specific
ports to avoid stale service interference:

```powershell
$env:OCENTRA_PARENT_AGENT_PORT = "4677"
$env:OCENTRA_PARENT_PORTAL_PORT = "4678"
cmd /c npm run dev:agent
```

```powershell
$env:OCENTRA_PARENT_AGENT_PORT = "4677"
$env:OCENTRA_PARENT_PORTAL_PORT = "4678"
cmd /c npm run dev:portal
```

For a default single-lane run, omit the two port overrides and use `4477` for
the Rust agent and `4478` for the portal.

Expected observations:

- the agent logs the selected bind address and allowed origins;
- `http://127.0.0.1:<agent-port>/health` returns the current service status;
- the portal opens at `http://127.0.0.1:<portal-port>/#/commands`;
- the portal WebSocket points at `ws://127.0.0.1:<agent-port>/api/dev/ws`;
- the parent surface renders data returned by the service path, not local-only
  fixture state.

Pass evidence:

- copied health payload or status output;
- service log snippet showing the request;
- portal screenshot or copied diagnostic output showing the same state;
- command output naming the port pair and commit.

Fail or gap evidence:

- occupied port details and whether the occupant was an Ocentra Parent process;
- stale service suspicion if the commit, log path, or port does not match;
- `permission-required` or `blocked` if Windows firewall, endpoint security, or
  service startup prevents the run.

## Local Evidence Preview Checks

Use the loopback service and portal to exercise each current pre-AI evidence
path. Each row must record the product path used, the expected observation, the
observed result, the proof label, and the artifact captured.

| Check                                   | Command or action                                                                                      | Expected observation                                                                                                | Do not count as proof                                                                                   |
| --------------------------------------- | ------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------- |
| Managed browser URL/tab evidence        | Launch the supported managed browser path, open a known test URL, request browser evidence.            | URL, title, normalized domain, managed profile state, and active-tab certainty or tab-list-only state.              | A personal browser process name, a URL typed into a fixture, or portal-only state.                      |
| Foreground process/window evidence      | Bring a known app to the foreground, request process/window activity.                                  | Real process/window identity, observation mode, timestamp, and unavailable/degraded state when no window exists.    | A manually inserted process row or hardcoded service reply.                                             |
| Network/domain evidence                 | Generate normal network activity, request network-flow evidence.                                       | Destination domain or endpoint summary, unknown attribution preserved when needed, and no decrypted payload data.   | Packet payload capture, decrypted HTTPS claims, or invented process-to-domain associations.             |
| App/game session evidence               | Run a known app or game long enough to create first/last-seen evidence, then request session evidence. | Duration or foreground time from stored evidence with source ids and unknown classification boundaries.             | Portal timers, AI guesses, or manually inserted session rows.                                           |
| Screen evidence queue state             | Enable the supported local setting if available, request screen queue state.                           | Permission, disabled, queued, summarized, deletion, and degraded states without raw screenshot exposure by default. | A screenshot dropped into a test folder or queue state not returned by the service/read-model path.     |
| Parent-rule context and dry-run preview | Create or select local parent-rule context, request dry-run policy preview.                            | Evidence-cited dry-run result, rule/context references, enforcement handoff disabled, and missing fields explicit.  | Enforcement action, remote AI result, hidden rule inference, or preview text without source references. |
| Local provider/runtime status           | Request runtime/provider status from the parent surface.                                               | Local-only, unavailable, disabled, degraded, or configured status with execution state and privacy boundary.        | Real model execution, remote fallback, or success-shaped model availability without a real local probe. |

If a path is not currently exposed through the parent surface, record the
closest typed service or diagnostic command used and label the parent-visible
surface as `not-yet-proven` or `scaffold-only`.

## LAN Smoke Proof

LAN proof must not be treated as V0.9 pairing proof until the pairing flow,
trusted-device registry, and authenticated control path exist. The current
manual run can prove LAN bind, origin allowlist, real service reachability, and
honest scaffold gaps for pairing.

Use two devices when available:

- child device: runs the Rust agent and portal dev server;
- parent device: opens the portal URL over the LAN and records the service
  response.

On the child device:

```powershell
$env:OCENTRA_PARENT_DEV_NETWORK = "lan"
$env:OCENTRA_PARENT_LAN_HOST = "<child-lan-ip>"
$env:OCENTRA_PARENT_AGENT_PORT = "4677"
$env:OCENTRA_PARENT_PORTAL_PORT = "4678"
cmd /c npm run dev:lan
```

On the parent device:

```powershell
curl.exe -i http://<child-lan-ip>:4677/health
```

Then open:

```text
http://<child-lan-ip>:4678/#/commands
```

Expected observations:

- the agent binds for LAN mode only when explicitly requested;
- allowed origins include the selected parent portal origin and not arbitrary
  unrelated origins;
- the parent device can load the portal over the selected LAN address;
- the portal talks to `ws://<child-lan-ip>:<agent-port>/api/dev/ws`;
- service responses include device status and capability state;
- pairing and remote control remain recorded as `scaffold-only` or
  `manual-required` until V0.9 pairing exists.

Negative checks:

- attempt a request from an unrelated origin or wrong port and record the
  rejection or connection failure;
- turn off the agent and confirm the portal shows a disconnected or degraded
  state rather than stale success;
- if firewall/router rules block the request, record firewall state and retry
  steps instead of changing the claim to implemented.

Pass evidence:

- child and parent OS/device names;
- LAN IP range and ports used;
- command output showing health or service status;
- service log snippet for the LAN request;
- parent screenshot or copied diagnostic output;
- explicit note that pairing is not proved by the current LAN smoke.

Fail or gap evidence:

- `manual-required` if only one machine was available;
- `permission-required` if firewall or OS prompts block access;
- `scaffold-only` if the product lacks pairing, trusted-device registry, or
  remote routing for the tested action;
- `blocked` only when a named dependency prevents proof after retry.

## Installer, Autostart, Reboot, And Uninstall Proof

Run this section only where an installable artifact exists for the platform.

Required actions:

- install from the package preview artifact;
- launch from the installed location;
- record service manager registration when claimed;
- stop and restart the app or service;
- reboot and record whether autostart behavior matches the package claim;
- uninstall and confirm process/service cleanup;
- record whether data is retained, removed, or user-controlled;
- record update behavior only where update scaffolding is wired.

Expected observations:

- package preview can prove scaffold mechanics;
- production signing, notarization, store distribution, TestFlight,
  device-owner, and entitlement claims remain unproven unless the real
  credential or device flow is used;
- missing service-manager, autostart, update, signing, or store behavior is
  recorded as `scaffold-only`, `manual-required`, `permission-required`, or
  `not-yet-proven`.

## Proof Record Template

Use this template for each run:

```text
Run name:
Commit:
Branch:
Package/app version:
Mode: loopback | LAN | package | reboot | uninstall
Child device:
Parent device:
Agent command:
Portal command:
Agent URL:
Portal URL:
Permission/signing/network state:
Action performed:
Expected observation:
Observed result:
Proof label:
Artifacts:
Sensitive details minimized:
Known gaps:
Follow-up owner:
```

## Acceptance Checklist

The local/LAN checkpoint is ready for coordinator review when:

- baseline validation and pre-AI proof commands are recorded;
- loopback parent-to-agent proof uses the real Rust service and parent surface;
- each current evidence-preview path has a pass, fail, or honest gap record;
- LAN proof records at least a real service reachability attempt and a negative
  or unavailable state;
- pairing is not overclaimed when only LAN dev smoke has run;
- package, autostart, reboot, update, and uninstall claims are labeled honestly;
- all screenshots or copied diagnostics minimize child-sensitive details;
- every omitted manual artifact has a reason and owner.

## Handoff Notes

When this runbook is used in a PR or checkpoint report, include:

- scope of the manual pass;
- touched files or proof artifacts;
- validation commands and exact results;
- platforms/devices used;
- proof labels for each local and LAN claim;
- known gaps and risks;
- whether the proof matrix should be updated.
