use std::{fs::File, io, path::Path};

use crate::{
    ndjson_operation::{append_operation_with_hooks, OperationMarkerKind},
    ndjson_operation_marker_publish::{write_marker, write_marker_with_fault, MarkerWriteFault},
};

pub(crate) fn append_operation_with_marker_fault(
    file: &mut File,
    path: &Path,
    operation_id: &str,
    record: &[u8],
    fault: OperationMarkerFault,
) -> io::Result<()> {
    let mut injected = false;
    append_operation_with_hooks(
        file,
        path,
        operation_id,
        record,
        |_| Ok(()),
        |path, content, kind| {
            if !injected && fault.kind() == kind {
                injected = true;
                write_marker_with_fault(path, content, fault.write_fault())
            } else {
                write_marker(path, content)
            }
        },
    )
}

#[derive(Clone, Copy)]
pub(crate) enum OperationMarkerFault {
    IntentWrite,
    IntentSync,
    CommitWrite,
    CommitSync,
}

impl OperationMarkerFault {
    fn kind(self) -> OperationMarkerKind {
        match self {
            Self::IntentWrite | Self::IntentSync => OperationMarkerKind::Intent,
            Self::CommitWrite | Self::CommitSync => OperationMarkerKind::Commit,
        }
    }

    fn write_fault(self) -> MarkerWriteFault {
        match self {
            Self::IntentWrite | Self::CommitWrite => MarkerWriteFault::Write,
            Self::IntentSync | Self::CommitSync => MarkerWriteFault::Sync,
        }
    }
}
