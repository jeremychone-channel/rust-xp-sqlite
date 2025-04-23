use pretty_sqlite::print_table;
use rusqlite::Connection;
use xp_sqlite::db_utils::create_schema;

const DB_PATH: &str = "_my-db.db3";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	// -- Create schema
	let conn = Connection::open(DB_PATH)?;
	create_schema(&conn)?;

	let names = &["Jen", "Mike"];

	for name in names {
		for i in 1..10 {
			let name = format!("{name}-{i}");
			let _res = tokio::task::spawn(async move {
				let conn = Connection::open(DB_PATH).map_err(|err| err.to_string())?;
				conn.execute("INSERT INTO person (name, yob) VALUES (?1, ?2)", (name, &2000))
					.map_err(|err| err.to_string())
			})
			.await??;
		}
	}

	// -- Debug print
	let conn = Connection::open(DB_PATH)?;
	print_table(&conn, "person")?;

	Ok(())
}
