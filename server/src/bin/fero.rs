use std::env;
use sqlx::{PgPool, Pool, Postgres};
use ferroscope_server::hash_password;



async fn get_pool()->Pool<Postgres>{
         #[cfg(not(debug_assertions))]
        let pg_pool = PgPool::connect(&env::var("PSQL_URL").unwrap_or_default())
            .await
            .unwrap();

        #[cfg(debug_assertions)]
        let pg_pool = PgPool::connect("postgres://myuser:mypassword@127.0.0.1:5432/mydatabase")
            .await
            .unwrap();
        pg_pool
}



#[tokio::main]
async fn main(){
    let args: Vec<String> = env::args().collect();
    if args.len() <=1{
        println!("Please Give Correct Arguments");
        return;
    }
      
        if args[1]==  "changepassword" && args.len() >=4 {
            change_password(&args[2],&args[3]).await;
        }
        else if args[1]==  "createuser" && args.len() >=4 {
            create_user(&args[2],&args[3]).await;
        }
        else {
            println!("wrong input \n options are:\nchangepassword\ncreateuser ")
        }
    }

    async fn change_password(user_name:&str,password:&str){
        println!("Changing NewUser with Username: {} and Password: {}",user_name,password);
        let pg_pool = get_pool().await;

        let query_status = sqlx::query("UPDATE users SET password_hash=$2 WHERE username=$1")
        .bind(user_name)
        .bind(hash_password(password))
        .execute(&pg_pool)
        .await;

    match query_status {
        Ok(_) => println!("Password Changed"),
        Err(e) => println!("something went wrong {}",e)
    }

    }


async fn create_user(user_name:&str,password:&str){
    println!("Createing New User {} with password {}",user_name,password);
    let pg_pool = get_pool().await;

    let query_status = sqlx::query("insert into users (username,password_hash) values ($1,$2)")
        .bind(user_name)
        .bind(hash_password(password))
        .execute(&pg_pool)
        .await;

    match query_status {
        Ok(_) => println!("User Created"),
        Err(e) => println!("something went wrong {}",e)
    }
    
}