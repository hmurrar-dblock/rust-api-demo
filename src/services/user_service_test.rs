use actix_web::{
    App,
    test::{read_body_json, init_service, TestRequest},
};
use rand::Rng;
use serde_json::{json};
use crate::db::init_db_connection;
use crate::types::types::{User};
use crate::services::user_service::{init_user_routes};
use sqlx::{Pool, Sqlite};

#[cfg(test)]
mod tests {
    use super::*;

    fn generate_random_number() -> String {
        let mut rng = rand::thread_rng();
        return rng.gen::<u32>().to_string();
    }

    fn generate_random_email() -> String {
        let num = generate_random_number();
        return format!("my.email-{}@domain.com", num);
    }

    async fn get_db_pool() -> Pool<Sqlite> {
        return init_db_connection().await;
    }

    #[actix_rt::test]
    async fn create_user() {
        let conn_pool = get_db_pool().await;
        let mut app = init_service(
            App::new()
                .data(conn_pool.clone())
                .configure(init_user_routes))
            .await;
        let phone = generate_random_number();
        let email = generate_random_email();
        let new_user = json!({
            "phone": phone,
            "email": email,
        });
        let create_user_resp = TestRequest::post()
            .uri("/users")
            .set_json(&new_user)
            .send_request(&mut app)
            .await;
        let created_user: User = read_body_json(create_user_resp).await;
        assert_eq!(created_user.phone, phone);
        assert_eq!(created_user.email, email);
    }

    #[actix_rt::test]
    async fn get_user() {
        let conn_pool = get_db_pool().await;
        let mut app = init_service(
            App::new()
                .data(conn_pool.clone())
                .configure(init_user_routes))
            .await;
        let phone = generate_random_number();
        let email = generate_random_email();
        let new_user = json!({
            "phone": phone,
            "email": email,
        });
        let create_user_resp = TestRequest::post()
            .uri("/users")
            .set_json(&new_user)
            .send_request(&mut app)
            .await;
        let created_user: User = read_body_json(create_user_resp).await;
        let resp = TestRequest::get()
            .uri(format!("/users/{}", created_user.id).as_str())
            .send_request(&mut app)
            .await;
        let user: User = read_body_json(resp).await;
        assert_eq!(created_user.phone, phone);
        assert_eq!(created_user.email, email);
        assert_eq!(created_user.id, user.id);
    }

    #[actix_rt::test]
    async fn update_user() {
        let conn_pool = get_db_pool().await;
        let mut app = init_service(
            App::new()
                .data(conn_pool.clone())
                .configure(init_user_routes))
            .await;
        let email = generate_random_email();
        let phone = generate_random_number();
        let new_user = json!({
            "phone": phone,
            "email": email,
        });
        let updated_email = generate_random_email();
        let updated_phone = generate_random_number();
        let updated_user = json!({
            "phone": updated_phone,
            "email": updated_email,
        });
        let create_user_resp = TestRequest::post()
            .uri("/users")
            .set_json(&new_user)
            .send_request(&mut app)
            .await;
        let created_user: User = read_body_json(create_user_resp).await;
        let resp = TestRequest::put()
            .uri(format!("/users/{}", created_user.id).as_str())
            .set_json(&updated_user)
            .send_request(&mut app)
            .await;
        let user: User = read_body_json(resp).await;
        assert_eq!(user.phone, updated_phone);
        assert_eq!(user.email, updated_email);
    }

    #[actix_rt::test]
    async fn get_all () {
        let conn_pool = get_db_pool().await;
        let mut app = init_service(
            App::new()
                .data(conn_pool.clone())
                .configure(init_user_routes))
            .await;
        let phone1 = generate_random_number();
        let email1 = generate_random_email();
        let phone2 = generate_random_number();
        let email2 = generate_random_email();
        let new_user1 = json!({
            "phone": phone1,
            "email": email1,
        });
        let new_user2 = json!({
            "phone": phone2,
            "email": email2,
        });
        let create_user1_resp = TestRequest::post()
            .uri("/users")
            .set_json(&new_user1)
            .send_request(&mut app)
            .await;
        let create_user2_resp = TestRequest::post()
            .uri("/users")
            .set_json(&new_user2)
            .send_request(&mut app)
            .await;
        let created_user1: User = read_body_json(create_user1_resp).await;
        let created_user2: User = read_body_json(create_user2_resp).await;
        let all_response = TestRequest::get()
            .uri("/users")
            .send_request(&mut app)
            .await;
        let all_users: Vec<User> = read_body_json(all_response).await;
        assert_eq!(created_user1.email, all_users[0].email);
        assert_eq!(created_user1.phone, all_users[0].phone);
        assert_eq!(created_user2.email, all_users[1].email);
        assert_eq!(created_user2.phone, all_users[1].phone);
    }

    #[actix_rt::test]
    async fn delete_user() {
        let conn_pool = get_db_pool().await;
        let mut app = init_service(
            App::new()
                .data(conn_pool.clone())
                .configure(init_user_routes))
            .await;
        let phone = generate_random_number();
        let email = generate_random_email();
        let new_user = json!({
            "phone": phone,
            "email": email,
        });
        let create_user_resp = TestRequest::post()
            .uri("/users")
            .set_json(&new_user)
            .send_request(&mut app)
            .await;
        let created_user: User = read_body_json(create_user_resp).await;
        let resp = TestRequest::delete()
            .uri(format!("/users/{}", created_user.id).as_str())
            .send_request(&mut app)
            .await;
        let user: User = read_body_json(resp).await;
        assert_eq!(created_user.phone, phone);
        assert_eq!(created_user.email, email);
        assert_eq!(created_user.id, user.id);
    }
}
