use std::{fs, io, io::Write, path::Path};

use getrandom::fill;

use super::{hex, install_generation, Error};

pub(super) fn create(path: &Path) -> Result<String, Error> {
    let value = fresh_value()?;
    match create_file(path)? {
        Some(mut file) => {
            file.write_all(value.as_bytes())
                .map_err(|_error| Error::Io)?;
            file.sync_all().map_err(|_error| Error::Io)?;
            Ok(value)
        }
        None => path.parent().ok_or(Error::Io).and_then(install_generation),
    }
}

fn fresh_value() -> Result<String, Error> {
    let mut bytes = [0_u8; 32];
    fill(&mut bytes).map_err(|_error| Error::Platform)?;
    Ok(hex(bytes))
}

fn create_file(path: &Path) -> Result<Option<fs::File>, Error> {
    match fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
    {
        Ok(file) => Ok(Some(file)),
        Err(error) if error.kind() == io::ErrorKind::AlreadyExists => Ok(None),
        Err(_error) => Err(Error::Io),
    }
}
