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

	// -- Insert `number` in `string` column
	// OK in `strict` mode
	conn.execute("INSERT INTO person (name, yob) VALUES (?1, ?2)", ("Jen", &2000))?;

	// -- Select
	let mut stmt = conn.prepare("SELECT person.id, person.name, person.yob FROM person WHERE yob > :yob")?;
	let mut rows = stmt.query(&[(":yob", &1900)])?;

	while let Some(row) = rows.next()? {
		let name: String = row.get(1)?;
		println!("->> name: {name}");
		println!("->>  row: {row:?}");
	}

	// -- Pretty print table
	print_table(&conn, "person")?;

	Ok(())
}
