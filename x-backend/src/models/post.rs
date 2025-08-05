use super::media::Media;
use super::user::UserPublic;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Post {
    pub id: Uuid,
    pub user_id: Uuid,
    pub content: String,
    pub reply_to_id: Option<Uuid>,
    pub quote_post_id: Option<Uuid>,
    pub like_count: i32,
    pub repost_count: i32,
    pub reply_count: i32,
    pub view_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct PostWithDetails {
    #[serde(flatten)]
    pub post: Post,
    pub author: UserPublic,
    pub media: Vec<Media>,
    pub is_liked: bool,
    pub is_reposted: bool,
    pub is_bookmarked: bool,
    pub reply_to: Option<Box<PostWithDetails>>,
    pub quote_post: Option<Box<PostWithDetails>>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreatePost {
    #[validate(length(min = 1, max = 280))]
    pub content: String,
    pub media_ids: Option<Vec<Uuid>>,
    pub reply_to_id: Option<Uuid>,
    pub quote_post_id: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct PostQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub cursor: Option<Uuid>,
}

impl Default for PostQuery {
    fn default() -> Self {
        Self {
            limit: Some(20),
            offset: Some(0),
            cursor: None,
        }
    }
}
