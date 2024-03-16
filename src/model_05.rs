use modql::field::{Fields, SeaFieldValue};
use modql::{FromSqliteRow, FromSqliteValue};

#[derive(Debug, SeaFieldValue, FromSqliteValue)]
pub enum Model {
	Gpt3,
	Gpt4,
}

#[derive(Debug, Fields, FromSqliteRow)]
pub struct Agent {
	pub id: i32,
	pub name: String,
	pub model: Option<Model>,
	pub level: Option<i64>,
	pub data_t: Option<serde_json::Value>,
	pub data_b: Option<Vec<u8>>,
}

#[derive(Debug, Fields, Default)]
pub struct AgentForCreate {
	pub name: String,
	pub level: Option<i64>,
	pub data_t: Option<serde_json::Value>,
	pub data_b: Option<Vec<u8>>,
}
