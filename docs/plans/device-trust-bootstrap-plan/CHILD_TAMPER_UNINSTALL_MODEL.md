# Child Tamper and Uninstall Model

This document defines the anti-tamper boundary for child devices.

## Rule

The child device cannot be the authority that removes its own trust. Any uninstall, tamper, or disable path must be parent-controlled or otherwise leave an auditable revocation trail.

## Acceptable signals

- Signed package integrity.
- Signed entitlement snapshot validity.
- Sealed local trust key presence.
- Runtime tamper evidence.
- Optional Android Play Integrity signal, if available.

## Response model

- On tamper detection, fail closed or degrade to a safe state.
- On uninstall attempt, do not claim universal blocking. Instead, require parent-authorized trust removal or a later revocation on next contact.
- On revocation, the stale child state must stop unlocking product behavior.
- On device compromise, support recovery must not pretend the device is still trusted.

## Non-goals

- No promise of perfect anti-root, anti-jailbreak, or anti-debugging magic.
- No antivirus claim.
- No custom device fingerprinting as a trust root.
- No child-side self-service trust reset.

## Negative cases

- A copied binary cannot self-authorize trust.
- A tampered package cannot silently become trusted.
- A child device cannot revoke its own parent relationship.
- A revoked device cannot keep using cached trust indefinitely.