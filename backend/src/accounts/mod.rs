use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use serde_json::Value;
use sqlx::PgPool;
use uuid::Uuid;

pub enum CreateAccountError {
    InvalidInput,
    Conflict,
    Internal,
}

pub enum LoginError {
    Unauthorized,
    Internal,
}

pub struct CreatedAccount {
    pub id: Uuid,
    pub email: String,
}

pub struct LoginSession {
    pub id: Uuid,
    pub email: String,
    pub token: String,
}

fn is_valid_email(email: &str) -> bool {
    let Some((local, domain)) = email.split_once('@') else {
        return false;
    };
    !local.is_empty() && !domain.is_empty() && domain.contains('.')
}

/// String fields only; wrong JSON types, null, and missing are treated as absent.
fn json_string_field(value: Option<&Value>) -> Option<&str> {
    match value {
        Some(Value::String(s)) => Some(s.as_str()),
        _ => None,
    }
}

fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
}

fn verify_password(password: &str, password_hash: &str) -> bool {
    let Ok(parsed) = PasswordHash::new(password_hash) else {
        return false;
    };
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed)
        .is_ok()
}

/// Validate, hash, and insert a new account.
///
/// `email` and `password` are raw JSON values from the request body (or absent).
pub async fn create(
    pool: &PgPool,
    email: Option<&Value>,
    password: Option<&Value>,
) -> Result<CreatedAccount, CreateAccountError> {
    let email = json_string_field(email)
        .unwrap_or("")
        .trim()
        .to_ascii_lowercase();
    let password = json_string_field(password).unwrap_or("").trim();

    if email.is_empty() || !is_valid_email(&email) || password.chars().count() < 8 {
        return Err(CreateAccountError::InvalidInput);
    }

    let password = password.to_owned();
    let hash_result = tokio::task::spawn_blocking(move || hash_password(&password)).await;
    let Ok(Ok(password_hash)) = hash_result else {
        return Err(CreateAccountError::Internal);
    };

    let id = Uuid::new_v4();
    let inserted = sqlx::query_scalar::<_, Uuid>(
        r"
        INSERT INTO accounts (id, email, password_hash)
        VALUES ($1, $2, $3)
        ON CONFLICT (email) DO NOTHING
        RETURNING id
        ",
    )
    .bind(id)
    .bind(&email)
    .bind(&password_hash)
    .fetch_optional(pool)
    .await;

    match inserted {
        Ok(Some(created_id)) => Ok(CreatedAccount {
            id: created_id,
            email,
        }),
        Ok(None) => Err(CreateAccountError::Conflict),
        Err(_) => Err(CreateAccountError::Internal),
    }
}

/// Look up an account by email and verify the password against the stored argon2 hash.
pub async fn login(
    pool: &PgPool,
    email: &str,
    password: &str,
) -> Result<LoginSession, LoginError> {
    let email = email.trim().to_ascii_lowercase();

    let row = sqlx::query_as::<_, (Uuid, String, String)>(
        r"
        SELECT id, email, password_hash
        FROM accounts
        WHERE email = $1
        ",
    )
    .bind(&email)
    .fetch_optional(pool)
    .await
    .map_err(|_| LoginError::Internal)?;

    let Some((id, email, password_hash)) = row else {
        return Err(LoginError::Unauthorized);
    };

    let password = password.to_owned();
    let verified = tokio::task::spawn_blocking(move || verify_password(&password, &password_hash))
        .await
        .map_err(|_| LoginError::Internal)?;

    if !verified {
        return Err(LoginError::Unauthorized);
    }

    Ok(LoginSession {
        id,
        email,
        token: Uuid::new_v4().to_string(),
    })
}
