use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,  // Subject (agent ID)
    pub exp: usize,   // Expiration time
    pub iat: usize,   // Issued at
}

pub struct TokenValidator {
    secret: String,
}

impl TokenValidator {
    pub fn new(secret: String) -> Self {
        Self { secret }
    }

    pub fn generate_token(&self, agent_id: &str, expiration_hours: u64) -> Result<String, jsonwebtoken::errors::Error> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;

        let claims = Claims {
            sub: agent_id.to_string(),
            exp: now + (expiration_hours as usize * 3600),
            iat: now,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
    }

    pub fn validate_token(&self, token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &Validation::default(),
        )?;

        Ok(token_data.claims)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_generation_and_validation() {
        let validator = TokenValidator::new("test-secret".to_string());
        
        let token = validator.generate_token("test-agent", 24).unwrap();
        let claims = validator.validate_token(&token).unwrap();
        
        assert_eq!(claims.sub, "test-agent");
    }
}
