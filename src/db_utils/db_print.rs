use crate::Result;
use rusqlite::types::ValueRef;
use rusqlite::{Connection, Rows};
use std::str::from_utf8;
use tabled::builder::Builder;
use tabled::settings::Style;

pub fn print_table(conn: &Connection, table: &str) -> Result<()> {
	let sql = format!("SELECT * FROM {table}");
	let mut stmt = conn.prepare(&sql)?;
	let rows = stmt.query([])?;
	println!(" TABLE: {table}");
	print_rows(rows)?;

	Ok(())
}

pub fn print_select(conn: &Connection, sql: &str) -> Result<()> {
	let mut stmt = conn.prepare(sql)?;
	let rows = stmt.query([])?;

	print_rows(rows)?;

	Ok(())
}

pub fn print_rows(mut rows: Rows<'_>) -> Result<()> {
	let stmt = rows.as_ref().ok_or("no statements")?;
	let names: Vec<String> = stmt.column_names().into_iter().map(String::from).collect();

	let mut table_builder = Builder::new();
	table_builder.push_record(names.clone());

	while let Some(row) = rows.next()? {
		// -- Extract row cells
		let mut row_cells: Vec<String> = Vec::new();

		for (i, _k) in names.iter().enumerate() {
			// Extract cell as string
			let v = row.get_ref(i)?;
			let v = match v {
				ValueRef::Null => "NULL".to_string(),
				ValueRef::Integer(num) => format!("{num}"),
				ValueRef::Real(num) => format!("{num}"),
				ValueRef::Text(bytes) => format!("\"{}\"", from_utf8(bytes)?),
				ValueRef::Blob(blob) => format!("BLOB (length: {})", blob.len()),
			};

			// Push to the row
			row_cells.push(v);
		}

		// -- Add the row celles to the table builder
		table_builder.push_record(row_cells);
	}

	let table_content = table_builder.build().with(Style::modern()).to_string();

	println!("{table_content}");

	Ok(())
}
