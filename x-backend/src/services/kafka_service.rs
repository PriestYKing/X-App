// use chrono::{DateTime, Utc};
// use rdkafka::{
//     consumer::{Consumer, StreamConsumer},
//     producer::{FutureProducer, FutureRecord},
//     ClientConfig, Message as KafkaMessage,
// };
// use serde::{Deserialize, Serialize};
// use tokio::time::Duration;
// use uuid::Uuid;

// #[derive(Serialize, Deserialize, Clone)]
// pub struct PostEvent {
//     pub event_type: String,
//     pub post_id: Uuid,
//     pub user_id: Uuid,
//     pub content: Option<String>,
//     pub timestamp: DateTime<Utc>,
// }

// #[derive(Serialize, Deserialize, Clone)]
// pub struct NotificationEvent {
//     pub event_type: String,
//     pub user_id: Uuid,
//     pub from_user_id: Option<Uuid>,
//     pub post_id: Option<Uuid>,
//     pub message: String,
//     pub timestamp: DateTime<Utc>,
// }

// #[derive(Serialize, Deserialize, Clone)]
// pub struct AiEvent {
//     pub event_type: String,
//     pub user_id: Uuid,
//     pub request_id: Uuid,
//     pub prompt: String,
//     pub context: Option<String>,
//     pub timestamp: DateTime<Utc>,
// }

// pub struct KafkaService {
//     producer: FutureProducer,
//     consumer: StreamConsumer,
// }

// impl KafkaService {
//     pub fn new(bootstrap_servers: &str) -> Result<Self, rdkafka::error::KafkaError> {
//         let producer: FutureProducer = ClientConfig::new()
//             .set("bootstrap.servers", bootstrap_servers)
//             .set("message.timeout.ms", "5000")
//             .set("retry.backoff.ms", "100")
//             .create()?;

//         let consumer: StreamConsumer = ClientConfig::new()
//             .set("group.id", "twitter_clone_group")
//             .set("bootstrap.servers", bootstrap_servers)
//             .set("enable.partition.eof", "false")
//             .set("session.timeout.ms", "6000")
//             .set("enable.auto.commit", "true")
//             .create()?;

//         Ok(KafkaService { producer, consumer })
//     }

//     pub async fn publish_post_event(
//         &self,
//         event: PostEvent,
//     ) -> Result<(), Box<dyn std::error::Error>> {
//         let payload = serde_json::to_string(&event)?;
//         let key = event.post_id.to_string();

//         // Create the record with proper lifetime management
//         let record = FutureRecord::to("post_events").key(&key).payload(&payload);

//         self.producer
//             .send(record, Duration::from_secs(0))
//             .await
//             .map_err(|(e, _)| e)?;

//         Ok(())
//     }

//     pub async fn publish_notification_event(
//         &self,
//         event: NotificationEvent,
//     ) -> Result<(), Box<dyn std::error::Error>> {
//         let payload = serde_json::to_string(&event)?;
//         let key = event.user_id.to_string();

//         // Create the record with proper lifetime management
//         let record = FutureRecord::to("notification_events")
//             .key(&key)
//             .payload(&payload);

//         self.producer
//             .send(record, Duration::from_secs(0))
//             .await
//             .map_err(|(e, _)| e)?;

//         Ok(())
//     }

//     pub async fn publish_ai_event(&self, event: AiEvent) -> Result<(), Box<dyn std::error::Error>> {
//         let payload = serde_json::to_string(&event)?;
//         let key = event.request_id.to_string();

//         // Create the record with proper lifetime management
//         let record = FutureRecord::to("ai_events").key(&key).payload(&payload);

//         self.producer
//             .send(record, Duration::from_secs(0))
//             .await
//             .map_err(|(e, _)| e)?;

//         Ok(())
//     }

//     pub async fn subscribe_to_events(&self) -> Result<(), Box<dyn std::error::Error>> {
//         use futures_util::stream::StreamExt;
//         use rdkafka::consumer::StreamConsumer;

//         self.consumer
//             .subscribe(&["post_events", "notification_events", "ai_events"])?;

//         let mut stream = self.consumer.stream();

//         while let Some(message) = stream.next().await {
//             match message {
//                 Ok(m) => {
//                     let topic = m.topic();
//                     let payload = match m.payload_view::<str>() {
//                         Some(Ok(s)) => s,
//                         Some(Err(e)) => {
//                             log::error!("Error while deserializing message payload: {:?}", e);
//                             continue;
//                         }
//                         None => {
//                             log::warn!("Empty payload received");
//                             continue;
//                         }
//                     };

//                     match topic {
//                         "post_events" => {
//                             if let Ok(event) = serde_json::from_str::<PostEvent>(payload) {
//                                 self.handle_post_event(event).await;
//                             }
//                         }
//                         "notification_events" => {
//                             if let Ok(event) = serde_json::from_str::<NotificationEvent>(payload) {
//                                 self.handle_notification_event(event).await;
//                             }
//                         }
//                         "ai_events" => {
//                             if let Ok(event) = serde_json::from_str::<AiEvent>(payload) {
//                                 self.handle_ai_event(event).await;
//                             }
//                         }
//                         _ => log::warn!("Unknown topic: {}", topic),
//                     }
//                 }
//                 Err(e) => log::error!("Kafka consumer error: {}", e),
//             }
//         }

//         Ok(())
//     }

//     async fn handle_post_event(&self, event: PostEvent) {
//         log::info!("Handling post event: {:?}", event.event_type);
//         // Process post events (update feeds, notify followers, etc.)
//     }

//     async fn handle_notification_event(&self, event: NotificationEvent) {
//         log::info!("Handling notification event for user: {}", event.user_id);
//         // Send real-time notifications via WebSocket
//     }

//     async fn handle_ai_event(&self, event: AiEvent) {
//         log::info!("Handling AI event: {}", event.request_id);
//         // Process AI requests asynchronously
//     }
// }
