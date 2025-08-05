// use actix_multipart::Multipart;
// use actix_web::{get, post, put, web, HttpMessage, HttpResponse, Responder};
// use futures_util::TryStreamExt;
// use serde::{Deserialize, Serialize};
// use sqlx::PgPool;
// use uuid::Uuid;
// use validator::Validate;

// #[derive(Debug, Deserialize, Validate)]
// pub struct UpdateProfileRequest {
//     #[validate(length(max = 100))]
//     pub display_name: Option<String>,
//     #[validate(length(max = 500))]
//     pub bio: Option<String>,
//     #[validate(length(max = 100))]
//     pub location: Option<String>,
//     #[validate(url)]
//     pub website: Option<String>,
// }

// #[derive(Debug, Serialize)]
// pub struct ProfileResponse {
//     pub id: Uuid,
//     pub username: String,
//     pub display_name: Option<String>,
//     pub bio: Option<String>,
//     pub avatar_url: Option<String>,
//     pub banner_url: Option<String>,
//     pub location: Option<String>,
//     pub website: Option<String>,
//     pub verified: bool,
//     pub premium: bool,
//     pub follower_count: i32,
//     pub following_count: i32,
//     pub post_count: i32,
//     pub is_following: bool,
//     pub is_blocked: bool,
//     pub join_date: chrono::DateTime<chrono::Utc>,
// }

// #[derive(Debug, Serialize)]
// pub struct ProfileStats {
//     pub total_likes_received: i64,
//     pub total_reposts_received: i64,
//     pub posts_this_month: i64,
//     pub engagement_rate: f64,
// }

// #[get("/profile/{username}")]
// pub async fn get_profile_by_username(
//     pool: web::Data<PgPool>,
//     username: web::Path<String>,
//     req: actix_web::HttpRequest,
// ) -> impl Responder {
//     let current_user_id = req.extensions().get::<Uuid>().copied();

//     // Get user profile
//     let user = sqlx::query!("SELECT * FROM users WHERE username = $1", username.as_str())
//         .fetch_optional(pool.get_ref())
//         .await;

//     match user {
//         Ok(Some(user)) => {
//             // Check if current user is following this profile
//             let is_following = if let Some(current_id) = current_user_id {
//                 sqlx::query!(
//                     "SELECT id FROM follows WHERE follower_id = $1 AND following_id = $2",
//                     current_id,
//                     user.id
//                 )
//                 .fetch_optional(pool.get_ref())
//                 .await
//                 .map(|row| row.is_some())
//                 .unwrap_or(false)
//             } else {
//                 false
//             };

//             let profile = ProfileResponse {
//                 id: user.id,
//                 username: user.username,
//                 display_name: user.display_name,
//                 bio: user.bio,
//                 avatar_url: user.avatar_url,
//                 banner_url: user.banner_url,
//                 location: user.location,
//                 website: user.website,
//                 verified: user.verified,
//                 premium: user.premium,
//                 follower_count: user.follower_count,
//                 following_count: user.following_count,
//                 post_count: user.post_count,
//                 is_following,
//                 is_blocked: false, // You'd implement blocking logic
//                 join_date: user.created_at,
//             };

//             HttpResponse::Ok().json(profile)
//         }
//         Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
//             "error": "User not found"
//         })),
//         Err(e) => {
//             log::error!("Database error: {}", e);
//             HttpResponse::InternalServerError().json(serde_json::json!({
//                 "error": "Internal server error"
//             }))
//         }
//     }
// }

// #[put("/profile")]
// pub async fn update_profile(
//     pool: web::Data<PgPool>,
//     req: actix_web::HttpRequest,
//     form: web::Json<UpdateProfileRequest>,
// ) -> impl Responder {
//     if let Err(errors) = form.validate() {
//         return HttpResponse::BadRequest().json(serde_json::json!({
//             "error": "Validation failed",
//             "details": errors
//         }));
//     }

//     let user_id = match req.extensions().get::<Uuid>() {
//         Some(id) => *id,
//         None => {
//             return HttpResponse::Unauthorized().json(serde_json::json!({
//                 "error": "Authentication required"
//             }));
//         }
//     };

//     let result = sqlx::query!(
//         r#"
//         UPDATE users
//         SET display_name = COALESCE($2, display_name),
//             bio = COALESCE($3, bio),
//             location = COALESCE($4, location),
//             website = COALESCE($5, website),
//             updated_at = NOW()
//         WHERE id = $1
//         "#,
//         user_id,
//         form.display_name,
//         form.bio,
//         form.location,
//         form.website
//     )
//     .execute(pool.get_ref())
//     .await;

//     match result {
//         Ok(_) => {
//             // Fetch updated profile
//             let updated_user = sqlx::query!("SELECT * FROM users WHERE id = $1", user_id)
//                 .fetch_one(pool.get_ref())
//                 .await;

//             match updated_user {
//                 Ok(user) => {
//                     let profile = ProfileResponse {
//                         id: user.id,
//                         username: user.username,
//                         display_name: user.display_name,
//                         bio: user.bio,
//                         avatar_url: user.avatar_url,
//                         banner_url: user.banner_url,
//                         location: user.location,
//                         website: user.website,
//                         verified: user.verified,
//                         premium: user.premium,
//                         follower_count: user.follower_count,
//                         following_count: user.following_count,
//                         post_count: user.post_count,
//                         is_following: false, // Not applicable for own profile
//                         is_blocked: false,
//                         join_date: user.created_at,
//                     };

//                     HttpResponse::Ok().json(profile)
//                 }
//                 Err(e) => {
//                     log::error!("Failed to fetch updated profile: {}", e);
//                     HttpResponse::InternalServerError().json(serde_json::json!({
//                         "error": "Profile updated but failed to fetch updated data"
//                     }))
//                 }
//             }
//         }
//         Err(e) => {
//             log::error!("Database error: {}", e);
//             HttpResponse::InternalServerError().json(serde_json::json!({
//                 "error": "Failed to update profile"
//             }))
//         }
//     }
// }

// #[post("/profile/avatar")]
// pub async fn upload_avatar(
//     pool: web::Data<PgPool>,
//     mut payload: Multipart,
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

//     while let Some(mut field) = payload.try_next().await.unwrap_or(None) {
//         let content_disposition = field.content_disposition();

//         if let Some(filename) = content_disposition.get_filename() {
//             let content_type = field.content_type();

//             // Validate image type
//             if let Some(mime_type) = content_type {
//                 if !mime_type.as_ref().starts_with("image/") {
//                     return HttpResponse::BadRequest().json(serde_json::json!({
//                         "error": "Only image files are allowed for avatars"
//                     }));
//                 }
//             }

//             // Generate unique filename
//             let file_id = Uuid::new_v4();
//             let extension = std::path::Path::new(filename)
//                 .extension()
//                 .and_then(|ext| ext.to_str())
//                 .unwrap_or("jpg");

//             let unique_filename = format!("avatar_{}_{}.{}", user_id, file_id, extension);
//             let file_path = format!("./uploads/avatars/{}", unique_filename);

//             // Create avatars directory
//             std::fs::create_dir_all("./uploads/avatars").unwrap_or(());

//             // Collect and save file data
//             let mut file_data = Vec::new();
//             while let Some(chunk) = field.try_next().await? {
//                 file_data.extend_from_slice(&chunk);
//             }

//             // Basic size limit for avatars (2MB)
//             if file_data.len() > 2 * 1024 * 1024 {
//                 return HttpResponse::BadRequest().json(serde_json::json!({
//                     "error": "Avatar file too large (max 2MB)"
//                 }));
//             }

//             // Write file
//             if let Err(e) = std::fs::write(&file_path, &file_data) {
//                 log::error!("Failed to write avatar file: {}", e);
//                 return HttpResponse::InternalServerError().json(serde_json::json!({
//                     "error": "Failed to save avatar file"
//                 }));
//             }

//             // Update user's avatar URL
//             let avatar_url = format!("/media/avatars/{}", unique_filename);
//             let update_result = sqlx::query!(
//                 "UPDATE users SET avatar_url = $1, updated_at = NOW() WHERE id = $2",
//                 avatar_url,
//                 user_id
//             )
//             .execute(pool.get_ref())
//             .await;

//             match update_result {
//                 Ok(_) => {
//                     return HttpResponse::Ok().json(serde_json::json!({
//                         "message": "Avatar updated successfully",
//                         "avatar_url": avatar_url
//                     }));
//                 }
//                 Err(e) => {
//                     log::error!("Failed to update avatar URL: {}", e);
//                     // Clean up file on database error
//                     let _ = std::fs::remove_file(&file_path);
//                     return HttpResponse::InternalServerError().json(serde_json::json!({
//                         "error": "Failed to update avatar URL"
//                     }));
//                 }
//             }
//         }
//     }

//     HttpResponse::BadRequest().json(serde_json::json!({
//         "error": "No valid image file found in request"
//     }))
// }

// #[post("/profile/banner")]
// pub async fn upload_banner(
//     pool: web::Data<PgPool>,
//     mut payload: Multipart,
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

//     // Similar implementation to avatar upload but for banner
//     // Banner files can be larger (5MB limit)

//     while let Some(mut field) = payload.try_next().await.unwrap_or(None) {
//         let content_disposition = field.content_disposition();

//         if let Some(filename) = content_disposition.get_filename() {
//             let content_type = field.content_type();

//             if let Some(mime_type) = content_type {
//                 if !mime_type.as_ref().starts_with("image/") {
//                     return HttpResponse::BadRequest().json(serde_json::json!({
//                         "error": "Only image files are allowed for banners"
//                     }));
//                 }
//             }

//             let file_id = Uuid::new_v4();
//             let extension = std::path::Path::new(filename)
//                 .extension()
//                 .and_then(|ext| ext.to_str())
//                 .unwrap_or("jpg");

//             let unique_filename = format!("banner_{}_{}.{}", user_id, file_id, extension);
//             let file_path = format!("./uploads/banners/{}", unique_filename);

//             std::fs::create_dir_all("./uploads/banners").unwrap_or(());

//             let mut file_data = Vec::new();
//             while let Some(chunk) = field.try_next().await? {
//                 file_data.extend_from_slice(&chunk);
//             }

//             // 5MB limit for banners
//             if file_data.len() > 5 * 1024 * 1024 {
//                 return HttpResponse::BadRequest().json(serde_json::json!({
//                     "error": "Banner file too large (max 5MB)"
//                 }));
//             }

//             if let Err(e) = std::fs::write(&file_path, &file_data) {
//                 log::error!("Failed to write banner file: {}", e);
//                 return HttpResponse::InternalServerError().json(serde_json::json!({
//                     "error": "Failed to save banner file"
//                 }));
//             }

//             let banner_url = format!("/media/banners/{}", unique_filename);
//             let update_result = sqlx::query!(
//                 "UPDATE users SET banner_url = $1, updated_at = NOW() WHERE id = $2",
//                 banner_url,
//                 user_id
//             )
//             .execute(pool.get_ref())
//             .await;

//             match update_result {
//                 Ok(_) => {
//                     return HttpResponse::Ok().json(serde_json::json!({
//                         "message": "Banner updated successfully",
//                         "banner_url": banner_url
//                     }));
//                 }
//                 Err(e) => {
//                     log::error!("Failed to update banner URL: {}", e);
//                     let _ = std::fs::remove_file(&file_path);
//                     return HttpResponse::InternalServerError().json(serde_json::json!({
//                         "error": "Failed to update banner URL"
//                     }));
//                 }
//             }
//         }
//     }

//     HttpResponse::BadRequest().json(serde_json::json!({
//         "error": "No valid image file found in request"
//     }))
// }

// #[get("/profile/{username}/stats")]
// pub async fn get_profile_stats(
//     pool: web::Data<PgPool>,
//     username: web::Path<String>,
// ) -> impl Responder {
//     // Get user ID from username
//     let user = sqlx::query!(
//         "SELECT id FROM users WHERE username = $1",
//         username.as_str()
//     )
//     .fetch_optional(pool.get_ref())
//     .await;

//     match user {
//         Ok(Some(user)) => {
//             // Calculate various statistics
//             let total_likes = sqlx::query!(
//                 "SELECT COUNT(*) as count FROM likes l JOIN posts p ON l.post_id = p.id WHERE p.user_id = $1",
//                 user.id
//             )
//             .fetch_one(pool.get_ref())
//             .await
//             .map(|row| row.count.unwrap_or(0))
//             .unwrap_or(0);

//             let total_reposts = sqlx::query!(
//                 "SELECT COUNT(*) as count FROM reposts r JOIN posts p ON r.post_id = p.id WHERE p.user_id = $1",
//                 user.id
//             )
//             .fetch_one(pool.get_ref())
//             .await
//             .map(|row| row.count.unwrap_or(0))
//             .unwrap_or(0);

//             let posts_this_month = sqlx::query!(
//                 "SELECT COUNT(*) as count FROM posts WHERE user_id = $1 AND created_at > NOW() - INTERVAL '30 days'",
//                 user.id
//             )
//             .fetch_one(pool.get_ref())
//             .await
//             .map(|row| row.count.unwrap_or(0))
//             .unwrap_or(0);

//             // Calculate engagement rate (simplified)
//             let total_posts = sqlx::query!(
//                 "SELECT COUNT(*) as count FROM posts WHERE user_id = $1",
//                 user.id
//             )
//             .fetch_one(pool.get_ref())
//             .await
//             .map(|row| row.count.unwrap_or(0))
//             .unwrap_or(0);

//             let engagement_rate = if total_posts > 0 {
//                 ((total_likes + total_reposts) as f64) / (total_posts as f64)
//             } else {
//                 0.0
//             };

//             let stats = ProfileStats {
//                 total_likes_received: total_likes,
//                 total_reposts_received: total_reposts,
//                 posts_this_month,
//                 engagement_rate,
//             };

//             HttpResponse::Ok().json(stats)
//         }
//         Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
//             "error": "User not found"
//         })),
//         Err(e) => {
//             log::error!("Database error: {}", e);
//             HttpResponse::InternalServerError().json(serde_json::json!({
//                 "error": "Internal server error"
//             }))
//         }
//     }
// }

// pub fn configure(cfg: &mut web::ServiceConfig) {
//     cfg.service(get_profile_by_username)
//         .service(update_profile)
//         .service(upload_avatar)
//         .service(upload_banner)
//         .service(get_profile_stats);
// }
