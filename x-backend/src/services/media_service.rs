// use actix_multipart::Multipart;
// use actix_web::{get, post, web, Error, HttpMessage, HttpResponse, Responder};
// use futures_util::TryStreamExt;
// use serde::{Deserialize, Serialize};
// use sqlx::PgPool;
// use std::io::Write;
// use uuid::Uuid;

// #[derive(Debug, Serialize)]
// pub struct MediaUploadResponse {
//     pub id: Uuid,
//     pub file_url: String,
//     pub file_type: String,
//     pub file_size: i64,
//     pub thumbnail_url: Option<String>,
// }

// #[derive(Debug, Deserialize)]
// pub struct MediaQuery {
//     pub limit: Option<i64>,
//     pub offset: Option<i64>,
//     pub file_type: Option<String>,
// }

// const MAX_FILE_SIZE: usize = 10 * 1024 * 1024; // 10MB
// const ALLOWED_TYPES: &[&str] = &[
//     "image/jpeg",
//     "image/png",
//     "image/gif",
//     "video/mp4",
//     "video/webm",
// ];

// #[post("/media/upload")]
// pub async fn upload_media(
//     pool: web::Data<PgPool>,
//     mut payload: Multipart,
//     req: actix_web::HttpRequest,
// ) -> Result<HttpResponse, Error> {
//     let user_id = match req.extensions().get::<Uuid>() {
//         Some(id) => *id,
//         None => {
//             return Ok(HttpResponse::Unauthorized().json(serde_json::json!({
//                 "error": "Authentication required"
//             })));
//         }
//     };

//     while let Some(mut field) = payload.try_next().await? {
//         // EXTRACT ALL METADATA FIRST (immutable borrows)
//         let content_disposition = field.content_disposition().clone();
//         let content_type = field.content_type().cloned();

//         // Get filename as owned String
//         let filename = content_disposition.get_filename().map(|s| s.to_string());

//         if let Some(filename) = filename {
//             // Validate file type using the cloned content_type
//             if let Some(mime_type) = &content_type {
//                 if !ALLOWED_TYPES.contains(&mime_type.as_ref()) {
//                     return Ok(HttpResponse::BadRequest().json(serde_json::json!({
//                         "error": "Unsupported file type",
//                         "allowed_types": ALLOWED_TYPES
//                     })));
//                 }
//             } else {
//                 return Ok(HttpResponse::BadRequest().json(serde_json::json!({
//                     "error": "Content type not specified"
//                 })));
//             }

//             // Generate unique filename
//             let file_id = Uuid::new_v4();
//             let extension = std::path::Path::new(&filename)
//                 .extension()
//                 .and_then(|ext| ext.to_str())
//                 .unwrap_or("bin");

//             let unique_filename = format!("{}.{}", file_id, extension);
//             let file_path = format!("./uploads/{}", unique_filename);

//             // Create uploads directory
//             std::fs::create_dir_all("./uploads").unwrap_or(());

//             // NOW USE MUTABLE BORROWS (no conflict since immutable borrows are done)
//             let mut file_data = Vec::new();
//             let mut total_size = 0;

//             while let Some(chunk) = field.try_next().await? {
//                 total_size += chunk.len();
//                 if total_size > MAX_FILE_SIZE {
//                     return Ok(HttpResponse::BadRequest().json(serde_json::json!({
//                         "error": "File too large",
//                         "max_size": MAX_FILE_SIZE
//                     })));
//                 }
//                 file_data.extend_from_slice(&chunk);
//             }

//             // Write file to disk
//             if let Err(e) = std::fs::write(&file_path, &file_data) {
//                 log::error!("Failed to write file: {}", e);
//                 return Ok(HttpResponse::InternalServerError().json(serde_json::json!({
//                     "error": "Failed to save file"
//                 })));
//             }

//             // Determine file type category
//             let file_type = match content_type.as_ref().map(|ct| ct.as_ref()) {
//                 Some(mime) if mime.starts_with("image/") => "image",
//                 Some(mime) if mime.starts_with("video/") => "video",
//                 _ => "other",
//             };

//             // Save to database and return response...
//             // (rest of your implementation)
//         }
//     }

//     Ok(HttpResponse::BadRequest().json(serde_json::json!({
//         "error": "No valid file found in request"
//     })))
// }

// #[get("/media/{filename}")]
// pub async fn serve_media(
//     filename: web::Path<String>,
//     req: actix_web::HttpRequest, // Add as parameter
// ) -> Result<HttpResponse, Error> {
//     let file_path = format!("./uploads/{}", filename.as_str());

//     match actix_files::NamedFile::open(&file_path) {
//         Ok(file) => Ok(file.into_response(&req)), // Use the actual request
//         Err(_) => Ok(HttpResponse::NotFound().json(serde_json::json!({
//             "error": "File not found"
//         }))),
//     }
// }

// #[get("/media/user/{user_id}")]
// pub async fn get_user_media(
//     pool: web::Data<PgPool>,
//     user_id: web::Path<Uuid>,
//     query: web::Query<MediaQuery>,
// ) -> impl Responder {
//     let limit = query.limit.unwrap_or(20).min(100);
//     let offset = query.offset.unwrap_or(0);

//     let media_query = if let Some(file_type) = &query.file_type {
//         sqlx::query!(
//             "SELECT * FROM media WHERE user_id = $1 AND file_type = $2 ORDER BY created_at DESC LIMIT $3 OFFSET $4",
//             *user_id,
//             file_type,
//             limit,
//             offset
//         )
//         .fetch_all(pool.get_ref())
//         .await
//     } else {
//         sqlx::query_as::<_, serde_json::Value>(
//     "SELECT id, file_url, file_type, file_size, thumbnail_url, created_at FROM media WHERE user_id = $1 ORDER BY created_at DESC LIMIT $2 OFFSET $3"
// )
// .bind(*user_id)
// .bind(limit)
// .bind(offset)
// .fetch_all(pool.get_ref())
// .await
//     };

//     match media_query {
//         Ok(media) => {
//             let response: Vec<serde_json::Value> = media
//                 .into_iter()
//                 .map(|m| {
//                     serde_json::json!({
//                         "id": m.id,
//                         "file_url": m.file_url,
//                         "file_type": m.file_type,
//                         "file_size": m.file_size,
//                         "thumbnail_url": m.thumbnail_url,
//                         "created_at": m.created_at
//                     })
//                 })
//                 .collect();

//             HttpResponse::Ok().json(response)
//         }
//         Err(e) => {
//             log::error!("Database error: {}", e);
//             HttpResponse::InternalServerError().json(serde_json::json!({
//                 "error": "Failed to fetch media"
//             }))
//         }
//     }
// }

// // Image resizing utility (basic implementation)
// pub async fn resize_image(
//     input_path: &str,
//     output_path: &str,
//     width: u32,
//     height: u32,
// ) -> Result<(), Box<dyn std::error::Error>> {
//     // This is a placeholder - you'd use image processing crates like `image` or `photon`
//     // For now, just copy the file
//     std::fs::copy(input_path, output_path)?;
//     Ok(())
// }

// pub fn configure(cfg: &mut web::ServiceConfig) {
//     cfg.service(upload_media)
//         .service(serve_media)
//         .service(get_user_media);
// }
