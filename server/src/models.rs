// table create code
use sqlx::PgPool;

pub async fn create_tables(pg_pool: &PgPool) -> Result<(), sqlx::Error> {
    // creating tables
    let mut tx = pg_pool.begin().await?;
    // nodes table
    sqlx::query(
        "
        CREATE TABLE IF NOT EXISTS nodes (
            id BIGSERIAL PRIMARY KEY,
            name VARCHAR(100) NOT NULL,
            created_time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            token VARCHAR(100) NOT NULL UNIQUE
        );
        ",
    )
    .execute(&mut *tx)
    .await?;

    sqlx::query(
        "
        CREATE TABLE IF NOT EXISTS cpu_stats (
            id BIGSERIAL PRIMARY KEY,
            date_time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            value DOUBLE PRECISION NOT NULL,
            node_id BIGINT NOT NULL,
            CONSTRAINT fk_cpu_nodes
                FOREIGN KEY (node_id)
                REFERENCES nodes(id)
                ON DELETE CASCADE
        );
        ",
    )
    .execute(&mut *tx)
    .await?;

    sqlx::query(
        "
        CREATE TABLE IF NOT EXISTS memory_metrics (
        id BIGSERIAL PRIMARY KEY,
        date_time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
        free VARCHAR(100)  NOT NULL,
        total VARCHAR(100) NOT NULL,
        node_id BIGINT NOT NULL UNIQUE,
        CONSTRAINT fk_memory_nodes
                FOREIGN KEY (node_id)
                REFERENCES nodes(id)
                ON DELETE CASCADE
    );
    ",
    )
    .execute(&mut *tx)
    .await?;

    sqlx::query(
        "
        CREATE TABLE IF NOT EXISTS service_monitor (
        id BIGSERIAL PRIMARY KEY,
        date_time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
        updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
        service_name VARCHAR(250)  NOT NULL,
        status VARCHAR(100) NOT NULL,
        error_msg VARCHAR(300),
        node_id BIGINT NOT NULL,
        CONSTRAINT fk_memory_nodes
                FOREIGN KEY (node_id)
                REFERENCES nodes(id)
                ON DELETE CASCADE,
        
        CONSTRAINT unique_service_per_node
        UNIQUE (node_id,service_name)
    );
    ",
    )
    .execute(&mut *tx)
    .await?;
    // status = Runing/Dead #make it bool
    // error_msg= error message if any
    // //creating User table
    sqlx::query(
        "
    CREATE TABLE IF NOT EXISTS users (
    id BIGSERIAL PRIMARY KEY,
    joined_date TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    username VARCHAR(250) NOT NULL UNIQUE,
    password_hash TEXT NOT NULL
    );
    ",
    )
    .execute(&mut *tx)
    .await?;
    // // Token Auth table
       sqlx::query(
        "
        CREATE TABLE IF NOT EXISTS auth_tokens (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL UNIQUE,
    token TEXT NOT NULL UNIQUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
   

    CONSTRAINT fk_auth_user
        FOREIGN KEY (user_id)
        REFERENCES users(id)
        ON DELETE CASCADE
    );
    ",
    )
    .execute(&mut *tx)
    .await?;
    

    // systeminfo
     sqlx::query(
        "
    CREATE TABLE IF NOT EXISTS sysinfo (
    id BIGSERIAL PRIMARY KEY,
    system_name VARCHAR(150),
    kernel_version VARCHAR(150),
    os_version VARCHAR(150),
    uptime BIGINT,
    cpu_threads SMALLINT,
    cpu_vendor VARCHAR(150),
    node_id BIGINT NOT NULL UNIQUE,
        CONSTRAINT fk_sysyteminfo_nodes
                FOREIGN KEY (node_id)
                REFERENCES nodes(id)
                ON DELETE CASCADE
    );
    ",
    )
    .execute(&mut *tx)
    .await?;



    
    tx.commit().await?;
    Ok(())
}
