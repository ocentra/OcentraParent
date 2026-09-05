# App/Game Windows Local Policy FFI

This crate is the narrow Windows owner boundary for reduced local-policy
observations used by App/Game proof status. It invokes only the trusted inbox
Windows PowerShell binary under a fixed script, deadline, environment, and
output schema.

The public observation contains booleans and bounded counts only. It does not
export rule XML, executable paths, publisher rules, user or device identities,
provider delivery, adapter dispatch, enforcement, rollback, or audit custody.
Non-Windows targets return `UnsupportedPlatform` without spawning a process.
