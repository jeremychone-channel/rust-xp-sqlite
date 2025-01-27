use rusqlite::Connection;
use xp_sqlite::db::Db;
use xp_sqlite::db_utils::{create_schema, print_table};

const DB_PATH: &str = "_my-db.db3";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	// -- Create schema
	let conn = Connection::open(DB_PATH)?;
	create_schema(&conn)?;

	let db = Db::new(DB_PATH)?;

	let names = &["Jen", "Mike"];
	let mut ids: Vec<i64> = Vec::new();

	for name in names {
		for i in 1..10 {
			let name = format!("{name}-{i}");
			let db = db.clone();
			let id = tokio::task::spawn(async move {
				// let conn = Connection::open(DB_PATH).map_err(|err| err.to_string())?;
				db.execute_with_running_i64(
					"INSERT INTO person (name, yob) VALUES (?1, ?2) RETURNING id",
					(name, &2000),
				)
				.await
				.map_err(|err| err.to_string())
			})
			.await??;
			ids.push(id);
		}
	}

	// -- Debug print
	let conn = Connection::open(DB_PATH)?;
	print_table(&conn, "person")?;

	Ok(())
}
