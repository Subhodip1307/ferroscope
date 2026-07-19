// check permission codes
use sqlx::PgPool;
use super::types::{PermissionData,Metrixs};
// will add cache later and add error handeling also

#[inline]
pub async fn user_is_admin(pool:&PgPool,user_id:i64)->bool{
    sqlx::query_scalar("SELECT is_admin FROM users WHERE id = $1")
    .bind(user_id).fetch_one(pool).await.unwrap()
}

#[inline]
pub async fn get_allowed_nodes_list(pool:&PgPool,user_id:i64)->PermissionData<i64>{    
    if user_is_admin(&pool, user_id).await{
        return PermissionData::<i64>::IsAdmin;
    };
    let data= sqlx::query_scalar::<_,i64>("SELECT node_id FROM user_node_access WHERE user_id = $1")
    .bind(user_id).fetch_all(pool).await.unwrap();
    PermissionData::<i64>::Data(data)
}

#[inline]
async fn check_user_full_power_on_node(pool:&PgPool,node_id:i64,user_id:i64)->bool{
     let data= sqlx::query_scalar::<_,bool>("SELECT is_full_access FROM user_node_access WHERE user_id = $1 AND node_id = $2")
    .bind(user_id)
    .bind(node_id)
    .fetch_optional(pool).await.unwrap();
    match data {
        Some(v)=>v,
        None=>false
    }
}




pub async fn check_permission_on_single_node(pool:&PgPool,node_id:i64,user_id:i64)->bool{
    if user_is_admin(&pool, user_id).await{
        true
    }else{
        let check=sqlx::query_scalar::<_,bool>
        ("SELECT EXISTS ( SELECT 1 FROM user_node_access WHERE user_id = $1 AND  node_id = $2 ) ")
        .bind(user_id)
        .bind(node_id)
        .fetch_optional(pool).await.unwrap();
        match check {
            Some(v)=>v,
            _=>false
        }
    }

}


async fn check_user_metrix_on_node(pool:&PgPool,node_id:i64,user_id:i64,metrix:Metrixs)->bool{
     let data= sqlx::query_scalar::<_,bool>(
        "SELECT EXISTS ( SELECT 1 FROM user_node_metric_access WHERE user_id = $1 AND node_id = $2 AND metric_name = $3 )")
    .bind(user_id)
    .bind(node_id)
    .bind(metrix.to_string())
    .fetch_optional(pool).await.unwrap();
    match data {
        Some(v)=>v,
        None=>false
    }
}



#[inline]
pub async fn check_node_metrix_permission(pool:&PgPool,node_id:i64,user_id:i64,metrix:Metrixs)->bool{
    // just need to check if that user have permission see that metrix of that node
    if user_is_admin(&pool, user_id).await || check_user_full_power_on_node(pool,node_id,user_id).await || check_user_metrix_on_node(pool,node_id,user_id,metrix).await {
        return true;
    };
    false
}


pub async fn check_node_allowed_services_with_node_id(pool:&PgPool,node_id:i64,user_id:i64)->PermissionData<i64>{
    // need to check if the user is admin then user have full access to a node or not then allowed list
    if user_is_admin(&pool, user_id).await || check_user_full_power_on_node(pool,node_id,user_id).await{
       PermissionData::<i64>::IsAdmin
    }else{
        let data= sqlx::query_scalar::<_,i64>("SELECT service_id FROM user_node_service_access s
            JOIN service_monitor sm  ON  sm.id = s.service_id
            WHERE s.user_id = $1 AND sm.node_id = $2")
    .bind(user_id)
    .bind(node_id)
    .fetch_all(pool).await.unwrap();
        PermissionData::<i64>::Data(data)
    }
}   
pub async fn check_node_allowed_services_with_node_id_service_name(pool:&PgPool,node_id:i64,user_id:i64,service_name:&str)->bool{
    // need to check if the user is admin then user have full access to a node or not then allowed list
    if user_is_admin(&pool, user_id).await || check_user_full_power_on_node(pool,node_id,user_id).await{
       true
    }else{
          let data= sqlx::query_scalar::<_,bool>(
            "SELECT EXISTS (SELECT 1 FROM user_node_service_access s
            JOIN service_monitor sm  ON  sm.id = s.service_id
            WHERE s.user_id = $1 AND sm.node_id = $2 AND sm.service_name=$3)"
          )
    .bind(user_id)
    .bind(node_id)
    .bind(service_name)
    .fetch_optional(pool).await.unwrap();
    match data {
        Some(v)=>v,
        None=>false
    }
}
}   