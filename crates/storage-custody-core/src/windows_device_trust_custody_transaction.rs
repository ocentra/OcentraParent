use std::path::Path;

use super::{
    commitment, platform,
    record::{remove, write},
    snapshot::PreviousActiveRecord,
    Error,
};

pub(super) fn finish(
    outcome: Result<(), Error>,
    binding: &[u8],
    record_path: &Path,
    previous: &Option<PreviousActiveRecord>,
) -> Result<(), Error> {
    match outcome {
        Ok(()) => Ok(()),
        Err(error) => {
            restore_or_clear(binding, record_path, previous);
            Err(error)
        }
    }
}

fn restore_or_clear(binding: &[u8], record_path: &Path, previous: &Option<PreviousActiveRecord>) {
    let _cleanup_result = platform::remove(binding);
    if let Some(previous) = previous {
        let _restore_result = write(record_path, &previous.record);
        let _restore_result = commitment::write(binding, record_path);
        let _restore_result = platform::activate(binding, &previous.epoch);
        return;
    }
    let _cleanup_result = remove(record_path);
}
