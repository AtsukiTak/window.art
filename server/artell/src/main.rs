#[macro_use]
extern crate serde;

pub mod res;
pub mod routes;
pub mod server;

#[tokio::main]
async fn main() {
    let port = get_env_var_u16_or_panic("PORT");

    server::bind(([0, 0, 0, 0], port)).await
}

fn get_env_var_or_panic(key: &'static str) -> String {
    std::env::var(key).unwrap()
}

fn get_env_var_u16_or_panic(key: &'static str) -> u16 {
    let s = get_env_var_or_panic(key);
    u16::from_str_radix(s.as_str(), 10).unwrap()
}
