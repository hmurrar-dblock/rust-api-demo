use actix_web::web::{Data};
use sqlx::{Pool, Sqlite, query, Error};
use sqlx::sqlite::{SqliteRow};

pub type QueryResults = Result<Vec<SqliteRow>, Error>;
pub type QueryResult = Result<SqliteRow, Error>;


pub async fn get_all(pool: Data<Pool<Sqlite>>) -> QueryResults {
    return query("SELECT * FROM users ORDER BY created_at DESC")
        .fetch_all(pool.get_ref())
        .await;
}

pub async fn get_by_id(id: String, pool: Data<Pool<Sqlite>>) -> QueryResult {
    return query("SELECT * FROM users where id = ?")
        .bind(&id)
        .fetch_one(pool.get_ref())
        .await;
}

pub async fn create(email: Option<&String>, phone: Option<&String>, pool: Data<Pool<Sqlite>>) -> QueryResult {
    return query(r#"INSERT INTO users (email, phone) VALUES ($1, $2) returning *"#)
        .bind(&email)
        .bind(&phone)
        .fetch_one(pool.get_ref())
        .await;
}

pub async fn update(id: String, email: Option<&String>, phone: Option<&String>, pool: Data<Pool<Sqlite>>) -> QueryResult {
    return query(r#"UPDATE users SET phone = $1, email = $2 where id = $3 returning *"#)
        .bind(&phone)
        .bind(&email)
        .bind(&id)
        .fetch_one(pool.get_ref())
        .await;
}

pub async fn delete(id: String, pool: Data<Pool<Sqlite>>) -> QueryResult {
    return query(r#"DELETE from users WHERE id = $1 returning *"#)
        .bind(&id)
        .fetch_one(pool.get_ref())
        .await;
}
