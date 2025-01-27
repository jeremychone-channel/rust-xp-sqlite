// region:    --- Modules

pub mod db;
pub mod db_utils;

pub type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

// endregion: --- Modules
