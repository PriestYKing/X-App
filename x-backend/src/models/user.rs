use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub banner_url: Option<String>,
    pub location: Option<String>,
    pub website: Option<String>,
    pub verified: bool,
    pub premium: bool,
    pub follower_count: i32,
    pub following_count: i32,
    pub post_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateUser {
    #[validate(length(min = 3, max = 50))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateProfile {
    #[validate(length(max = 100))]
    pub display_name: Option<String>,
    #[validate(length(max = 500))]
    pub bio: Option<String>,
    #[validate(length(max = 100))]
    pub location: Option<String>,
    #[validate(url)]
    pub website: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UserPublic {
    pub id: Uuid,
    pub username: String,
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub banner_url: Option<String>,
    pub location: Option<String>,
    pub website: Option<String>,
    pub verified: bool,
    pub premium: bool,
    pub follower_count: i32,
    pub following_count: i32,
    pub post_count: i32,
    pub created_at: DateTime<Utc>,
}

impl From<User> for UserPublic {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            display_name: user.display_name,
            bio: user.bio,
            avatar_url: user.avatar_url,
            banner_url: user.banner_url,
            location: user.location,
            website: user.website,
            verified: user.verified,
            premium: user.premium,
            follower_count: user.follower_count,
            following_count: user.following_count,
            post_count: user.post_count,
            created_at: user.created_at,
        }
    }
}
