# 08 Platform Adapter Abstraction

## Target State

Capture adapter interface exists for Windows, macOS, Linux, Android, iOS, and fake/dev adapters with capability and proof tiers.

## Current State

Platform implementation proof is open.

## Checklist

- [ ] Define adapter ID and platform contract.
- [ ] Define capability probe.
- [ ] Define capture request/result.
- [ ] Define protected/unavailable result.
- [ ] Define proof tier.
- [ ] Ensure fake/dev adapter cannot be product proof.

## Proof

- Adapter contract tests.
- Proof tier mapping in platform deep dive.
