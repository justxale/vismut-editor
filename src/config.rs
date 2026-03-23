#[derive(Clone)]
pub struct EnvConfig {
    host: String,
    port: String,
    database_path: String,
}

impl EnvConfig {
    pub fn new() -> Self {
        match dotenvy::dotenv() {
            Ok(_) => Self {
                host: std::env::var("VISMUT_HOST").unwrap_or(String::from("0.0.0.0")),
                port: std::env::var("VISMUT_PORT").unwrap_or(String::from("11811")),
                database_path: std::env::var("VISMUT_DATABASE_PATH").unwrap_or(EnvConfig::get_default_database_path()),
            },
            Err(_) => EnvConfig::default()
        }
    }

    fn default() -> Self {
        Self {
            host: String::from("0.0.0.0"),
            port: String::from("11811"),
            database_path: EnvConfig::get_default_database_path()
        }
    }

    pub fn get_default_database_path() -> String {
        std::env::current_dir().unwrap().join("./vismut.sqlite").into_os_string().into_string().unwrap()
    }

    pub fn get_host(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
    pub fn get_db_path(&self) -> &String {
        &self.database_path
    }
}