use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Media {
    pub id: Uuid,
    pub post_id: Option<Uuid>,
    pub user_id: Uuid,
    pub file_type: String,
    pub file_url: String,
    pub thumbnail_url: Option<String>,
    pub alt_text: Option<String>,
    pub file_size: Option<i64>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub duration: Option<i32>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MediaType {
    Image,
    Video,
    Gif,
    Audio,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateMedia {
    pub file_type: String,
    pub alt_text: Option<String>,
}
