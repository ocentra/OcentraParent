# Research And UI Guidance

## Purpose

This document tells future agents how to research, model, and write setup/install/provisioning work without producing vague first-run docs.

Do not write implementation code here. This is platform guidance, first-run journey guidance, UI expectation guidance, proof guidance, and no-claim guidance.

## Research anchors

Use current official docs for:

- Cloudflare Pages and Workers
- custom domains and preview deployments
- Vite deploy guidance
- Windows Installer / MSI and service install or uninstall
- code signing, notarization, and store review
- Linux package formats and systemd
- Android parent app and child agent install constraints
- iOS parent app and child agent entitlement constraints
- OAuth native app browser flow, PKCE, deep links, and invite / recovery token handling
- OWASP authentication, session, and authorization guidance
- secret scan, least privilege, and no child-data collection

## Core product rule

The public site is not the product-data owner.

`family.ocentra.ca` may be:

- public information site
- download entry
- account/login route
- support/privacy/status route
- installer help route

It must not be:

- child activity dashboard
- child evidence store
- hidden telemetry collector
- raw setup log collector
- parent rules store
- child data warehouse

## First-run mental model

Setup is a state machine, not a checklist.

Correct sequence:

public site visit
-> invite/code entry
-> account/login handoff
-> household create/join
-> parent bootstrap install
-> child bootstrap install
-> child agent running
-> permission/disclosure check
-> pairing
-> device trust
-> readiness evaluation
-> policy/data/report handoff

Incorrect sequence:

download clicked -> setup ready
login success -> setup ready
child profile created -> child device protected
LAN discovery row -> child device trusted
installer exists -> platform supported
portal page renders -> product setup complete

## Bootstrap correction

Do not model parent install as a direct static download only.

Do not model child install as a direct static download only.

The parent bootstrap code and child pairing bootstrap code are separate authority tokens.

## UI guidance

The first-run UI must show:

- public site / invite entry
- sign in / create account
- create or join household
- parent install link / QR / code
- parent bootstrap tutorial / agreement
- parent bootstrap code entry
- parent package download / install progress
- parent portal guided setup start
- create child profile
- generate child pairing link / QR / code
- child install instructions
- waiting for child device
- child detected / confirm trust
- permission readiness checklist
- policy baseline setup
- data custody status
- setup complete / setup blocked / manual required

The UI must not imply that installed means trusted or trusted means policy-ready.

## Mobile guidance

Never claim mobile support as one thing.

Split:

- parent Android app
- parent iOS app
- child Android agent
- child iOS agent
- Android package preview
- iOS package preview
- Play store signing
- TestFlight / App Store signing
- child mobile entitlement / device proof

## Desktop guidance

Windows is the likely first child-agent proof target.

Windows child setup must show:

- agent installed
- service registered
- service running
- loopback / LAN reachable
- permissions / capabilities
- pairing / trust
- revocation / reinstall
- manual-required gaps

macOS and Linux must stay manual-required until packaging, service/runtime, permission, and signing/notarization/distro proof exist.
