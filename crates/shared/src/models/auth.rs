use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// JWT Claims structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    /// User ID
    pub sub: String,
    /// Username
    pub username: String,
    /// Is admin
    pub is_admin: bool,
    /// Issued at
    pub iat: i64,
    /// Expiration time
    pub exp: i64,
}

impl Claims {
    pub fn user_id(&self) -> Option<Uuid> {
        Uuid::parse_str(&self.sub).ok()
    }
}

/// Authenticated user context (extracted from JWT)
#[derive(Debug, Clone)]
pub struct AuthUser {
    pub id: Uuid,
    pub username: String,
    pub is_admin: bool,
}

impl From<Claims> for Option<AuthUser> {
    fn from(claims: Claims) -> Self {
        let id = Uuid::parse_str(&claims.sub).ok()?;
        Some(AuthUser {
            id,
            username: claims.username,
            is_admin: claims.is_admin,
        })
    }
}
