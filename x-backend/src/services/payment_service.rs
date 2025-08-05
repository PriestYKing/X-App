// use actix_web::{get, post, web, HttpMessage, HttpResponse, Responder};
// use serde::{Deserialize, Serialize};
// use sqlx::PgPool;
// use std::collections::HashMap;
// use uuid::Uuid;
// use validator::Validate;

// #[derive(Debug, Deserialize, Validate)]
// pub struct CreatePaymentIntentRequest {
//     #[validate(range(min = 50, max = 100000))] // $0.50 to $1000
//     pub amount_cents: i32,
//     pub currency: String,
//     pub description: Option<String>,
//     pub recipient_user_id: Option<Uuid>, // For tips
// }

// #[derive(Debug, Deserialize, Validate)]
// pub struct SubscriptionRequest {
//     pub plan_id: String,
//     pub payment_method_id: String,
// }

// #[derive(Debug, Serialize)]
// pub struct PaymentIntentResponse {
//     pub client_secret: String,
//     pub payment_intent_id: String,
//     pub amount: i32,
//     pub currency: String,
// }

// #[derive(Debug, Serialize)]
// pub struct SubscriptionResponse {
//     pub subscription_id: String,
//     pub status: String,
//     pub current_period_end: i64,
// }

// #[derive(Debug, Serialize)]
// pub struct PaymentHistory {
//     pub id: Uuid,
//     pub amount: i32,
//     pub currency: String,
//     pub status: String,
//     pub description: Option<String>,
//     pub created_at: chrono::DateTime<chrono::Utc>,
// }

// // Mock Stripe-like payment service
// pub struct StripeService {
//     api_key: String,
//     client: reqwest::Client,
// }

// impl StripeService {
//     pub fn new(api_key: String) -> Self {
//         Self {
//             api_key,
//             client: reqwest::Client::new(),
//         }
//     }

//     pub async fn create_payment_intent(
//         &self,
//         amount: i32,
//         currency: &str,
//         description: Option<&str>,
//     ) -> Result<PaymentIntentResponse, Box<dyn std::error::Error>> {
//         // In a real implementation, you'd call Stripe's API
//         let mut params = HashMap::new();
//         params.insert("amount", amount.to_string());
//         params.insert("currency", currency.to_string());

//         if let Some(desc) = description {
//             params.insert("description", desc.to_string());
//         }

//         // Mock response for development
//         let mock_response = PaymentIntentResponse {
//             client_secret: format!("pi_mock_{}_secret", Uuid::new_v4()),
//             payment_intent_id: format!("pi_mock_{}", Uuid::new_v4()),
//             amount,
//             currency: currency.to_string(),
//         };

//         Ok(mock_response)
//     }

//     pub async fn create_subscription(
//         &self,
//         customer_id: &str,
//         price_id: &str,
//         payment_method_id: &str,
//     ) -> Result<SubscriptionResponse, Box<dyn std::error::Error>> {
//         // Mock subscription response
//         let mock_response = SubscriptionResponse {
//             subscription_id: format!("sub_mock_{}", Uuid::new_v4()),
//             status: "active".to_string(),
//             current_period_end: chrono::Utc::now().timestamp() + (30 * 24 * 60 * 60), // 30 days
//         };

//         Ok(mock_response)
//     }
// }

// #[post("/payments/create-intent")]
// pub async fn create_payment_intent(
//     pool: web::Data<PgPool>,
//     stripe: web::Data<StripeService>,
//     req: actix_web::HttpRequest,
//     form: web::Json<CreatePaymentIntentRequest>,
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

//     match stripe
//         .create_payment_intent(
//             form.amount_cents,
//             &form.currency,
//             form.description.as_deref(),
//         )
//         .await
//     {
//         Ok(payment_intent) => {
//             // Save payment record to database
//             let payment_record = sqlx::query!(
//                 r#"
//                 INSERT INTO payments (id, user_id, amount_cents, currency, status, stripe_payment_intent_id, description)
//                 VALUES ($1, $2, $3, $4, $5, $6, $7)
//                 "#,
//                 Uuid::new_v4(),
//                 user_id,
//                 form.amount_cents,
//                 form.currency,
//                 "pending",
//                 payment_intent.payment_intent_id,
//                 form.description
//             )
//             .execute(pool.get_ref())
//             .await;

//             match payment_record {
//                 Ok(_) => HttpResponse::Ok().json(payment_intent),
//                 Err(e) => {
//                     log::error!("Failed to save payment record: {}", e);
//                     HttpResponse::InternalServerError().json(serde_json::json!({
//                         "error": "Failed to create payment record"
//                     }))
//                 }
//             }
//         }
//         Err(e) => {
//             log::error!("Stripe error: {}", e);
//             HttpResponse::InternalServerError().json(serde_json::json!({
//                 "error": "Failed to create payment intent"
//             }))
//         }
//     }
// }

// #[post("/payments/tip")]
// pub async fn send_tip(
//     pool: web::Data<PgPool>,
//     stripe: web::Data<StripeService>,
//     req: actix_web::HttpRequest,
//     form: web::Json<CreatePaymentIntentRequest>,
// ) -> impl Responder {
//     if let Err(errors) = form.validate() {
//         return HttpResponse::BadRequest().json(serde_json::json!({
//             "error": "Validation failed",
//             "details": errors
//         }));
//     }

//     let sender_id = match req.extensions().get::<Uuid>() {
//         Some(id) => *id,
//         None => {
//             return HttpResponse::Unauthorized().json(serde_json::json!({
//                 "error": "Authentication required"
//             }));
//         }
//     };

//     let recipient_id = match form.recipient_user_id {
//         Some(id) => id,
//         None => {
//             return HttpResponse::BadRequest().json(serde_json::json!({
//                 "error": "Recipient user ID required for tips"
//             }));
//         }
//     };

//     // Verify recipient exists
//     let recipient = sqlx::query!("SELECT username FROM users WHERE id = $1", recipient_id)
//         .fetch_optional(pool.get_ref())
//         .await;

//     match recipient {
//         Ok(Some(user)) => {
//             let description = format!("Tip to @{}", user.username);

//             match stripe
//                 .create_payment_intent(form.amount_cents, &form.currency, Some(&description))
//                 .await
//             {
//                 Ok(payment_intent) => {
//                     // Save tip record
//                     let tip_record = sqlx::query!(
//                         r#"
//                         INSERT INTO tips (id, sender_id, recipient_id, amount_cents, currency, status, stripe_payment_intent_id)
//                         VALUES ($1, $2, $3, $4, $5, $6, $7)
//                         "#,
//                         Uuid::new_v4(),
//                         sender_id,
//                         recipient_id,
//                         form.amount_cents,
//                         form.currency,
//                         "pending",
//                         payment_intent.payment_intent_id
//                     )
//                     .execute(pool.get_ref())
//                     .await;

//                     match tip_record {
//                         Ok(_) => HttpResponse::Ok().json(payment_intent),
//                         Err(e) => {
//                             log::error!("Failed to save tip record: {}", e);
//                             HttpResponse::InternalServerError().json(serde_json::json!({
//                                 "error": "Failed to create tip record"
//                             }))
//                         }
//                     }
//                 }
//                 Err(e) => {
//                     log::error!("Stripe error: {}", e);
//                     HttpResponse::InternalServerError().json(serde_json::json!({
//                         "error": "Failed to create payment intent"
//                     }))
//                 }
//             }
//         }
//         Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
//             "error": "Recipient user not found"
//         })),
//         Err(e) => {
//             log::error!("Database error: {}", e);
//             HttpResponse::InternalServerError().json(serde_json::json!({
//                 "error": "Internal server error"
//             }))
//         }
//     }
// }

// #[post("/payments/subscribe")]
// pub async fn create_subscription(
//     pool: web::Data<PgPool>,
//     stripe: web::Data<StripeService>,
//     req: actix_web::HttpRequest,
//     form: web::Json<SubscriptionRequest>,
// ) -> impl Responder {
//     let user_id = match req.extensions().get::<Uuid>() {
//         Some(id) => *id,
//         None => {
//             return HttpResponse::Unauthorized().json(serde_json::json!({
//                 "error": "Authentication required"
//             }));
//         }
//     };

//     // In a real implementation, you'd first create a Stripe customer if not exists
//     let customer_id = format!("cust_mock_{}", user_id);

//     match stripe
//         .create_subscription(&customer_id, &form.plan_id, &form.payment_method_id)
//         .await
//     {
//         Ok(subscription) => {
//             // Update user to premium status
//             let update_result =
//                 sqlx::query!("UPDATE users SET premium = TRUE WHERE id = $1", user_id)
//                     .execute(pool.get_ref())
//                     .await;

//             match update_result {
//                 Ok(_) => HttpResponse::Ok().json(subscription),
//                 Err(e) => {
//                     log::error!("Failed to update user premium status: {}", e);
//                     HttpResponse::InternalServerError().json(serde_json::json!({
//                         "error": "Failed to update premium status"
//                     }))
//                 }
//             }
//         }
//         Err(e) => {
//             log::error!("Stripe subscription error: {}", e);
//             HttpResponse::InternalServerError().json(serde_json::json!({
//                 "error": "Failed to create subscription"
//             }))
//         }
//     }
// }

// #[get("/payments/history")]
// pub async fn get_payment_history(
//     pool: web::Data<PgPool>,
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

//     let payments = sqlx::query!(
//         "SELECT * FROM payments WHERE user_id = $1 ORDER BY created_at DESC LIMIT 50",
//         user_id
//     )
//     .fetch_all(pool.get_ref())
//     .await;

//     match payments {
//         Ok(payments) => {
//             let history: Vec<PaymentHistory> = payments
//                 .into_iter()
//                 .map(|p| PaymentHistory {
//                     id: p.id,
//                     amount: p.amount_cents,
//                     currency: p.currency,
//                     status: p.status,
//                     description: p.description,
//                     created_at: p.created_at,
//                 })
//                 .collect();

//             HttpResponse::Ok().json(history)
//         }
//         Err(e) => {
//             log::error!("Database error: {}", e);
//             HttpResponse::InternalServerError().json(serde_json::json!({
//                 "error": "Failed to fetch payment history"
//             }))
//         }
//     }
// }

// pub fn configure(cfg: &mut web::ServiceConfig) {
//     cfg.service(create_payment_intent)
//         .service(send_tip)
//         .service(create_subscription)
//         .service(get_payment_history);
// }
