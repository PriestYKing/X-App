// use actix::{
//     Actor, ActorContext, ActorFutureExt, AsyncContext, ContextFutureSpawner, Handler, Message,
//     Recipient, StreamHandler, WrapFuture,
// };
// use actix_web::{get, web, Error, HttpRequest, HttpResponse};
// use actix_web_actors::ws;
// use chrono::{DateTime, Utc};
// use serde::{Deserialize, Serialize};
// use std::collections::HashMap;
// use uuid::Uuid;

// #[derive(Message)]
// #[rtype(result = "()")]
// pub struct Connect {
//     pub addr: Recipient<WsMessage>,
//     pub user_id: Uuid,
// }

// #[derive(Message)]
// #[rtype(result = "()")]
// pub struct Disconnect {
//     pub user_id: Uuid,
// }

// #[derive(Message, Clone)]
// #[rtype(result = "()")]
// pub struct WsMessage(pub String);

// #[derive(Serialize, Deserialize)]
// #[serde(tag = "type")]
// pub enum MessageType {
//     NewPost {
//         post_id: Uuid,
//         user_id: Uuid,
//         content: String,
//         username: String,
//         created_at: DateTime<Utc>,
//     },
//     NewLike {
//         post_id: Uuid,
//         user_id: Uuid,
//         username: String,
//     },
//     NewReply {
//         post_id: Uuid,
//         reply_id: Uuid,
//         user_id: Uuid,
//         username: String,
//         content: String,
//     },
//     NewFollow {
//         follower_id: Uuid,
//         following_id: Uuid,
//         follower_username: String,
//     },
//     UserOnline {
//         user_id: Uuid,
//         username: String,
//     },
//     UserOffline {
//         user_id: Uuid,
//         username: String,
//     },
//     Notification {
//         id: Uuid,
//         message: String,
//         notification_type: String,
//     },
//     AiResponse {
//         id: Uuid,
//         response: String,
//         context: Option<String>,
//     },
// }

// pub struct WebSocketServer {
//     sessions: HashMap<Uuid, Recipient<WsMessage>>,
// }

// impl WebSocketServer {
//     pub fn new() -> Self {
//         Self {
//             sessions: HashMap::new(),
//         }
//     }

//     fn send_message(&self, user_id: &Uuid, message: &str) {
//         if let Some(addr) = self.sessions.get(user_id) {
//             let _ = addr.do_send(WsMessage(message.to_string()));
//         }
//     }

//     pub fn broadcast_to_followers(&self, user_id: &Uuid, follower_ids: &[Uuid], message: &str) {
//         for follower_id in follower_ids {
//             self.send_message(follower_id, message);
//         }
//     }

//     pub fn broadcast_to_all(&self, message: &str) {
//         for addr in self.sessions.values() {
//             let _ = addr.do_send(WsMessage(message.to_string()));
//         }
//     }
// }

// impl Actor for WebSocketServer {
//     type Context = actix::Context<Self>;
// }

// impl Handler<Connect> for WebSocketServer {
//     type Result = ();

//     fn handle(&mut self, msg: Connect, _: &mut Self::Context) {
//         self.sessions.insert(msg.user_id, msg.addr);

//         // Broadcast user online status
//         let online_msg = MessageType::UserOnline {
//             user_id: msg.user_id,
//             username: "User".to_string(), // You'd get this from the database
//         };

//         if let Ok(json_msg) = serde_json::to_string(&online_msg) {
//             self.broadcast_to_all(&json_msg);
//         }
//     }
// }

// impl Handler<Disconnect> for WebSocketServer {
//     type Result = ();

//     fn handle(&mut self, msg: Disconnect, _: &mut Self::Context) {
//         self.sessions.remove(&msg.user_id);

//         // Broadcast user offline status
//         let offline_msg = MessageType::UserOffline {
//             user_id: msg.user_id,
//             username: "User".to_string(),
//         };

//         if let Ok(json_msg) = serde_json::to_string(&offline_msg) {
//             self.broadcast_to_all(&json_msg);
//         }
//     }
// }

// impl Handler<WsMessage> for WebSocketServer {
//     type Result = ();

//     fn handle(&mut self, msg: WsMessage, _: &mut Self::Context) -> Self::Result {
//         // Handle incoming WebSocket messages
//         log::info!("Received WebSocket message: {}", msg.0);
//     }
// }

// pub struct WsSession {
//     pub user_id: Uuid,
//     pub addr: actix::Addr<WebSocketServer>,
// }

// impl Actor for WsSession {
//     type Context = ws::WebsocketContext<Self>;

//     fn started(&mut self, ctx: &mut Self::Context) {
//         let addr = ctx.address();
//         self.addr
//             .send(Connect {
//                 addr: addr.recipient(),
//                 user_id: self.user_id,
//             })
//             .into_actor(self)
//             .then(|_, _, ctx| {
//                 ctx.stop();
//                 actix::fut::ready(())
//             })
//             .wait(ctx);
//     }

//     fn stopping(&mut self, _: &mut Self::Context) -> actix::Running {
//         self.addr.do_send(Disconnect {
//             user_id: self.user_id,
//         });
//         actix::Running::Stop
//     }
// }

// impl Handler<WsMessage> for WsSession {
//     type Result = ();

//     fn handle(&mut self, msg: WsMessage, ctx: &mut Self::Context) {
//         ctx.text(msg.0);
//     }
// }

// impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsSession {
//     fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
//         match msg {
//             Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
//             Ok(ws::Message::Text(text)) => {
//                 // Handle incoming text messages
//                 log::info!("Received text: {}", text);
//                 // You can process different message types here
//             }
//             Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
//             _ => (),
//         }
//     }
// }

// #[get("/ws")]
// pub async fn websocket_handler(
//     req: HttpRequest,
//     stream: web::Payload,
//     srv: web::Data<actix::Addr<WebSocketServer>>,
// ) -> Result<HttpResponse, Error> {
//     // Extract user_id from JWT token (implement your auth logic)
//     let user_id = Uuid::new_v4(); // Replace with actual user extraction

//     let ws_session = WsSession {
//         user_id,
//         addr: srv.get_ref().clone(),
//     };

//     ws::start(ws_session, &req, stream)
// }

// pub fn configure(cfg: &mut web::ServiceConfig) {
//     cfg.service(websocket_handler);
// }
