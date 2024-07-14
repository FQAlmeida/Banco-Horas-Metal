use lazy_static::lazy_static;

#[cfg(target_os = "android")]
pub const DATA_FOLDER: &str = "/data/data/banco.horas.metal.fqa.dev";
#[cfg(not(target_os = "android"))]
pub const DATA_FOLDER: &str = ".";

pub const DATABASE_FILE: &str = "data.db";
lazy_static! {
    pub static ref DATABASE_FOLDER: String = format!("{}/databases", DATA_FOLDER);
    pub static ref DATABASE_PATH: String = format!("{}/{}", DATABASE_FOLDER.clone(), DATABASE_FILE);
    pub static ref DATABASE_URL: String = format!("sqlite://{}?mode=rwc", DATABASE_PATH.clone());
}
