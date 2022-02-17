use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UserForm {
    pub email: Option<String>,
    pub phone: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub phone: String,
    pub created_at: Option<NaiveDateTime>,
}
