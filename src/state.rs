use std::sync::{Arc};
use sea_orm::{Database, DatabaseConnection};
use vismut_core::ExecutionEnvironment;
use crate::config::EnvConfig;

#[derive(Clone)]
pub struct VismutState {
    connection: DatabaseConnection,
    env: Arc<EnvConfig>,
    executor: Arc<ExecutionEnvironment>
}


impl VismutState {
    pub async fn new() -> Self {
        let env = Arc::new(EnvConfig::new());
        let mut executor = ExecutionEnvironment::default();
        executor.get_schema_mut();
        let connection = Database::connect(
            format!("sqlite://{}?mode=rwc", env.get_db_path())
        ).await.unwrap();
        connection
            .get_schema_registry("vismut_editor::database")
            .sync(&connection)
            .await
            .unwrap();
        VismutState { connection, env, executor: Arc::new(executor) }
    }
    
    pub fn get_connection(&self) -> &DatabaseConnection {
        &self.connection
    }
    
    pub fn get_env(&self) -> &EnvConfig { &self.env }
    pub fn get_executor(&self) -> &ExecutionEnvironment {
        &self.executor
    }
}
