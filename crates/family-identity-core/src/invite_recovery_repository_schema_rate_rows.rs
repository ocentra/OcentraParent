use rusqlite::Connection;

use super::hex_digest;

pub(super) fn validate(connection: &Connection, table: &str) -> Result<(), ()> {
    let mut statement = connection
        .prepare(&format!(
            "SELECT subject_digest, window_started_at_epoch_millis, attempt_count FROM {table}"
        ))
        .map_err(|_error| ())?;
    let mut rows = statement.query([]).map_err(|_error| ())?;
    while let Some(row) = rows.next().map_err(|_error| ())? {
        if !hex_digest(&row.get::<_, String>(0).map_err(|_error| ())?)
            || row.get::<_, i64>(1).map_err(|_error| ())? <= 0
            || row.get::<_, i64>(2).map_err(|_error| ())? < 0
        {
            return Err(());
        }
    }
    Ok(())
}
