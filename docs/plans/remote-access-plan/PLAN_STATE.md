# Remote Access Plan State

Status: first-pass plan created because remote desktop/control and remote live view were scattered across screen, LAN, architecture, and roadmap docs.

Research status: incomplete. This plan requires a full follow-up research pass against existing screen capture, LAN transport, portal remote routes, local service capabilities, RustDesk comparison docs, and Sujan's privacy/control decisions before implementation claims.

Current truth:

- `screen-plan` can own capture primitives, but not the remote session product.
- `lan-plan` can own local transport, but not relay-backed remote access.
- Remote input/control is higher risk than remote viewing and must have separate authority, proof, and failure states.
- Remote access requires account/household/device authority before any session is opened.

Open gaps:

- No remote capability grant model.
- No remote session lifecycle with consent, expiry, revocation, and audit.
- No relay availability/fallback state machine.
- No proof matrix for remote viewing versus remote control.
- No retention/delete/export boundary for remote artifacts.
