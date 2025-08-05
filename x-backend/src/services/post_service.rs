use actix_web::{delete, get, post, web, HttpMessage, HttpResponse, Responder};
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

use crate::models::media::Media;
use crate::models::post::{CreatePost, Post, PostQuery, PostWithDetails};
use crate::models::user::UserPublic;

#[post("/posts")]
pub async fn create_post(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
    form: web::Json<CreatePost>,
) -> impl Responder {
    if let Err(errors) = form.validate() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Validation failed",
            "details": errors
        }));
    }

    let user_id = match req.extensions().get::<Uuid>() {
        Some(id) => *id,
        None => {
            return HttpResponse::Unauthorized().json(serde_json::json!({
                "error": "Unauthorized"
            }));
        }
    };

    let mut tx = match pool.begin().await {
        Ok(tx) => tx,
        Err(e) => {
            log::error!("Transaction begin error: {}", e);
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Internal server error"
            }));
        }
    };

    // Create post
    let post_result = sqlx::query_as::<_, Post>(
        r#"
    INSERT INTO posts (user_id, content, reply_to_id, quote_post_id)
    VALUES ($1, $2, $3, $4)
    RETURNING *
    "#,
    )
    .bind(user_id)
    .bind(&form.content)
    .bind(form.reply_to_id)
    .bind(form.quote_post_id)
    .fetch_one(&mut *tx)
    .await;

    let post = match post_result {
        Ok(post) => post,
        Err(e) => {
            log::error!("Post creation error: {}", e);
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to create post"
            }));
        }
    };

    // Link media if provided
    if let Some(media_ids) = &form.media_ids {
        for media_id in media_ids {
            if let Err(e) = sqlx::query!(
                "UPDATE media SET post_id = $1 WHERE id = $2 AND user_id = $3",
                post.id,
                media_id,
                user_id
            )
            .execute(&mut *tx)
            .await
            {
                log::error!("Media linking error: {}", e);
                return HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": "Failed to link media"
                }));
            }
        }
    }

    // Update user post count
    if let Err(e) = sqlx::query!(
        "UPDATE users SET post_count = post_count + 1 WHERE id = $1",
        user_id
    )
    .execute(&mut *tx)
    .await
    {
        log::error!("User post count update error: {}", e);
    }

    // Update reply count if this is a reply
    if let Some(reply_to_id) = form.reply_to_id {
        if let Err(e) = sqlx::query!(
            "UPDATE posts SET reply_count = reply_count + 1 WHERE id = $1",
            reply_to_id
        )
        .execute(&mut *tx)
        .await
        {
            log::error!("Reply count update error: {}", e);
        }
    }

    if let Err(e) = tx.commit().await {
        log::error!("Transaction commit error: {}", e);
        return HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "Internal server error"
        }));
    }

    HttpResponse::Created().json(post)
}

#[get("/posts/{post_id}")]
pub async fn get_post(
    pool: web::Data<PgPool>,
    post_id: web::Path<Uuid>,
    req: actix_web::HttpRequest,
) -> impl Responder {
    let current_user_id = req.extensions().get::<Uuid>().copied();

    let post_result = get_post_with_details(pool.get_ref(), *post_id, current_user_id).await;

    match post_result {
        Ok(Some(post)) => HttpResponse::Ok().json(post),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Post not found"
        })),
        Err(e) => {
            log::error!("Database error: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Internal server error"
            }))
        }
    }
}

#[get("/users/{user_id}/posts")]
pub async fn get_user_posts(
    pool: web::Data<PgPool>,
    user_id: web::Path<Uuid>,
    query: web::Query<PostQuery>,
    req: actix_web::HttpRequest,
) -> impl Responder {
    let current_user_id = req.extensions().get::<Uuid>().copied();
    let limit = query.limit.unwrap_or(20).min(100);
    let offset = query.offset.unwrap_or(0);

    let posts_result = sqlx::query_as::<_, Post>(
        "SELECT * FROM posts WHERE user_id = $1 ORDER BY created_at DESC LIMIT $2 OFFSET $3",
    )
    .bind(*user_id)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool.get_ref())
    .await;

    match posts_result {
        Ok(posts) => {
            let mut posts_with_details = Vec::new();
            for post in posts {
                if let Ok(Some(post_detail)) =
                    get_post_with_details(pool.get_ref(), post.id, current_user_id).await
                {
                    posts_with_details.push(post_detail);
                }
            }
            HttpResponse::Ok().json(posts_with_details)
        }
        Err(e) => {
            log::error!("Database error: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Internal server error"
            }))
        }
    }
}

#[delete("/posts/{post_id}")]
pub async fn delete_post(
    pool: web::Data<PgPool>,
    post_id: web::Path<Uuid>,
    req: actix_web::HttpRequest,
) -> impl Responder {
    let user_id = match req.extensions().get::<Uuid>() {
        Some(id) => *id,
        None => {
            return HttpResponse::Unauthorized().json(serde_json::json!({
                "error": "Unauthorized"
            }));
        }
    };

    let result = sqlx::query!(
        "DELETE FROM posts WHERE id = $1 AND user_id = $2",
        *post_id,
        user_id
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(rows) => {
            if rows.rows_affected() > 0 {
                // Update user post count
                if let Err(e) = sqlx::query!(
                    "UPDATE users SET post_count = post_count - 1 WHERE id = $1",
                    user_id
                )
                .execute(pool.get_ref())
                .await
                {
                    log::error!("User post count update error: {}", e);
                }

                HttpResponse::Ok().json(serde_json::json!({
                    "message": "Post deleted successfully"
                }))
            } else {
                HttpResponse::NotFound().json(serde_json::json!({
                    "error": "Post not found or not authorized"
                }))
            }
        }
        Err(e) => {
            log::error!("Database error: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Internal server error"
            }))
        }
    }
}

async fn get_post_with_details(
    pool: &PgPool,
    post_id: Uuid,
    current_user_id: Option<Uuid>,
) -> Result<Option<PostWithDetails>, sqlx::Error> {
    // Get the main post
    let post = sqlx::query_as::<_, Post>("SELECT * FROM posts WHERE id = $1")
        .bind(post_id)
        .fetch_optional(pool)
        .await?;

    let post = match post {
        Some(p) => p,
        None => return Ok(None),
    };

    // Get author information
    let author =
        sqlx::query_as::<_, crate::models::user::User>("SELECT * FROM users WHERE id = $1")
            .bind(post.user_id)
            .fetch_one(pool)
            .await?;

    // Get media
    let media = sqlx::query_as::<_, Media>("SELECT * FROM media WHERE post_id = $1")
        .bind(post.id)
        .fetch_all(pool)
        .await?;

    // Check if current user has interacted with this post
    let (is_liked, is_reposted, is_bookmarked) = if let Some(user_id) = current_user_id {
        let like_exists = sqlx::query!(
            "SELECT id FROM likes WHERE user_id = $1 AND post_id = $2",
            user_id,
            post.id
        )
        .fetch_optional(pool)
        .await?
        .is_some();

        let repost_exists = sqlx::query!(
            "SELECT id FROM reposts WHERE user_id = $1 AND post_id = $2",
            user_id,
            post.id
        )
        .fetch_optional(pool)
        .await?
        .is_some();

        let bookmark_exists = sqlx::query!(
            "SELECT id FROM bookmarks WHERE user_id = $1 AND post_id = $2",
            user_id,
            post.id
        )
        .fetch_optional(pool)
        .await?
        .is_some();

        (like_exists, repost_exists, bookmark_exists)
    } else {
        (false, false, false)
    };

    // Get reply_to post if exists (recursive, but limited depth)
    let reply_to = if let Some(reply_to_id) = post.reply_to_id {
        Box::pin(get_post_with_details(pool, reply_to_id, current_user_id))
            .await?
            .map(Box::new)
    } else {
        None
    };

    // Get quote post if exists
    let quote_post = if let Some(quote_post_id) = post.quote_post_id {
        Box::pin(get_post_with_details(pool, quote_post_id, current_user_id))
            .await?
            .map(Box::new)
    } else {
        None
    };

    Ok(Some(PostWithDetails {
        post,
        author: UserPublic::from(author),
        media,
        is_liked,
        is_reposted,
        is_bookmarked,
        reply_to,
        quote_post,
    }))
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(create_post)
        .service(get_post)
        .service(get_user_posts)
        .service(delete_post);
}
