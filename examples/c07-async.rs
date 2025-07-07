use pretty_sqlite::print_table;
use rusqlite::Connection;
use xp_sqlite::db_utils::create_schema;

const DB_PATH: &str = "_my-db.db3";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	// -- Memory SQLite
	let conn = Connection::open(DB_PATH)?;

	// -- Create schema
	create_schema(&conn)?;

	let names = &["Jen", "Mike"];

	for name in names {
		for i in 1..10 {
			let name = format!("{name}-{i}");
			// NOTE: Using spawn here just to demonstrate Send/Sync restrictions.
			//       As mentioned in the video, for production code use a DB pool, queue, or mutex
			//       to manage concurrency.
			// Reddit: https://www.reddit.com/r/rust/comments/1kc8xuy/comment/mq2wj0p/
			let _res = tokio::task::spawn(async move {
				let conn = Connection::open(DB_PATH).map_err(|err| err.to_string())?;
				conn.execute(
					"INSERT INTO person (name, yob) 
				               VALUES (?1, ?2)",
					(name, &2000),
				)
				.map_err(|err| err.to_string())
			})
			.await?;
		}
	}

	// -- Final print
	let conn = Connection::open(DB_PATH)?;
	print_table(&conn, "person")?;

	Ok(())
}
