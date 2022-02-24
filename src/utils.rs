use std::{collections::HashMap, env};
use dotenv::dotenv;
use mysql::*;

/// returns .env data as hashmap
pub fn get_env_vars() -> HashMap<String, String> {
    dotenv().ok();
    let mut env_vars = HashMap::new();
    for (key, value) in env::vars() {
        env_vars.insert(key, value);
    }
    env_vars
}

/// returns a new OptsBuilder instance
pub fn build_connection_options(
    user: String,
    password: String,
    host: String,
    db_name: String,
    port: String,
    socket: String
) -> OptsBuilder {
    let mut opts_hash = HashMap::new();
    opts_hash.insert("user".to_owned(), user);
    opts_hash.insert("password".to_owned(), password);
    opts_hash.insert("host".to_owned(), host);
    opts_hash.insert("db_name".to_owned(), db_name);
    opts_hash.insert("port".to_owned(), port);
    opts_hash.insert("socket".to_owned(), socket);
    OptsBuilder::new().from_hash_map(&opts_hash)
        .expect("UrlError when building options: OptBuilder")
}

/// returns a new OptsBuilder instance by using env vars as arguments
pub fn build_connection_options_from_env() -> OptsBuilder {
    let env_vars = get_env_vars();
    build_connection_options(
        env_vars.get("DATABASE_USER").unwrap().to_owned(),
        env_vars.get("DATABASE_PASSWORD").unwrap().to_owned(),
        env_vars.get("DATABASE_HOST").unwrap().to_owned(),
        env_vars.get("DATABASE_NAME").unwrap().to_owned(),
        env_vars.get("DATABASE_PORT").unwrap().to_owned(),
        env_vars.get("DATABASE_SOCKET").unwrap().to_owned(),
    )
}
