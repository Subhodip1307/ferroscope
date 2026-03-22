// table create code
use ferroscope_server::hash_password;
use sqlx::PgPool;
use std::env;


pub async fn create_user_if_not_exist(pg_pool: &PgPool) -> Result<(), sqlx::Error> {
    let mut tx = pg_pool.begin().await?;

    let user_name = env::var("Username").unwrap_or_else(|_| "admin".to_string());
    let password = env::var("Password").unwrap_or_else(|_| "admin".to_string());

    sqlx::query(
        "
    insert into users (username,password_hash)
    select $1,$2 where NOT EXISTS (  SELECT 1 FROM users); 
    ",
    )
    .bind(user_name)
    .bind(hash_password(&password))
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(())
}
