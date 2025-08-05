// use actix_web::{delete, get, post, web, HttpMessage, HttpResponse, Responder};
// use chrono::{DateTime, Utc};
// use serde::{Deserialize, Serialize};
// use sqlx::PgPool;
// use uuid::Uuid;

// #[derive(Debug, Deserialize)]
// pub struct InteractionRequest {
//     pub post_id: Uuid,
// }

// #[derive(Debug, Deserialize)]
// pub struct FollowRequest {
//     pub user_id: Uuid,
// }

// #[derive(Debug, Serialize)]
// pub struct InteractionResponse {
//     pub success: bool,
//     pub message: String,
//     pub new_count: Option<i32>,
// }

// #[derive(Debug, Serialize)]
// pub struct FollowStatus {
//     pub is_following: bool,
//     pub follower_count: i32,
//     pub following_count: i32,
// }

// // Like/Unlike Posts
// #[post("/posts/{post_id}/like")]
// pub async fn like_post(
//     pool: web::Data<PgPool>,
//     post_id: web::Path<Uuid>,
//     req: actix_web::HttpRequest,
// ) -> impl Responder {
//     let user_id = match req.extensions().get::<Uuid>() {
//         Some(id) => *id,
//         None => {
//             return HttpResponse::Unauthorized().json(serde_json::json!({
//                 "error": "Authentication required"
//             }));
//         }
//     };

//     // Check if already liked
//     let existing_like = sqlx::query!(
//         "SELECT id FROM likes WHERE user_id = $1 AND post_id = $2",
//         user_id,
//         *post_id
//     )
//     .fetch_optional(pool.get_ref())
//     .await;

//     match existing_like {
//         Ok(Some(_)) => {
//             // Unlike the post
//             let mut tx = match pool.begin().await {
//                 Ok(tx) => tx,
//                 Err(e) => {
//                     log::error!("Transaction error: {}", e);
//                     return HttpResponse::InternalServerError().json(serde_json::json!({
//                         "error": "Internal server error"
//                     }));
//                 }
//             };

//             // Remove like
//             if let Err(e) = sqlx::query!(
//                 "DELETE FROM likes WHERE user_id = $1 AND post_id = $2",
//                 user_id,
//                 *post_id
//             )
//             .execute(&mut *tx)
//             .await
//             {
//                 log::error!("Failed to remove like: {}", e);
//                 return HttpResponse::InternalServerError().json(serde_json::json!({
//                     "error": "Failed to unlike post"
//                 }));
//             }

//             // Decrement like count
//             if let Err(e) = sqlx::query!(
//                 "UPDATE posts SET like_count = like_count - 1 WHERE id = $1",
//                 *post_id
//             )
//             .execute(&mut *tx)
//             .await
//             {
//                 log::error!("Failed to update like count: {}", e);
//                 return HttpResponse::InternalServerError().json(serde_json::json!({
//                     "error": "Failed to update like count"
//                 }));
//             }

//             if let Err(e) = tx.commit().await {
//                 log::error!("Transaction commit error: {}", e);
//                 return HttpResponse::InternalServerError().json(serde_json::json!({
//                     "error": "Internal server error"
//                 }));
//             }

//             HttpResponse::Ok().json(InteractionResponse {
//                 success: true,
//                 message: "Post unliked successfully".to_string(),
//                 new_count: None,
//             })
//         }
//         Ok(None) => {
//             // Like the post
//             let mut tx = match pool.begin().await {
//                 Ok(tx) => tx,
//                 Err(e) => {
//                     log::error!("Transaction error: {}", e);
//                     return HttpResponse::InternalServerError().json(serde_json::json!({
//                         "error": "Internal server error"
//                     }));
//                 }
//             };

//             // Add like
//             if let Err(e) = sqlx::query!(
//                 "INSERT INTO likes (user_id, post_id) VALUES ($1, $2)",
//                 user_id,
//                 *post_id
//             )
//             .execute(&mut *tx)
//             .await
//             {
//                 log::error!("Failed to add like: {}", e);
//                 return HttpResponse::InternalServerError().json(serde_json::json!({
//                     "error": "Failed to like post"
//                 }));
//             }

//             // Increment like count
//             if let Err(e) = sqlx::query!(
//                 "UPDATE posts SET like_count = like_count + 1 WHERE id = $1",
//                 *post_id
//             )
//             .execute(&mut *tx)
//             .await
//             {
//                 log::error!("Failed to update like count: {}", e);
//                 return HttpResponse::InternalServerError().json(serde_json::json!({
//                     "error": "Failed to update like count"
//                 }));
//             }

//             if let Err(e) = tx.commit().await {
//                 log::error!("Transaction commit error: {}", e);
//                 return HttpResponse::InternalServerError().json(serde_json::json!({
//                     "error": "Internal server error"
//                 }));
//             }

//             HttpResponse::Ok().json(InteractionResponse {
//                 success: true,
//                 message: "Post liked successfully".to_string(),
//                 new_count: None,
//             })
//         }
//         Err(e) => {
//             log::error!("Database error: {}", e);
//             HttpResponse::InternalServerError().json(serde_json::json!({
//                 "error": "Internal server error"
//             }))
//         }
//     }
// }

// // Repost functionality
// #[post("/posts/{post_id}/repost")]
// pub async fn repost_post(
//     pool: web::Data<PgPool>,
//     post_id: web::Path<Uuid>,
//     req: actix_web::HttpRequest,
// ) -> impl Responder {
//     let user_id = match req.extensions().get::<Uuid>() {
//         Some(id) => *id,
//         None => {
//             return HttpResponse::Unauthorized().json(serde_json::json!({
//                 "error": "Authentication required"
//             }));
//         }
//     };

//     // Check if already reposted
//     let existing_repost = sqlx::query!(
//         "SELECT id FROM reposts WHERE user_id = $1 AND post_id = $2",
//         user_id,
//         *post_id
//     )
//     .fetch_optional(pool.get_ref())
//     .await;

//     match existing_repost {
//         Ok(Some(_)) => HttpResponse::BadRequest().json(serde_json::json!({
//             "error": "Post already reposted"
//         })),
//         Ok(None) => {
//             let mut tx = match pool.begin().await {
//                 Ok(tx) => tx,
//                 Err(e) => {
//                     log::error!("Transaction error: {}", e);
//                     return HttpResponse::InternalServerError().json(serde_json::json!({
//                         "error": "Internal server error"
//                     }));
//                 }
//             };

//             // Add repost
//             if let Err(e) = sqlx::query!(
//                 "INSERT INTO reposts (user_id, post_id) VALUES ($1, $2)",
//                 user_id,
//                 *post_id
//             )
//             .execute(&mut *tx)
//             .await
//             {
//                 log::error!("Failed to add repost: {}", e);
//                 return HttpResponse::InternalServerError().json(serde_json::json!({
//                     "error": "Failed to repost"
//                 }));
//             }

//             // Increment repost count
//             if let Err(e) = sqlx::query!(
//                 "UPDATE posts SET repost_count = repost_count + 1 WHERE id = $1",
//                 *post_id
//             )
//             .execute(&mut *tx)
//             .await
//             {
//                 log::error!("Failed to update repost count: {}", e);
//                 return HttpResponse::InternalServerError().json(serde_json::json!({
//                     "error": "Failed to update repost count"
//                 }));
//             }

//             if let Err(e) = tx.commit().await {
//                 log::error!("Transaction commit error: {}", e);
//                 return HttpResponse::InternalServerError().json(serde_json::json!({
//                     "error": "Internal server error"
//                 }));
//             }

//             HttpResponse::Ok().json(InteractionResponse {
//                 success: true,
//                 message: "Post reposted successfully".to_string(),
//                 new_count: None,
//             })
//         }
//         Err(e) => {
//             log::error!("Database error: {}", e);
//             HttpResponse::InternalServerError().json(serde_json::json!({
//                 "error": "Internal server error"
//             }))
//         }
//     }
// }

// // Follow/Unfollow users
// #[post("/users/{user_id}/follow")]
// pub async fn follow_user(
//     pool: web::Data<PgPool>,
//     target_user_id: web::Path<Uuid>,
//     req: actix_web::HttpRequest,
// ) -> impl Responder {
//     let follower_id = match req.extensions().get::<Uuid>() {
//         Some(id) => *id,
//         None => {
//             return HttpResponse::Unauthorized().json(serde_json::json!({
//                 "error": "Authentication required"
//             }));
//         }
//     };

//     if follower_id == *target_user_id {
//         return HttpResponse::BadRequest().json(serde_json::json!({
//             "error": "Cannot follow yourself"
//         }));
//     }

//     // Check if already following
//     let existing_follow = sqlx::query!(
//         "SELECT id FROM follows WHERE follower_id = $1 AND following_id = $2",
//         follower_id,
//         *target_user_id
//     )
//     .fetch_optional(pool.get_ref())
//     .await;

//     match existing_follow {
//         Ok(Some(_)) => HttpResponse::BadRequest().json(serde_json::json!({
//             "error": "Already following this user"
//         })),
//         Ok(None) => {
//             let mut tx = match pool.begin().await {
//                 Ok(tx) => tx,
//                 Err(e) => {
//                     log::error!("Transaction error: {}", e);
//                     return HttpResponse::InternalServerError().json(serde_json::json!({
//                         "error": "Internal server error"
//                     }));
//                 }
//             };

//             // Add follow relationship
//             if let Err(e) = sqlx::query!(
//                 "INSERT INTO follows (follower_id, following_id) VALUES ($1, $2)",
//                 follower_id,
//                 *target_user_id
//             )
//             .execute(&mut *tx)
//             .await
//             {
//                 log::error!("Failed to create follow relationship: {}", e);
//                 return HttpResponse::InternalServerError().json(serde_json::json!({
//                     "error": "Failed to follow user"
//                 }));
//             }

//             // Update follower count for target user
//             if let Err(e) = sqlx::query!(
//                 "UPDATE users SET follower_count = follower_count + 1 WHERE id = $1",
//                 *target_user_id
//             )
//             .execute(&mut *tx)
//             .await
//             {
//                 log::error!("Failed to update follower count: {}", e);
//             }

//             // Update following count for current user
//             if let Err(e) = sqlx::query!(
//                 "UPDATE users SET following_count = following_count + 1 WHERE id = $1",
//                 follower_id
//             )
//             .execute(&mut *tx)
//             .await
//             {
//                 log::error!("Failed to update following count: {}", e);
//             }

//             if let Err(e) = tx.commit().await {
//                 log::error!("Transaction commit error: {}", e);
//                 return HttpResponse::InternalServerError().json(serde_json::json!({
//                     "error": "Internal server error"
//                 }));
//             }

//             HttpResponse::Ok().json(InteractionResponse {
//                 success: true,
//                 message: "User followed successfully".to_string(),
//                 new_count: None,
//             })
//         }
//         Err(e) => {
//             log::error!("Database error: {}", e);
//             HttpResponse::InternalServerError().json(serde_json::json!({
//                 "error": "Internal server error"
//             }))
//         }
//     }
// }

// // Bookmark posts
// #[post("/posts/{post_id}/bookmark")]
// pub async fn bookmark_post(
//     pool: web::Data<PgPool>,
//     post_id: web::Path<Uuid>,
//     req: actix_web::HttpRequest,
// ) -> impl Responder {
//     let user_id = match req.extensions().get::<Uuid>() {
//         Some(id) => *id,
//         None => {
//             return HttpResponse::Unauthorized().json(serde_json::json!({
//                 "error": "Authentication required"
//             }));
//         }
//     };

//     // Check if already bookmarked
//     let existing_bookmark = sqlx::query!(
//         "SELECT id FROM bookmarks WHERE user_id = $1 AND post_id = $2",
//         user_id,
//         *post_id
//     )
//     .fetch_optional(pool.get_ref())
//     .await;

//     match existing_bookmark {
//         Ok(Some(_)) => {
//             // Remove bookmark
//             let result = sqlx::query!(
//                 "DELETE FROM bookmarks WHERE user_id = $1 AND post_id = $2",
//                 user_id,
//                 *post_id
//             )
//             .execute(pool.get_ref())
//             .await;

//             match result {
//                 Ok(_) => HttpResponse::Ok().json(InteractionResponse {
//                     success: true,
//                     message: "Bookmark removed successfully".to_string(),
//                     new_count: None,
//                 }),
//                 Err(e) => {
//                     log::error!("Failed to remove bookmark: {}", e);
//                     HttpResponse::InternalServerError().json(serde_json::json!({
//                         "error": "Failed to remove bookmark"
//                     }))
//                 }
//             }
//         }
//         Ok(None) => {
//             // Add bookmark
//             let result = sqlx::query!(
//                 "INSERT INTO bookmarks (user_id, post_id) VALUES ($1, $2)",
//                 user_id,
//                 *post_id
//             )
//             .execute(pool.get_ref())
//             .await;

//             match result {
//                 Ok(_) => HttpResponse::Ok().json(InteractionResponse {
//                     success: true,
//                     message: "Post bookmarked successfully".to_string(),
//                     new_count: None,
//                 }),
//                 Err(e) => {
//                     log::error!("Failed to add bookmark: {}", e);
//                     HttpResponse::InternalServerError().json(serde_json::json!({
//                         "error": "Failed to bookmark post"
//                     }))
//                 }
//             }
//         }
//         Err(e) => {
//             log::error!("Database error: {}", e);
//             HttpResponse::InternalServerError().json(serde_json::json!({
//                 "error": "Internal server error"
//             }))
//         }
//     }
// }

// pub fn configure(cfg: &mut web::ServiceConfig) {
//     cfg.service(like_post)
//         .service(repost_post)
//         .service(follow_user)
//         .service(bookmark_post);
// }
