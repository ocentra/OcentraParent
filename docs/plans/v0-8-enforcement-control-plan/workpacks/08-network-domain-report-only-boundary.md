# 08 Network/Domain Report-Only Boundary

Sources: [20-step plan](../v0-8-enforcement-control-20-step-plan.md),
[test blueprint](../v0-8-enforcement-control-test-blueprint.md), and
[folder README](../README.md).

## Where We Are

Network/domain observation exists as metadata evidence. Real network/domain
blocking adapter proof is still manual-required.

## Where We Want To Be

Network/domain states can support visibility and policy preview without implying
decrypted content, exact URL, or proved block capability.

## Requirement Checklist

- [ ] Keep report-only/manual-required states for block actions until adapter
      proof exists.
- [ ] Preserve process-attribution, domain-known, IP-only, and unknown states.
- [ ] Add proof checks that prevent claim upgrades.
- [ ] Show VPN/proxy/tunnel indicators only where evidence supports them.
- [ ] Keep network flow evidence separate from browser URL evidence.

## Acceptance And Proof

Network/domain proof output remains manual-required unless a named adapter test
and manual/CI artifact proves otherwise.

## Parallel Ownership Notes

This workpack should not block network visibility work, but it blocks network
blocking claims.
