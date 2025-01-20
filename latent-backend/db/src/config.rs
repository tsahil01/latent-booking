use std::env;

pub struct Config {
    pub db_url: String
}

impl Default for Config {
    fn default() -> Self {
        let db_url = env::var("DB_URL").unwrap();
        println!("{}", db_url);
        Self {
            db_url
        }
    }
}