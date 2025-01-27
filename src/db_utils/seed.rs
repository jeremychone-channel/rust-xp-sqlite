use crate::Result;
use rusqlite::Connection;

// region:    --- Create Tables

pub fn create_schema(conn: &Connection) -> Result<()> {
	conn.execute(
		"CREATE TABLE IF NOT EXISTS org  (
            id    INTEGER PRIMARY KEY,
            name  TEXT NOT NULL
        ) STRICT",
		(), // empty list of parameters.
	)?;
	conn.execute("DELETE FROM org", ())?;

	conn.execute(
		"CREATE TABLE IF NOT EXISTS person (
            id     INTEGER PRIMARY KEY,
						org_id INTEGER,
            name   TEXT NOT NULL,
						yob    INTEGER, -- year of birth
            data_t TEXT,
						data_b BLOB
        ) STRICT",
		(), // empty list of parameters.
	)?;
	conn.execute("DELETE FROM person", ())?;

	Ok(())
}

// endregion: --- Create Tables
