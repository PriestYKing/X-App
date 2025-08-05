// use crate::models::post::*;
// use crate::services::{kafka_service::*, websocket_service::*};
// use actix_web::{get, post, web, HttpMessage, HttpResponse, Responder};
// use chrono::Utc;
// use sqlx::PgPool;
// use uuid::Uuid;

// #[get("/feed/realtime")]
// pub async fn get_realtime_feed(
//     pool: web::Data<PgPool>,
//     kafka: web::Data<KafkaService>,
//     ws_server: web::Data<actix::Addr<WebSocketServer>>,
//     req: actix_web::HttpRequest,
// ) -> impl Responder {
//     let user_id = match req.extensions().get::<Uuid>() {
//         Some(id) => *id,
//         None => {
//             return HttpResponse::Unauthorized().json(serde_json::json!({
//                 "error": "Unauthorized"
//             }));
//         }
//     };

//     // Get user's feed with real-time updates
//     let posts_result = sqlx::query_as::<_, Post>(
//         r#"
//     SELECT p.* FROM posts p
//     JOIN follows f ON p.user_id = f.following_id
//     WHERE f.follower_id = $1
//     ORDER BY p.created_at DESC
//     LIMIT 50
//     "#,
//     )
//     .bind(user_id)
//     .fetch_all(pool.get_ref())
//     .await;

//     match posts_result {
//         Ok(posts) => {
//             // Publish feed access event to Kafka for analytics
//             let event = PostEvent {
//                 event_type: "feed_accessed".to_string(),
//                 post_id: Uuid::new_v4(),
//                 user_id,
//                 content: None,
//                 timestamp: Utc::now(),
//             };

//             if let Err(e) = kafka.publish_post_event(event).await {
//                 log::error!("Failed to publish feed access event: {}", e);
//             }

//             HttpResponse::Ok().json(posts)
//         }
//         Err(e) => {
//             log::error!("Database error: {}", e);
//             HttpResponse::InternalServerError().json(serde_json::json!({
//                 "error": "Internal server error"
//             }))
//         }
//     }
// }

// #[post("/feed/ai-summary")]
// pub async fn get_ai_feed_summary(
//     pool: web::Data<PgPool>,
//     ai_service: web::Data<crate::services::ai_service::AiService>,
//     req: actix_web::HttpRequest,
// ) -> impl Responder {
//     let user_id = match req.extensions().get::<Uuid>() {
//         Some(id) => *id,
//         None => {
//             return HttpResponse::Unauthorized().json(serde_json::json!({
//                 "error": "Unauthorized"
//             }));
//         }
//     };

//     // Get recent posts for AI summarization
//     let posts_result = sqlx::query!(
//         r#"
//         SELECT content FROM posts p
//         JOIN follows f ON p.user_id = f.following_id
//         WHERE f.follower_id = $1 AND p.created_at > NOW() - INTERVAL '24 hours'
//         ORDER BY p.created_at DESC
//         LIMIT 20
//         "#,
//         user_id
//     )
//     .fetch_all(pool.get_ref())
//     .await;

//     match posts_result {
//         Ok(posts) => {
//             let post_contents: Vec<String> = posts.into_iter().map(|p| p.content).collect();

//             match ai_service.summarize_thread(post_contents).await {
//                 Ok(summary) => HttpResponse::Ok().json(serde_json::json!({
//                     "summary": summary,
//                     "generated_at": Utc::now()
//                 })),
//                 Err(e) => {
//                     log::error!("AI summary error: {}", e);
//                     HttpResponse::InternalServerError().json(serde_json::json!({
//                         "error": "Failed to generate AI summary"
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
//     cfg.service(get_realtime_feed).service(get_ai_feed_summary);
// }
