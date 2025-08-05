use crate::utils::jwt::JwtUtil;
use actix_web::{
    body::{EitherBody, MessageBody},
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage, HttpResponse,
};
use futures_util::future::LocalBoxFuture;
use std::{
    future::{ready, Ready},
    rc::Rc,
};
use uuid::Uuid;

pub struct AuthMiddleware {
    jwt_util: Rc<JwtUtil>,
}

impl AuthMiddleware {
    pub fn new(jwt_util: JwtUtil) -> Self {
        Self {
            jwt_util: Rc::new(jwt_util),
        }
    }
}

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddlewareService {
            service,
            jwt_util: self.jwt_util.clone(),
        }))
    }
}

pub struct AuthMiddlewareService<S> {
    service: S,
    jwt_util: Rc<JwtUtil>,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<EitherBody<B>>; // This must match Transform::Response
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let jwt_util = self.jwt_util.clone();

        // Skip auth for certain routes
        let path = req.path();
        if path.starts_with("/auth/") || path == "/health" || path == "/ws" {
            let fut = self.service.call(req);
            return Box::pin(async move {
                let res = fut.await?;
                Ok(res.map_into_left_body())
            });
        }

        // Extract token from Authorization header
        let token = req
            .headers()
            .get("Authorization")
            .and_then(|header| header.to_str().ok())
            .and_then(|header| {
                if header.starts_with("Bearer ") {
                    Some(&header[7..])
                } else {
                    None
                }
            });

        if let Some(token) = token {
            match jwt_util.verify_token(token) {
                Ok(token_data) => {
                    if let Ok(user_id) = Uuid::parse_str(&token_data.claims.sub) {
                        req.extensions_mut().insert(user_id);
                        req.extensions_mut().insert(token_data.claims.username);

                        let fut = self.service.call(req);
                        return Box::pin(async move {
                            let res = fut.await?;
                            Ok(res.map_into_left_body())
                        });
                    }
                }
                Err(_) => {}
            }
        }

        // Authentication failed - return error response
        Box::pin(async move {
            let (req, _) = req.into_parts();
            let response = HttpResponse::Unauthorized()
                .json(serde_json::json!({"error": "Authentication required"}));
            Ok(ServiceResponse::new(req, response).map_into_right_body())
        })
    }
}
