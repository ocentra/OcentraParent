# V0.5 Location AI Safety Analysis Plan

<!-- agent-capsule -->

> Agent Capsule
> Plan: `tracking-plan`
> Doc: `V0.5 Location AI Safety Analysis Plan`
> Kind: plan reference document; read only when routed by AGENTS, DOC_INDEX, or workpack.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

AI is useful in tracking only after structured evidence exists. This plan keeps
AI as evidence, not authority.

## Rule

```text
AI is evidence, not authority.
AI can say "this evidence suggests concern."
AI cannot say "the child is in danger" from one weak sample.
Policy decides parent notifications and actions.
Parents decide acknowledgement and exceptions.
```

## Inputs

AI receives:

- location evidence;
- device status;
- geofence transition;
- expected-place context;
- nearby-place evidence;
- parent exceptions;
- recent history summary;
- schedule;
- parent rules;
- custody labels;
- model runtime refs;
- prompt/template version.

AI must not receive:

- raw unbounded child activity by default;
- direct OS scanning access;
- emergency-contact authority;
- default remote/API uploads;
- location history without custody and retention policy;
- inferred precise GPS from LAN/IP/network metadata.

## Outputs

- `where_expected`
- `not_where_expected`
- `near_sensitive_place`
- `possible_emergency`
- `device_stale_or_offline`
- `location_ambiguous`
- `normal_exception_active`
- `unknown`

## Risk Levels

- `none`
- `low`
- `medium`
- `high`
- `critical`

Risk level is AI evidence. It is not final alert severity until policy compiles
it with parent rules, capability state, exceptions, and acknowledgement
requirements.

## Recommended Policy Inputs

- `observe_candidate`
- `notify_parent_candidate`
- `ask_child_checkin_candidate`
- `ask_parent_ack_candidate`
- `escalate_candidate`
- `unknown_candidate`

The suffix is intentional. AI recommends candidates. Parent policy emits final
actions.

## Hard Boundaries

AI cannot:

- call emergency services;
- accuse child;
- decide final alert;
- override parent exception;
- suppress configured critical alert;
- turn low-accuracy nearby POI into exact place claim.

## Copy Rules

Allowed parent copy:

```text
The child device was reported near a cinema area during school time.
Confidence: medium.
Expected place: school.
Recommended action: notify parent or ask for acknowledgement.
```

Rejected copy:

```text
The child is inside a bar.
The child is in danger.
The child is skipping school.
The child is with bad friends.
The child was kidnapped.
AI thinks the child is lying.
```

## AI Runtime Modes

| Mode                   | Allowed behavior                                                                                |
| ---------------------- | ----------------------------------------------------------------------------------------------- |
| Child-device local     | Default safety path when local model/runtime proof exists.                                      |
| Parent local           | Parent-owned explanation mode; not a child-device enforcement path.                             |
| Family AI hub local    | Optional LAN/family-owned runtime with explicit custody and availability state.                 |
| Parent-approved remote | Optional, explicit parent action with data-custody review and no default child-activity upload. |
| Metadata-only/no-AI    | Deterministic policy runs without AI.                                                           |
| Unavailable            | Return degraded/unknown; do not block evidence capture or deterministic policy.                 |

## Tests

AI tests must prove:

- valid AI input accepted;
- missing location evidence rejected for tasks that require it;
- missing expected-place context allowed only for nearby-place task;
- missing source refs rejected;
- confidence stays `0..1`;
- low confidence maps to unknown candidate;
- ambiguous location maps `location_ambiguous`;
- accusation language is rejected;
- AI cannot final-notify parent;
- AI cannot call emergency services;
- AI cannot override explicit parent rule or acknowledgement;
- remote AI is disabled by default.
