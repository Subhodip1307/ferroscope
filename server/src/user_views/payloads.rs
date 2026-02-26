use serde::Deserialize;

#[derive(Deserialize)]
pub (super) struct Login{
    pub username: String,
    pub password: String
}

#[derive(Deserialize)]
pub(super) struct IdQuery {
    pub node: i64,
}

#[derive(Deserialize)]
pub(super) struct ServiceQuery {
    // use to query the node and a specific service of it.
    pub node: i64,
    pub service_name: String,
}

