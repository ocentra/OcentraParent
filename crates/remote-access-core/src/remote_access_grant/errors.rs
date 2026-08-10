use std::fmt;

use super::RemoteAccessGrantError;

impl fmt::Display for RemoteAccessGrantError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = [
            "grant contains an empty field",
            "grant context belongs to another household",
            "grant context actor is not authorized",
            "grant context belongs to another child device",
            "current parent authority is required",
            "child disclosure is required",
            "support access requires an explicit parent grant",
            "grant transition is invalid for the current state",
            "serialized grant state violates lifecycle invariants",
            "grant reconnect is denied",
        ][*self as usize];
        formatter.write_str(message)
    }
}

impl std::error::Error for RemoteAccessGrantError {}
