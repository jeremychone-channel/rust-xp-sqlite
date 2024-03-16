use modql::field::HasSeaFields;
use modql::FromSqliteRow as _;
use rusqlite::Connection;
use sea_query::{Iden, IntoIden, Order, Query, SqliteQueryBuilder};
use sea_query_rusqlite::RusqliteBinder;
use serde_json::json;
use xp_sqlite::db_utils::create_schema;
use xp_sqlite::model_05::{Agent, AgentForCreate};
use xp_sqlite::Result;

#[derive(Iden)]
enum AgentIden {
	#[iden = "agent"]
	Table,
	Id,
}

fn main() -> Result<()> {
	// -- Memory SQLite
	let conn = Connection::open_in_memory()?;
	create_schema(&conn)?;

	// -- Inserts
	for i in 1..=3 {
		let data_t = json!({
			"name": "Some Object",
			"subObject": {
				"num": 123
			}
		});

		let agent = AgentForCreate {
			name: format!("buddy-{:02}", i),
			level: Some(111),
			data_t: Some(data_t),
			..Default::default()
		};

		let (columns, values) = agent.not_none_sea_fields().for_sea_insert();

		let mut query = Query::insert();
		let query = query.into_table(AgentIden::Table).columns(columns).values(values)?;
		let (sql, values) = query.build_rusqlite(SqliteQueryBuilder);
		conn.execute(&sql, &*values.as_params())?;
	}

	// -- Build the Query with sea-query
	let mut query = Query::select();
	let query = query
		.from(AgentIden::Table)
		.columns(Agent::sea_idens())
		.order_by(AgentIden::Id.into_iden(), Order::Asc);

	// -- Execute the query with rusqlite
	let (sql, values) = query.build_rusqlite(SqliteQueryBuilder);

	println!("Select statement: {sql}");
	let mut stmt = conn.prepare(&sql)?;
	let iter = stmt.query_and_then(&*values.as_params(), Agent::from_sqlite_row)?;

	println!("\nResult:\n");
	for agent in iter {
		println!("{:?}", agent);
	}
	Ok(())
}
