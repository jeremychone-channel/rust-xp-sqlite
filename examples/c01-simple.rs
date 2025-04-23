use pretty_sqlite::print_table;
use rusqlite::Connection;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	// -- Memory SQLite
	let conn = Connection::open_in_memory()?;

	// -- Create schema
	conn.execute(
		"CREATE TABLE person (
            id    INTEGER PRIMARY KEY,
            name  TEXT NOT NULL,
						yob   INTEGER, -- year of birth
            data  BLOB
        ) STRICT",
		(), // empty list of parameters.
	)?;
	// Note: `id INTEGER PRIMARY KEY AUTOINCREMENT` for no rowid reuse

	// -- Insert
	// OK in `strict` mode
	conn.execute(
		"INSERT INTO   person (name, yob) 
	          VALUES (?1, ?2)",
		("Jen", 2020),
	)?;

	// -- Select
	let select_sql = "SELECT person.id, person.name, person.yob 
										FROM   person 
	                  WHERE  yob > :yob";
	let mut stmt = conn.prepare(select_sql)?;
	let mut rows = stmt.query(&[(":yob", &1900)])?;

	while let Some(row) = rows.next()? {
		let name: String = row.get(1)?;
		println!("->> name: {name}");
		println!("->>  row: {row:?}");
	}

	print_table(&conn, "person")?;

	Ok(())
}
