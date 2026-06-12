# WP09 Network Consumer Event Chain

Scope: align network consumer use of reusable eventing with AI, policy, enforcement, audit, and read-model authority boundaries.

Source rows: `05-implementation-workpacks.md` rows 57-62.

Read next:

- `../../network-plan/AGENTS.md` after identifying exact network workpack
- `../05-implementation-workpacks.md` rows 57-62 only
- `../TEST_PROOF_EXPECTATIONS.md`

Expected outcome:

- Network plan consumes the reusable eventing crate instead of a private bus when implementation reaches this boundary.
- Network event chain distinguishes observation, classification, policy preview, enforcement command, adapter result, audit, and read-model events.
- Weak network evidence and AI classification cannot publish enforcement commands.
- Proof artifacts link back to both eventing and network plan state.

Expected tests/proof:

- `eventing.network-consumer.chain-contract`
- `eventing.network-consumer.weak-evidence-negative`
- `eventing.network-consumer.ai-cannot-enforce`
- `eventing.network-consumer.policy-authority-required`
- `eventing.network-consumer.proof-linkage`
- Proof includes network-plan workpack, event family manifest, and denied-authority cases.

Failure conditions:

- Do not mark this complete from eventing-only tests.
- Do not claim live network capture, DNS/firewall enforcement, or child delivery from reusable eventing.
- Do not let AI or network observations become enforcement without policy/enforcement consumer proof.
