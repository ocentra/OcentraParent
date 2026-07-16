# 15 Household Device Store

<!-- agent-capsule -->

> Agent Capsule
> Plan: `lan-plan`
> Doc: `15 Household Device Store`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

Sources: [20-step plan](../v0-9-lan-discovery-20-step-plan.md),
[test blueprint](../v0-9-lan-discovery-test-blueprint.md),
[UI/UX guide](../ui-ux-requirements-guide.md), and
[folder README](../README.md).

## Where We Are

The lane direction is to avoid duplicate truth: canonical household rows are
derived from LAN add-device state today. Production still needs durable device
registry behavior that Devices, Policy, Activity, Network, Tracking, AI, and
Account can all trust.

Current B-lane proof adds service-backed household rename/type persistence for
LAN-discovered neighbors. The portal sends canonical household decisions through
the Rust LAN service, receives `agent.lan-pairing.add-device.reported`, and
renders the updated add-device read model after refresh. This removes the
portal-only optimistic identity state for this path, but does not complete full
restart/recovery proof across every parent decision.

Current A-lane proof also persists prior LAN scan rows in the agent-service
sidecar JSON and reuses that history only as weak continuity truth. When a
later neighbor row is missing hostname/label/platform, the Rust LAN runtime may
hydrate those fields from the previous scan, but the add-device read model now
surfaces that explicitly as `previous-scan-snapshot` source/evidence instead of
silently upgrading it to confirmed truth. Paired/trusted child-device truth
still suppresses redundant service probing and outranks historical hints. The
current scan-smart slice also makes that stronger truth live in the neighbor
normalization and active-refresh paths: MAC-matched trusted registry rows now
hydrate hostname/platform/label before weak previous-scan history, and IP reuse
with a mismatched MAC no longer suppresses bounded active refresh or
service-identity probe checks. Stored child truth now suppresses an active
refresh only when the current neighbor table still confirms the same MAC at
that IP. Recent sidecar history for already observed child-app devices now
feeds a short-lived probe suppression list as scan input only; it does not
become current trusted state, paired state, or confirmed read-model truth by
itself.

That same sidecar now also records scan-plan metadata for audit and restart
continuity: session id, refresh mode, selected interface, local IPv4/CIDR,
default gateway, bounded target counts, timeout, paired-registry truth count,
recent previous-scan child-truth reuse count, durable household-truth reuse
count, and suppressed active-target list. Previous canonical household truth
that is still provable as paired, child-agent-backed, revoked/ignored, or
network-infrastructure state now feeds the Rust LAN active-refresh, passive
identity-hint, and service-probe suppression paths directly, so repeated scans
stop re-pinging and re-probing those devices while still reusing known
label/hostname/platform truth when a matching neighbor reappears.

Current A-lane proof now also persists canonical `knownHouseholdDevices` inside
the trusted-registry JSON and merges fresh scan output back into that same
store. The runtime keeps the same canonical device shape for storage and
read-model truth, merges evidence timestamp history instead of replacing it,
reuses that registry-backed known-device store for later scan suppression, and
restores those known devices into the add-device read model as stale restart
truth rather than silently forgetting them when a fresh scan has not yet
re-observed them.

## Where We Want To Be

The household device store is the canonical durable registry for known devices,
evidence, role badges, manual assignment, rename, trust, ignore, revoked,
stale, offline, unsupported, and manual-required states. Derived read models do
not own separate truth.

## Requirement Checklist

- [x] Persist device records, evidence, manual name, assigned child, trusted
  state, ignored state, revoked state, online state, first-seen, and
  last-seen. Current local proof covers known-household persistence, manual
  decision recovery, stale/offline restoration, and durable evidence merge
  timing.
- [x] Preserve parent decisions across rescan and restart. Current local proof
  covers rename/type readback, selected-route recovery, known-device restart
  recovery, and previous-scan continuity as weak evidence only.
- [x] Support migrations and safe fallback to unpaired state when registry proof
      is unavailable.
- [x] Keep routers and unsupported devices visible but non-enrollable.
- [x] Expose custody/source labels for local, LAN, cache, unavailable, and
      manual-required states. Previous-scan continuity is explicit in the LAN
      read model as a weak source label rather than hidden fallback state.

## Acceptance And Proof

- Store integration tests insert, update, reload, migrate, trust, ignore,
  revoke, mark stale/offline, and recover after restart.
- Rescan tests prove manual assignment and rename survive weak contradictory
  evidence.
- Product docs/checklist are updated only when proof status actually moves.

Current proof:

- `output/lan-plan-proof/15-household-device-store/01-local-validation.md`
- Focused Rust proof: `cargo test -p ocentra-lan-core network_inventory -- --nocapture`
- Focused Rust proof:
  `cargo test -p ocentra-parent-agent-core trusted_device_registry -- --nocapture`
- Focused Rust proof:
  `cargo test -p ocentra-parent-agent-service lan_pairing_browser_runtime -- --nocapture`
- Restart/runtime proof now explicitly covers stale and offline read-model
  restoration from `knownHouseholdDevices`, migration and fail-closed trusted
  registry recovery, router-visible/non-enrollable handling, and
  scan-suppression truth for stored child-agent, paired, and router devices.

## Parallel Ownership Notes

Storage workers must coordinate closely with merge and read-model workers. Do
not introduce a second "canonical" table or portal-only registry.
