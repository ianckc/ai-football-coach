mod common;

/// Acceptance slice (#59): Add a Postgres migration for session tokens
/// (e.g. a `sessions` table keyed by opaque `token`, linked to `accounts.id`,
/// with a created-at timestamp).
#[tokio::test]
async fn sessions_migration_creates_table_keyed_by_token_linked_to_accounts() {
    let pool = common::test_pool().await;

    let table_exists: bool = sqlx::query_scalar(
        "SELECT EXISTS (
            SELECT 1
            FROM information_schema.tables
            WHERE table_schema = 'public'
              AND table_name = 'sessions'
        )",
    )
    .fetch_one(&pool)
    .await
    .expect("query whether public.sessions exists");

    assert!(
        table_exists,
        "Postgres migration must create a public.sessions table for session tokens"
    );

    let column_names: Vec<String> = sqlx::query_scalar(
        "SELECT column_name
         FROM information_schema.columns
         WHERE table_schema = 'public'
           AND table_name = 'sessions'",
    )
    .fetch_all(&pool)
    .await
    .expect("query sessions columns");

    let has_token = column_names.iter().any(|c| c == "token");
    assert!(
        has_token,
        "sessions must include an opaque token column named token; columns={column_names:?}"
    );

    let has_created_at = column_names.iter().any(|c| c == "created_at");
    assert!(
        has_created_at,
        "sessions must include a created-at timestamp column named created_at; columns={column_names:?}"
    );

    let account_link_columns: Vec<String> = sqlx::query_scalar(
        "SELECT kcu.column_name
         FROM information_schema.table_constraints AS tc
         JOIN information_schema.key_column_usage AS kcu
           ON tc.constraint_name = kcu.constraint_name
          AND tc.table_schema = kcu.table_schema
         JOIN information_schema.constraint_column_usage AS ccu
           ON ccu.constraint_name = tc.constraint_name
          AND ccu.table_schema = tc.table_schema
         WHERE tc.constraint_type = 'FOREIGN KEY'
           AND tc.table_schema = 'public'
           AND tc.table_name = 'sessions'
           AND ccu.table_name = 'accounts'
           AND ccu.column_name = 'id'",
    )
    .fetch_all(&pool)
    .await
    .expect("query sessions → accounts.id foreign keys");

    assert!(
        !account_link_columns.is_empty(),
        "sessions must be linked to accounts.id via a foreign key"
    );

    let token_is_key: bool = sqlx::query_scalar(
        "SELECT EXISTS (
            SELECT 1
            FROM information_schema.table_constraints AS tc
            JOIN information_schema.key_column_usage AS kcu
              ON tc.constraint_name = kcu.constraint_name
             AND tc.table_schema = kcu.table_schema
            WHERE tc.table_schema = 'public'
              AND tc.table_name = 'sessions'
              AND kcu.column_name = 'token'
              AND tc.constraint_type IN ('PRIMARY KEY', 'UNIQUE')
        )",
    )
    .fetch_one(&pool)
    .await
    .expect("query whether token is a primary/unique key");

    assert!(
        token_is_key,
        "sessions.token must be a PRIMARY KEY or UNIQUE key so tokens can be looked up"
    );
}
