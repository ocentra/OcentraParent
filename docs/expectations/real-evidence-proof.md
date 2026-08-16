<!-- agent-capsule -->

> Agent Capsule
> Doc: Real Evidence Proof Expectations
> Kind: expectation/acceptance documentation; read only when selected by feature doc, plan route, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# Real Evidence Proof Expectations

Runtime claims are valid only when they are proved through the real product
path. A green test that bypasses the Rust agent, local transport, evidence
store, or parent UI does not prove the product works.

## Outcome

- The Rust agent or service launches for every runtime proof.
- The parent portal sends typed requests over the real local transport.
- Rust replies from real OS/runtime capture or real persisted state created by
  app code.
- The parent UI renders the returned state after validating the service payload.
- CI proves repeatable mechanics. Real machines and devices prove privileged
  OS/device capabilities.

## Valid Setup

The proof runner may create temporary runtime resources so the app can run
safely and deterministically:

- temporary data directories;
- temporary SQLite paths;
- temporary encrypted journal paths;
- temporary journal keys;
- explicit dev ports;
- test-only parent/child identities that still pass real contract parsing.

Those resources are setup only. They do not replace capture, storage, service,
or portal behavior.

## Not Valid Proof

These patterns do not count as proof for a completed runtime claim:

- manually inserted database rows counted as product capture;
- WebSocket or service responses replaced by test-only handlers;
- hardcoded Rust replies that avoid the real service/read-model path;
- portal-local state shown as if it came from the child-device agent;
- MSW, Nock, Sinon, `vi.mock`, `vi.fn`, or equivalent replacement paths;
- screenshots, browser URLs, network flows, app sessions, or policy decisions
  invented directly in test code.

## CI Proof

CI should prove every mechanic that hosted runners can exercise honestly:

- TypeScript contracts build and reject invalid payloads.
- Rust protocol parity compiles and tests.
- The Rust service launches.
- The parent portal talks to the real Rust service over local transport.
- SQLite and encrypted journal code paths write, replay, rebuild, and query
  through product code.
- Package previews build and install or launch-smoke on each available runner.
- Platform adapters report unavailable, disabled, degraded, or permission
  required states honestly when hosted CI cannot grant the real capability.

CI must not claim that a privileged OS capability works merely because the app
can render a success-shaped result.

## Real Machine Proof

Real host or device checks are required before a privileged capability is called
fully working:

- Windows/macOS screen capture with user-granted permissions.
- macOS Screen Recording, Accessibility, launchd, signing, or notarization
  behavior.
- Android UsageStats, foreground service, VPN/DNS, accessibility, device-owner,
  or managed-profile behavior.
- iOS Family Controls, DeviceActivity, Screen Time, Network Extension,
  notification, background execution, signing, or entitlement behavior.
- Household LAN discovery and parent/child pairing across actual devices.
- Existing browser profile or tab behavior outside the managed-browser boundary.
- Long-running app/game monitoring on a real child machine.
- Network observation that needs administrator, kernel, driver, VPN, or packet
  visibility privileges.

Each real-machine proof record should include the commit SHA, platform, package
or app version, permission state, command or UI action, Rust/service log snippet,
parent UI screenshot or copied diagnostic output, and observed result.

Local multi-service LAN proof is valid for repeatable service mechanics such as
typed discovery states, pairing, route selection, controller lease rejection,
trusted-registry restart recovery, and LAN AI provider routing states. It is not
valid proof of household router discovery, firewall prompts, or physical
parent/child device behavior unless the record also includes distinct device
identities, LAN addresses, reachability artifacts, and the manual two-device
commands used.

## Done Signal

A runtime claim is done only when the proof matrix names the claim, maps it to
CI and manual proof levels, and records honest platform coverage. If CI cannot
exercise a privileged capability, the matrix must say `manual-required`,
`scaffold-only`, or `not-yet-proven` instead of presenting fake certainty.

## V0.7 Checkpoint Scenarios

The pre-AI checkpoint also carries explicit scenario records in
`pre-ai-proof-matrix.json`. These records are not feature claims by themselves;
they are the runbook and gate metadata that separates real product proof from
scaffold, read-model, or packaging-only proof.

Required checkpoint scenarios cover:

- managed-browser exact URL, title, domain, managed profile, and active-tab or
  tab-list-only proof;
- foreground process/window proof on a real Windows host;
- network/domain attribution proof with unknown and degraded states preserved;
- app/game duration proof from stored evidence, not portal timers or AI guesses;
- screen evidence queue proof with permission and deletion-state visibility;
- localhost/LAN smoke using the real Rust service and managed ports;
- package preview, installed-service autostart, signing, store, notarization,
  device-owner, and entitlement gaps.

Each scenario must name the claims it supports, the real product path, CI
commands that prove repeatable mechanics, required manual artifacts where
privileged OS behavior is involved, known gaps, and patterns that do not count
as proof.
