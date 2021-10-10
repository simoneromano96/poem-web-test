use sea_orm::{Database, DatabaseConnection};
use tokio::sync::OnceCell;

pub static DB: OnceCell<DatabaseConnection> = OnceCell::const_new();

pub async fn get_db() -> &'static DatabaseConnection {
    DB.get_or_init(|| async {
        let db = Database::connect("postgres://poem_test:poem_test@localhost/poem_test")
            .await
            .expect("Connection to the database failed");
        db
    })
    .await
}
