// use actix_web::{get, web, HttpMessage, HttpResponse, Responder};
// use serde::{Deserialize, Serialize};
// use sqlx::PgPool;
// use std::collections::HashMap;
// use uuid::Uuid;

// #[derive(Debug, Deserialize)]
// pub struct SearchQuery {
//     pub q: String,
//     pub search_type: Option<String>, // "users", "posts", "hashtags", "all"
//     pub limit: Option<i64>,
//     pub offset: Option<i64>,
// }

// #[derive(Debug, Serialize)]
// pub struct SearchResults {
//     pub users: Vec<UserSearchResult>,
//     pub posts: Vec<PostSearchResult>,
//     pub hashtags: Vec<HashtagResult>,
//     pub total_results: usize,
// }

// #[derive(Debug, Serialize)]
// pub struct UserSearchResult {
//     pub id: Uuid,
//     pub username: String,
//     pub display_name: Option<String>,
//     pub bio: Option<String>,
//     pub avatar_url: Option<String>,
//     pub verified: bool,
//     pub follower_count: i32,
// }

// #[derive(Debug, Serialize)]
// pub struct PostSearchResult {
//     pub id: Uuid,
//     pub user_id: Uuid,
//     pub username: String,
//     pub content: String,
//     pub like_count: i32,
//     pub repost_count: i32,
//     pub created_at: chrono::DateTime<chrono::Utc>,
// }

// #[derive(Debug, Serialize)]
// pub struct HashtagResult {
//     pub hashtag: String,
//     pub post_count: i64,
//     pub trending_score: f64,
// }

// #[derive(Debug, Serialize)]
// pub struct TrendingTopics {
//     pub hashtags: Vec<HashtagResult>,
//     pub updated_at: chrono::DateTime<chrono::Utc>,
// }

// // Mock Elasticsearch service
// pub struct SearchService {
//     pool: sqlx::PgPool,
// }

// impl SearchService {
//     pub fn new(pool: sqlx::PgPool) -> Self {
//         Self { pool }
//     }

//     pub async fn search_users(
//         &self,
//         query: &str,
//         limit: i64,
//     ) -> Result<Vec<UserSearchResult>, sqlx::Error> {
//         let users = sqlx::query!(
//             r#"
//             SELECT id, username, display_name, bio, avatar_url, verified, follower_count
//             FROM users
//             WHERE username ILIKE $1 OR display_name ILIKE $1 OR bio ILIKE $1
//             ORDER BY follower_count DESC
//             LIMIT $2
//             "#,
//             format!("%{}%", query),
//             limit
//         )
//         .fetch_all(&self.pool)
//         .await?;

//         Ok(users
//             .into_iter()
//             .map(|user| UserSearchResult {
//                 id: user.id,
//                 username: user.username,
//                 display_name: user.display_name,
//                 bio: user.bio,
//                 avatar_url: user.avatar_url,
//                 verified: user.verified,
//                 follower_count: user.follower_count,
//             })
//             .collect())
//     }

//     pub async fn search_posts(
//         &self,
//         query: &str,
//         limit: i64,
//     ) -> Result<Vec<PostSearchResult>, sqlx::Error> {
//         let posts = sqlx::query!(
//             r#"
//             SELECT p.id, p.user_id, u.username, p.content, p.like_count, p.repost_count, p.created_at
//             FROM posts p
//             JOIN users u ON p.user_id = u.id
//             WHERE p.content ILIKE $1
//             ORDER BY p.created_at DESC, p.like_count DESC
//             LIMIT $2
//             "#,
//             format!("%{}%", query),
//             limit
//         )
//         .fetch_all(&self.pool)
//         .await?;

//         Ok(posts
//             .into_iter()
//             .map(|post| PostSearchResult {
//                 id: post.id,
//                 user_id: post.user_id,
//                 username: post.username,
//                 content: post.content,
//                 like_count: post.like_count,
//                 repost_count: post.repost_count,
//                 created_at: post.created_at,
//             })
//             .collect())
//     }

//     pub async fn search_hashtags(
//         &self,
//         query: &str,
//         limit: i64,
//     ) -> Result<Vec<HashtagResult>, sqlx::Error> {
//         // This is a simplified hashtag search
//         // In a real implementation, you'd have a hashtags table
//         let hashtag_pattern = if query.starts_with('#') {
//             query.to_string()
//         } else {
//             format!("#{}", query)
//         };

//         let hashtags = sqlx::query!(
//             r#"
//             SELECT
//                 $1 as hashtag,
//                 COUNT(*) as post_count,
//                 COUNT(*) * 1.0 as trending_score
//             FROM posts
//             WHERE content ILIKE $2
//             GROUP BY content
//             LIMIT $3
//             "#,
//             hashtag_pattern,
//             format!("%{}%", hashtag_pattern),
//             limit
//         )
//         .fetch_all(&self.pool)
//         .await?;

//         Ok(hashtags
//             .into_iter()
//             .map(|row| HashtagResult {
//                 hashtag: row.hashtag,
//                 post_count: row.post_count.unwrap_or(0),
//                 trending_score: row.trending_score.unwrap_or(0.0),
//             })
//             .collect())
//     }
// }

// #[get("/search")]
// pub async fn search(
//     pool: web::Data<PgPool>,
//     query: web::Query<SearchQuery>,
//     req: actix_web::HttpRequest,
// ) -> impl Responder {
//     if query.q.trim().is_empty() {
//         return HttpResponse::BadRequest().json(serde_json::json!({
//             "error": "Search query cannot be empty"
//         }));
//     }

//     let search_service = SearchService::new(pool.get_ref().clone());
//     let limit = query.limit.unwrap_or(10).min(50);
//     let search_type = query.search_type.as_deref().unwrap_or("all");

//     let mut results = SearchResults {
//         users: Vec::new(),
//         posts: Vec::new(),
//         hashtags: Vec::new(),
//         total_results: 0,
//     };

//     match search_type {
//         "users" => match search_service.search_users(&query.q, limit).await {
//             Ok(users) => {
//                 results.total_results = users.len();
//                 results.users = users;
//             }
//             Err(e) => {
//                 log::error!("User search error: {}", e);
//                 return HttpResponse::InternalServerError().json(serde_json::json!({
//                     "error": "Failed to search users"
//                 }));
//             }
//         },
//         "posts" => match search_service.search_posts(&query.q, limit).await {
//             Ok(posts) => {
//                 results.total_results = posts.len();
//                 results.posts = posts;
//             }
//             Err(e) => {
//                 log::error!("Post search error: {}", e);
//                 return HttpResponse::InternalServerError().json(serde_json::json!({
//                     "error": "Failed to search posts"
//                 }));
//             }
//         },
//         "hashtags" => match search_service.search_hashtags(&query.q, limit).await {
//             Ok(hashtags) => {
//                 results.total_results = hashtags.len();
//                 results.hashtags = hashtags;
//             }
//             Err(e) => {
//                 log::error!("Hashtag search error: {}", e);
//                 return HttpResponse::InternalServerError().json(serde_json::json!({
//                     "error": "Failed to search hashtags"
//                 }));
//             }
//         },
//         "all" | _ => {
//             // Search all types with reduced limits
//             let per_type_limit = (limit / 3).max(3);

//             if let Ok(users) = search_service.search_users(&query.q, per_type_limit).await {
//                 results.users = users;
//             }

//             if let Ok(posts) = search_service.search_posts(&query.q, per_type_limit).await {
//                 results.posts = posts;
//             }

//             if let Ok(hashtags) = search_service
//                 .search_hashtags(&query.q, per_type_limit)
//                 .await
//             {
//                 results.hashtags = hashtags;
//             }

//             results.total_results =
//                 results.users.len() + results.posts.len() + results.hashtags.len();
//         }
//     }

//     HttpResponse::Ok().json(results)
// }

// #[get("/search/trending")]
// pub async fn get_trending_topics(pool: web::Data<PgPool>) -> impl Responder {
//     // Calculate trending hashtags based on recent activity
//     let trending_hashtags = sqlx::query!(
//         r#"
//         SELECT
//             REGEXP_REPLACE(content, '.*?(#\w+).*', '\1', 'g') as hashtag,
//             COUNT(*) as post_count,
//             COUNT(*) * (EXTRACT(EPOCH FROM NOW() - MIN(created_at)) / 3600.0) as trending_score
//         FROM posts
//         WHERE content ~ '#\w+'
//         AND created_at > NOW() - INTERVAL '24 hours'
//         GROUP BY hashtag
//         HAVING COUNT(*) > 2
//         ORDER BY trending_score DESC
//         LIMIT 20
//         "#
//     )
//     .fetch_all(pool.get_ref())
//     .await;

//     match trending_hashtags {
//         Ok(hashtags) => {
//             let trending_topics = TrendingTopics {
//                 hashtags: hashtags
//                     .into_iter()
//                     .map(|row| HashtagResult {
//                         hashtag: row.hashtag.unwrap_or_default(),
//                         post_count: row.post_count.unwrap_or(0),
//                         trending_score: row.trending_score.unwrap_or(0.0),
//                     })
//                     .collect(),
//                 updated_at: chrono::Utc::now(),
//             };

//             HttpResponse::Ok().json(trending_topics)
//         }
//         Err(e) => {
//             log::error!("Failed to fetch trending topics: {}", e);
//             HttpResponse::InternalServerError().json(serde_json::json!({
//                 "error": "Failed to fetch trending topics"
//             }))
//         }
//     }
// }

// #[get("/search/suggestions")]
// pub async fn get_search_suggestions(
//     pool: web::Data<PgPool>,
//     query: web::Query<HashMap<String, String>>,
// ) -> impl Responder {
//     let search_query = match query.get("q") {
//         Some(q) if !q.trim().is_empty() => q,
//         _ => {
//             return HttpResponse::BadRequest().json(serde_json::json!({
//                 "error": "Query parameter 'q' is required"
//             }));
//         }
//     };

//     // Get user suggestions
//     let user_suggestions = sqlx::query!(
//         "SELECT username FROM users WHERE username ILIKE $1 ORDER BY follower_count DESC LIMIT 5",
//         format!("{}%", search_query)
//     )
//     .fetch_all(pool.get_ref())
//     .await
//     .unwrap_or_default();

//     // Get hashtag suggestions (simplified)
//     let hashtag_suggestions = if search_query.starts_with('#') {
//         vec![search_query.to_string()]
//     } else {
//         vec![format!("#{}", search_query)]
//     };

//     let suggestions = serde_json::json!({
//         "users": user_suggestions.into_iter().map(|u| u.username).collect::<Vec<_>>(),
//         "hashtags": hashtag_suggestions,
//         "recent_searches": [] // You'd implement this with user search history
//     });

//     HttpResponse::Ok().json(suggestions)
// }

// pub fn configure(cfg: &mut web::ServiceConfig) {
//     cfg.service(search)
//         .service(get_trending_topics)
//         .service(get_search_suggestions);
// }
