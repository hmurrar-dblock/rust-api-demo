use sqlx::{SqlitePool, Pool, Sqlite, migrate};
use dotenv_codegen::dotenv;

pub async fn init_db_connection() -> Pool<Sqlite> {
    return SqlitePool::connect(dotenv!("DATABASE_URL"))
        .await
        .expect("failed to create DB pool");
}

pub async fn run_migrations(pool: &Pool<Sqlite>) {
    return migrate!("./migrations")
        .run(pool)
        .await
        .expect("failed to run migrations");
}
