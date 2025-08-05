// use chrono::{DateTime, Utc};
// use futures_util::TryStreamExt;
// use lapin::{
//     options::*, publisher_confirm::Confirmation, types::FieldTable, BasicProperties, Connection,
//     ConnectionProperties, Result,
// };
// use serde::{Deserialize, Serialize};
// use std::sync::Arc;
// use uuid::Uuid;

// #[derive(Serialize, Deserialize, Clone)]
// pub struct RabbitMQMessage {
//     pub id: Uuid,
//     pub message_type: String,
//     pub payload: serde_json::Value,
//     pub timestamp: DateTime<Utc>,
// }

// #[derive(Clone)]
// pub struct RabbitMQService {
//     connection: Arc<Connection>,
// }

// impl RabbitMQService {
//     pub async fn new(amqp_url: &str) -> Result<Self> {
//         let connection = Connection::connect(amqp_url, ConnectionProperties::default()).await?;
//         Ok(Self { connection })
//     }

//     pub async fn setup_queues(&self) -> Result<()> {
//         let channel = self.connection.create_channel().await?;

//         // Declare exchanges with durability
//         for exchange_name in ["posts", "notifications", "ai"] {
//             channel
//                 .exchange_declare(
//                     exchange_name,
//                     lapin::ExchangeKind::Topic,
//                     ExchangeDeclareOptions {
//                         durable: true,
//                         ..ExchangeDeclareOptions::default()
//                     },
//                     FieldTable::default(),
//                 )
//                 .await?;
//         }

//         // Declare durable queues
//         for queue_name in [
//             "post_feed_updates",
//             "real_time_notifications",
//             "ai_processing",
//         ] {
//             channel
//                 .queue_declare(
//                     queue_name,
//                     QueueDeclareOptions {
//                         durable: true,
//                         ..QueueDeclareOptions::default()
//                     },
//                     FieldTable::default(),
//                 )
//                 .await?;
//         }

//         // Bind queues to exchanges
//         let bindings = [
//             ("post_feed_updates", "posts", "post.*"),
//             ("real_time_notifications", "notifications", "notification.*"),
//             ("ai_processing", "ai", "ai.*"),
//         ];

//         for (queue, exchange, routing_key) in bindings {
//             channel
//                 .queue_bind(
//                     queue,
//                     exchange,
//                     routing_key,
//                     QueueBindOptions::default(),
//                     FieldTable::default(),
//                 )
//                 .await?;
//         }

//         Ok(())
//     }

//     pub async fn publish_message(
//         &self,
//         exchange: &str,
//         routing_key: &str,
//         message: RabbitMQMessage,
//     ) -> Result<Confirmation> {
//         let channel = self.connection.create_channel().await?;
//         let payload = serde_json::to_vec(&message)
//             .map_err(|e| lapin::Error::InvalidChannel(format!("Serialization error: {}", e)))?;

//         let confirm = channel
//             .basic_publish(
//                 exchange,
//                 routing_key,
//                 BasicPublishOptions::default(),
//                 &payload,
//                 BasicProperties::default().with_delivery_mode(2), // Make message persistent
//             )
//             .await?
//             .await?;

//         Ok(confirm)
//     }

//     pub async fn consume_messages<F>(&self, queue: &str, handler: F) -> Result<()>
//     where
//         F: Fn(RabbitMQMessage) + Send + 'static,
//     {
//         let channel = self.connection.create_channel().await?;

//         // Set QoS to process one message at a time
//         channel.basic_qos(1, BasicQosOptions::default()).await?;

//         let mut consumer = channel
//             .basic_consume(
//                 queue,
//                 "consumer",
//                 BasicConsumeOptions::default(),
//                 FieldTable::default(),
//             )
//             .await?;

//         log::info!("Starting to consume messages from queue: {}", queue);

//         while let Some(delivery) = consumer.try_next().await? {
//             match serde_json::from_slice::<RabbitMQMessage>(&delivery.data) {
//                 Ok(message) => {
//                     handler(message);
//                     delivery.ack(BasicAckOptions::default()).await?;
//                 }
//                 Err(e) => {
//                     log::error!("Failed to deserialize message: {}", e);
//                     // Reject and don't requeue malformed messages
//                     delivery
//                         .nack(BasicNackOptions {
//                             requeue: false,
//                             ..BasicNackOptions::default()
//                         })
//                         .await?;
//                 }
//             }
//         }

//         Ok(())
//     }

//     // Helper method to create different types of messages
//     pub fn create_post_message(post_id: Uuid, user_id: Uuid, content: String) -> RabbitMQMessage {
//         RabbitMQMessage {
//             id: Uuid::new_v4(),
//             message_type: "post_created".to_string(),
//             payload: serde_json::json!({
//                 "post_id": post_id,
//                 "user_id": user_id,
//                 "content": content
//             }),
//             timestamp: Utc::now(),
//         }
//     }

//     pub fn create_notification_message(
//         user_id: Uuid,
//         message: String,
//         notification_type: String,
//     ) -> RabbitMQMessage {
//         RabbitMQMessage {
//             id: Uuid::new_v4(),
//             message_type: notification_type,
//             payload: serde_json::json!({
//                 "user_id": user_id,
//                 "message": message
//             }),
//             timestamp: Utc::now(),
//         }
//     }

//     pub fn create_ai_message(
//         request_id: Uuid,
//         prompt: String,
//         context: Option<String>,
//     ) -> RabbitMQMessage {
//         RabbitMQMessage {
//             id: request_id,
//             message_type: "ai_request".to_string(),
//             payload: serde_json::json!({
//                 "prompt": prompt,
//                 "context": context
//             }),
//             timestamp: Utc::now(),
//         }
//     }
// }
