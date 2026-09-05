use sha2::{Digest, Sha256};
use std::os::windows::fs::MetadataExt;
use std::{fs, io::Read, path::Path};

use super::super::{
    active_record,
    record::{binding, hex, Record},
    Error,
};

const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x0000_0400;

pub(super) fn is_active(path: &Path, generation: &str) -> Result<bool, Error> {
    if path
        .extension()
        .is_none_or(|extension| extension != "sealed")
    {
        return Ok(false);
    }
    validate_file_kind(path)?;
    let Some(encoded) = read_bounded(path)? else {
        return Ok(false);
    };
    let Ok(record) = serde_json::from_slice::<Record>(&encoded) else {
        return Ok(false);
    };
    let binding = binding([&record.family, &record.account, &record.device, generation])?;
    let expected_name = format!("{}.sealed", hex(Sha256::digest(&binding)));
    if path.file_name().and_then(|name| name.to_str()) != Some(expected_name.as_str()) {
        return Ok(false);
    }
    active_record::record_is_active(&record, &binding)
}

fn validate_file_kind(path: &Path) -> Result<(), Error> {
    let metadata = fs::symlink_metadata(path).map_err(|_error| Error::Io)?;
    if !metadata.is_file()
        || metadata.file_type().is_symlink()
        || metadata.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0
    {
        return Err(Error::Platform);
    }
    Ok(())
}

fn read_bounded(path: &Path) -> Result<Option<Vec<u8>>, Error> {
    let mut encoded = Vec::new();
    let read_result = fs::File::open(path).and_then(|mut file| {
        file.by_ref()
            .take(super::MAX_ACTIVE_RECORD_BYTES.saturating_add(1))
            .read_to_end(&mut encoded)
    });
    match read_result {
        Ok(_) if encoded.len() as u64 > super::MAX_ACTIVE_RECORD_BYTES => Err(Error::Platform),
        Ok(_) => Ok(Some(encoded)),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(_error) => Err(Error::Io),
    }
}
