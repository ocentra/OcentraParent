#[path = "read_model_project_boundary.rs"]
mod read_model_project_boundary;
#[path = "read_model_project_rows.rs"]
mod read_model_project_rows;

pub(super) use read_model_project_rows::{project_stored_row, StoredAppGameJournalRow};
