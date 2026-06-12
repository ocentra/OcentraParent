# Account Identity Family Plan State

Status: first-pass plan created because login/user/household authority was not owned by a dedicated plan.

Research status: incomplete. This plan requires a full follow-up research pass against existing portal-domain, parent-domain, local API, agent protocol, games Cloudflare/Firebase auth, current official provider docs, and Sujan's account/privacy choices before implementation claims.

Current direction:

- Cloudflare-first app and custody architecture.
- Firebase Auth may be evaluated as a pragmatic identity provider/token issuer, not as the family product data store.
- Cloudflare D1/Durable Objects/R2/KV roles must be deliberate: D1 for relational account/household state, Durable Objects for live coordination, R2 for encrypted blobs/artifacts, KV for non-authoritative cache/rate limits.
- Cloudflare Access is not a consumer family identity product by itself.

Open gaps:

- No provider decision record.
- No household role/device authority model.
- No session/token lifecycle proof matrix.
- No invite/recovery/delete/transfer state machine.
- No cross-family authorization test inventory.
