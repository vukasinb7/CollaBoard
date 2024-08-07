use std::env;
use dotenv::dotenv;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref TOKEN:String=set_env("TOKEN".to_string());
    pub static ref DATABASE_URL:String=set_env("DATABASE_URL".to_string());
    pub static ref SMTP_USERNAME:String=set_env("SMTP_USERNAME".to_string());
    pub static ref SMTP_PASSWORD:String=set_env("SMTP_PASSWORD".to_string());
}
fn set_env(name: String) -> String {
    dotenv().ok();
    env::var(name).unwrap()
}
