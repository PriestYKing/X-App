// use actix_web::{delete, get, post, put, web, HttpMessage, HttpResponse, Responder};
// use chrono::{DateTime, Utc};
// use serde::{Deserialize, Serialize};
// use sqlx::{prelude::FromRow, PgPool};
// use uuid::Uuid;
// use validator::Validate;

// #[derive(Debug, Serialize, Deserialize)]
// pub struct AdminStats {
//     pub total_users: i64,
//     pub total_posts: i64,
//     pub total_reports: i64,
//     pub daily_active_users: i64,
//     pub monthly_active_users: i64,
// }

// #[derive(Debug, Deserialize, Validate)]
// pub struct BanUserRequest {
//     pub user_id: Uuid,
//     #[validate(length(min = 10, max = 500))]
//     pub reason: String,
//     pub ban_duration_days: Option<i32>,
// }

// #[derive(Debug, Deserialize, Validate)]
// pub struct ModerationAction {
//     pub target_id: Uuid,     // Post or user ID
//     pub action_type: String, // "ban", "delete", "warn", "suspend"
//     #[validate(length(min = 10, max = 500))]
//     pub reason: String,
//     pub duration_days: Option<i32>,
// }

// #[derive(Debug, Serialize)]
// pub struct ReportSummary {
//     pub id: Uuid,
//     pub reported_user_id: Option<Uuid>,
//     pub reported_post_id: Option<Uuid>,
//     pub reporter_id: Uuid,
//     pub report_type: String,
//     pub reason: String,
//     pub status: String,
//     pub created_at: DateTime<Utc>,
//     pub reporter_username: String,
//     pub reported_username: Option<String>,
// }

// #[derive(Debug, Serialize, FromRow)]
// pub struct UserProfile {
//     pub id: Uuid,
//     pub username: String,
//     pub email: String,
//     pub display_name: Option<String>,
//     pub verified: bool,
//     pub premium: bool,
//     pub is_banned: bool,
//     pub created_at: DateTime<Utc>,
//     pub last_active: Option<DateTime<Utc>>,
//     pub follower_count: i32,
//     pub following_count: i32,
//     pub post_count: i32,
// }

// #[get("/admin/dashboard")]
// pub async fn get_dashboard_stats(
//     pool: web::Data<PgPool>,
//     req: actix_web::HttpRequest,
// ) -> impl Responder {
//     // Check if user is admin
//     let user_id = match req.extensions().get::<Uuid>() {
//         Some(id) => *id,
//         None => {
//             return HttpResponse::Unauthorized().json(serde_json::json!({
//                 "error": "Authentication required"
//             }));
//         }
//     };

//     // Verify admin status
//     let is_admin = sqlx::query!("SELECT verified FROM users WHERE id = $1", user_id)
//         .fetch_optional(pool.get_ref())
//         .await;

//     match is_admin {
//         Ok(Some(user)) if user.verified.unwrap_or(false) => {
//             // Proceed with admin functionality
//         }
//         _ => {
//             return HttpResponse::Forbidden().json(serde_json::json!({
//                 "error": "Admin access required"
//             }));
//         }
//     }

//     // Get dashboard statistics
//     let total_users = sqlx::query!("SELECT COUNT(*) as count FROM users")
//         .fetch_one(pool.get_ref())
//         .await
//         .map(|row| row.count.unwrap_or(0))
//         .unwrap_or(0);

//     let total_posts = sqlx::query!("SELECT COUNT(*) as count FROM posts")
//         .fetch_one(pool.get_ref())
//         .await
//         .map(|row| row.count.unwrap_or(0))
//         .unwrap_or(0);

//     let daily_active_users = sqlx::query!(
//         "SELECT COUNT(DISTINCT user_id) as count FROM posts WHERE created_at > NOW() - INTERVAL '1 day'"
//     )
//     .fetch_one(pool.get_ref())
//     .await
//     .map(|row| row.count.unwrap_or(0))
//     .unwrap_or(0);

//     let monthly_active_users = sqlx::query!(
//         "SELECT COUNT(DISTINCT user_id) as count FROM posts WHERE created_at > NOW() - INTERVAL '30 days'"
//     )
//     .fetch_one(pool.get_ref())
//     .await
//     .map(|row| row.count.unwrap_or(0))
//     .unwrap_or(0);

//     let stats = AdminStats {
//         total_users,
//         total_posts,
//         total_reports: 0, // You'd implement reports table
//         daily_active_users,
//         monthly_active_users,
//     };

//     HttpResponse::Ok().json(stats)
// }

// #[get("/admin/users")]
// pub async fn get_all_users(
//     pool: web::Data<PgPool>,
//     req: actix_web::HttpRequest,
//     query: web::Query<std::collections::HashMap<String, String>>,
// ) -> impl Responder {
//     // Admin check (same as above)
//     let limit = query
//         .get("limit")
//         .and_then(|l| l.parse::<i64>().ok())
//         .unwrap_or(20);
//     let offset = query
//         .get("offset")
//         .and_then(|o| o.parse::<i64>().ok())
//         .unwrap_or(0);

//     let users = sqlx::query_as::<_, UserProfile>(
//         r#"
//     SELECT 
//         id, username, email, display_name, verified, premium,
//         FALSE as is_banned, created_at, created_at as last_active,
//         follower_count, following_count, post_count
//     FROM users 
//     ORDER BY created_at DESC 
//     LIMIT $1 OFFSET $2
//     "#,
//     )
//     .bind(limit)
//     .bind(offset)
//     .fetch_all(pool.get_ref())
//     .await;

//     match users {
//         Ok(users) => HttpResponse::Ok().json(users),
//         Err(e) => {
//             log::error!("Database error: {}", e);
//             HttpResponse::InternalServerError().json(serde_json::json!({
//                 "error": "Failed to fetch users"
//             }))
//         }
//     }
// }

// #[post("/admin/users/ban")]
// pub async fn ban_user(
//     pool: web::Data<PgPool>,
//     req: actix_web::HttpRequest,
//     form: web::Json<BanUserRequest>,
// ) -> impl Responder {
//     if let Err(errors) = form.validate() {
//         return HttpResponse::BadRequest().json(serde_json::json!({
//             "error": "Validation failed",
//             "details": errors
//         }));
//     }

//     // Admin verification would go here

//     // For now, we'll update a field (you'd need to add this to your schema)
//     let result = sqlx::query!(
//         "UPDATE users SET verified = FALSE WHERE id = $1",
//         form.user_id
//     )
//     .execute(pool.get_ref())
//     .await;

//     match result {
//         Ok(_) => HttpResponse::Ok().json(serde_json::json!({
//             "message": "User banned successfully",
//             "user_id": form.user_id,
//             "reason": form.reason
//         })),
//         Err(e) => {
//             log::error!("Database error: {}", e);
//             HttpResponse::InternalServerError().json(serde_json::json!({
//                 "error": "Failed to ban user"
//             }))
//         }
//     }
// }

// #[delete("/admin/posts/{post_id}")]
// pub async fn delete_post_admin(
//     pool: web::Data<PgPool>,
//     post_id: web::Path<Uuid>,
//     req: actix_web::HttpRequest,
// ) -> impl Responder {
//     // Admin verification would go here

//     let result = sqlx::query!("DELETE FROM posts WHERE id = $1", *post_id)
//         .execute(pool.get_ref())
//         .await;

//     match result {
//         Ok(rows) => {
//             if rows.rows_affected() > 0 {
//                 HttpResponse::Ok().json(serde_json::json!({
//                     "message": "Post deleted successfully"
//                 }))
//             } else {
//                 HttpResponse::NotFound().json(serde_json::json!({
//                     "error": "Post not found"
//                 }))
//             }
//         }
//         Err(e) => {
//             log::error!("Database error: {}", e);
//             HttpResponse::InternalServerError().json(serde_json::json!({
//                 "error": "Failed to delete post"
//             }))
//         }
//     }
// }

// pub fn configure(cfg: &mut web::ServiceConfig) {
//     cfg.service(get_dashboard_stats)
//         .service(get_all_users)
//         .service(ban_user)
//         .service(delete_post_admin);
// }
