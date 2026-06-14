# Remote Access Research And UI Guidance

This document tells future agents how to research, model, and write remote-access work without collapsing it into local capture, LAN pairing, or vague remote-desktop claims.

Do not write implementation code here. This is architecture guidance, platform guidance, UI guidance, proof guidance, and no-claim guidance.

## Primary research anchors

Use official docs and the local research snapshots when updating this plan.

### Remote access reference

- RustDesk product site: <https://rustdesk.com/>
- RustDesk client: <https://github.com/rustdesk/rustdesk>
- RustDesk server: <https://github.com/rustdesk/rustdesk-server>
- RustDesk self-host docs:
  <https://rustdesk.com/docs/en/self-host/rustdesk-server-oss/>
- RustDesk client configuration docs:
  <https://rustdesk.com/docs/en/self-host/client-configuration/>

### Local research snapshots

- `C:\Users\sujan\.codex\research\rustdesk`
- `C:\Users\sujan\.codex\research\rustdesk-server`
- `C:\Users\sujan\.codex\research\rustdesk-server-demo`

### Platform / product anchors

- `screen-plan` for capture primitives, protected surfaces, and local screen custody.
- `lan-plan` for local pairing and LAN transport.
- `account-identity-family-plan` for parent, child, co-parent, support/admin, and device authority.
- `data-custody-storage-plan` for retention/export/delete custody.
- `portal-ux-household-surfaces-plan` for rendered parent surfaces once the remote model exists.

## RustDesk carry-forward

RustDesk is useful because it already solves a related class of problems: device identity, rendezvous, LAN discovery, NAT traversal, relay fallback, remote screen transport, session permissions, platform-specific service installation, and self-hostable infrastructure.

Borrow these ideas:

- direct-first route selection with relay fallback;
- separate rendezvous/control-plane responsibilities from relay forwarding;
- visible route state to the product UI;
- session-scoped access rather than one hidden or repeated approval loop;
- controlled-device visible state, including parent identity, paired state, active capability, stop/revoke, and platform permission state where applicable;
- platform-specific service lifecycle and permission proof;
- parent-owned or self-host relay as an advanced custody option;
- notifications for exact denial or degraded reasons;
- forced relay mode for deterministic proof harnesses.

Do not carry forward:

- unattended permanent password as the default trust model;
- hidden privacy-mode behavior as a first product path;
- clipboard, file transfer, terminal, restart, or elevation in the current pass;
- any fallback to weaker or ambiguous route/session authority;
- any Ocentra-hosted default storage of child activity evidence;
- repeated permission prompts after a device is paired.

## Platform guidance

### Windows

- long-running service lifecycle matters;
- screen capture and visibility state are separate problems;
- secure desktop and privilege boundaries must be visible in proof;
- service state, permissions, and stop/revoke behavior must be explicit.

### Android

- MediaProjection, Accessibility, foreground service, and DevicePolicyManager matter;
- child-visible paired state must be explicit;
- background restrictions and notification/action delivery limits must be modeled.

### Apple

- FamilyControls, ManagedSettings, Screen Time authorization, and Network Extension where relevant matter;
- entitlement/device proof limits must remain honest;
- parent-visible access and child-visible stop/revoke state must be modeled.

### Browser / web

- browser state is a consumer of remote-state, not the source of truth;
- route/session/access decisions must not be hidden inside browser widgets.

## UI guidance

The parent should see:

```text
selected child and device
device health and heartbeat
route kind and custody state
paired access and active capabilities
what is blocked by platform or permission
what can be done now
what happened last
```

Use product words, not protocol words:

- "Paired" instead of a generic approval state.
- "Reachable at home" instead of raw route labels.
- "Remote relay" instead of transport jargon.
- "View only" and "Deferred control" as separate visible states.
- "Waiting for child device" instead of a generic queue state.
- "Screen permission needed on child device" instead of a generic failure.

The UI should be calm and decisive. It should expose route, pairing, and custody state, not transport trivia or repeated permission churn.
