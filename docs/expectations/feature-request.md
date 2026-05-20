# Feature Request Expectations

Every feature request should make these expectations explicit before implementation starts. The AI agent may choose the implementation path, but these expectations define what the feature must satisfy.

## Required Expectations

- Parent outcome: what parent-visible problem is solved.
- Child-device outcome: what changes on the child device, if anything.
- Platform scope: Windows, macOS, Linux, Android, iOS, web portal, Cloudflare, or shared domain only.
- Data scope: what data is observed, stored, queried, synced, or displayed.
- Trust boundary: local-only, LAN, parent-owned storage, cloud-routed,
  authenticated parent, unauthenticated dev, notification provider, stateless
  compile, or installer/update path.
- Contract boundary: TypeScript domain, Rust protocol, service intent/event, portal UI, release asset, or external provider.
- Delivery boundary: feature branch, final PR to `main`, or explicit product release request.
- Success evidence: exact behavior that proves the feature works.
- Failure behavior: what happens when permissions, OS APIs, network, storage, cloud, provider, or model calls fail.
- Non-goals: what the feature must not claim or implement yet.
- Validation gate: focused tests during development and final gate before merge.

## Preferred Expectations

- Example event or payload shape.
- Example parent-facing copy or UI state, owned by text/domain packages.
- Security/privacy notes.
- Performance or concurrency expectations.
- Rollback/update considerations if release code changes.

## Done Signal

A feature request is ready for implementation when an agent can name the relevant expectation files, the product claim, the real behavior to prove, the branch/release boundary, and the validation gate without guessing.
