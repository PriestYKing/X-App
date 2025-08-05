use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Notification {
    pub id: Uuid,
    pub user_id: Uuid,
    pub from_user_id: Option<Uuid>,
    pub post_id: Option<Uuid>,
    pub notification_type: String,
    pub message: String,
    pub read_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum NotificationType {
    Like,
    Repost,
    Reply,
    Follow,
    Mention,
    Quote,
    Poll,
    DirectMessage,
}

impl ToString for NotificationType {
    fn to_string(&self) -> String {
        match self {
            NotificationType::Like => "like".to_string(),
            NotificationType::Repost => "repost".to_string(),
            NotificationType::Reply => "reply".to_string(),
            NotificationType::Follow => "follow".to_string(),
            NotificationType::Mention => "mention".to_string(),
            NotificationType::Quote => "quote".to_string(),
            NotificationType::Poll => "poll".to_string(),
            NotificationType::DirectMessage => "direct_message".to_string(),
        }
    }
}

impl std::str::FromStr for NotificationType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "like" => Ok(NotificationType::Like),
            "repost" => Ok(NotificationType::Repost),
            "reply" => Ok(NotificationType::Reply),
            "follow" => Ok(NotificationType::Follow),
            "mention" => Ok(NotificationType::Mention),
            "quote" => Ok(NotificationType::Quote),
            "poll" => Ok(NotificationType::Poll),
            "direct_message" => Ok(NotificationType::DirectMessage),
            _ => Err(format!("Invalid notification type: {}", s)),
        }
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateNotification {
    pub user_id: Uuid,
    pub from_user_id: Option<Uuid>,
    pub post_id: Option<Uuid>,
    #[validate(length(min = 1, max = 50))]
    pub notification_type: String,
    #[validate(length(min = 1, max = 500))]
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct NotificationWithDetails {
    #[serde(flatten)]
    pub notification: Notification,
    pub from_user: Option<NotificationUser>,
    pub post_preview: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationUser {
    pub id: Uuid,
    pub username: String,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub verified: bool,
}

#[derive(Debug, Deserialize)]
pub struct NotificationQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub unread_only: Option<bool>,
    pub notification_types: Option<Vec<String>>,
}

impl Default for NotificationQuery {
    fn default() -> Self {
        Self {
            limit: Some(20),
            offset: Some(0),
            unread_only: Some(false),
            notification_types: None,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct MarkAsReadRequest {
    pub notification_ids: Vec<Uuid>,
}

#[derive(Debug, Serialize)]
pub struct NotificationStats {
    pub total_unread: i64,
    pub unread_by_type: std::collections::HashMap<String, i64>,
}

// Helper functions for creating notifications
impl CreateNotification {
    pub fn new_like(user_id: Uuid, from_user_id: Uuid, post_id: Uuid, from_username: &str) -> Self {
        Self {
            user_id,
            from_user_id: Some(from_user_id),
            post_id: Some(post_id),
            notification_type: NotificationType::Like.to_string(),
            message: format!("{} liked your post", from_username),
        }
    }

    pub fn new_repost(
        user_id: Uuid,
        from_user_id: Uuid,
        post_id: Uuid,
        from_username: &str,
    ) -> Self {
        Self {
            user_id,
            from_user_id: Some(from_user_id),
            post_id: Some(post_id),
            notification_type: NotificationType::Repost.to_string(),
            message: format!("{} reposted your post", from_username),
        }
    }

    pub fn new_reply(
        user_id: Uuid,
        from_user_id: Uuid,
        post_id: Uuid,
        from_username: &str,
    ) -> Self {
        Self {
            user_id,
            from_user_id: Some(from_user_id),
            post_id: Some(post_id),
            notification_type: NotificationType::Reply.to_string(),
            message: format!("{} replied to your post", from_username),
        }
    }

    pub fn new_follow(user_id: Uuid, from_user_id: Uuid, from_username: &str) -> Self {
        Self {
            user_id,
            from_user_id: Some(from_user_id),
            post_id: None,
            notification_type: NotificationType::Follow.to_string(),
            message: format!("{} started following you", from_username),
        }
    }

    pub fn new_mention(
        user_id: Uuid,
        from_user_id: Uuid,
        post_id: Uuid,
        from_username: &str,
    ) -> Self {
        Self {
            user_id,
            from_user_id: Some(from_user_id),
            post_id: Some(post_id),
            notification_type: NotificationType::Mention.to_string(),
            message: format!("{} mentioned you in a post", from_username),
        }
    }

    pub fn new_quote(
        user_id: Uuid,
        from_user_id: Uuid,
        post_id: Uuid,
        from_username: &str,
    ) -> Self {
        Self {
            user_id,
            from_user_id: Some(from_user_id),
            post_id: Some(post_id),
            notification_type: NotificationType::Quote.to_string(),
            message: format!("{} quoted your post", from_username),
        }
    }

    pub fn new_poll_ended(user_id: Uuid, post_id: Uuid) -> Self {
        Self {
            user_id,
            from_user_id: None,
            post_id: Some(post_id),
            notification_type: NotificationType::Poll.to_string(),
            message: "Your poll has ended".to_string(),
        }
    }

    pub fn new_direct_message(user_id: Uuid, from_user_id: Uuid, from_username: &str) -> Self {
        Self {
            user_id,
            from_user_id: Some(from_user_id),
            post_id: None,
            notification_type: NotificationType::DirectMessage.to_string(),
            message: format!("{} sent you a message", from_username),
        }
    }
}

// Utility functions for notification processing
pub mod utils {
    use super::*;

    /// Check if notification should be sent based on user preferences
    pub fn should_send_notification(
        notification_type: &NotificationType,
        user_preferences: &NotificationPreferences,
    ) -> bool {
        match notification_type {
            NotificationType::Like => user_preferences.likes,
            NotificationType::Repost => user_preferences.reposts,
            NotificationType::Reply => user_preferences.replies,
            NotificationType::Follow => user_preferences.follows,
            NotificationType::Mention => user_preferences.mentions,
            NotificationType::Quote => user_preferences.quotes,
            NotificationType::Poll => user_preferences.polls,
            NotificationType::DirectMessage => user_preferences.direct_messages,
        }
    }

    /// Generate notification message with proper formatting
    pub fn format_notification_message(
        notification_type: &NotificationType,
        from_username: &str,
        additional_info: Option<&str>,
    ) -> String {
        match notification_type {
            NotificationType::Like => format!("{} liked your post", from_username),
            NotificationType::Repost => format!("{} reposted your post", from_username),
            NotificationType::Reply => format!("{} replied to your post", from_username),
            NotificationType::Follow => format!("{} started following you", from_username),
            NotificationType::Mention => format!("{} mentioned you in a post", from_username),
            NotificationType::Quote => format!("{} quoted your post", from_username),
            NotificationType::Poll => additional_info.unwrap_or("Your poll has ended").to_string(),
            NotificationType::DirectMessage => format!("{} sent you a message", from_username),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationPreferences {
    pub likes: bool,
    pub reposts: bool,
    pub replies: bool,
    pub follows: bool,
    pub mentions: bool,
    pub quotes: bool,
    pub polls: bool,
    pub direct_messages: bool,
    pub push_notifications: bool,
    pub email_notifications: bool,
}

impl Default for NotificationPreferences {
    fn default() -> Self {
        Self {
            likes: true,
            reposts: true,
            replies: true,
            follows: true,
            mentions: true,
            quotes: true,
            polls: true,
            direct_messages: true,
            push_notifications: true,
            email_notifications: false,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct NotificationBatch {
    pub notifications: Vec<NotificationWithDetails>,
    pub total_count: i64,
    pub unread_count: i64,
    pub has_more: bool,
}

// Aggregate notification for grouping similar notifications
#[derive(Debug, Serialize)]
pub struct AggregateNotification {
    pub id: Uuid,
    pub notification_type: String,
    pub message: String,
    pub count: i64,
    pub users: Vec<NotificationUser>,
    pub post_id: Option<Uuid>,
    pub latest_created_at: DateTime<Utc>,
    pub is_read: bool,
}

impl AggregateNotification {
    pub fn new(
        notification_type: String,
        post_id: Option<Uuid>,
        users: Vec<NotificationUser>,
    ) -> Self {
        let count = users.len() as i64;
        let message = match notification_type.as_str() {
            "like" => {
                if count == 1 {
                    format!("{} liked your post", users[0].username)
                } else {
                    format!(
                        "{} and {} others liked your post",
                        users[0].username,
                        count - 1
                    )
                }
            }
            "follow" => {
                if count == 1 {
                    format!("{} started following you", users[0].username)
                } else {
                    format!(
                        "{} and {} others started following you",
                        users[0].username,
                        count - 1
                    )
                }
            }
            _ => format!("{} users interacted with your content", count),
        };

        Self {
            id: Uuid::new_v4(),
            notification_type,
            message,
            count,
            users,
            post_id,
            latest_created_at: Utc::now(),
            is_read: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notification_type_string_conversion() {
        let like_type = NotificationType::Like;
        assert_eq!(like_type.to_string(), "like");

        let parsed: NotificationType = "like".parse().unwrap();
        assert!(matches!(parsed, NotificationType::Like));
    }

    #[test]
    fn test_create_notification_helpers() {
        let user_id = Uuid::new_v4();
        let from_user_id = Uuid::new_v4();
        let post_id = Uuid::new_v4();

        let like_notification =
            CreateNotification::new_like(user_id, from_user_id, post_id, "testuser");

        assert_eq!(like_notification.notification_type, "like");
        assert_eq!(like_notification.message, "testuser liked your post");
        assert_eq!(like_notification.user_id, user_id);
        assert_eq!(like_notification.from_user_id, Some(from_user_id));
        assert_eq!(like_notification.post_id, Some(post_id));
    }

    #[test]
    fn test_notification_preferences_default() {
        let prefs = NotificationPreferences::default();
        assert!(prefs.likes);
        assert!(prefs.follows);
        assert!(!prefs.email_notifications);
    }

    #[test]
    fn test_should_send_notification() {
        let mut prefs = NotificationPreferences::default();
        prefs.likes = false;

        assert!(!utils::should_send_notification(
            &NotificationType::Like,
            &prefs
        ));
        assert!(utils::should_send_notification(
            &NotificationType::Follow,
            &prefs
        ));
    }
}
