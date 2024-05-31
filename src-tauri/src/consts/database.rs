use lazy_static::lazy_static;

pub const DATABASE_FOLDER: &str = "databases";
pub const DATABASE_FILE: &str = "data.db";
lazy_static! {
    pub static ref DATABASE_PATH: String = format!("{}/{}", DATABASE_FOLDER, DATABASE_FILE);
    pub static ref DATABASE_URL: String = format!("sqlite://{}?mode=rwc", DATABASE_PATH.clone());
}
