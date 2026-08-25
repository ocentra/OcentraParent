use rusqlite::Connection;

use super::hex_digest;

pub(super) fn validate(connection: &Connection, table: &str) -> Result<(), ()> {
    let mut statement = connection
        .prepare(&format!(
            "SELECT subject_digest, window_started_at_epoch_millis, attempt_count FROM {table}"
        ))
        .map_err(|_| ())?;
    let mut rows = statement.query([]).map_err(|_| ())?;
    while let Some(row) = rows.next().map_err(|_| ())? {
        if !hex_digest(&row.get::<_, String>(0).map_err(|_| ())?)
            || row.get::<_, i64>(1).map_err(|_| ())? <= 0
            || row.get::<_, i64>(2).map_err(|_| ())? < 0
        {
            return Err(());
        }
    }
    Ok(())
}
