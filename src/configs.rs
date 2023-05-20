use figment::{providers::Env, Figment};
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct Configs {
    //super secret token
    pub sst: String,
    pub database_url: String,
    pub bind_url: String
}

impl Configs {
    pub fn new() -> Self {
        Figment::new()
            .merge(Env::prefixed("HIYORI_"))
            .extract()
            .unwrap()
    }
}