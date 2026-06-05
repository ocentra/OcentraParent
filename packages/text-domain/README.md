# @ocentra-parent/text-domain

Schema-backed display text tokens and shared copy values.

## Owns

- Shared text values that runtime surfaces need to render consistently.
- Dev portal copy tokens that should not live as loose app strings.
- Text schemas for copy that crosses packages or tests.
- Screen child disclosure UX copy tokens for disabled, paused, ready,
  capture-active, and protected-surface local disclosure states.

## Must Not Own

- Arbitrary page prose that is local to documentation.
- Policy ids, route ids, event names, or protocol values.
- Product decisions hidden inside wording.

## Flow

```mermaid
flowchart LR
  Text["text-domain token"]
  PortalDomain["portal-domain descriptors"]
  Portal["portal UI"]
  Text --> PortalDomain --> Portal
```

## Connected Docs

- [Contract expectations](../../docs/expectations/contracts.md)
- [Portal expectations](../../docs/expectations/portal.md)

## Gaps To Fill

- Expand only when text is reused or contract-visible.
- Keep parent-facing product language aligned with the README and constitution.
- Production child app, OS notification/tray/foreground overlay, and persisted
  disclosure state copy still need their runtime surfaces before being claimed.
