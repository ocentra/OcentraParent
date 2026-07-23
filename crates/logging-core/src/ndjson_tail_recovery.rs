use std::{
    fs::File,
    io::{self, Read, Seek, SeekFrom},
};

const SCAN_CHUNK_BYTES: usize = 8 * 1024;

pub(crate) fn recover_partial_tail(file: &mut File) -> io::Result<()> {
    let length = file.seek(SeekFrom::End(0))?;
    if tail_is_complete(file, length)? {
        file.seek(SeekFrom::End(0))?;
        return Ok(());
    }

    let retained = find_last_complete_record_end(file, length - 1)?;
    file.set_len(retained)?;
    file.seek(SeekFrom::Start(retained))?;
    file.sync_data()
}

pub(crate) fn has_complete_tail(file: &mut File) -> io::Result<bool> {
    let length = file.seek(SeekFrom::End(0))?;
    let complete = tail_is_complete(file, length)?;
    file.seek(SeekFrom::End(0))?;
    Ok(complete)
}

fn tail_is_complete(file: &mut File, length: u64) -> io::Result<bool> {
    Ok(length == 0 || last_byte_is_newline(file)?)
}

fn last_byte_is_newline(file: &mut File) -> io::Result<bool> {
    file.seek(SeekFrom::End(-1))?;
    let mut last = [0_u8; 1];
    file.read_exact(&mut last)?;
    Ok(last[0] == b'\n')
}

fn find_last_complete_record_end(file: &mut File, mut cursor: u64) -> io::Result<u64> {
    let mut buffer = [0_u8; SCAN_CHUNK_BYTES];
    while cursor > 0 {
        let chunk_len = usize::try_from(cursor.min(SCAN_CHUNK_BYTES as u64))
            .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
        let chunk_start = cursor - chunk_len as u64;
        file.seek(SeekFrom::Start(chunk_start))?;
        file.read_exact(&mut buffer[..chunk_len])?;
        if let Some(index) = buffer[..chunk_len].iter().rposition(|byte| *byte == b'\n') {
            return Ok(chunk_start + index as u64 + 1);
        }
        cursor = chunk_start;
    }
    Ok(0)
}
