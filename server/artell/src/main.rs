extern crate openssl;
#[macro_use]
extern crate serde;

pub mod config;
pub mod res;
pub mod routes;
pub mod server;

pub use config::Config;
pub use res::*;

use artell_infra::pg::Postgres;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    openssl_probe::init_ssl_cert_env_vars();

    let db_url = get_env_var_or_panic("DATABASE_URL");
    let pg = Postgres::new(db_url);

    let port = get_env_var_u16_or_panic("PORT");

    server::bind(Config::new(pg, "artell".to_string()), ([0, 0, 0, 0], port)).await
}

fn get_env_var_or_panic(key: &'static str) -> String {
    std::env::var(key).unwrap()
}

fn get_env_var_u16_or_panic(key: &'static str) -> u16 {
    let s = get_env_var_or_panic(key);
    u16::from_str_radix(s.as_str(), 10).unwrap()
}
