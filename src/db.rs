use crate::Result;
use rusqlite::types::Value;
use rusqlite::Connection;
use std::sync::{Arc, Mutex};

pub struct Db {
	/// If None, then, in memory
	/// Otherwise path of the file e.g., `./my-db.db3`
	path: Option<String>,
	conn: Arc<Mutex<Connection>>,
}

/// Constructors
impl Db {
	/// Create a new Db instance
	pub fn new(path: impl Into<String>) -> Result<Self> {
		let path = path.into();
		let conn = Connection::open(&path)?;
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
	pub async fn execute(&self, sql: &str) -> Result<usize> {
		let conn = self.conn.lock().map_err(|err| err.to_string())?;
		let num = conn.execute(sql, ())?;
		Ok(num)
	}

	/// Assume the sql will end with `RETURNING _some_i64_column_`
	pub async fn execute_with_running_i64(&self) -> Result<i64> {
		todo!()
	}

	pub async fn query(sql: &str, params: ()) -> Result<Vec<Field>> {
		todo!()
	}
}

pub struct Field {
	name: String,
	value: Value,
}
