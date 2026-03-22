use sea_orm::{Database, DatabaseConnection};

#[derive(Clone)]
pub struct VismutState {
    connection: DatabaseConnection,
}

impl VismutState {
    pub async fn new() -> Self {
        let path = format!(
            "sqlite://{}?mode=rwc",
            std::env::var("VISMUT_DB_PATH").unwrap_or(
                std::env::current_dir()
                    .unwrap()
                    .join("./vismut.sqlite")
                    .into_os_string()
                    .into_string()
                    .unwrap()
            )
        );
        let connection = Database::connect(path).await.unwrap();
        connection
            .get_schema_registry("vismut_editor::database")
            .sync(&connection)
            .await
            .unwrap();
        VismutState { connection }
    }
}
