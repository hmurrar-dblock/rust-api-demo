use actix_web::{web::{
    Data,
    Json,
    HttpResponse,
    Path,
    ServiceConfig,
    scope,
    resource,
    put,
    post,
    get,
    delete,
}};
use chrono::NaiveDateTime;
use sqlx::{Pool, Row, Sqlite};
use sqlx::sqlite::SqliteRow;

use crate::types::types::{User, UserForm};
use crate::models::user_model;

pub fn serialize_user(row: SqliteRow) -> User {
    let created_at_string = NaiveDateTime::parse_from_str(row.get("created_at"), "%Y-%m-%d %H:%M:%S");
    return match created_at_string {
        Ok(created_at) => {
            User {
                id: row.get(0),
                email: row.get::<&str, &str>("email").parse().unwrap(),
                phone: row.get("phone"),
                created_at: Option::from(created_at),
            }
        },
        _ => User {
            id: row.get("id"),
            email: row.get("email"),
            phone: row.get("phone"),
            created_at: None,
        }
    }
}

// get /users}
pub async fn get_all_users(pool: Data<Pool<Sqlite>>) -> HttpResponse {
    let rows = user_model::get_all(pool).await;
    match rows {
        Ok(rows) => {
            let mut users_arr: Vec<User> = vec![];
            if rows.len() == 0 {
                return HttpResponse::Ok().json(serde_json::json!(&users_arr));
            }
            for row in rows {
                users_arr.push(serialize_user(row));
            }
            HttpResponse::Ok().json(serde_json::json!(&users_arr))
        },
        Err(err) => HttpResponse::InternalServerError().json(format!("failed to fetch users: {:?}", err))
    }
}
// get /users/{id}
pub async fn get_single_user(id: Path<String>, pool: Data<Pool<Sqlite>>) -> HttpResponse {
    let row = user_model::get_by_id(id.to_string(), pool).await;
    match row {
        Ok(row) => {
            HttpResponse::Ok().json(serde_json::json!(serialize_user(row)))
        },
        _ => HttpResponse::NotFound().json(format!("User {} does not exist", &id.to_string()))
    }
}

// post /users
pub async fn create_user(new_user: Json<UserForm>, pool: Data<Pool<Sqlite>>) -> HttpResponse {
    if new_user.email.is_none() {
        return HttpResponse::BadRequest().json("Missing user email");
    }
    if new_user.phone.is_none() {
        return HttpResponse::BadRequest().json("Missing user phone");
    }
    let row = user_model::create(
        new_user.email.as_ref(),
        new_user.phone.as_ref(),
        pool,
    ).await;
    match row {
        Ok(row) => {
            HttpResponse::Ok().json(serde_json::json!(serialize_user(row)))
        },
        Err(err) => {
            HttpResponse::InternalServerError().json(format!("FAILED to create user: {:?}", err))
        }
    }
}

// put /users/{id}
pub async fn update_user(id: Path<String>, updated_user: Json<UserForm>, pool: Data<Pool<Sqlite>>) -> HttpResponse {
    if updated_user.email.is_none() {
        return HttpResponse::BadRequest().json("Missing user email");
    }
    if updated_user.phone.is_none() {
        return HttpResponse::BadRequest().json("Missing user phone");
    }
    let row = user_model::update(
        id.to_string(),
        updated_user.email.as_ref(),
        updated_user.phone.as_ref(),
        pool,
    ).await;
    match row {
        Ok(row) => {
            HttpResponse::Ok().json(serde_json::json!(serialize_user(row)))
        },
        Err(err) => {
            HttpResponse::InternalServerError().json(format!("FAILED to update user: {:?}", err))
        }
    }
}

// delete /users/{id}
pub async fn delete_user(id: Path<String>, pool: Data<Pool<Sqlite>>) -> HttpResponse {
    let row = user_model::delete(
        id.to_string(),
        pool,
    ).await;
    match row {
        Ok(row) => {
            HttpResponse::Ok().json(serde_json::json!(serialize_user(row)))
        },
        Err(err) => {
            HttpResponse::InternalServerError().json(format!("FAILED to delete user: {:?}", err))
        }
    }
}

pub fn init_user_routes(cfg: &mut ServiceConfig) {
    cfg.service(
        resource("/users")
            .route(post().to(create_user))
            .route(get().to(get_all_users))
    )
        .service(
            scope("/users")
                .route("/{id}", get().to(get_single_user))
                .route("/{id}", put().to(update_user))
                .route("/{id}", delete().to(delete_user))
        );
}
