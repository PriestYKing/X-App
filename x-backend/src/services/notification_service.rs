// use crate::models::notifiication::Notification;
// use crate::services::{kafka_service::*, websocket_service::*};
// use actix_web::{get, post, web, HttpMessage, HttpResponse, Responder};
// use chrono::{DateTime, Utc};
// use serde::{Deserialize, Serialize};
// use sqlx::PgPool;
// use uuid::Uuid;

// #[get("/notifications/realtime")]
// pub async fn get_realtime_notifications(
//     pool: web::Data<PgPool>,
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

//     let notifications_result = sqlx::query_as::<_, Notification>(
//         "SELECT * FROM notifications WHERE user_id = $1 ORDER BY created_at DESC LIMIT 50",
//     )
//     .bind(user_id)
//     .fetch_all(pool.get_ref())
//     .await;

//     match notifications_result {
//         Ok(notifications) => HttpResponse::Ok().json(notifications),
//         Err(e) => {
//             log::error!("Database error: {}", e);
//             HttpResponse::InternalServerError().json(serde_json::json!({
//                 "error": "Internal server error"
//             }))
//         }
//     }
// }

// pub async fn send_realtime_notification(
//     pool: &PgPool,
//     kafka: &KafkaService,
//     ws_server: &actix::Addr<WebSocketServer>,
//     notification: Notification,
// ) -> Result<(), Box<dyn std::error::Error>> {
//     // Save to database
//     sqlx::query!(
//         r#"
//         INSERT INTO notifications (id, user_id, from_user_id, post_id, notification_type, message)
//         VALUES ($1, $2, $3, $4, $5, $6)
//         "#,
//         notification.id,
//         notification.user_id,
//         notification.from_user_id,
//         notification.post_id,
//         notification.notification_type,
//         notification.message
//     )
//     .execute(pool)
//     .await?;

//     // Publish to Kafka
//     let kafka_event = crate::services::kafka_service::NotificationEvent {
//         event_type: notification.notification_type.clone(),
//         user_id: notification.user_id,
//         from_user_id: notification.from_user_id,
//         post_id: notification.post_id,
//         message: notification.message.clone(),
//         timestamp: notification.created_at,
//     };

//     kafka.publish_notification_event(kafka_event).await?;

//     // Send via WebSocket
//     let ws_message = crate::services::websocket_service::MessageType::Notification {
//         id: notification.id,
//         message: notification.message,
//         notification_type: notification.notification_type,
//     };

//     if let Ok(json_msg) = serde_json::to_string(&ws_message) {
//         // Send to specific user (implement this method in WebSocketServer)
//         // ws_server.do_send(SendToUser { user_id: notification.user_id, message: json_msg });
//     }

//     Ok(())
// }

// pub fn configure(cfg: &mut web::ServiceConfig) {
//     cfg.service(get_realtime_notifications);
// }
