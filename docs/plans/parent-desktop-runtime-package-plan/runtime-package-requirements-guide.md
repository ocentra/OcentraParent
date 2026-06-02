# Runtime Package Requirements Guide

This guide defines D-lane package/runtime requirements. It is not an
implementation claim.

## Main Runtime Rule

The parent desktop app is a parent-owned shell. It may display and route typed
state. It must not capture child-device activity, run local AI, evaluate policy,
execute enforcement, run timers, or silently store child activity outside the
documented custody model.

## Required States

- Local service available/unavailable.
- LAN route available/stale/offline.
- Relay unavailable or future-enabled.
- Parent cache available/unavailable.
- Parent-owned storage available/unavailable.
- Parent controller lease.
- Parent observer read-only.
- Package preview/signed/store/notarized/manual-required.
- Update/rollback scaffold/available/unavailable.
- Support diagnostics available/redacted/unavailable.

## Platform Claim Requirements

Every package/runtime report must split:

- parent desktop shell;
- child Windows agent;
- parent Android app;
- child Android agent;
- parent iOS app;
- child iOS agent;
- macOS/Linux package preview;
- signing/notarization/store;
- relay/cloud;
- support/privacy/account surfaces.

## Release Requirements

- `main` is a preview/CI branch.
- Production publishing belongs to explicit promotion.
- Unsigned preview artifacts must be labeled as preview.
- Store/signing/notarization claims require real artifact proof.
- Package smoke does not prove capture, enforcement, mobile child-agent, or
  remote relay behavior.

## Support Requirements

Support output should include version, commit, branch, platform, package state,
service URL/state, selected route, source/custody labels, and recent non-private
diagnostic ids. It must redact secrets, tokens, private child activity, raw
journals, raw SQLite contents, and unnecessary private filesystem paths.
