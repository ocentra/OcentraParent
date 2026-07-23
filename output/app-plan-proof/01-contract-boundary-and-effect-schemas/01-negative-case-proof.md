# Negative Case Proof

The schema-domain edge decoder rejects:

- display-name fields in place of opaque app aggregate identifiers;
- aggregate identifiers without the Rust-owned `app.aggregate.` prefix;
- runtime-decision identifiers without a non-empty suffix;
- inventory-only input that claims foreground runtime evidence;
- AI-required input that publishes policy directly;
- manual-required input that publishes policy.

The Rust contract test rejects noncanonical aggregate/runtime-decision prefixes
and empty suffixes. The Rust invariant matrix keeps inventory-only
classifications at `record-inventory`, including when a foreground observation
is available.

No mocks, stubs, or source-string contract matching are used. Rust serde output
is compared with the shared JSON golden, and the TypeScript edge decoder consumes
that same golden.
