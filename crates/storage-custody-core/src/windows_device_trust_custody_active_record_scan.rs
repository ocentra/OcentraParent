use std::{fs, path::Path};

use super::{
    active_record,
    record::{binding, Record},
    Error,
};

pub(super) fn any_present(root: &Path, generation: &str) -> Result<bool, Error> {
    let entries = fs::read_dir(root).map_err(|_error| Error::Io)?;
    for entry in entries {
        let entry = entry.map_err(|_error| Error::Io)?;
        let path = entry.path();
        if path
            .extension()
            .is_none_or(|extension| extension != "sealed")
        {
            continue;
        }
        let encoded = match fs::read(&path) {
            Ok(encoded) => encoded,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => continue,
            Err(_error) => return Err(Error::Io),
        };
        let record = match serde_json::from_slice::<Record>(&encoded) {
            Ok(record) => record,
            Err(_error) => continue,
        };
        let binding = binding([&record.family, &record.account, &record.device, generation])?;
        if active_record::record_is_active(&record, &binding)? {
            return Ok(true);
        }
    }
    Ok(false)
}
