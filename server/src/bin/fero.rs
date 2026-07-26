use ferroscope_server::global::utils_functions::hash_password;
use sqlx::{PgPool, Pool, Postgres};
use std::env;

async fn get_pool() -> Pool<Postgres> {
    #[cfg(not(debug_assertions))]
    let pg_pool = PgPool::connect(&env::var("PSQL_URL").unwrap_or_default())
        .await
        .expect("Failed to connect to database. Is PSQL_URL set correctly?");

    #[cfg(debug_assertions)]
    let pg_pool = PgPool::connect("postgres://myuser:mypassword@127.0.0.1:5432/mydatabase")
        .await
        .expect("Failed to connect to local database. Is Postgres running on 127.0.0.1:5432?");
    pg_pool
}

fn print_usage(program: &str) {
    println!("Usage:");
    println!("  {program} createuser <username> <password>       Create a new user");
    println!("  {program} createsuperuser <username> <password>       Create a new super-user");
    println!(
        "  {program} changepassword <username> <password>   Change an existing user's password"
    );
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        println!("Error: no command given.\n");
        print_usage(&args[0]);
        return;
    }

    match args[1].as_str() {
        "changepassword" => {
            if args.len() < 4 {
                println!("Error: 'changepassword' needs a username and a password.\n");
                print_usage(&args[0]);
                return;
            }
            change_password(&args[2], &args[3]).await;
        }
        "createuser" => {
            if args.len() < 4 {
                println!("Error: 'createuser' needs a username and a password.\n");
                print_usage(&args[0]);
                return;
            }
            create_user(&args[2], &args[3], false).await;
        }
        "createsuperuser" => {
            if args.len() < 4 {
                println!("Error: 'createuser' needs a username and a password.\n");
                print_usage(&args[0]);
                return;
            }
            create_superuser(&args[2], &args[3]).await;
        }

        other => {
            println!("Error: unknown command '{other}'.\n");
            print_usage(&args[0]);
        }
    }
}

async fn change_password(user_name: &str, password: &str) {
    println!("Changing password for user '{user_name}'...");
    let pg_pool = get_pool().await;

    let query_status = sqlx::query("UPDATE users SET password_hash=$2 WHERE username=$1")
        .bind(user_name)
        .bind(hash_password(password))
        .execute(&pg_pool)
        .await;

    match query_status {
        Ok(result) => {
            if result.rows_affected() == 0 {
                println!("No user named '{user_name}' was found. Password not changed.");
            } else {
                println!("Password changed successfully for '{user_name}'.");
            }
        }
        Err(e) => println!("Failed to change password: {e}"),
    }
}

async fn create_user(user_name: &str, password: &str, is_super_user: bool) {
    println!("Creating new user '{user_name}'...");
    let pg_pool = get_pool().await;

    let query_status =
        sqlx::query("insert into users (username,password_hash,is_admin) values ($1,$2,$3)")
            .bind(user_name)
            .bind(hash_password(password))
            .bind(is_super_user)
            .execute(&pg_pool)
            .await;

    match query_status {
        Ok(_) => println!("User '{user_name}' created successfully."),
        Err(e) => println!("Failed to create user: {e}"),
    }
}

async fn create_superuser(user_name: &str, password: &str) {
    create_user(user_name, password, true).await;
}
