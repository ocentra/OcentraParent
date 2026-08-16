# Session Pairing And Standing Access Model

Remote access is established by initial pairing and then remains standing until parent revokes access or removes the device.

No repeated permission prompts are part of the current model after pairing.

## Required grant fields

```text
request id
household
child profile
child device
selected route
requested capability
reason
pairing state
audit ref
child-visible disclosure state
parent confirmation state
revoke/remove-device state
```

## Required lifecycle

```text
requested
parentConfirmed
childDisclosed
paired
active
paused
stopped
revoked
removed
denied
failed
reconnectPending
superseded
```

## Required rules

- Pairing is the one-time authorization step for standing access.
- Parent authority is necessary for pairing and standing access.
- Child disclosure is part of pairing where applicable.
- Support/admin access must remain parent-visible and redacted.
- Reconnect must not revive revoked or removed grants.
- No repeated permission prompts are part of the standing-access model.
- Expiry is optional policy, not part of the default standing-access contract.

## Grant types

```text
view-only grant
live-view grant
screen snapshot grant
diagnostic grant
support-access grant
```

## Deferred grant types

```text
remote-input grant
remote-control grant
```

## Negative cases

```text
grant requires re-approval on every visit
reconnect resurrects revoked access
child-visible disclosure is omitted
support/admin bypasses parent visibility
wrong household or wrong device receives grant
parent removes device but access still works
```

## Proof expectation

The model is closed only when the proof inventory shows pairing, standing-access persistence, revoke/remove-device denial, disclosure, wrong-household denial, and support-access visibility evidence for the current pass.
