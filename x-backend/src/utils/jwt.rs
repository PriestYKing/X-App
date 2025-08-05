use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user_id
    pub username: String,
    pub exp: usize,
    pub iat: usize,
}

#[derive(Clone)]
pub struct JwtUtil {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}

impl JwtUtil {
    pub fn new(secret: &str) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(secret.as_ref()),
            decoding_key: DecodingKey::from_secret(secret.as_ref()),
        }
    }

    pub fn generate_token(
        &self,
        user_id: Uuid,
        username: &str,
    ) -> Result<String, jsonwebtoken::errors::Error> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;

        let claims = Claims {
            sub: user_id.to_string(),
            username: username.to_string(),
            exp: now + 24 * 60 * 60, // 24 hours
            iat: now,
        };

        encode(&Header::default(), &claims, &self.encoding_key)
    }

    pub fn verify_token(
        &self,
        token: &str,
    ) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
        decode::<Claims>(token, &self.decoding_key, &Validation::default())
    }
}
