use rusqlite::Connection;
use xp_sqlite::db_utils::{create_schema, print_table};

const DB_PATH: &str = "_my-db.db3";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	// -- Memory SQLite
	let conn = Connection::open(DB_PATH)?;

	// -- Create schema
	create_schema(&conn)?;

	let names = &["Jen", "Mike"];

	for name in names {
		// tokio::task::spawn(async move {
		// -- Insert `number` in `string` column
		// OK in `strict` mode
		if let Err(err) = conn.execute("INSERT INTO person (name, yob) VALUES (?1, ?2)", (name, &2000)) {
			println!("->> Error while insert - {err}");
		}
		// })
		// .await;
	}

	// -- Debug print
	print_table(&conn, "person")?;

	Ok(())
}
