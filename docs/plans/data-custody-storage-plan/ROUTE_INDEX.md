<!-- agent-capsule -->

> Agent Capsule
> Plan: `data-custody-storage-plan`
> Doc: `ROUTE_INDEX.md`
> Kind: local route map.
> Read when: When deciding whether this plan owns the task or an adjacent plan owns the handoff.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Route changes require PLAN_INDEX and FEATURE_ROUTE_INDEX updates.

<!-- /agent-capsule -->

# Data Custody Storage Plan Route Index

## Owns

- Data custody guarantees, data taxonomy, encrypted storage policy, key-custody model, provider matrix, bundle protocol, event model, retention, export/import, sync, deletion and tombstones, no-stolen-data boundaries, cloud or relay custody, report or query custody, parent storage settings/apply flow, and custody proof gates.
- Custody policy/proof/rules for storage, sync, export/import/restore, report/query, assistant citation, deletion/tombstone, and parent-owned provider choices.
- No-claim boundaries for Ocentra-hosted fallback storage, raw child evidence upload, readable provider payloads, restore resurrection, and derived report/query/assistant outputs.

## Boundary split

```text
eventing-plan owns event bus implementation, journal/replay/runtime bus behavior, and eventing crate evolution.
portal-ux-household-surfaces-plan owns parent-visible projection/UI and interaction surfaces.
account-identity-family-plan owns actor, household, role, guardian/admin/support authority.
device-trust-bootstrap-plan owns device key/trust material and trusted-device state.
cloudflare-control-plane-plan owns Cloudflare Worker/backend runtime and storage bindings, but must consume this plan's custody handoffs for retention/export/delete policy.
payment-subscription-plan owns billing/payment semantics and must not infer custody readiness from payment storage.
setup-install-provisioning-plan owns setup journey and install/bootstrap surfaces.
remote-access-plan, LAN, notification, report producers, and AI plans own their own runtime behavior and consume custody rules by typed handoff.
```

## Does Not Own

- Adjacent implementation completion in: eventing-plan, portal-ux-household-surfaces-plan, parent-client-runtime-distribution-plan, account-identity-family-plan, payment-subscription-plan, device-trust-bootstrap-plan, cloudflare-control-plane-plan, setup-install-provisioning-plan, remote-access-plan, LAN, notification, report producers, or AI plans.
- Broad source rewrites without selected workpack proof.
- Event bus implementation, portal shell UX, account/provider authority, payment semantics, Cloudflare deployment, remote transport, setup journey, notification delivery, report rendering, or AI runtime behavior.
- Release or production claims outside this plan's evidence.

## Handoff Rule

Open an adjacent plan only after the selected workpack records the exact handoff reason, owner path, expected proof, and no-claim boundary.
