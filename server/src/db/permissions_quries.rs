// where will be written all types of quries related to permission model
async fn bulk_insert_user_node_access(user_id:i64,nodes_id:Vec<i64>,pg_connection:&mut sqlx::Transaction<'_, sqlx::Postgres>)
{
// TODO: will use 'QueryBuilder' for this later
 for i in nodes_id {
        sqlx::query(
            "INSERT INTO user_node_access (user_id, node_id)
             VALUES ($1, $2)"
        )
        .bind(user_id)
        .bind(i)
        .execute(&mut **pg_connection)
        .await
        .unwrap();
    }
}