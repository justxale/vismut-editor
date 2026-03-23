use sea_orm::{Database, DatabaseConnection};
use crate::config::EnvConfig;

#[derive(Clone)]
pub struct VismutState {
    connection: DatabaseConnection,
    pub env: EnvConfig
}

impl VismutState {
    pub async fn new() -> Self {
        let env = EnvConfig::new();
        let path = format!("sqlite://{}?mode=rwc", env.get_db_path());
        let connection = Database::connect(path).await.unwrap();
        connection
            .get_schema_registry("vismut_editor::database")
            .sync(&connection)
            .await
            .unwrap();
        VismutState { connection, env }
    }
}
