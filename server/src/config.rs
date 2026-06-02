use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub port: u16,
}

// impl is used to define methods on a type
impl Config {
    // doesn't take self as an argument, so it's an associated function (like a static method)
    pub fn from_env() -> Self {
        let port = env::var("PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse()
            .expect("PORT must be a valid u16");
        Self { port }
    }
}