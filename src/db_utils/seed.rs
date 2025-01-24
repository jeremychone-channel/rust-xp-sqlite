use crate::Result;
use rusqlite::Connection;

// region:    --- Create Tables

pub fn create_schema(conn: &Connection) -> Result<()> {
	conn.execute(
		"CREATE TABLE org (
            id    INTEGER PRIMARY KEY,
            name  TEXT NOT NULL
        ) STRICT",
		(), // empty list of parameters.
	)?;

	conn.execute(
		"CREATE TABLE person (
            id     INTEGER PRIMARY KEY,
						org_id INTEGER,
            name   TEXT NOT NULL,
						yob    INTEGER, -- year of birth
            data_t TEXT,
						data_b BLOB
        ) STRICT",
		(), // empty list of parameters.
	)?;

	Ok(())
}

// endregion: --- Create Tables
