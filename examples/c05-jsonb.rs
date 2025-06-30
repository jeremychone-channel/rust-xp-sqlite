use pretty_sqlite::print_rows;
use rusqlite::Connection;
use serde_json::json;
use xp_sqlite::db_utils::create_schema;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	// -- Memory SQLite
	let conn = Connection::open_in_memory()?;

	// -- Create schema
	create_schema(&conn)?;

	// -- Insert `number` in `string` column
	let data = &[("Jen", 94114), ("Mike", 94115)];
	let mut ids: Vec<i64> = Vec::new();
	for (name, zip) in data {
		let data_json = json!({ "address": {
			"city": "San Francisco",
			"zip": zip
		}});

		let mut stmt = conn.prepare(
			"
		INSERT INTO person (name, yob, data_b) 
		            VALUES (?1, ?2, jsonb(?3)) RETURNING id
",
		)?;
		let person_id = stmt.query_row((name, &2000, data_json.to_string()), |r| r.get::<_, i64>(0))?;
		ids.push(person_id);
	}

	let person_1_id = ids.first().ok_or("Should have at least one person")?;
	conn.execute(
		r#"UPDATE person SET data_b =
						jsonb_set(data_b,
							'$.address.zip', ?2,
							'$.address.home_owner', json(?3)
						)
						WHERE id = ?1"#,
		(&person_1_id, &94222, true.to_string()),
	)?;

	// -- Select home owner = true only
	println!("== People owning homes:");
	// Note: using the `->>` notation to get the value back
	//       (get sqlite type for primitive)
	// Note: If use `->` then it will be `= json(:ho)`
	let mut stmt = conn.prepare(
		"
		SELECT id, name, yob, json(data_b) FROM person WHERE
           data_b ->> '$.address.home_owner' = :ho			 
		",
	)?;

	// Note: true below will be set at 1 by rusqlite ToValue impl
	let rows = stmt.query(&[(":ho", &true)])?;
	print_rows(rows)?;

	// -- Select not home owners
	println!("== People NOT owning homes:");
	// Note: Using the jsonb_extract,
	//       to get the value back (get sqlite type for primitive)
	let mut stmt = conn.prepare(
		"SELECT name, yob, json(data_b) FROM person
		 WHERE jsonb_extract(data_b, '$.address.home_owner') IS NULL
		    OR jsonb_extract(data_b, '$.address.home_owner') = 0
	",
	)?;
	let rows = stmt.query(())?;
	print_rows(rows)?;

	Ok(())
}
