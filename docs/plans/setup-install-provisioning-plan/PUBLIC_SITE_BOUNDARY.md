# Public Site Boundary

## Scope

`family.ocentra.ca` is a public information and entry surface. It may route parents into account creation/login, invite/code entry, download help, support, privacy, status, and setup help.

It must not become the child-activity store or a hidden telemetry collector.

## Allowed public routes

- `/`
- `/download`
- `/install-help`
- `/register`
- `/login`
- `/invite`
- `/privacy`
- `/data-custody`
- `/support`
- `/status`
- `/release-notes`

## Allowed entry states

- normal public entry
- invite-link entry
- manual code entry
- QR/deep-link entry
- expired invite state
- revoked invite state
- wrong-household state
- already-used state
- manual-required state

## Allowed data collection

- account registration data
- login/session data
- support contact data
- download request metadata
- anonymous operational telemetry if explicitly approved
- consent state if analytics exists

## Forbidden public data

- child activity
- child evidence
- screenshots
- browser URLs
- device logs
- parent rules
- child profile data
- raw install logs with secrets
- pairing codes
- provider tokens

## Copy rules

Use copy such as:

- "Download Windows preview"
- "Production installer not ready"
- "Sign in to create a household"
- "Child device pairing happens after install"
- "Some platform capabilities require manual proof"

Do not say:

- "Fully protects your child"
- "Works on all devices"
- "Remote control ready"
- "Stores nothing"
- "Production ready"
- "Install complete = protected"

unless proof exists.

## Handoff rule

The public site can collect account/contact data only through explicit account flows. It must hand off child-device setup to the owning account, package, LAN, or portal plan at the exact boundary where ownership changes.
