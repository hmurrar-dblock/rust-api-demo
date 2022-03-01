use sqlx::{SqlitePool, Pool, Sqlite, migrate};
use dotenv_codegen::dotenv;

pub async fn init_db_connection() -> Pool<Sqlite> {
    let pool: Pool<Sqlite>;
    if cfg!(test) {
        pool = SqlitePool::connect("sqlite::memory:")
            .await
            .expect("failed to create DB pool");
    } else {
        pool = SqlitePool::connect(dotenv!("DATABASE_URL"))
            .await
            .expect("failed to create DB pool");
    }
    run_migrations(&pool).await;
    pool
}

async fn run_migrations(pool: &Pool<Sqlite>) {
    return migrate!()
        .run(pool)
        .await
        .expect("failed to run migrations");
}
