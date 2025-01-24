use rusqlite::Connection;
use serde_json::json;
use xp_sqlite::db_utils::{create_schema, print_table};

fn main() -> Result<(), Box<dyn std::error::Error>> {
	// -- Memory SQLite
	let conn = Connection::open_in_memory()?;

	// -- Create schema
	create_schema(&conn)?;

	// -- Insert `number` in `string` column
	let data = &[("Jen", 94114), ("Mike", 94115)];
	let mut ids: Vec<i64> = Vec::new();
	for (name, zip) in data {
		let data_json = json!({
			"address": "San Francisco",
			"zip": zip
		});

		let mut stmt = conn.prepare("INSERT INTO person (name, yob, data_t) VALUES (?1, ?2, ?3) RETURNING id")?;
		let person_id = stmt.query_row((name, &2000, data_json.to_string()), |r| r.get::<_, i64>(0))?;
		ids.push(person_id);
	}

	let person_1_id = ids.first().ok_or("Should have at least one person")?;
	conn.execute(
		r#"UPDATE person SET data_t = 
						json_set(data_t, 
							'$.zip', ?2,
							'$.home_owner', json(?3)
						) 
						WHERE id = ?1"#,
		(&person_1_id, &94222, true.to_string()),
	)?;

	// -- Select
	let mut stmt = conn.prepare("SELECT person.id, person.name, person.yob FROM person WHERE yob > :yob")?;
	let mut rows = stmt.query(&[(":yob", &1900)])?;

	// -- Pretty print table
	print_table(&conn, "person")?;

	Ok(())
}
