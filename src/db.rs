use crate::Result;
use rusqlite::types::Value;
use rusqlite::{Connection, Params, Row, Rows, Statement};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct Db {
	/// If None, then, in memory
	/// Otherwise path of the file e.g., `./my-db.db3`
	path: Option<Arc<str>>,
	conn: Arc<Mutex<Connection>>,
}

/// Constructors
impl Db {
	/// Create a new Db instance
	pub fn new(path: impl Into<Arc<str>>) -> Result<Self> {
		let path = path.into();
		let conn = Connection::open(path.as_ref())?;
		Ok(Self {
			path: Some(path),
			conn: Arc::new(Mutex::new(conn)),
		})
	}

	pub fn from_memory() -> Self {
		let conn = Connection::open_in_memory().unwrap();
		Self {
			path: None,
			conn: Arc::new(Mutex::new(conn)),
		}
	}
}

/// Executors
impl Db {
	pub async fn execute(&self, sql: &str, params: impl Params) -> Result<usize> {
		let conn = self.conn.lock().map_err(|err| err.to_string())?;
		let num = conn.execute(sql, params)?;
		Ok(num)
	}

	/// Assume the sql will end with `RETURNING _some_i64_column_`
	pub async fn execute_with_running_i64(&self, sql: &str, params: impl Params) -> Result<i64> {
		let conn = self.conn.lock().map_err(|err| err.to_string())?;
		let mut stmt = conn.prepare(sql)?;
		let org_id = stmt.query_row(params, |r| r.get::<_, i64>(0))?;

		Ok(org_id)
	}

	pub async fn query(&self, sql: &str, params: impl Params) -> Result<Vec<Vec<Field>>> {
		let conn = self.conn.lock().map_err(|err| err.to_string())?;
		let mut statement = conn.prepare(sql)?;
		let rows = statement.query(params)?;

		let mut owned_rows: Vec<Vec<Field>> = Vec::new();

		// Ok(rows)
		todo!()
	}
}

pub struct Field {
	name: String,
	value: Value,
}
