# WP07 Parent Protocol Event Contracts

Scope: define parent protocol event namespaces and typed contracts consumed by parent/controller, child-agent, network, AI, policy, enforcement, audit, portal, and read-model lanes.

Source rows: `05-implementation-workpacks.md` rows 42-50.

Read next:

- `../03-event-taxonomy-and-parent-integration.md`
- `../06-type-safety-validation-and-ownership.md`
- Owning consumer plan only when the selected contract belongs to that consumer

Expected outcome:

- Parent protocol event namespace constants and versioning rules exist before runtime consumption.
- Parent/controller, child-agent, network, AI, policy, enforcement, audit, portal, and read-model event contracts are typed and validated.
- Consumer plans retain business authority; this workpack only defines reusable event contract obligations and proof hooks.

Expected tests/proof:

- `eventing.parent-protocol.namespace-registry`
- `eventing.consumer-contract.serialization`
- `eventing.consumer-contract.version-skew`
- `eventing.consumer-contract.authority-negative`
- Proof links the owning consumer plan row for every product-specific event family.

Failure conditions:

- Do not let generic eventing own policy, enforcement, AI, or UI decisions.
- Do not add product event contracts without a consumer-plan owner.
- Do not claim runtime integration; WP08 and consumer plans own runtime proof.
