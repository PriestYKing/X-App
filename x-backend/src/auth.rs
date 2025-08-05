use crate::utils::jwt::JwtUtil;
use actix_web::{dev::ServiceRequest, Error, HttpMessage, HttpRequest, HttpResponse};
use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use actix_web_httpauth::extractors::AuthenticationError;
use serde_json::json;
use uuid::Uuid;

/// Extract user ID from JWT token in the request
pub fn extract_user_id(req: &ServiceRequest) -> Option<Uuid> {
    req.extensions().get::<Uuid>().copied()
}

/// Extract username from JWT token in the request
pub fn extract_username(req: &ServiceRequest) -> Option<String> {
    req.extensions().get::<String>().cloned()
}

/// Validator function for JWT tokens using actix-web-httpauth
pub async fn jwt_validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    // Get jwt_util without moving req
    let jwt_util = match req.app_data::<actix_web::web::Data<JwtUtil>>() {
        Some(util) => util,
        None => {
            let config = Config::default().realm("Restricted area");
            return Err((AuthenticationError::from(config).into(), req));
        }
    };

    match jwt_util.verify_token(credentials.token()) {
        Ok(token_data) => {
            if let Ok(user_id) = Uuid::parse_str(&token_data.claims.sub) {
                req.extensions_mut().insert(user_id);
                req.extensions_mut()
                    .insert(token_data.claims.username.clone());
                Ok(req)
            } else {
                let config = Config::default().realm("Restricted area");
                Err((AuthenticationError::from(config).into(), req))
            }
        }
        Err(_) => {
            let config = Config::default().realm("Restricted area");
            Err((AuthenticationError::from(config).into(), req))
        }
    }
}

/// Optional JWT validator that doesn't fail if no token is provided
pub async fn optional_jwt_validator(
    req: ServiceRequest,
    credentials: Option<BearerAuth>,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    if let Some(credentials) = credentials {
        jwt_validator(req, credentials).await
    } else {
        // No credentials provided, but that's okay for optional auth
        Ok(req)
    }
}

// ... rest of your auth.rs helper functions remain the same
