use pretty_sqlite::print_table;
use rusqlite::types::Value;
use rusqlite::{Connection, ToSql};
use xp_sqlite::db_utils::create_schema;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	// -- Memory SQLite
	let conn = Connection::open_in_memory()?;

	// -- Create schema
	create_schema(&conn)?;

	// -- Seed users
	let names = &["Jen", "Mike", "Paul", "Pierre"];
	for name in names.iter() {
		let org_id: Option<i64> = None;
		conn.execute(
			"INSERT INTO person (name, org_id, yob) 
			             VALUES (?1, ?2, ?3)",
			(name, &org_id, &2000),
		)?;
	}

	// -- updated

	// building the cols
	let nv_list: Vec<(String, Value)> = vec![
		//
		("org_id".to_string(), Value::Integer(123)),
		("name".to_string(), Value::Text("New name 111".to_string())),
	];
	let (cols, vals): (Vec<String>, Vec<Value>) = nv_list.into_iter().unzip();

	let cols = cols.iter().map(|col| format!("\"{}\" = ?", col)).collect::<Vec<_>>().join(", ");

	let sql = format!("UPDATE person SET {cols}");
	let mut values: Vec<&dyn ToSql> = vals.iter().map(|x| x as &dyn ToSql).collect();

	// Build the where clause
	let sql = format!("{sql} where id = ?");
	let person_id = Value::Integer(1);
	values.push(&person_id);

	let num_of_rows = conn.execute(&sql, &*values)?;
	println!("number of rows updated: {num_of_rows}");

	print_table(&conn, "person")?;

	Ok(())
}
