# tracking-plan Instruction

## Verdict

`partial / false-green`. WP33 is false-green; WP34-WP39 are open; current closure proof is red.

## Assign first

`tracking-wp33-closure-precondition-repair`:

- fix the schema/import crash in `packages/agent-protocol-domain/src/network-runtime-events.ts` path imported by tracking retention command;
- migrate remaining WP33 notification/provider proof wrappers from `parent-domain` to `tracking-domain`;
- clear architecture debt in tracking-domain generated catalog and Rust tracking re-export surfaces where touched;
- regenerate missing WP33 artifacts and closure/gap-map proof.

## Then

1. WP34 tracking event contracts/protocol constants.
2. WP35 parent config command/event flow.
3. WP36 detection cascade event flow.
4. WP37 journal/replay/projection.
5. WP38 notification/escalation event flow.
6. WP39 portal event read-model proof.

## Coordinate with

- `data-custody-storage-plan` for retention/export/delete custody substrate.
- `eventing-plan` for event flow patterns.
- `screen-plan` because screen should follow tracking for shared retention/event surfaces.

## Do not

- Do not count stale WP33 checked status.
- Do not count `parent-domain` tracking wrappers as ownership.
- Do not claim Apple-host proof locally.
