use rusqlite::Connection;
use xp_sqlite::db_utils::{create_schema, print_rows, print_select, print_table};

fn main() -> Result<(), Box<dyn std::error::Error>> {
	println!("Hello, world!");
	// -- Memory SQLite
	let conn = Connection::open_in_memory()?;

	// -- Create schema
	create_schema(&conn)?;

	// -- Insert `number` in `string` column
	// OK in `strict` mode
	// create the org
	let mut stmt = conn.prepare("INSERT INTO org (name) VALUES (?1) RETURNING id")?;
	let org_id = stmt.query_row(("ACME, Inc",), |r| r.get::<_, i64>(0))?;

	let names = &["Jen", "Mike", "Paul", "Pierre"];
	for (idx, name) in names.iter().enumerate() {
		let org_id = if idx % 2 == 0 { Some(org_id) } else { None };
		conn.execute(
			"INSERT INTO person (name, org_id, yob) VALUES (?1, ?2, ?3)",
			(name, &org_id, &2000),
		)?;
	}

	// -- Select
	let query = "
    SELECT 
        person.id,
        person.name,
        person.yob,
        org.name as org_name
    FROM 
        person
    INNER JOIN 
        org ON person.org_id = org.id
		WHERE
				org.id = :org_id
";

	let mut stmt = conn.prepare(query)?;
	let mut rows = stmt.query(&[(":org_id", &org_id)])?;

	print_rows(rows)?;
	// print_select(&conn, query)?;

	Ok(())
}
