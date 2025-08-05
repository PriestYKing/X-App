use actix_web::{get, post, web, HttpMessage, HttpResponse, Responder};
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

use crate::models::user::{CreateUser, LoginUser, User, UserPublic};
use crate::utils::{jwt::JwtUtil, password};

#[post("/auth/register")]
pub async fn register(
    pool: web::Data<PgPool>,
    jwt_util: web::Data<JwtUtil>,
    form: web::Json<CreateUser>,
) -> impl Responder {
    if let Err(errors) = form.validate() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Validation failed",
            "details": errors
        }));
    }

    // Check if user already exists
    let existing_user = sqlx::query!(
        "SELECT id FROM users WHERE email = $1 OR username = $2",
        form.email,
        form.username
    )
    .fetch_optional(pool.get_ref())
    .await;

    match existing_user {
        Ok(Some(_)) => {
            return HttpResponse::Conflict().json(serde_json::json!({
                "error": "User already exists"
            }));
        }
        Err(e) => {
            log::error!("Database error: {}", e);
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Internal server error"
            }));
        }
        _ => {}
    }

    // Hash password
    let password_hash = match password::hash_password(&form.password) {
        Ok(hash) => hash,
        Err(e) => {
            log::error!("Password hashing error: {}", e);
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Internal server error"
            }));
        }
    };

    // Insert user
    let user_result = sqlx::query_as::<_, User>(
        r#"
    INSERT INTO users (username, email, password_hash, display_name)
    VALUES ($1, $2, $3, $4)
    RETURNING *
    "#,
    )
    .bind(&form.username)
    .bind(&form.email)
    .bind(&password_hash)
    .bind(&form.username)
    .fetch_one(pool.get_ref())
    .await;

    match user_result {
        Ok(user) => {
            let token = match jwt_util.generate_token(user.id, &user.username) {
                Ok(token) => token,
                Err(e) => {
                    log::error!("JWT generation error: {}", e);
                    return HttpResponse::InternalServerError().json(serde_json::json!({
                        "error": "Internal server error"
                    }));
                }
            };

            HttpResponse::Created().json(serde_json::json!({
                "user": UserPublic::from(user),
                "token": token
            }))
        }
        Err(e) => {
            log::error!("Database error: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to create user"
            }))
        }
    }
}

#[post("/auth/login")]
pub async fn login(
    pool: web::Data<PgPool>,
    jwt_util: web::Data<JwtUtil>,
    form: web::Json<LoginUser>,
) -> impl Responder {
    let user_result = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
        .bind(&form.email)
        .fetch_optional(pool.get_ref())
        .await;

    match user_result {
        Ok(Some(user)) => match password::verify_password(&form.password, &user.password_hash) {
            Ok(true) => {
                let token = match jwt_util.generate_token(user.id, &user.username) {
                    Ok(token) => token,
                    Err(e) => {
                        log::error!("JWT generation error: {}", e);
                        return HttpResponse::InternalServerError().json(serde_json::json!({
                            "error": "Internal server error"
                        }));
                    }
                };

                HttpResponse::Ok().json(serde_json::json!({
                    "user": UserPublic::from(user),
                    "token": token
                }))
            }
            Ok(false) => HttpResponse::Unauthorized().json(serde_json::json!({
                "error": "Invalid credentials"
            })),
            Err(e) => {
                log::error!("Password verification error: {}", e);
                HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": "Internal server error"
                }))
            }
        },
        Ok(None) => HttpResponse::Unauthorized().json(serde_json::json!({
            "error": "Invalid credentials"
        })),
        Err(e) => {
            log::error!("Database error: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Internal server error"
            }))
        }
    }
}

#[get("/users/{user_id}")]
pub async fn get_user(pool: web::Data<PgPool>, user_id: web::Path<Uuid>) -> impl Responder {
    let user_result = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(*user_id)
        .fetch_optional(pool.get_ref())
        .await;

    match user_result {
        Ok(Some(user)) => HttpResponse::Ok().json(UserPublic::from(user)),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "User not found"
        })),
        Err(e) => {
            log::error!("Database error: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Internal server error"
            }))
        }
    }
}

#[get("/auth/me")]
pub async fn get_current_user(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
) -> impl Responder {
    if let Some(user_id) = req.extensions().get::<Uuid>() {
        let user_result = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
            .bind(*user_id)
            .fetch_optional(pool.get_ref())
            .await;

        match user_result {
            Ok(Some(user)) => HttpResponse::Ok().json(UserPublic::from(user)),
            Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
                "error": "User not found"
            })),
            Err(e) => {
                log::error!("Database error: {}", e);
                HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": "Internal server error"
                }))
            }
        }
    } else {
        HttpResponse::Unauthorized().json(serde_json::json!({
            "error": "Unauthorized"
        }))
    }
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(register)
        .service(login)
        .service(get_user)
        .service(get_current_user);
}
